// Adapted from https://github.com/dtolnay/prettyplease/blob/0.2.15/src/algorithm.rs.

#![allow(dead_code)]

use std::borrow::Cow;
use std::cmp;
use std::collections::VecDeque;
use std::iter;
use std::ops::Index;
use std::ops::IndexMut;
use std::option;

const MARGIN: isize = 89;
const MIN_SPACE: isize = 60;
const SIZE_INFINITY: isize = 0xffff;

#[derive(Clone, Copy, PartialEq)]
pub enum Breaks {
    Consistent,
    Inconsistent,
}

#[derive(Clone, Copy, Default)]
pub struct BreakToken {
    pub offset: isize,
    pub blank_space: usize,
    pub pre_break: option::Option<char>,
    pub post_break: option::Option<char>,
    pub no_break: option::Option<char>,
    pub if_nonempty: bool,
    pub never_break: bool,
}

#[derive(Clone, Copy)]
pub struct BeginToken {
    pub offset: isize,
    pub breaks: Breaks,
}

#[derive(Clone)]
enum Token {
    String(Cow<'static, str>),
    Break(BreakToken),
    Begin(BeginToken),
    End,
}

#[derive(Copy, Clone)]
enum PrintFrame {
    Fits(Breaks),
    Broken(usize, Breaks),
}

pub struct Printer {
    out: String,
    // Number of spaces left on line
    space: isize,
    // Ring-buffer of tokens and calculated sizes
    buf: RingBuffer<BufEntry>,
    // Total size of tokens already printed
    left_total: isize,
    // Total size of tokens enqueued, including printed and not yet printed
    right_total: isize,
    // Holds the ring-buffer index of the Begin that started the current block,
    // possibly with the most recent Break after that Begin (if there is any) on
    // top of it. Values are pushed and popped on the back of the queue using it
    // like stack, and elsewhere old values are popped from the front of the
    // queue as they become irrelevant due to the primary ring-buffer advancing.
    scan_stack: VecDeque<usize>,
    // Stack of blocks-in-progress being flushed by print
    print_stack: Vec<PrintFrame>,
    // Level of indentation of current line
    indent: usize,
    // Buffered indentation to avoid writing trailing whitespace
    pending_indentation: usize,
}

#[derive(Clone)]
struct BufEntry {
    token: Token,
    size: isize,
}

impl Printer {
    pub fn new() -> Self {
        Printer {
            out: String::new(),
            space: MARGIN,
            buf: RingBuffer::new(),
            left_total: 0,
            right_total: 0,
            scan_stack: VecDeque::new(),
            print_stack: Vec::new(),
            indent: 0,
            pending_indentation: 0,
        }
    }

    pub fn eof(mut self) -> String {
        if !self.scan_stack.is_empty() {
            self.check_stack(0);
            self.advance_left();
        }
        self.out
    }

    pub fn scan_begin(&mut self, token: BeginToken) {
        if self.scan_stack.is_empty() {
            self.left_total = 1;
            self.right_total = 1;
            self.buf.clear();
        }
        let right = self.buf.push(BufEntry {
            token: Token::Begin(token),
            size: -self.right_total,
        });
        self.scan_stack.push_back(right);
    }

    pub fn scan_end(&mut self) {
        if self.scan_stack.is_empty() {
            self.print_end();
        } else {
            if !self.buf.is_empty() {
                if let Token::Break(break_token) = self.buf.last().token {
                    if self.buf.len() >= 2 {
                        if let Token::Begin(_) = self.buf.second_last().token {
                            self.buf.pop_last();
                            self.buf.pop_last();
                            self.scan_stack.pop_back();
                            self.scan_stack.pop_back();
                            self.right_total -= break_token.blank_space as isize;
                            return;
                        }
                    }
                    if break_token.if_nonempty {
                        self.buf.pop_last();
                        self.scan_stack.pop_back();
                        self.right_total -= break_token.blank_space as isize;
                    }
                }
            }
            let right = self.buf.push(BufEntry {
                token: Token::End,
                size: -1,
            });
            self.scan_stack.push_back(right);
        }
    }

    pub fn scan_break(&mut self, token: BreakToken) {
        if self.scan_stack.is_empty() {
            self.left_total = 1;
            self.right_total = 1;
            self.buf.clear();
        } else {
            self.check_stack(0);
        }
        let right = self.buf.push(BufEntry {
            token: Token::Break(token),
            size: -self.right_total,
        });
        self.scan_stack.push_back(right);
        self.right_total += token.blank_space as isize;
    }

    pub fn scan_string(&mut self, string: Cow<'static, str>) {
        if self.scan_stack.is_empty() {
            self.print_string(string);
        } else {
            let len = string.len() as isize;
            self.buf.push(BufEntry {
                token: Token::String(string),
                size: len,
            });
            self.right_total += len;
            self.check_stream();
        }
    }

