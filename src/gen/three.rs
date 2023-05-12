use genco::{
    lang::js::import,
    prelude::*
};

use nom::{
    Err,
    error::VerboseError
};

use crate::{
    parsers::scene,
    ast::{GeometryKind, MaterialKind, LiteralKind}
};

pub fn three(input: &str) -> Result<String, Err<VerboseError<&str>>> {
    let three = &import("three", "THREE").into_wildcard();
    let ast = scene(input)?.1;

    let output_stream = quote! {
        export default function createScene() {
            let scene = new $three.Scene()
            let geometry, material, mesh
            
            $(for mesh in ast.meshes =>
                geometry = $(match mesh.geometry.kind {
                    GeometryKind::Box => new $three.BoxGeometry($(mesh.geometry.args.join(", "))),                
                    GeometryKind::Sphere => new $three.SphereGeometry($(mesh.geometry.args.join(", ")))            
                })
                material = $(match mesh.material.kind {
                    MaterialKind::Basic => new $three.MeshBasicMaterial({
                        $(for kvp in mesh.material.properties => 
                            $[' ']$(kvp.key): $(match kvp.value.kind {
                                LiteralKind::String => $("'")$(kvp.value.value)$("'"),
                                _ => $(kvp.value.value)
                            }),
                        )
                    $[' ']}),                
                    MaterialKind::Standard => new $three.MeshStandardMaterial({
                        $(for kvp in mesh.material.properties => 
                            $[' ']$(kvp.key): $(match kvp.value.kind {
                                LiteralKind::String => $("'")$(kvp.value.value)$("'"),
                                _ => $(kvp.value.value)
                            }),
                        )
                    $[' ']})
                })
                mesh = new $three.Mesh(geometry, material)
                $(match mesh.position {
                    Some(vec) => mesh.position.set($(vec.x), $(vec.y), $(vec.z)),
                    None => ()
                })
                $(match mesh.rotation {
                    Some(vec) => mesh.rotation.set($(vec.x), $(vec.y), $(vec.z)),
                    None => ()
                })
                scene.add(mesh)$['\n']
            )

            return scene
        }
    };

    let output = output_stream.to_file_string().unwrap();

    return Ok(output);
}

#[test]
fn test() {
    let input = "scene {\n\r\tmesh box(1, 1, 0.5) basic(color: 'red') position(0, 0, 0)\n\tmesh box(1, 1, 1) basic(color: 'blue') position(1, 0, 0)\n\r}";
    let output = three(input);

    let expected: Tokens<JavaScript> = quote!{
        import * as THREE from $("\"")three$("\"");

        export default function createScene() {
            let scene = new THREE.Scene()
            let geometry, material, mesh
        
            geometry = new THREE.BoxGeometry(1, 1, 0.5)
            material = new THREE.MeshBasicMaterial({ color: $("'")red$("'"), })
            mesh = new THREE.Mesh(geometry, material)
            mesh.position.set(0, 0, 0)
            scene.add(mesh)
        
            geometry = new THREE.BoxGeometry(1, 1, 1)
            material = new THREE.MeshBasicMaterial({ color: $("'")blue$("'"), })
            mesh = new THREE.Mesh(geometry, material)
            mesh.position.set(1, 0, 0)
            scene.add(mesh)
        
            return scene
        }
    };
    
    assert_eq!(output.unwrap(), expected.to_file_string().unwrap());
}