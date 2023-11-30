pub enum Name {
    Undefined,
    Bpchar,
    Varchar,
    Numeric,
    Bool,
    Int2,
    Int4,
    Int8,
    Real,
    Float8,
    Time,
    Timetz,
    Timestamp,
    Timestamptz,
    Interval,
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "bpchar" => Self::Bpchar,
            "varchar" => Self::Varchar,
            "numeric" => Self::Numeric,
            "bool" => Self::Bool,
            "int2" => Self::Int2,
            "int4" => Self::Int4,
            "int8" => Self::Int8,
            "real" | "float4" => Self::Real,
            "float8" => Self::Float8,
            "time" => Self::Time,
            "timetz" => Self::Timetz,
            "timestamp" => Self::Timestamp,
            "timestamptz" => Self::Timestamptz,
            "interval" => Self::Interval,
            _ => Self::Undefined,
        }
    }
}
