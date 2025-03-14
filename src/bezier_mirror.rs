use nalgebra::{Point, SMatrix, SVector, Unit};
use std::io::Write;

use crate::{mirror::Mirror, ray::Ray, DIM};

#[derive(PartialEq, Debug)]
pub struct BezierMirror {
    control_points: Vec<Point<f32, DIM>>,
}

impl Mirror for BezierMirror {
    fn reflect(&self, ray: Ray) -> Vec<(f32, Unit<SMatrix<f32, DIM, DIM>>)> {
        // use the other mirror to reflect the ray
        vec![]
    }
    fn get_type(&self) -> String {
        "bezier".to_string()
    }
}

impl BezierMirror {
    // Method to calculate a point on the Bezier curve
    fn calculate_point(&self, t: f32) -> Point<f32, DIM> {
        let mut point: Point<f32, DIM> = Point::origin();
        let n = self.control_points.len() - 1; // degree of the curve

        for (i, control_point) in self.control_points.iter().enumerate() {
            let bernstein_polynomial = binomial_coefficient(n, i) as f32
                * t.powi(i as i32)
                * (1.0 - t).powi((n - i) as i32);

            for (j, coordinate) in point.iter_mut().enumerate() {
                *coordinate += bernstein_polynomial * control_point[j];
            }
        }

        point
    }

    fn calculate_tangent(&self, t: f32) -> SVector<f32, DIM> {
        let n = self.control_points.len() - 1; // degree of the curve
        let mut tangent: SVector<f32, DIM> = SVector::zeros();

        for i in 0..n {
            let bernstein_derivative = (n as f32)
                * binomial_coefficient(n - 1, i) as f32
                * t.powi(i as i32)
                * (1.0 - t).powi((n - 1 - i) as i32);

            let difference = self.control_points[i + 1] - self.control_points[i];
            tangent += bernstein_derivative * difference;
        }

        tangent.normalize()
    }

    fn from_json(json: &serde_json::Value) -> Self {
        /* example json
        {
            "control_points": [
                [1.0, 2.0, 3.0, ...],
                [4.0, 5.0, 6.0, ...],
                [7.0, 8.0, 9.0, ...],
                ...
            ]
        }
         */
        let control_points = json["control_points"]
            .as_array()
            .unwrap()
            .iter()
            .map(|point| {
                let point = point
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_f64().unwrap() as f32)
                    .collect::<Vec<_>>();

                Point::from_slice(&point)
            })
            .collect::<Vec<_>>();

        Self { control_points }
    }
}

