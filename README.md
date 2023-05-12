# Locus

a 3d scene specification language built using nom parser combinator library in rust

(this is just a practice project and its barely useful at it's current state)

## demo

currently locus compiler supports code generation of a threejs scene in javascript

```js
scene {
    mesh box(1, 1, 1) basic(color: 'red') position(0, 0, 0) rotation(0.2, 0, 0)
    mesh sphere(0.5) basic(color: 'purple') position(1, 0, 0)
}
```

the above locus scene can be compiled to a threejs scene using rust executable project inside `/compiler` directory

```
./compiler scene.ls -o scene.js
```

above command produces the following module which can be imported into your javascript code

```js
import * as THREE from "three";

export default function createScene() {
    let scene = new THREE.Scene()
    let geometry, material, mesh

    geometry = new THREE.BoxGeometry(1, 1, 1)
    material = new THREE.MeshBasicMaterial({ color: 'red', })
    mesh = new THREE.Mesh(geometry, material)
    mesh.position.set(0, 0, 0)
    mesh.rotation.set(0.2, 0, 0)
    scene.add(mesh)

    geometry = new THREE.SphereGeometry(0.5)
    material = new THREE.MeshBasicMaterial({ color: 'purple', })
    mesh = new THREE.Mesh(geometry, material)
    mesh.position.set(1, 0, 0)
    scene.add(mesh)

    return scene
}

```

![scene in threejs](https://i.imgur.com/M19KHwL.png)

## language support

the vscode extension inside `/locus-language-support` provides basic syntax highlighting