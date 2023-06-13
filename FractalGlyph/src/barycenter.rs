use nalgebra::Point2;
use rayon::prelude::*;
use spade::HasPosition;

pub type Point = Point2<f64>;
use spade::delaunay::FloatDelaunayTriangulation;

pub struct BarycenterLayout {
    border: Vec<Point>,
    points: Vec<Point>,
}

impl BarycenterLayout {
    pub fn new(points: Vec<Point>) -> Self {
        let border = Self::compute_border(&points);
        BarycenterLayout { border, points }
    }

    /// For a given list of points, provide the border around which the drawing has to be computed
    fn compute_border(points: &[Point]) -> Vec<Point> {
        // Acquire the minimum and maximum points
        let (min, max) = {
            let (first, others) = points.split_first().unwrap();
            let (min, max) =
                others
                    .iter()
                    .fold((first.clone(), first.clone()), |(min, max), point| {
                        (
                            Point::new(min.x.min(point.x), min.y.min(point.y)),
                            Point::new(max.x.max(point.x), max.y.max(point.y)),
                        )
                    });

            (
                Point::new(min.x - 1., min.y - 1.),
                Point::new(max.x + 1., max.y + 1.),
            )
        };

        // Compute width and height
        let width = max.x - min.x;
        let height = max.y - min.y;

        // Generate the borders
        eprintln!("Border are not properly computed. We consider we re in pixel space before calling the method");
        let mut border = Vec::new();
        for i in 0..(width as u32) {
            let x = min.x + i as f64;
            border.push(Point::new(x, min.y));
            border.push(Point::new(x, max.y));
        }
        for j in 0..(height as u32) {
            let y = min.y + j as f64;
            border.push(Point::new(min.x, y));
            border.push(Point::new(max.x, y));
        }

        border
    }

    fn energy(&self, next: &Vec<Point>) -> f64 {
        self.points
            .par_iter()
            .zip(next.par_iter())
            .map(|(p1, p2)| nalgebra::distance(p1, p2))
            .sum()
    }

	pub fn run(&mut self) -> &Vec<Point> {
		// TODO better manage the loop
		let max_iterations = 1000;
		let mut pb = progress::Bar::new();
		pb.set_job_title("Barycenter step");
		for iteration in 0..max_iterations {
			pb.reach_percent(100*iteration/max_iterations);
			let new_points = self.one_step();
			let cost = self.energy(&new_points);
			self.points = new_points;
		}
		&self.points
	}
    /// Run a barycenter pass
    fn one_step(&mut self) -> Vec<Point> {
        // Build the delaunay structure
        let mut delaunay = FloatDelaunayTriangulation::with_tree_locate();
        // Add the border // we do not care of their handles
        self.border.iter().for_each(|point| {
            delaunay.insert(*point);
        });

        // Add the points
        let handles = self
            .points
            .iter()
            .map(|point| delaunay.insert(*point))
            .collect::<Vec<_>>();
        let delaunay = delaunay;

        // Compute the barycenter of each point
        let new_points = handles
            .par_iter()
            .map(|handle| {
                let vertex = delaunay.vertex(*handle);
                let (nb, sum) =
                    vertex
                        .ccw_out_edges()
                        .fold((0, Point::new(0.0, 0.0)), |(nb, sum), point| {
                            (nb + 1, {
                                let position = point.to().position();
                                Point::new(sum.x + position.x, sum.y + position.y)
                            })
                        });
                sum / f64::from(nb)
            })
            .collect::<Vec<_>>();
        new_points
    }
}
