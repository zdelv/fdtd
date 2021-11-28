pub mod consts;
pub mod grid;
pub mod position;
pub mod problem;
pub mod problem_def;
pub mod grid_set;
pub mod error;

#[cfg(test)]
mod tests {
    use crate::grid::{CartesianGrid, CartesianGrid2D, Size};
    use crate::problem::Problem;
    use crate::consts::Courant;

    #[test]
    fn basic_test() {
        let (x, y) = (10, 20);
        let (dx, dy) = (1.0 / x as f64, 1.0 / y as f64);
        let grid = CartesianGrid2D::new(Size::new(x,y,0));

        //let courant = 0.95;
        let courant = Courant { 
            dx,
            dy: Some(dy),
            ..Default::default()
        };
        let mut problem = Problem::default();

        problem.add_grid(grid);
        problem.add_courant(courant);

        let run_time = 10; 

        match problem.run_iterations(run_time) {
            Ok(o) => return,
            Err(e) => println!("{}", e)
        }
    }
}
