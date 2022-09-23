use crate::config::{Config, Line, LineType};
use pyo3::prelude::*;
use std::collections::HashMap;
use std::f64::consts::PI;

/// A general data structure to represent 3d coordinates.
#[pyclass]
#[derive(Clone)]
pub struct Coordinate {
    #[pyo3(get)]
    pub x: f64,
    #[pyo3(get)]
    pub y: f64,
    #[pyo3(get)]
    pub z: f64,
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
    #[pyo3(get)]
    pub coords: Coordinate,
}

impl Node {
    fn new() -> Node {
        Node {
            tension: 0.0,
            declination_angle: 0.0,
            arc_length: 0.0,
            x_corr: 0.0,
            y_corr: 0.0,
            coords: Coordinate {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

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
        let config = Config::from_file(filename)?;
        let system = MooringSystem::new(config);
        Ok(system)
    }

    /// Construct a new MooringSystem from a Config.
    #[new]
    fn new(config: Config) -> Self {
        MooringSystem { config }
    }

    /// Solve the static equilibrium of the system.
    /// Returns a hash map whose key is a reference to the line name and value is
    /// a vector of `Node` objects.
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

/// Rust-only methods
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
        let submerged_weight = submerged_weight;
        let submerged_length = submerged_length;

        dbg!(total_length, submerged_length, submerged_weight);

        let depth = line.top_position[2] - line.bottom_position[2];
        let mut err_lower = 0.1 * depth;
        let mut err_upper = depth;
        let mut phi_lower = 0.0;
        let mut phi_upper = 89.0 * PI / 180.0;

        let max_it = 100;

        let mut nodes: Vec<Node> = vec![Node::new(); total_num_elements + 1];

        // TODO: This assumes no pre-tension
        let top_tension = submerged_weight;

        // Here, we will iterate through multiple times until the solution converges
        for i in 0..max_it {
            // Set the top angle in first and second iterations (to bound solution)
            let top_ang = match i {
                0 => phi_lower,
                1 => phi_upper,
                _ => phi_lower - err_lower * (phi_upper - phi_lower) / (err_upper - err_lower),
            };
            dbg!(top_ang);

            self.calculate_line_shape(top_tension, top_ang, line, &mut nodes);

            for (i, node) in nodes.iter().enumerate() {
                println!("Node {:3}, arc_length={:.5e}, tension={:.5e}, declination_angle={:.5e}, x_corr={:.5e}, y_corr={:.5e}",
                         i, node.arc_length, node.tension, node.declination_angle, node.x_corr, node.y_corr);
            }

            let err = -(nodes.last().unwrap().y_corr + depth);
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
            top_ang = phi_lower - err_lower * (phi_upper - phi_lower) / (err_upper - err_lower);
        }

        self.rotate_nodes(line, &mut nodes);

        nodes
    }

    /// For a given top tension and angle, integrate the position and tension
    /// at all nodes for a specific line.
    /// Uses Runge-Kutta 4th order integration scheme, starting from top node.
    fn calculate_line_shape(
        &self,
        top_tension: f64,
        top_ang: f64,
        line: &Line,
        nodes: &mut Vec<Node>,
    ) {
        // Set the properties at the top node, before integration
        let mut top_node = nodes.first_mut().unwrap();
        top_node.tension = top_tension;
        top_node.declination_angle = top_ang;
        top_node.x_corr = 0.0;
        top_node.y_corr = 0.0;
        top_node.arc_length = 0.0;

        for node_index in 0..(nodes.len() - 1) {
            let mut count = 0;
            let mut seg = 0.0;
            let mut line_type_name = &String::new();
            for segment in &line.segments {
                count += segment.num_elements;
                if node_index < count {
                    seg = segment.element_length();
                    line_type_name = &segment.line_type;
                    break;
                }
            }
            let line_type = &self.config.line_types[line_type_name];

            let current_node = &nodes[node_index];
            let mut y = [
                current_node.tension,
                current_node.declination_angle,
                current_node.x_corr,
                current_node.y_corr,
            ];
            let y_init = y;
            // dim 0 is iteration k, dim 1 is [tension, phi, x, y]
            let mut slope = [[0.0; 4]; 4];

            // There are 4 iterations in Runge-Kutta method
            for k in 0..4 {
                slope[k] = self.rhs(line_type, &y);

                let step = match k {
                    2 => seg, // 2  because we set the new y before the next iteration
                    _ => seg / 2.0,
                };
                // New y at which to evaluate RHS
                for i in 0..4 {
                    y[i] = y_init[i] + step * slope[k][i];
                }
                dbg!(&y);
            }

            let mut y_solved = y_init;
            for i in 0..y_solved.len() {
                y_solved[i] +=
                    seg / 6.0 * (slope[0][i] + 2.0 * slope[1][i] + 2.0 * slope[2][i] + slope[3][i]);
            }
            nodes[node_index + 1].tension = y_solved[0];
            nodes[node_index + 1].declination_angle = y_solved[1];
            nodes[node_index + 1].x_corr = y_solved[2];
            nodes[node_index + 1].y_corr = y_solved[3];
            nodes[node_index + 1].arc_length = nodes[node_index].arc_length + seg;
        }
    }

    /// Rotate the nodes from in-plane to 3d space, and set the coords property of the nodes.
    fn rotate_nodes(&self, line: &Line, nodes: &mut Vec<Node>) {
        let dx = line.bottom_position[0] - line.top_position[0];
        let dy = line.bottom_position[1] - line.top_position[1];
        let spread_angle = dy.atan2(dx);
        dbg!(dx, dy, spread_angle);

        for node in nodes.iter_mut() {
            node.coords.x = line.top_position[0] + node.x_corr.abs() * spread_angle.cos();
            node.coords.y = line.top_position[1] + node.x_corr.abs() * spread_angle.sin();
            node.coords.z = line.top_position[2] + node.y_corr;
        }
    }

    /// Calculate the right-hand side of the catenary equation.
    /// This is used when performing RK4 integration to solve the differential equation.
    fn rhs(&self, line_type: &LineType, y: &[f64; 4]) -> [f64; 4] {
        let wetted_weight = self.config.general.gravity
            * (line_type.total_mass_per_length()
                - self.config.general.water_density * line_type.external_area());
        let stretch = y[0] / line_type.axial_stiffness();
        // These have been negated to prevent the need for the step to be negative ...
        [
            -wetted_weight * y[1].sin(),
            -wetted_weight * y[1].cos() / y[0],
            -(1.0 + stretch) * y[1].cos(),
            -(1.0 + stretch) * y[1].sin(),
        ]
    }
}
