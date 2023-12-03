#[derive(Clone, Default, PartialEq)]
pub enum Shape {
    Quad,
    Cube,
    #[default]
    Sphere,
    Custom {
        vertices: Vec<[f32; 3]>,
        indices: Vec<u32>,
    },
}

impl Shape {
    pub fn vertexs(&self) -> Vec<[f32; 3]> {
        match self {
            Shape::Quad => {
                vec![
                    [1.0, 1.0, 0.0],
                    [-1.0, 1.0, 0.0],
                    [-1.0, -1.0, 0.0],
                    [1.0, -1.0, 0.0],
                ]
            }

            Shape::Cube => {
                vec![
                    // Front face
                    [0.5, 0.5, 0.5],
                    [-0.5, 0.5, 0.5],
                    [-0.5, -0.5, 0.5],
                    [0.5, -0.5, 0.5],
                    // Back face
                    [-0.5, -0.5, -0.5],
                    [0.5, -0.5, -0.5],
                    [0.5, 0.5, -0.5],
                    [-0.5, 0.5, -0.5],
                    // Top face
                    [0.5, 0.5, -0.5],
                    [0.5, 0.5, 0.5],
                    [-0.5, 0.5, 0.5],
                    [-0.5, 0.5, -0.5],
                    // Bottom face
                    [-0.5, -0.5, -0.5],
                    [-0.5, -0.5, 0.5],
                    [0.5, -0.5, 0.5],
                    [0.5, -0.5, -0.5],
                    // Left face
                    [-0.5, 0.5, -0.5],
                    [-0.5, -0.5, -0.5],
                    [-0.5, -0.5, 0.5],
                    [-0.5, 0.5, 0.5],
                    // Right face
                    [0.5, 0.5, 0.5],
                    [0.5, -0.5, 0.5],
                    [0.5, -0.5, -0.5],
                    [0.5, 0.5, -0.5],
                ]
            }

            Shape::Sphere => {
                let mut vertices = Vec::new();

                // Generate vertices using an iterative method
                for i in 0..=90 {
                    for j in 0..=360 {
                        let lat = i as f32 / 90.0 * std::f32::consts::PI;
                        let lon = j as f32 / 360.0 * std::f32::consts::PI * 2.0;

                        let x = lat.cos() * lon.sin();
                        let y = lat.sin();
                        let z = lat.cos() * lon.cos();

                        vertices.push([x, y, z]);
                    }
                }

                vertices
            }

            Shape::Custom {
                vertices,
                indices: _,
            } => vertices.clone(),
        }
    }

    pub fn indexs(&self) -> Vec<u32> {
        match self {
            Shape::Quad => {
                vec![0, 1, 2, 2, 3, 0]
            }

            Shape::Cube => {
                vec![
                    // Front face
                    0, 1, 2, 2, 3, 0, // Back face
                    4, 5, 6, 6, 7, 4, // Top face
                    8, 9, 10, 10, 11, 8, // Bottom face
                    12, 13, 14, 14, 15, 12, // Left face
                    16, 17, 18, 18, 19, 16, // Right face
                    20, 21, 22, 22, 23, 20,
                ]
            }

            Shape::Sphere => {
                let mut indices = Vec::new();

                // Generate indices using an iterative method
                for i in 0..=90 {
                    for j in 0..=360 {
                        let lat = i as f32 / 90.0 * std::f32::consts::PI;
                        let lon = j as f32 / 360.0 * std::f32::consts::PI * 2.0;

                        let i_prev = if i == 0 { 90 } else { i - 1 };
                        let j_prev = if j == 0 { 360 } else { j - 1 };
                        let i_next = if i == 90 { 0 } else { i + 1 };
                        let j_next = if j == 360 { 0 } else { j + 1 };

                        indices.push(i * 360 + j);
                        indices.push(i_prev * 360 + j);
                        indices.push(i * 360 + j_next);

                        indices.push(i_prev * 360 + j_next);
                        indices.push(i_next * 360 + j);
                        indices.push(i_next * 360 + j_next);
                    }
                }

                indices
            }

            Shape::Custom {
                vertices: _,
                indices,
            } => indices.clone(),
        }
    }
}

impl ToString for Shape {
    fn to_string(&self) -> String {
        match self {
            Shape::Quad => "Quad".to_string(),
            Shape::Cube => "Cube".to_string(),
            Shape::Sphere => "Sphere".to_string(),
            Shape::Custom { .. } => unimplemented!(),
        }
    }
}
