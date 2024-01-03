use coffee::graphics::{Color, Frame, Window, WindowSettings};
use coffee::input::KeyboardAndMouse;
use coffee::load::Task;
use coffee::{Game, Result, Timer};

mod astar;
mod node;

use astar::AStar;

fn main() -> Result<()> {
    AStar::run(WindowSettings {
        title: String::from("A* pathfinding"),
        size: (800, 800),
        resizable: false,
        fullscreen: false,
        maximized: false,
    })
}

impl Game for AStar {
    const TICKS_PER_SECOND: u16 = 60;
    type Input = KeyboardAndMouse;
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<AStar> {
        // Load your game assets here. Check out the `load` module!
        Task::succeed(|| AStar::new())
    }

    fn interact(&mut self, input: &mut Self::Input, _window: &mut Window) {
        self.mouse = Some(input.mouse().clone());
        self.keyboard = Some(input.keyboard().clone());
    }

    fn update(&mut self, _window: &Window) {
        // println!("Changed!")
    }

    fn draw(&mut self, frame: &mut Frame, timer: &Timer) {
        if timer.has_ticked() {
            frame.clear(Color::WHITE);

            self.handle_input();
            self.path_find();

            self.draw_nodes(frame);
            self.draw_lines(frame);
        }
    }
}
