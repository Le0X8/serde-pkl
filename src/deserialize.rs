use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use std::{collections::HashMap, fmt, io::Read};

pub struct PklDeserializer<R> {
    reader: R,
}

impl<R: Read> PklDeserializer<R> {}
