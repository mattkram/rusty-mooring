use crate::config::{Config, Line, LineType};
use pyo3::prelude::*;
use std::collections::HashMap;
use std::f64::consts::PI;

#[pyclass]
pub struct MooringSystem {
    #[pyo3(get)]
    config: Config,
}

#[pymethods]
impl MooringSystem {
    /// Load the configuration from a TOML file and create a new instance.
    #[staticmethod]
    pub fn from_file(filename: String) -> PyResult<MooringSystem> {
        let config = match Config::from_file(filename) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        let system = MooringSystem::new(config);
        Ok(system)
    }

    /// Construct a new MooringSystem from a Config.
    #[new]
    fn new(config: Config) -> Self {
        MooringSystem { config: config }
    }

    /// Solve the static equilibrium of the system.
    fn solve_static(&self) -> HashMap<&String, Vec<Node>> {
        let mut results: HashMap<&String, Vec<Node>> = HashMap::new();
        for (line_name, line) in self.config.lines.iter() {
            let nodes = self.solve_catenary_equation(line);
            results.insert(line_name, nodes);
        }

        results
    }

    /// Return a vector of all coordinates along each line.
    /// The return type is a mapping of line name to a vector of 3d coordinates.
    fn get_line_coordinates(&self) -> HashMap<String, Vec<[f64; 3]>> {
        HashMap::new()
    }
}

/// A data structure to store the nodal properties along a line.
#[pyclass]
#[derive(Clone)]
pub struct Node {
    #[pyo3(get)]
    pub tension: f64,
    #[pyo3(get)]
    pub declination_angle: f64,
    #[pyo3(get)]
    pub arc_length: f64,
    #[pyo3(get)]
    pub x_corr: f64,
    #[pyo3(get)]
    pub y_corr: f64,
}

impl Node {
    fn new() -> Node {
        Node {
            tension: 0.0,
            declination_angle: 0.0,
            arc_length: 0.0,
            x_corr: 0.0,
            y_corr: 0.0,
        }
    }
}

