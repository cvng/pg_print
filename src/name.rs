pub enum Name {
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

impl TryFrom<String> for Name {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "bpchar" => Ok(Self::Bpchar),
            "varchar" => Ok(Self::Varchar),
            "numeric" => Ok(Self::Numeric),
            "bool" => Ok(Self::Bool),
            "int2" => Ok(Self::Int2),
            "int4" => Ok(Self::Int4),
            "int8" => Ok(Self::Int8),
            "real" | "float4" => Ok(Self::Real),
            "float8" => Ok(Self::Float8),
            "time" => Ok(Self::Time),
            "timetz" => Ok(Self::Timetz),
            "timestamp" => Ok(Self::Timestamp),
            "timestamptz" => Ok(Self::Timestamptz),
            "interval" => Ok(Self::Interval),
            _ => Err(()),
        }
    }
}