    pub fn offset(&mut self, offset: isize) {
        match &mut self.buf.last_mut().token {
            Token::Break(token) => token.offset += offset,
            Token::Begin(_) => {}
            Token::String(_) | Token::End => unreachable!(),
        }
    }

    pub fn end_with_max_width(&mut self, max: isize) {
        let mut depth = 1;
        for &index in self.scan_stack.iter().rev() {
            let entry = &self.buf[index];
            match entry.token {
                Token::Begin(_) => {
                    depth -= 1;
                    if depth == 0 {
                        if entry.size < 0 {
                            let actual_width = entry.size + self.right_total;
                            if actual_width > max {
                                self.buf.push(BufEntry {
                                    token: Token::String(Cow::Borrowed("")),
                                    size: SIZE_INFINITY,
                                });
                                self.right_total += SIZE_INFINITY;
                            }
                        }
                        break;
                    }
                }
                Token::End => depth += 1,
                Token::Break(_) => {}
                Token::String(_) => unreachable!(),
            }
        }
        self.scan_end();
    }

    fn check_stream(&mut self) {
        while self.right_total - self.left_total > self.space {
            if *self.scan_stack.front().unwrap() == self.buf.index_of_first() {
                self.scan_stack.pop_front().unwrap();
                self.buf.first_mut().size = SIZE_INFINITY;
            }

            self.advance_left();

            if self.buf.is_empty() {
                break;
            }
        }
    }

    fn advance_left(&mut self) {
        while self.buf.first().size >= 0 {
            let left = self.buf.pop_first();

            match left.token {
                Token::String(string) => {
                    self.left_total += left.size;
                    self.print_string(string);
                }
                Token::Break(token) => {
                    self.left_total += token.blank_space as isize;
                    self.print_break(token, left.size);
                }
                Token::Begin(token) => self.print_begin(token, left.size),
                Token::End => self.print_end(),
            }

            if self.buf.is_empty() {
                break;
            }
        }
    }

    fn check_stack(&mut self, mut depth: usize) {
        while let Some(&index) = self.scan_stack.back() {
            let entry = &mut self.buf[index];
            match entry.token {
                Token::Begin(_) => {
                    if depth == 0 {
                        break;
                    }
                    self.scan_stack.pop_back().unwrap();
                    entry.size += self.right_total;
                    depth -= 1;
                }
                Token::End => {
                    self.scan_stack.pop_back().unwrap();
                    entry.size = 1;
                    depth += 1;
                }
                Token::Break(_) => {
                    self.scan_stack.pop_back().unwrap();
                    entry.size += self.right_total;
                    if depth == 0 {
                        break;
                    }
                }
                Token::String(_) => unreachable!(),
            }
        }
    }

    fn get_top(&self) -> PrintFrame {
        const OUTER: PrintFrame = PrintFrame::Broken(0, Breaks::Inconsistent);
        self.print_stack.last().map_or(OUTER, PrintFrame::clone)
    }

    fn print_begin(&mut self, token: BeginToken, size: isize) {
        if cfg!(prettyplease_debug) {
            self.out.push(match token.breaks {
                Breaks::Consistent => '«',
                Breaks::Inconsistent => '‹',
            });
            if cfg!(prettyplease_debug_indent) {
                self.out
                    .extend(token.offset.to_string().chars().map(|ch| match ch {
                        '0'..='9' => ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉']
                            [(ch as u8 - b'0') as usize],
                        '-' => '₋',
                        _ => unreachable!(),
                    }));
            }
        }
        if size > self.space {
            self.print_stack
                .push(PrintFrame::Broken(self.indent, token.breaks));
            self.indent = usize::try_from(self.indent as isize + token.offset).unwrap();
        } else {
            self.print_stack.push(PrintFrame::Fits(token.breaks));
        }
    }

    fn print_end(&mut self) {
        let breaks = match self.print_stack.pop().unwrap() {
            PrintFrame::Broken(indent, breaks) => {
                self.indent = indent;
                breaks
            }
            PrintFrame::Fits(breaks) => breaks,
        };
        if cfg!(prettyplease_debug) {
            self.out.push(match breaks {
                Breaks::Consistent => '»',
                Breaks::Inconsistent => '›',
            });
        }
    }

    fn print_break(&mut self, token: BreakToken, size: isize) {
        let fits = token.never_break
            || match self.get_top() {
                PrintFrame::Fits(..) => true,
                PrintFrame::Broken(.., Breaks::Consistent) => false,
                PrintFrame::Broken(.., Breaks::Inconsistent) => size <= self.space,
            };
        if fits {
            self.pending_indentation += token.blank_space;
            self.space -= token.blank_space as isize;
            if let Some(no_break) = token.no_break {
                self.out.push(no_break);
                self.space -= no_break.len_utf8() as isize;
            }
            if cfg!(prettyplease_debug) {
                self.out.push('·');
            }
        } else {
            if let Some(pre_break) = token.pre_break {
                self.print_indent();
                self.out.push(pre_break);
            }
            if cfg!(prettyplease_debug) {
                self.out.push('·');
            }
            self.out.push('\n');
            let indent = self.indent as isize + token.offset;
            self.pending_indentation = usize::try_from(indent).unwrap();
            self.space = cmp::max(MARGIN - indent, MIN_SPACE);
            if let Some(post_break) = token.post_break {
                self.print_indent();
                self.out.push(post_break);
                self.space -= post_break.len_utf8() as isize;
            }
        }
    }

