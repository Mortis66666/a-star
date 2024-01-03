use coffee::graphics::{Color, Rectangle, Shape};

#[derive(PartialEq, Clone, Copy)]
pub enum NodeType {
    Start,
    End,
    Wall,
    Empty,
    Expanded,
    Explored,
    Path,
}

#[derive(Clone)]
pub struct Node {
    pub x: i32,
    pub y: i32,
    node_type: NodeType,
    pub g: i32,
    pub h: i32,
    pub parent: Option<Box<Node>>,
}

impl Node {
    pub fn new(x: i32, y: i32) -> Node {
        Node {
            x,
            y,
            node_type: NodeType::Empty,
            g: 80 * 80,
            h: 80 * 80,
            parent: None,
        }
    }

    pub fn get_type(&mut self) -> &NodeType {
        return &mut self.node_type;
    }

    pub fn shape(&mut self) -> Shape {
        Shape::Rectangle(Rectangle {
            x: (self.x * 20) as f32,
            y: (self.y * 20) as f32,
            width: 20.0,
            height: 20.0,
        })
    }

    pub fn color(&self) -> Color {
        match self.node_type {
            NodeType::Start => Color::BLUE,
            NodeType::End => Color::from_rgb(255, 255, 0),
            NodeType::Wall => Color::from_rgb(0, 0, 0),
            NodeType::Empty => Color::from_rgb(255, 255, 255),
            NodeType::Expanded => Color::GREEN,
            NodeType::Explored => Color::RED,
            NodeType::Path => Color::from_rgb(255, 115, 0),
        }
    }

    pub fn distance(&self, other: &Node) -> i32 {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();

        return dx + dy;
    }

    pub fn set_start(&mut self) {
        self.node_type = NodeType::Start;
    }

    pub fn set_end(&mut self) {
        self.node_type = NodeType::End;
    }

    pub fn set_wall(&mut self) {
        self.node_type = NodeType::Wall;
    }

    pub fn set_expanded(&mut self) {
        self.node_type = NodeType::Expanded;
    }

    pub fn set_explored(&mut self) {
        self.node_type = NodeType::Explored;
    }

    pub fn set_path(&mut self) {
        self.node_type = NodeType::Path;
    }

    pub fn set_parent(&mut self, parent: Box<Node>) {
        self.parent = Some(parent);
    }

    pub fn get_f(&self) -> i32 {
        self.get_g() + self.h
    }

    pub fn get_g(&self) -> i32 {
        if self.node_type == NodeType::Start {
            return 0;
        }

        if let Some(parent) = &self.parent {
            parent.get_g() + 1
        } else {
            80 * 80
        }
    }
}
