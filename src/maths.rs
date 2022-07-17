use std::thread::panicking;
use std::cmp::min;
use serde_derive::Deserialize;

#[derive(Deserialize, Default)]
pub struct Matrix{
    rows: usize,
    cols: usize,
    elems: Vec<Vec<f32>>,
}

impl Matrix {

    pub fn mul_float(&mut self, a: f32) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.elems[i][j] *= a;
            }
        }
    }
    pub fn get_value(&self, x: usize, y: usize) -> f32 {
        *self.elems.get(x).map(|row| row.get(y)).flatten().unwrap()
    }
    pub fn get_mut_value(&mut self, x: usize, y: usize) -> &mut f32 {
        self.elems.get_mut(x).map(|row| row.get_mut(y)).flatten().unwrap()
    }
    pub fn det3x3(&self) -> f32 {
        let a = Vector{x: self.get_value(0, 0), y: self.get_value(0, 1), z: self.get_value(0, 2)};
        let b = Vector{x: self.get_value(1, 0), y: self.get_value(1, 1), z: self.get_value(1, 2)};
        let c = Vector{x: self.get_value(2, 0), y: self.get_value(2, 1), z: self.get_value(2, 2)};
        a.cross(&b).dot(&c)
    }
    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.rows = rows;
        self.rows = cols;

        self.elems.resize(rows, Vec::new());
        for x in  0..self.elems.len() {
            self.elems.get_mut(x).unwrap().resize(cols, 0f32);
        }
    }
    pub fn new(rows: usize, cols: usize) -> Matrix {

        let mut mat = Matrix {rows: rows, cols: cols, elems: Vec::new()};
        mat.elems.resize(rows, Vec::new());

        for x in  0..mat.elems.len() {
            mat.elems.get_mut(x).unwrap().resize(cols, 0f32);
        }

        mat
    }
    pub fn new_from_vec(a: &Vector, b: &Vector, c: &Vector) -> Matrix {
        //TODO: Check this may be wrong
        let mut mat = Matrix::new(3,3);

        *mat.get_mut_value(0,0) = a.x;
        *mat.get_mut_value(0,1) = b.x;
        *mat.get_mut_value(0,2) = c.x;

        *mat.get_mut_value(1,0) = a.y;
        *mat.get_mut_value(1,1) = b.y;
        *mat.get_mut_value(1,2) = c.y;

        *mat.get_mut_value(2,0) = a.z;
        *mat.get_mut_value(2,1) = b.z;
        *mat.get_mut_value(2,2) = c.z;

        mat
    }
    pub fn copy(&mut self, b: &Matrix) {
        //possible fail point
        if self.rows != b.rows || self.cols != b.cols {
            print!("WARNING: rows and cols not the same, possible problem");
        }
        self.resize(b.rows, b.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                *self.get_mut_value(i, j) = b.get_value(i, j);
            }
        }
    }
    pub fn sub(&mut self, a: &Matrix) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                *self.get_mut_value(i, j) -= a.get_value(i, j);
            }
        }
    }
    pub fn identity(&mut self, n: usize) {
        self.resize(n, n);
        for i in 0..n {
            self.elems[i][i] = 1f32;
        }
    }
    pub fn trace(&self) -> f32 {
        let mut s: f32 = 0f32;
        for i in 0..min(self.cols, self.rows) {
            s += self.elems[i][i];
        }
        s

    }
    pub fn add(&mut self, b: &Matrix) {

        for x in 0..self.rows {
            for y in 0..self.cols {
                *self.get_mut_value(x,y) += b.get_value(x, y);
            }
        }
    }
    pub fn mul(&mut self, a: &Matrix, b: &Matrix) {
        let mut ret = Matrix::new(a.rows, b.cols);
        if a.cols != b.rows {
            panic!("number of cols and rows no equal")
        }
        for i in 0..a.rows {
            for j in 0..b.cols {
                ret.elems[i][j] = 0f32;
                for k in 0..a.cols {
                    *ret.get_mut_value(i, j) += a.get_value(i, k) * b.get_value(k, j);
                }
            }
        }
        self.copy(&ret);
    }

    pub fn transpose(&mut self) {
        let mut a = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                *a.elems.get_mut(j).map(|row| row.get_mut(i)).flatten().unwrap() = *self.elems.get(i).map(|row| row.get(j)).flatten().unwrap();
            }
        }
        self.resize(self.cols, self.rows);
        self.copy(&a);
    }
    pub fn tetrahedron_matrix(a: &Vector, b: &Vector, c: &Vector) -> Matrix {
        let mut im = Matrix::new(3,3);
        
        let cov: Matrix = Matrix::new_from_vec(
            &Vector {x: 1f32/60f32, y:1f32/120f32, z:1f32/120f32}, 
            &Vector {x: 1f32/120f32, y:1f32/60f32, z:1f32/120f32}, 
            &Vector {x: 1f32/120f32, y:1f32/120f32, z:1f32/60f32}
        );

        let transform = Matrix::new_from_vec(a, b, c);
        let mut transp = Matrix::new_from_vec(a, b, c);

        transp.transpose();
        im.mul(&cov, &transp);
        let d: f32 = transform.det3x3();

        let mut mi = Matrix::new(3,3);
        mi.copy(&im);

        im.mul(&transform, &mi);
        im.mul_float(d);
        im

    }
}

#[derive(Copy, Clone, Deserialize, Default)]
pub struct Vector{ 
    pub x: f32, 
    pub y: f32,
    pub z: f32
}
impl Vector {
    //vec &vec::normalize() { div(norm()); return *this; }

    pub fn normalize(a: f32) -> Vector {
        
    }
    pub fn cross(&self, b: &Vector) -> Vector {
        Vector {x: self.y * b.z - self.z * b.y, y: self.z * b.x - self.x * b.z, z: self.x * b.y - self.y * b.x }
    }
    pub fn sub(&self, b: &Vector) -> Vector {
        Vector {x: self.x - b.x, y: self.y - b.y, z: self.z - b.z }
    }
    pub fn norm(&self) -> f32 {
        self.squared_len().sqrt()
    }
    pub fn squared_len(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn dot(&self, b: &Vector) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    pub fn add(&self, b: &Vector) -> Vector {
        Vector { x: self.x + b.x, y: self.y + b.y, z: self.z + b.z }
    }
    pub fn div(&self, b: f32) -> Vector {
        Vector {x: self.x / b, y: self.y / b, z: self.z /b}
    }
    pub fn new() -> Vector {
        Vector { x: 0f32, y: 0f32, z: 0f32 }
    }
    pub fn det3x3(a: &Vector, b: &Vector, c: &Vector) -> f32 {
        a.x * b.y * c.z + a.y * b.z * c.x + a.z * b.x * c.y - a.z * b.y * c.z - a.y * b.x * c.z - a.x * b.z * c.y
    }
    pub fn mul(&self, a: f32) -> Vector {
        Vector { x: self.x * a, y: self.y * a, z: self.z * a }
    }
}