
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
    Reload,
    Sprint,
}

#[derive(Debug, Clone)]
pub struct Keybinds {
    code_to_action: HashMap<String, Action>,
}

impl Default for Keybinds {
    fn default() -> Self {
        let mut code_to_action = HashMap::new();

        code_to_action.insert("KeyW".to_string(), Action::MoveForward);
        code_to_action.insert("ArrowUp".to_string(), Action::MoveForward);

        code_to_action.insert("KeyS".to_string(), Action::MoveBackward);
        code_to_action.insert("ArrowDown".to_string(), Action::MoveBackward);

        code_to_action.insert("KeyA".to_string(), Action::MoveLeft);
        code_to_action.insert("ArrowLeft".to_string(), Action::MoveLeft);

        code_to_action.insert("KeyD".to_string(), Action::MoveRight);
        code_to_action.insert("ArrowRight".to_string(), Action::MoveRight);

        code_to_action.insert("Space".to_string(), Action::Jump);
        code_to_action.insert("KeyR".to_string(), Action::Reload);

        code_to_action.insert("ShiftLeft".to_string(), Action::Sprint);
        code_to_action.insert("ShiftRight".to_string(), Action::Sprint);

        Self { code_to_action }
    }
}

impl Keybinds {
    pub fn on_key_down(&self, input: &mut InputState, code: &str) {
        let Some(action) = self.code_to_action.get(code).copied() else {
            return;
        };

        match action {
            Action::MoveForward => input.up = true,
            Action::MoveBackward => input.down = true,
            Action::MoveLeft => input.left = true,
            Action::MoveRight => input.right = true,
            Action::Jump => { input.jump_pressed = true; },
            Action::Reload => input.reload = true,
            Action::Sprint => input.sprint = true,
        }
    }

    pub fn on_key_up(&self, input: &mut InputState, code: &str) {
        let Some(action) = self.code_to_action.get(code).copied() else {
            return;
        };

        match action {
            Action::MoveForward => input.up = false,
            Action::MoveBackward => input.down = false,
            Action::MoveLeft => input.left = false,
            Action::MoveRight => input.right = false,
            Action::Jump => {},
            Action::Reload => input.reload = false,
            Action::Sprint => input.sprint = false,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub shoot: bool,
    pub sprint: bool,
    pub reload: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_dx: f32,
    pub mouse_dy: f32,
    pub jump_pressed: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn end_frame(&mut self) {
        self.mouse_dx = 0.0;
        self.mouse_dy = 0.0;
        self.jump_pressed = false;
    }
}