    fn print_string(&mut self, string: Cow<'static, str>) {
        self.print_indent();
        self.out.push_str(&string);
        self.space -= string.len() as isize;
    }

    fn print_indent(&mut self) {
        self.out.reserve(self.pending_indentation);
        self.out
            .extend(iter::repeat(' ').take(self.pending_indentation));
        self.pending_indentation = 0;
    }
}

impl Printer {
    pub fn ibox(&mut self, indent: isize) {
        self.scan_begin(BeginToken {
            offset: indent,
            breaks: Breaks::Inconsistent,
        });
    }

    pub fn cbox(&mut self, indent: isize) {
        self.scan_begin(BeginToken {
            offset: indent,
            breaks: Breaks::Consistent,
        });
    }

    pub fn end(&mut self) {
        self.scan_end();
    }

    pub fn word<S: Into<Cow<'static, str>>>(&mut self, wrd: S) {
        let s = wrd.into();
        self.scan_string(s);
    }

    pub fn keyword<S: Into<Cow<'static, str>>>(&mut self, wrd: S) {
        self.word(wrd.into().to_ascii_uppercase());
    }

    pub fn ident<S: Into<Cow<'static, str>>>(&mut self, wrd: S) {
        self.word(wrd);
    }

    fn spaces(&mut self, n: usize) {
        self.scan_break(BreakToken {
            blank_space: n,
            ..BreakToken::default()
        });
    }

    pub fn zerobreak(&mut self) {
        self.spaces(0);
    }

    pub fn space(&mut self) {
        self.spaces(1);
    }

    pub fn nbsp(&mut self) {
        self.word(" ");
    }

    pub fn hardbreak(&mut self) {
        self.spaces(SIZE_INFINITY as usize);
    }

    pub fn space_if_nonempty(&mut self) {
        self.scan_break(BreakToken {
            blank_space: 1,
            if_nonempty: true,
            ..BreakToken::default()
        });
    }

    pub fn hardbreak_if_nonempty(&mut self) {
        self.scan_break(BreakToken {
            blank_space: SIZE_INFINITY as usize,
            if_nonempty: true,
            ..BreakToken::default()
        });
    }

    pub fn trailing_comma(&mut self, is_last: bool) {
        if is_last {
            self.scan_break(BreakToken {
                pre_break: Some(','),
                ..BreakToken::default()
            });
        } else {
            self.word(",");
            self.space();
        }
    }

    pub fn trailing_comma_or_space(&mut self, is_last: bool) {
        if is_last {
            self.scan_break(BreakToken {
                blank_space: 1,
                pre_break: Some(','),
                ..BreakToken::default()
            });
        } else {
            self.word(",");
            self.space();
        }
    }

    pub fn comma(&mut self, is_last: bool) {
        if !is_last {
            self.word(",");
            self.space();
        }
    }

    pub fn neverbreak(&mut self) {
        self.scan_break(BreakToken {
            never_break: true,
            ..BreakToken::default()
        });
    }
}

struct RingBuffer<T> {
    data: VecDeque<T>,
    // Abstract index of data[0] in infinitely sized queue
    offset: usize,
}

impl<T> RingBuffer<T> {
    fn new() -> Self {
        RingBuffer {
            data: VecDeque::new(),
            offset: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, value: T) -> usize {
        let index = self.offset + self.data.len();
        self.data.push_back(value);
        index
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn index_of_first(&self) -> usize {
        self.offset
    }

    fn first(&self) -> &T {
        &self.data[0]
    }

    fn first_mut(&mut self) -> &mut T {
        &mut self.data[0]
    }

    fn pop_first(&mut self) -> T {
        self.offset += 1;
        self.data.pop_front().unwrap()
    }

    fn last(&self) -> &T {
        self.data.back().unwrap()
    }

    fn last_mut(&mut self) -> &mut T {
        self.data.back_mut().unwrap()
    }

    fn second_last(&self) -> &T {
        &self.data[self.data.len() - 2]
    }

    fn pop_last(&mut self) {
        self.data.pop_back().unwrap();
    }
}

impl<T> Index<usize> for RingBuffer<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index.checked_sub(self.offset).unwrap()]
    }
}

impl<T> IndexMut<usize> for RingBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index.checked_sub(self.offset).unwrap()]
    }
}
