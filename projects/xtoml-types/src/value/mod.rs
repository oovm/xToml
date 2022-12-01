use indexmap::IndexMap;
use num::BigInt;
use toml::value::Datetime;

mod ser;
mod der;

#[derive(PartialEq, Clone, Debug)]
pub enum XToml {
    /// Represents a xToml boolean
    Boolean(bool),
    /// Represents a xToml integer
    Integer(BigInt),
    /// Represents a xToml float
    Float(f64),
    /// Represents a xToml datetime
    Datetime(Datetime),
    /// Represents a xToml string
    String(String),
    /// Represents a xToml array
    Array(Vec<XToml>),
    /// Represents a xToml table
    Table(IndexMap<String, XToml>),
}