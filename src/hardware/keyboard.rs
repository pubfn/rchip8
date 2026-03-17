use raylib::prelude::*;

pub struct Keyboard {}

static KEY_BINDINGS: [KeyboardKey; 16] = [
    KeyboardKey::KEY_X,
    KeyboardKey::KEY_ONE,
    KeyboardKey::KEY_TWO,
    KeyboardKey::KEY_THREE,
    KeyboardKey::KEY_Q,
    KeyboardKey::KEY_W,
    KeyboardKey::KEY_E,
    KeyboardKey::KEY_A,
    KeyboardKey::KEY_S,
    KeyboardKey::KEY_D,
    KeyboardKey::KEY_Z,
    KeyboardKey::KEY_C,
    KeyboardKey::KEY_FOUR,
    KeyboardKey::KEY_R,
    KeyboardKey::KEY_F,
    KeyboardKey::KEY_V,
];

impl Keyboard
{
    pub fn get_key(rl: &RaylibHandle) -> Option<usize>
    {
        for i in 0..KEY_BINDINGS.len()
        {
            if rl.is_key_down(KEY_BINDINGS[i])
            {
                return Some(i);
            }
        }
        None
    }

    pub fn get_key_pressed(rl: &RaylibHandle) -> Option<usize>
    {
        for i in 0..KEY_BINDINGS.len()
        {
            if rl.is_key_pressed(KEY_BINDINGS[i])
            {
                return Some(i);
            }
        }
        None
    }
}
