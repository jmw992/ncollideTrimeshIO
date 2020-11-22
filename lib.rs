use std::fs::{OpenOptions, File};
use std::path::{PathBuf, Prefix};

use nalgebra::Point3;
use ncollide3d::na::{Isometry3, Point, Vector3, U3};
use ncollide3d::shape::{Cuboid , TriMesh/*,Compound, ShapeHandle,  Triangle*/};
use ncollide3d::shape as nc3ds;
use ncollide3d::shape as nc3dp;
use stl_io::{Vertex, Normal};
use ncollide3d::transformation::ToTriMesh;

pub type Prcsn = f32;

pub fn read_stl(stl_path: &PathBuf) -> nc3ds::TriMesh<Prcsn> {
    let mut f = OpenOptions::new()
        .read(true)
        .open(stl_path)
        .expect("Couldn't open file");

    let stl_obj = stl_io::read_stl(&mut f).expect("Couldn't parse STL");

    let mesh: nc3ds::TriMesh<Prcsn> = nc3ds::TriMesh::new(
        stl_obj
            .vertices
            .iter()
            .map(|v| Point3::new(v[0] as Prcsn, v[1] as Prcsn, v[2] as Prcsn))
            .collect(),
        stl_obj
            .faces
            .iter()
            .map(|t| Point3::new(t.vertices[0], t.vertices[1], t.vertices[2]))
            .collect(),
        None,
    );

    println!("{}", mesh.faces().len());
    mesh
}

// fn convert_2_shape_trimesh(procedural_mesh: nc3dp::TriMesh<Prcsn>) -> nc3ds::TriMesh<Prcsn>{
//     let cmp_indices = procedural_mesh.indices.unwrap_split();
//     let mut indices :Vec<Point3<usize>> = Vec::with_capacity(cmp_indices.len());
//     for (ii, p) in cmp_indices.iter().enumerate() {
//         indices[ii] = Point::from([p.x.x as usize, p.x.y as usize, p.x.z as usize]);
//         println!("x {}, y{}, z{}", p.x.x as usize, p.x.y as usize, p.x.z as usize);
//     }
//     nc3ds::TriMesh::new(
//         procedural_mesh.coords,
//         indices,
//         None
//     )
// }

pub fn write_stl(/*_in_mesh: &nc3ds::TriMesh<Prcsn>, filename: &str*/) -> std::io::Result<()> {
    let cube: Cuboid<Prcsn> = Cuboid::new(Vector3::new(0.5, 0.5, 0.5));
    let cube_mesh_p = cube.to_trimesh(());
    let cube_mesh_s: nc3ds::TriMesh<Prcsn> = nc3ds::TriMesh::from(cube_mesh_p);

    let stl_io_mesh = [stl_io::Triangle { normal: Normal::new([1.0, 0.0, 0.0]),
        vertices: [Vertex::new([0.0, -1.0, 0.0]),
            Vertex::new([0.0, 1.0, 0.0]),
            Vertex::new([0.0, 0.0, 0.5])]}];


    let mut stl_io_mesh : Vec<stl_io::Triangle> = Vec::new();
    for face in cube_mesh_s.faces().iter() {
        let normal_v: Vector3<Prcsn> = face.normal.expect("degenerate").into_inner();
        //let a :Point3<Prcsn>= ;
        stl_io_mesh.push(stl_io::Triangle{
            normal: Normal::new([normal_v.x, normal_v.y, normal_v.z]),
            vertices: [
                Vertex::new([cube_mesh_s.points()[face.indices.x].x, cube_mesh_s.points()[face.indices.x].y, cube_mesh_s.points()[face.indices.x].z]),
                Vertex::new([cube_mesh_s.points()[face.indices.y].x, cube_mesh_s.points()[face.indices.y].y, cube_mesh_s.points()[face.indices.y].z]),
                Vertex::new([cube_mesh_s.points()[face.indices.z].x, cube_mesh_s.points()[face.indices.z].y, cube_mesh_s.points()[face.indices.z].z])
            ]});
    };

    /*
    let stl_io_mesh = [stl_io::Triangle { normal: Normal::new([1.0, 0.0, 0.0]),
        vertices: [Vertex::new([0.0, -1.0, 0.0]),
            Vertex::new([0.0, 1.0, 0.0]),
            Vertex::new([0.0, 0.0, 0.5])]}];

    let mut file = OpenOptions::new().write(true).create_new(true).open(filename).unwrap();
    let res = stl_io::write_stl(&mut file, stl_io_mesh.iter())?;
    */

    let mut file = OpenOptions::new().write(true).create_new(true).open("cubeStl").unwrap();
    let res = stl_io::write_stl(&mut file, stl_io_mesh.iter())?;

    std::io::Result::Ok(res)
}

mod tests {
    use crate::write_stl;

    #[allow(warnings)]
    #[test]
    fn test_write_stl(){
        write_stl();
    }

}