// Function to calculate binomial coefficients
fn binomial_coefficient(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }

    let mut result = 1;
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn complete_with_0(mut vec: Vec<f32>) -> Vec<f32> {
        vec.resize(DIM, 0.0);
        vec
    }

    #[test]
    fn test_binomial_coefficient() {
        assert_eq!(binomial_coefficient(0, 0), 1);
        assert_eq!(binomial_coefficient(1, 0), 1);
        assert_eq!(binomial_coefficient(1, 1), 1);
        assert_eq!(binomial_coefficient(2, 0), 1);
        assert_eq!(binomial_coefficient(2, 1), 2);
        assert_eq!(binomial_coefficient(2, 2), 1);
        assert_eq!(binomial_coefficient(3, 0), 1);
        assert_eq!(binomial_coefficient(3, 1), 3);
        assert_eq!(binomial_coefficient(3, 2), 3);
        assert_eq!(binomial_coefficient(3, 3), 1);
        assert_eq!(binomial_coefficient(4, 0), 1);
        assert_eq!(binomial_coefficient(4, 1), 4);
        assert_eq!(binomial_coefficient(4, 2), 6);
        assert_eq!(binomial_coefficient(4, 3), 4);
        assert_eq!(binomial_coefficient(4, 4), 1);
    }

    #[test]
    fn test_calculate_linear_point_2d() {
        let bezier_mirror = BezierMirror {
            control_points: vec![
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.0, 0.0])),
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![1.0, 1.0])),
            ],
        };
        assert_eq!(
            bezier_mirror.calculate_point(0.0),
            Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.0, 0.0]))
        );
        assert_eq!(
            bezier_mirror.calculate_point(0.5),
            Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.5, 0.5]))
        );
        assert_eq!(
            bezier_mirror.calculate_point(1.0),
            Point::from_slice(&complete_with_0(vec![1.0, 1.0]))
        );
    }

    #[test]
    fn test_calculate_cubic_point_2d() {
        let bezier_mirror = BezierMirror {
            control_points: vec![
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.0, 0.0])),
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.5, 1.0])),
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![1.0, 0.0])),
            ],
        };
        assert_eq!(
            bezier_mirror.calculate_point(0.0),
            Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.0, 0.0]))
        );
        assert_eq!(
            bezier_mirror.calculate_point(0.5),
            Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.5, 0.5]))
        );
        assert_eq!(
            bezier_mirror.calculate_point(1.0),
            Point::from_slice(&complete_with_0(vec![1.0, 0.0]))
        );
    }

    #[test]
    fn test_calculate_quadratic_point_2d() {
        let bezier_mirror = BezierMirror {
            control_points: vec![
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.0, 0.0])),
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.5, 0.0])),
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.5, 1.0])),
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![1.0, 1.0])),
            ],
        };
        assert_eq!(
            bezier_mirror.calculate_point(0.0),
            Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.0, 0.0]))
        );

        assert_eq!(
            bezier_mirror.calculate_point(0.5),
            Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.5, 0.5]))
        );

        assert_eq!(
            bezier_mirror.calculate_point(1.0),
            Point::from_slice(&complete_with_0(vec![1.0, 1.0]))
        );
    }

    #[test]
    fn generate_point_in_csv() {
        //simple function to visualize the bezier curve to check that I dont do shit
        let bezier_mirror = BezierMirror {
            control_points: vec![
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.0, 0.0])),
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.5, 1.0])),
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.0, 1.0])),
            ],
        };

        let mut file = std::fs::File::create("points.csv").unwrap();
        for i in 0..100 {
            let t = i as f32 / 100.0;
            let point = bezier_mirror.calculate_point(t);
            writeln!(file, "{},{}", point[0], point[1]).unwrap();
            println!("{} : {}", t, point);
        }
    }

    #[test]
    fn test_calculate_tangent() {
        let bezier_mirror = BezierMirror {
            control_points: vec![
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.0, 0.0])),
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![0.5, 1.0])),
                Point::<f32, DIM>::from_slice(&complete_with_0(vec![1.0, 0.0])),
            ],
        };

        let vector = bezier_mirror.calculate_tangent(1.0);
        let axis = SVector::<f32, DIM>::from_vec(complete_with_0(vec![1.0, 0.0]));
        let dot_product = vector.dot(&axis);
        let reflected_vector = 2.0 * dot_product * axis - vector;

        assert_eq!(bezier_mirror.calculate_tangent(0.0), reflected_vector);
    }

    #[test]
    fn test_from_json() {
        let json = serde_json::json!({
            "control_points": [
                complete_with_0(vec![1.0, 2.0, 3.0]),
                complete_with_0(vec![4.0, 5.0, 6.0]),
                    complete_with_0(vec![7.0, 8.0, 9.0]),
            ]
        });
        assert_eq!(
            BezierMirror::from_json(&serde_json::to_value(json).unwrap()),
            BezierMirror {
                control_points: vec![
                    Point::<f32, DIM>::from_slice(&complete_with_0(vec![1.0, 2.0, 3.0])),
                    Point::<f32, DIM>::from_slice(&complete_with_0(vec![4.0, 5.0, 6.0])),
                    Point::<f32, DIM>::from_slice(&complete_with_0(vec![7.0, 8.0, 9.0])),
                ],
            }
        );
    }
}
