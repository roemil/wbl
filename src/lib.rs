use num::complex::ComplexFloat;

pub mod calc_wb;
pub mod moa;
pub mod ken;
pub mod two_seater;

#[derive(PartialEq, Clone)]
pub enum UseFuel {
    Yes,
    No,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct ViktArm {
    pub weight: f32,
    pub lever: f32,
}

impl ViktArm {
    pub fn new(weight: f32, lever: f32) -> Self {
        ViktArm { weight, lever }
    }

    pub fn torque(&self) -> f32 {
        self.lever * self.weight
    }
}

//ref: https://www.linkedin.com/pulse/short-formula-check-given-point-lies-inside-outside-polygon-ziemecki/
fn is_inside_polygon(point: ViktArm, vertices: Vec<ViktArm>, valid_border: bool) -> bool {
    let mut sum = num::complex::Complex::new(0.0, 0.0);

    for i in 1..vertices.len() + 1 {
        let v0 = &vertices[i - 1];
        let v1 = &vertices[i % vertices.len()];

        if is_point_in_segment(&point, v0, v1) {
            return valid_border;
        }
        let v1_c = num::complex::Complex::new(v1.lever, v1.weight);
        let p_c = num::complex::Complex::new(point.lever, point.weight);
        let v0_c = num::complex::Complex::new(v0.lever, v0.weight);
        sum += num::complex::Complex::ln((v1_c - p_c) / (v0_c - p_c));
    }

    sum.abs() > 1.0
}

fn is_point_in_segment(p: &ViktArm, p0: &ViktArm, p1: &ViktArm) -> bool {
    let p0 = ViktArm::new(p0.weight - p.weight, p0.lever - p.lever);
    let p1 = ViktArm::new(p1.weight - p.weight, p1.lever - p.lever);

    let det = p0.weight * p1.lever - p1.weight * p0.lever;
    let prod = p0.weight * p1.weight + p0.lever * p1.lever;

    (det == 0.0 && prod < 0.0)
        || (p0.weight == 0.0 && p0.lever == 0.0)
        || (p1.weight == 0.0 && p1.lever == 0.0)
}