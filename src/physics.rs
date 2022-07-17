use crate::maths::*;
use std::fs;
use crate::model::*;
use serde_derive::{Serialize, Deserialize};
use serde_derive::*;
use serde::*;
use serde_json::*;

#[derive(Deserialize)]
struct Force{
    f: Vector,
    p: Vector,
    gravity: bool,
    torque: bool
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Body{
    name: String,

    #[serde(skip)]
    surface: f32,
    density: f32,
    #[serde(skip)]
    mass: f32,
    time_step:f32,
    #[serde(skip)]
    mdl: Model,


    conductivity: f32,
    diffusivity: f32,
    #[serde(skip)]
    capacity: f32,
    pub initial_temp: f32,
    #[serde(skip)]
    center_of_mass: Vector,
    period: f32,

    #[serde(skip)]
    pub size: f32,

    pos: Vector,
    #[serde(skip)]
    rot_matrix: Matrix,
    vel: Vector,
    #[serde(skip)]
    forces: Vec<Force>,


    pub is_NEO: bool,
    angle: Vector,

    accel: Vector,

    plan: Vector,

    #[serde(skip)]
    volume: f32,
    #[serde(skip)]
    inertia: Matrix,
    #[serde(skip)]
    inertia_determinant: f32,
    albedo: f32,
    emissivity: f32,
    rotvel: Vector,
    #[serde(skip)]
    rotaccel: Vector,
    #[serde(skip)]
    momentum: Vector,
    semi_axis: f32,
    eccentricity: f32,
    #[serde(skip)]
    tota_energy: f32,
    #[serde(skip)]
    energy_variation: f32,
    

} 
impl Body {
    pub fn calc_solid(&mut self){
        let mut tmp_cm: Vector = Vector::new();
        let mut tmp_cm1: Vector = Vector::new();
        let mut total_volume: f32 = 0f32;
        self.surface = 0f32;

        for face in &self.mdl.faces {
            self.surface += face.area;
            let vol: f32 = Vector::det3x3(face.vertices.get(0).unwrap(), face.vertices.get(1).unwrap(), face.vertices.get(2).unwrap()).abs();
            tmp_cm1 = face.vertices[0]
            .add(&face.vertices[1]
                .add(&face.vertices[2])
            );

            tmp_cm.add(&tmp_cm1.mul(vol));
            total_volume += vol;

        }
        self.center_of_mass = tmp_cm.div(4f32 * total_volume);
        self.volume = total_volume/6f32;
        self.mass = self.volume * self.density;

    }

    pub fn calc_matrix(&mut self){
        self.inertia = Matrix::new(3, 3);

        let mut tmp_inertia = Matrix::new(3,3);

        for x in 0..self.mdl.faces.len() {
            tmp_inertia.add(&Matrix::tetrahedron_matrix(&self.mdl.faces.get(x).unwrap().vertices[0], &self.mdl.faces.get(x).unwrap().vertices[1], &self.mdl.faces.get(x).unwrap().vertices[2]));
        }
        let mut cmass = Matrix::new(3,1);

        *cmass.get_mut_value(0, 0) = self.center_of_mass.x;
        *cmass.get_mut_value(1, 0) = self.center_of_mass.y;
        *cmass.get_mut_value(2, 0) = self.center_of_mass.z;

        let mut cmass_transpose = Matrix::new(3,1);
        cmass_transpose.copy(&cmass);

        let mut translate = Matrix::new(3, 3);

        translate.mul(&cmass, &cmass_transpose);
        tmp_inertia.add(&translate);
        let t: f32 = tmp_inertia.trace();

        self.inertia.identity(3);
        self.inertia.mul_float(t);
        self.inertia.sub(&tmp_inertia);
        self.inertia.mul_float(self.density);

        self.inertia_determinant = self.inertia.det3x3();

    }
}
