#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    String,
    Number,
    Boolean
}

#[derive(Debug, PartialEq)]
pub struct Literal<'a> {
    pub kind: LiteralKind,
    pub value: &'a str
}

#[derive(Debug, PartialEq)]
pub struct Kvp<'a> {
    pub key: &'a str,
    pub value: Literal<'a>
}

#[derive(Debug, PartialEq)]
pub struct Vec3<'a> {
    pub x: &'a str,
    pub y: &'a str,
    pub z: &'a str
}