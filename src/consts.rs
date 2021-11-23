use std::f32::EPSILON;

pub const EPS_0: f64 = 8.854187e-12;
pub const MU_0: f64 = 4.0*std::f64::consts::PI*1e-7;
// (EPS_0/MU_0).sqrt()
pub const ETA_0: f64 = 0.00265441873;
// 1.0/(MU_0*EPS_0).sqrt()
pub const C: f64 = 299792458.0;

