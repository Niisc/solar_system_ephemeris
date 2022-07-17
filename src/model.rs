use crate::physics::*;
use crate::maths::*;
use std::fs::File;
use std::io::*;
use std::io::{self, Read};
use std::ops::Deref;
use serde_derive::{Serialize, Deserialize};
use obj::*;

pub const TEMP_DIV: usize = 256;
pub struct Face{
    pub vertices: [Vector; 3],
    n: Vector,
    min: Vector,
    max: Vector,
    pos: Vector,
    pub area: f32,
    depth: f32,
    pub temp: [f32; 256],
    pub tempn: [f32; 256],
    view_factor: f32,
    h: f32,
    enlightened: bool

}

//#[derive(Copy)]
#[derive(Default)]
pub struct Model{
    filename: String,
    scale: f32,

    vertices: Vec<Vector>,
    pub faces: Vec<Face>,

    pub bbmin: Vector,
    pub bbmax: Vector,
    delta: Vector,
    o: Vector,
}
impl Model {
    pub fn new(filename: &String, scale: f32) -> Self {
        Self { filename: filename.to_string(), scale: scale, vertices: Vec::new(), faces: Vec::new(), bbmin: Vector {x: 0f32, y: 0f32, z: 0f32}, bbmax: Vector {x: 0f32, y: 0f32, z: 0f32}, delta: Vector {x: 0f32, y: 0f32, z: 0f32}, o: Vector {x: 0f32, y: 0f32, z: 0f32} }
    }

    pub fn load(&mut self) {
        let object = Obj::load(&self.filename).expect("cannot open file");

        if object.data.objects.len() > 1 ||  object.data.objects.get(0).expect("object has 0 elements").groups.len() > 1{
            panic!("not implemented behaviour")
        }

        for pos in object.data.position {
            self.vertices.push(Vector { x: pos[0], y: pos[1], z: pos[2] });
            //idk maybe add scale
            if pos[0] < self.bbmin.x { self.bbmin.x = pos[0] }//bbmax and bbmin start with 0 possible bug there
            if pos[1] < self.bbmin.y { self.bbmin.y = pos[1] }
            if pos[2] < self.bbmin.z { self.bbmin.z = pos[2] }

            if pos[0] > self.bbmax.x { self.bbmax.x = pos[0] }
            if pos[1] > self.bbmax.y { self.bbmax.y = pos[1] }
            if pos[2] > self.bbmax.z { self.bbmax.z = pos[2] }
        }

        for poly in object.data.objects.get(0).unwrap().groups.get(0).unwrap().polys.iter() {
            if poly.deref().0.len() != 3 {
                panic!("Not enough vertices?");
            }
            let mut face: Face = Face {
                vertices: [Vector { x: 0f32, y: 0f32, z: 0f32 };3],
                pos: Vector { x: 0f32, y: 0f32, z: 0f32 },
                n: Vector { x: 0f32, y: 0f32, z: 0f32 },
                min: Vector { x: 0f32, y: 0f32, z: 0f32 },
                max: Vector { x: 0f32, y: 0f32, z: 0f32 },
                area: 0f32,
                depth: 5f32,
                temp: [0f32; TEMP_DIV],
                tempn: [0f32; TEMP_DIV],
                view_factor: 0f32,
                h: 0f32,
                enlightened: true,

            };
            for x in 0..3 {
                let a = *poly.deref().0.get(x).unwrap();
                face.vertices[x] = self.vertices[a.0];
            }

            face.pos.x =  (face.vertices[0].x + face.vertices[1].x + face.vertices[2].x) /3f32;
            face.pos.y =  (face.vertices[0].y + face.vertices[1].y + face.vertices[2].y) /3f32;
            face.pos.z =  (face.vertices[0].z + face.vertices[1].z + face.vertices[2].z) /3f32;

            let vec1 = face.vertices[1].sub(&face.vertices[0]);
            let vec2 = face.vertices[2].sub(&face.vertices[0]);
            
            face.area = vec1.cross(&vec2).norm()/2f32;
            
            //possible problem with normal as it isnt the average of every vertex
            face.n.x = object.data.normal[poly.deref().0.get(0).unwrap().2.expect("normals not loaded")][0];
            face.n.y = object.data.normal[poly.deref().0.get(0).unwrap().2.expect("normals not loaded")][1];
            face.n.z = object.data.normal[poly.deref().0.get(0).unwrap().2.expect("normals not loaded")][2];

            face.h = face.n.dot(&face.pos).abs();

            if face.h < face.depth {
                print!("Not deep enough")
            }

            self.delta = self.bbmax.sub(&self.bbmin);

        }
        self.o = self.bbmax.add(&self.bbmin).div(2f32);
        
    }
}