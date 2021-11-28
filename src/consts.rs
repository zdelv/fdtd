use std::{default, f32::EPSILON};
use crate::error::Error;

pub const EPS_0: f64 = 8.854187e-12;
pub const MU_0: f64 = 4.0*std::f64::consts::PI*1e-7;
// (EPS_0/MU_0).sqrt()
pub const ETA_0: f64 = 0.00265441873;
// 1.0/(MU_0*EPS_0).sqrt()
pub const C: f64 = 299792458.0;

/// I don't like this implementation. This could be a lot better if I knew
/// a bit more about how to implement N-dimensional structures.
#[derive(Default, Clone, Copy)]
pub struct Courant {
    pub dx: f64,
    pub dy: Option<f64>,
    pub dz: Option<f64>,
    pub dt: Option<f64>
}

impl Courant {
    /// Sum over all of the possible dimensions
    fn get_dim(&self) -> u8 {
        let mut dim = 1_u8;
        if self.dy.is_some() { dim += 1; }
        if self.dz.is_some() { dim += 1; }
        dim
    }

    pub fn get(&self) -> Result<f64, Error> {
        if self.dt.is_none() {
            return Err(Error::CourantError("dt is None when attempting to calculate courant".to_string()))
        }
        match self.get_dim() {
            1 => Ok(self.dt.unwrap() * C * ( 1.0/self.dx )),
            2 => Ok(self.dt.unwrap() * C * ( (1.0/self.dx) + (1.0/self.dy.unwrap()) )),
            3 => Ok(self.dt.unwrap() * C * ( (1.0/self.dx) + (1.0/self.dy.unwrap()) + (1.0/self.dz.unwrap()) )),
            _ => Err(Error::CourantError("Unknown number of dimensions".to_string()))
        }
    }

    pub fn get_dt(&self, courant: f64) -> Result<f64, Error> {
        match self.get_dim() {
            1 => Ok(courant * C * ( 1.0/self.dx )),
            2 => Ok(courant * C * ( (1.0/self.dx) + (1.0/self.dy.unwrap()) )),
            3 => Ok(courant * C * ( (1.0/self.dx) + (1.0/self.dy.unwrap()) + (1.0/self.dz.unwrap()) )),
            _ => Err(Error::CourantError("Unknown number of dimensions".to_string()))
        }
    }
}