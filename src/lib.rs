mod consts;
mod grid;
mod problem;

#[cfg(test)]
mod tests {
    use crate::grid::CartesianGrid;

    #[test]
    fn basic_test() {
        let (x, y) = (10, 20);
        let (dx, dy) = (1.0 / x as f64, 1.0 / y as f64);
        let grid = CartesianGrid::new(x, y, dx, dy);

        let courant = 0.95;
        let problem = Problem::default();

        problem.add_grid(grid);
        problem.add_courant(courant);

        let run_time = 10.0;
        problem.run(run_time);
    }
}
