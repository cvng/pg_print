create table a (id serial primary key, value integer)
CREATE TABLE films (code        char(5), title       varchar(40), did         integer, date_prod   date, kind        varchar(10), len         interval hour to second(3), CONSTRAINT code_title PRIMARY KEY(code,title))
create temporary table a (id serial) on commit drop
CREATE TABLE distributors (did     integer, name    varchar(40), UNIQUE(name) WITH (fillfactor=70) USING INDEX TABLESPACE indexes) WITH (fillfactor=70)
CREATE TABLE measurement_y2016m07 PARTITION OF measurement (unitsales DEFAULT 0) FOR VALUES FROM ('2016-07-01') TO ('2016-08-01') TABLESPACE olddata
CREATE TEMP TABLE films_recent (title, director) ON COMMIT DELETE ROWS AS SELECT title, director FROM films WHERE date_prod >= '2002-01-01'
