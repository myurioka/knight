pub mod fire {
    use crate::game::{Piece, Point, Renderer, Rect, FONT_L, FONT_LEFT, FIRE_WIDTH, FIRE_HEIGHT, State, StateMachine};

    pub const FIRE_RUNNING: [&str; 3] = ["⌇⌇⌇","",""];
    pub struct Fire {
        pub state_machine: StateMachine,
    }
    impl Piece for Fire {
        fn new(position: Point, velocity: Point) -> Self {
            Fire {
                state_machine: StateMachine::Running(State::new(position, velocity)),
            }
        }
        fn get_state_machine(&self) -> StateMachine {
            self.state_machine
        }
        fn set_state_machine(&mut self, _state_machine: StateMachine) {
            self.state_machine = _state_machine.update();
        }
        fn draw(&self, renderer: &Renderer) {
            renderer.draw_text(
                &Rect {
                    x: self.state_machine.context().position.x,
                    y: self.state_machine.context().position.y + FIRE_HEIGHT / 2,
                    width: FIRE_WIDTH,
                    height: FIRE_HEIGHT,
                    character: FIRE_RUNNING,
                    font_size: FONT_L,
                    font_align: FONT_LEFT,
                }
            );
        }
    }
}