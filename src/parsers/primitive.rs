use nom::{
    IResult,
    error::{VerboseError, context},
    sequence::{tuple, delimited, preceded, pair}, 
    combinator::{map, opt}, 
    bytes::complete::tag,
    branch::{alt, permutation},
    character::complete::{multispace0, char},
    multi::{separated_list0, many0}
};

use crate::ast::{Material, MaterialKind, Literal, LiteralKind, Kvp, Geometry, GeometryKind, Mesh, Vec3, Scene};
use super::{kvp, number, vec3};

pub fn geometry_kind(input: &str) -> IResult<&str, GeometryKind, VerboseError<&str>> {
    context(
        "geometry kind",
        map(
            alt((
                tag("box"),
                tag("sphere")
            )),
            |value| { 
                match value {
                    "box" => GeometryKind::Box,
                    "sphere" => GeometryKind::Sphere,
                    _ => panic!()
                }
            }
        )
    )(input)
}

pub fn geometry(input: &str) -> IResult<&str, Geometry, VerboseError<&str>> {
    context(
        "geometry",
        map(
            pair(
                geometry_kind,
                delimited(
                    char('('),
                    separated_list0(
                        delimited(
                            multispace0,
                            char(','),
                            multispace0
                        ),
                        number
                    ),
                    char(')'),
                )
            ),
            |(kind, args)| Geometry { kind, args }
        )
    )(input)
}

pub fn material_kind(input: &str) -> IResult<&str, MaterialKind, VerboseError<&str>> {
    context(
        "material kind",
        map(
            alt((
                tag("basic"),
                tag("standard")
            )),
            |value| { 
                match value {
                    "basic" => MaterialKind::Basic,
                    "standard" => MaterialKind::Standard,
                    _ => panic!()
                }
            }
        )
    )(input)
}

pub fn material(input: &str) -> IResult<&str, Material, VerboseError<&str>> {
    context(
        "material",
        map(
            tuple((
                material_kind,
                delimited(
                    char('('),
                    separated_list0(
                        delimited(
                            multispace0,
                            char(','),
                            multispace0
                        ),
                        kvp
                    ),
                    char(')'),
                ),
            )),
            |(kind, properties)| Material { kind, properties }
        )
    )(input)
}

pub fn mesh(input: &str) -> IResult<&str, Mesh, VerboseError<&str>> {
    context(
        "mesh",
        map(
            preceded(
                tag("mesh"),
                permutation((
                    delimited(
                        multispace0,
                        geometry,
                        multispace0
                    ),
                    delimited(
                        multispace0,
                        material,
                        multispace0
                    ),
                    opt(
                        delimited(
                            multispace0,
                            preceded(
                                alt((
                                    tag("position"),
                                    tag("pos")
                                )),
                                delimited(
                                    char('('),
                                    vec3,
                                    char(')')
                                )
                            ),
                            multispace0
                        ),
                    ),
                    opt(
                        delimited(
                            multispace0,
                            preceded(
                                alt((
                                    tag("rotation"),
                                    tag("rot")
                                )),
                                delimited(
                                    char('('),
                                    vec3,
                                    char(')')
                                )
                            ),
                            multispace0
                        )
                    )
                )
            )),
            |(geometry, material, position, rotation)| Mesh { 
                geometry, 
                material,
                position,
                rotation
            }
        )
    )(input)
}

pub fn scene(input: &str) -> IResult<&str, Scene, VerboseError<&str>> {
    context(
        "scene",
        map(
            preceded(
                tag("scene"),
                delimited(
                    delimited(
                        multispace0,
                        char('{'),
                        multispace0
                    ),
                    many0(
                        delimited(
                            multispace0,
                            mesh,
                            multispace0
                        )
                    ),
                    delimited(
                        multispace0,
                        char('}'),
                        multispace0
                    )
                )
            ),
            |meshes| Scene { meshes }
        )
    )(input)
}

#[test]
fn test() {
    assert_eq!(
        scene("scene {\n\r\tmesh box(1, 1, 0.5) standard(color: 'red', roughness: 0.5) position(1, 1, 2) rot(1.5, 2, 2)\n\r}--"), 
        Ok((
            "--",
            Scene {
                meshes: vec![
                    Mesh {
                        geometry: Geometry { 
                            kind: GeometryKind::Box, 
                            args: vec!["1", "1", "0.5"] 
                        },
        
                        material: Material { 
                            kind: MaterialKind::Standard, 
                            properties: vec![
                                Kvp { 
                                    key: "color", 
                                    value: Literal {
                                        kind: LiteralKind::String,
                                        value: "red"
                                    }
                                },
            
                                Kvp { 
                                    key: "roughness", 
                                    value: Literal {
                                        kind: LiteralKind::Number,
                                        value: "0.5"
                                    }
                                }
                            ] 
                        },
        
                        position: Some(Vec3 { x: "1", y: "1", z: "2" }),
                        rotation: Some(Vec3 { x: "1.5", y: "2", z: "2" })
                    }
                ]
            }
        ))
    );
}