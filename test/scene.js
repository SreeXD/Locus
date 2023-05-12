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