impl MooringSystem {
    /// Solve the catenary equation for a specific line.
    fn solve_catenary_equation(&self, line: &Line) -> Vec<Node> {
        let mut total_num_elements = 0;
        let mut total_length = 0.0;
        let mut submerged_length = 0.0;
        let mut submerged_weight = 0.0;
        for segment in line.segments.iter() {
            total_num_elements += segment.num_elements;
            total_length += segment.length;

            // FIXME: this is wrong because the submerged length is at an angle,
            //        and depth is vertical
            if total_length + segment.length <= self.config.general.water_depth {
                submerged_length += segment.length;
            }

            let line_type = match self.config.line_types.get(&segment.line_type) {
                Some(l) => l,
                _ => panic!("No line type with name '{}' specified", segment.line_type),
            };

            submerged_weight += segment.length
                * self.config.general.gravity
                * (line_type.total_mass_per_length()
                    - self.config.general.water_density * line_type.external_area());
        }

        // Make immutable after the summing
        let total_length = total_length;
        let total_num_elements = total_num_elements;

        println!("total_length: {:?}", total_length);
        println!("submerged_length: {:?}", submerged_length);
        println!("submerged_weight: {:?}", submerged_weight);

        // TODO: This assumes no pre-tension
        let top_tension = submerged_weight;
        println!("top_tension: {:?}", top_tension);

        let depth = line.top_position[2] - line.bottom_position[2];
        // TODO: Remove the magic 2000 and figure out a better heuristic for lower-bound
        let err_depth = -depth / 2000.0;
        let mut err = -depth;
        let mut err_lower = depth / 10.0;
        let mut err_upper = err;
        let mut phi_lower = 0.0;
        let mut phi_upper = 89.0;

        println!("err_depth: {:?}, err: {:?}", err_depth, err);

        let num_nodes = (total_num_elements + 1) as usize;
        let mut node_index;
        let max_it = 100;
        let mut top_new = 0.0;

        let mut nodes: Vec<Node> = vec![Node::new(); num_nodes];

        let mut y: Vec<f64> = vec![0.0; 4];
        let mut y0 = 0.0;
        let mut y1 = 0.0;
        let mut y2 = 0.0;
        let mut y3 = 0.0;
        let mut f0 = vec![0.0; 4];
        let mut f1 = vec![0.0; 4];
        let mut f2 = vec![0.0; 4];
        let mut f3 = vec![0.0; 4];

        // Here, we will iterate through multiple times until the solution converges
        for i in 0..max_it {
            node_index = num_nodes - 1;
            // Set the top angle
            let top_ang = match i {
                0 => 0.0,
                1 => 89.0 * PI / 180.0,
                _ => top_new,
            };
            println!("top_ang: {:?}", top_ang);

            nodes[node_index].tension = top_tension;
            nodes[node_index].declination_angle = top_ang;
            nodes[node_index].x_corr = 0.0;
            nodes[node_index].y_corr = 0.0;
            nodes[node_index].arc_length = total_length;

            for j in 0..40 {
                let current_node = &nodes[node_index];
                y[0] = current_node.tension;
                y[1] = current_node.declination_angle;
                y[2] = current_node.x_corr;
                y[3] = current_node.y_corr;
                println!("{:?}", y);

                // TODO: This is hard-coded
                let seg = if j < 10 {
                    line.segments[0].element_length()
                } else if j < 30 {
                    line.segments[1].element_length()
                } else {
                    line.segments[2].element_length()
                };
                // We negate because we are going in decreasing arclength ...
                let seg = -seg;

                let line_type_name = if j < 10 {
                    &line.segments[0].line_type
                } else if j < 30 {
                    &line.segments[1].line_type
                } else {
                    &line.segments[2].line_type
                };
                let line_type = &self.config.line_types[line_type_name];

                for k in 0..4 {
                    if k == 0 {
                        y0 = y[0];
                        y1 = y[1];
                        y2 = y[2];
                        y3 = y[3];
                    }
                    (f0[k], f1[k], f2[k], f3[k]) = self.rhs(line_type, &y);

                    let coeff = match k {
                        2 => 1.0,
                        _ => 2.0,
                    };
                    println!("{}", seg);
                    y[0] = y0 + f0[k] * seg / coeff;
                    y[1] = y1 + f1[k] * seg / coeff;
                    y[2] = y2 + f2[k] * seg / coeff;
                    y[3] = y3 + f3[k] * seg / coeff;
                    println!("{:?}", y);
                }

                nodes[node_index - 1].tension = nodes[node_index].tension
                    + seg * (f0[0] + 2.0 * f0[1] + 2.0 * f0[2] + f0[3]) / 6.0;
                nodes[node_index - 1].declination_angle = nodes[node_index].declination_angle
                    + seg * (f1[0] + 2.0 * f1[1] + 2.0 * f1[2] + f1[3]) / 6.0;
                nodes[node_index - 1].x_corr = nodes[node_index].x_corr
                    + seg * (f2[0] + 2.0 * f2[1] + 2.0 * f2[2] + f2[3]) / 6.0;
                nodes[node_index - 1].y_corr = nodes[node_index].y_corr
                    + seg * (f3[0] + 2.0 * f3[1] + 2.0 * f3[2] + f3[3]) / 6.0;

                nodes[node_index - 1].arc_length = nodes[node_index].arc_length - seg;
                node_index -= 1;
            }

            for (i, node) in nodes.iter().enumerate() {
                println!("Node {:3}, arc_length={:.5e}, tension={:.5e}, declination_angle={:.5e}, x_corr={:.5e}, y_corr={:.5e}",
                         i, node.arc_length, node.tension, node.declination_angle, node.x_corr, node.y_corr);
            }

            err = -(nodes[0].y_corr + depth);
            if i == 0 {
                err_lower = err;
                phi_lower = top_ang;
            } else if i == 1 {
                err_upper = err;
                phi_upper = top_ang;
            } else if (err_lower * err) > 0.0 {
                err_lower = err;
                phi_lower = top_ang;
            } else if (err_upper * err) > 0.0 {
                err_upper = err;
                phi_upper = top_ang;
            }
            top_new = phi_lower - err_lower * (phi_upper - phi_lower) / (err_upper - err_lower);
            println!("top_new = {}", top_new);
        }

        nodes
    }

    fn rhs(&self, line_type: &LineType, y: &Vec<f64>) -> (f64, f64, f64, f64) {
        let wetted_weight = self.config.general.gravity
            * (line_type.total_mass_per_length()
                - self.config.general.water_density * line_type.external_area());
        let stretch = y[0] / line_type.axial_stiffness();
        (
            wetted_weight * y[1].sin(),
            wetted_weight * y[1].cos() / y[0],
            (1.0 + stretch) * y[1].cos(),
            (1.0 + stretch) * y[1].sin(),
        )
    }
}
