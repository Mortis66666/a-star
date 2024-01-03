use crate::node::*;
use coffee::{
    graphics::{Color, Frame, Mesh, Point, Shape},
    input::{
        keyboard::KeyCode,
        mouse::{Button, Mouse},
        Keyboard,
    },
};
use std::collections::HashSet;
use std::time::{Duration, Instant};

pub struct AStar {
    nodes: Vec<Vec<Node>>,
    pub mouse: Option<Mouse>,
    pub keyboard: Option<Keyboard>,
    last_click: Instant,
    click_cooldown: std::time::Duration,
    start_node: Option<(usize, usize)>,
    end_node: Option<(usize, usize)>,
    start_finding: bool,
    open_set: HashSet<(usize, usize)>,
    trace: Option<(usize, usize)>,
    end: bool,
}

impl AStar {
    pub fn new() -> Self {
        let mut nodes = Vec::new();

        for y in 0..40 {
            let mut row = Vec::new();
            for x in 0..40 {
                let node = Node::new(x, y);
                row.push(node);
            }
            nodes.push(row);
        }

        Self {
            nodes,
            mouse: None,
            keyboard: None,
            last_click: Instant::now(),
            click_cooldown: Duration::from_millis(100),
            start_node: None,
            end_node: None,
            start_finding: false,
            open_set: HashSet::new(),
            trace: None,
            end: false,
        }
    }

    pub fn handle_input(&mut self) {
        if let Some(keyboard) = self.keyboard.as_mut() {
            if keyboard.is_key_pressed(KeyCode::Space) {
                self.start_finding = true;
            }
        }

        // TODO: fix stupid right click ( or not )

        if self.end_node.is_none() && Instant::now() - self.last_click < self.click_cooldown {
            return;
        }

        if let Some(mouse) = self.mouse.as_mut() {
            if mouse.is_button_pressed(Button::Left) {
                let position = mouse.cursor_position();
                let x = position.x as usize;
                let y = position.y as usize;

                let gx = x / 20;
                let gy = y / 20;

                let target_node = &mut self.nodes[gy][gx];

                let result = match target_node.get_type() {
                    NodeType::Wall => {
                        // target_node.set_empty();
                        // TODO
                        0
                    }
                    NodeType::Empty => {
                        target_node.set_wall();
                        0
                    }
                    _ => 1,
                };

                if result == 1 {
                    return;
                }

                if self.start_node.is_none() {
                    target_node.set_start();
                    target_node.g = 0;
                    // target_node.h = 100 * 100;
                    self.start_node = Some((gx, gy));
                    self.open_set.insert((gx, gy));
                } else if self.end_node.is_none() {
                    target_node.set_end();
                    self.end_node = Some((gx, gy));
                }
            }
        } else {
            panic!("Mouse not initialized");
        };

        self.last_click = Instant::now();
    }

    pub fn draw_lines(&mut self, frame: &mut Frame) {
        let mut mesh = Mesh::new();
        for i in 0..40 {
            // println!("i: {}", i);
            mesh.stroke(
                Shape::Polyline {
                    points: vec![
                        Point::new((i * 20) as f32, 0.0),
                        Point::new((i * 20) as f32, 800.0),
                    ],
                },
                Color::BLACK,
                2.0,
            );
            mesh.stroke(
                Shape::Polyline {
                    points: vec![
                        Point::new(0.0, (i * 20) as f32),
                        Point::new(800.0, (i * 20) as f32),
                    ],
                },
                Color::BLACK,
                2.0,
            );
        }

        mesh.draw(&mut frame.as_target())
    }

    pub fn draw_nodes(&mut self, frame: &mut Frame) {
        let mut mesh = Mesh::new();
        for row in self.nodes.iter_mut() {
            for node in row.iter_mut() {
                mesh.fill(node.shape(), node.color());
            }
        }
        mesh.draw(&mut frame.as_target())
    }

    pub fn path_find(&mut self) {
        if self.end {
            return;
        }

        if let Some((x, y)) = &self.trace {
            self.nodes[*y][*x].set_path();
            if let Some(parent) = &self.nodes[*y][*x].parent {
                self.trace = Some((parent.x as usize, parent.y as usize));
            } else {
                self.end = true;
            }
        }

        if self.start_finding {
            let (x, y) = self.find_best_node();
            let mut node = self.nodes[y][x].clone();

            let (ex, ey) = self.end_node.unwrap();
            let end_node = self.nodes[ey][ex].clone();

            if *node.get_type() != NodeType::Start {
                self.nodes[y][x].set_expanded();
            }

            for (nx, ny) in self.find_neighbors((x, y)) {
                let neighbor = &mut self.nodes[ny][nx];
                if *neighbor.get_type() == NodeType::Wall || *neighbor.get_type() == NodeType::Start
                {
                    continue;
                }

                let g = node.get_g() + 1;
                if g < neighbor.get_g() {
                    // neighbor.g = g;
                    neighbor.set_parent(Box::new(node.clone()));
                }

                neighbor.h = neighbor.distance(&end_node);

                match neighbor.get_type() {
                    NodeType::Empty => {
                        neighbor.set_explored();
                        self.open_set.insert((nx, ny));
                    }
                    NodeType::End => {
                        self.trace = Some((node.x as usize, node.y as usize));
                    }
                    _ => (),
                }
            }
        }
    }

    fn find_best_node(&mut self) -> (usize, usize) {
        let result = *self
            .open_set
            .iter()
            .min_by(|a, b| {
                let af = self.nodes[a.1][a.0].get_f();
                let bf = self.nodes[b.1][b.0].get_f();
                af.partial_cmp(&bf).unwrap()
            })
            .unwrap();

        self.open_set.remove(&result);

        result
    }

    fn find_neighbors(&self, coordinate: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = coordinate;
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < 39 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < 39 {
            neighbors.push((x, y + 1));
        }
        neighbors
    }
}
