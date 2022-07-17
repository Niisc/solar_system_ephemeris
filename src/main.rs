use std::fs;

use physics::*;
use model::*;

mod physics;
mod maths;
mod model;

fn main() {
    let model_path = String::from("");
    let config_path = String::from("");
    let log_path = String::from("");
    let mut bodies: Vec<Body> = read_config_file(&config_path);

    for x in 0..bodies.len() {
        let mut body = bodies.get_mut(x).unwrap();
        if body.is_NEO {
            load_model(&config_path, &model_path, &log_path, &mut body);
        }
    }
    
}


pub fn load_model(config_file: &String, model_file: &String, log_file: &String, body: &mut Body)  {
        
    let mut mdl = Model::new(&model_file, 1f32);
    let mut bodies = read_config_file(config_file);
    let mut index: i32 = 0;

    for x in 0..bodies.len() {
        if bodies.get(x).unwrap().is_NEO{
            index = x as i32;
            break;
        }
    }
    if index == -1 {
        panic!("No NEO found");
    }
    let mut body = bodies.get_mut(index as usize).unwrap();
    mdl.load();

    for x in 0..mdl.faces.len() {
        let mut face = mdl.faces.get_mut(x).unwrap();
        for k in 0..TEMP_DIV {
            face.temp[k] = body.initial_temp;
            face.tempn[k] = body.initial_temp;
        }
    }
    //implement scale
    body.calc_solid();
    body.calc_matrix();
    body.size =  mdl.bbmax.sub(&mdl.bbmin).norm();
    

}

pub fn read_config_file(filename: &String ) -> Vec<Body> {
    let contents = fs::read_to_string(filename).expect("err in opening file");
    let bodies: Vec<Body> = serde_json::from_str(&contents).expect("error in json thing");
    return bodies;
}
