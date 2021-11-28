use crate::error::Error;
use crate::grid::CartesianGrid;
use crate::consts::Courant;

pub enum ProblemType {
    MaxwellWave
}

pub struct Problem<G> 
where
    G: CartesianGrid,
    <G as CartesianGrid>::GridData: Copy
{
    grid: G,
    courant: Courant
}

impl<G> Default for Problem<G> 
where
    G: CartesianGrid,
    <G as CartesianGrid>::GridData: Copy
{
    fn default() -> Self {
        Self { grid: G::empty(), courant: Default::default() }
    }
}

impl<G> Problem<G> 
where
    G: CartesianGrid,
    <G as CartesianGrid>::GridData: Copy
{
    pub fn add_grid(&mut self, grid: G) {
        self.grid = grid
    }

    pub fn add_courant(&mut self, courant: Courant) {
        self.courant = courant
    }

    pub fn run_iterations(&mut self, run_iters: u32) -> Result<(), Error> {
        for i in 0..run_iters {
            self.step(run_iters)?;
        }

        Ok(())
    }

    fn step(&mut self, step_num: u32) -> Result<(), Error> {

    }
}
