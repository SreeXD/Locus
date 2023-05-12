use super::{Kvp, Vec3};

#[derive(Debug, PartialEq)]
pub enum GeometryKind {
    Box,
    Sphere
}

#[derive(Debug, PartialEq)]
pub struct Geometry<'a> {
    pub kind: GeometryKind,
    pub args: Vec<&'a str>
}

#[derive(Debug, PartialEq)]
pub enum MaterialKind {
    Basic,
    Standard
}

#[derive(Debug, PartialEq)]
pub struct Material<'a> {
    pub kind: MaterialKind,
    pub properties: Vec<Kvp<'a>>
}

#[derive(Debug, PartialEq)]
pub struct Mesh<'a> {
    pub geometry: Geometry<'a>,
    pub material: Material<'a>,
    pub position: Option<Vec3<'a>>,
    pub rotation: Option<Vec3<'a>>
}

#[derive(Debug, PartialEq)]
pub struct Scene<'a> {
    pub meshes: Vec<Mesh<'a>>
}