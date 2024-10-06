use std::collections::HashMap;
pub use std::{result::Result as StdResult, string::String as StdString};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub struct Error {}

#[derive(Debug, Clone)]
pub enum Dynamic {
    Number(Number),
    Bool(Bool),
    String(String),
    Duration(Duration),
    DataSize(DataSize),
    Object(Object),
    Listing(Listing),
    Mapping(Mapping),
    Null(Null),
    List(List),
    Set(Set),
    Map(Map),
    Regex(Regex),
    Pair(Pair),
}

/// The pkl [`Number`](https://pkl-lang.org/package-docs/pkl/0.26.3/base/Number) type.
#[derive(Debug, Copy, Clone)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub struct Bool(bool);

#[derive(Debug, Clone)]
pub struct String(StdString);

#[derive(Debug, Copy, Clone)]
pub struct Duration {
    pub value: Number,
    pub unit: DurationUnit,
}

#[derive(Debug, Copy, Clone)]
pub enum DurationUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

#[derive(Debug, Copy, Clone)]
pub struct DataSize {
    pub value: Number,
    pub unit: DataSizeUnit,
}

#[derive(Debug, Copy, Clone)]
pub enum DataSizeUnit {
    Bytes,
    Kilobytes,
    Kibibytes,
    Megabytes,
    Mebibytes,
    Gigabytes,
    Gibibytes,
    Terabytes,
    Tebibytes,
    Petabytes,
    Pebibytes,
}

#[derive(Debug, Clone)]
pub struct Object(HashMap<StdString, Dynamic>);

#[derive(Debug, Clone)]
pub struct Listing(Vec<Dynamic>);

#[derive(Debug, Clone)]
pub struct Mapping(HashMap<Dynamic, Dynamic>);

#[derive(Debug, Copy, Clone)]
pub struct Null;

#[derive(Debug, Clone)]
pub struct List(Vec<Dynamic>);

#[derive(Debug, Clone)]
pub struct Set(Vec<Dynamic>);

#[derive(Debug, Clone)]
pub struct Map(HashMap<Dynamic, Dynamic>);

#[derive(Debug, Clone)]
pub struct Regex(StdString);

#[derive(Debug, Clone)]
pub struct Pair(Box<Dynamic>, Box<Dynamic>);
