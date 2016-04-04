

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum KeyPress {
    NULL,
    CTRL_A,
    CTRL_B,
    CTRL_C,
    CTRL_D,
    CTRL_E,
    CTRL_F,
    CTRL_G,
    CTRL_H,
    TAB,
    CTRL_J,
    CTRL_K,
    CTRL_L,
    ENTER,
    CTRL_N,
    CTRL_P,
    CTRL_R,
    CTRL_S,
    CTRL_T,
    CTRL_U,
    CTRL_W,
    CTRL_Y,
    ESC,
    BACKSPACE,
    UNKNOWN_ESC_SEQ,
    ESC_SEQ_DELETE,
    ESC_Y,
}

pub fn char_to_key_press(c: char) -> KeyPress {
    match c {
        '\x00' => KeyPress::NULL,
        '\x01' => KeyPress::CTRL_A,
        '\x02' => KeyPress::CTRL_B,
        '\x03' => KeyPress::CTRL_C,
        '\x04' => KeyPress::CTRL_D,
        '\x05' => KeyPress::CTRL_E,
        '\x06' => KeyPress::CTRL_F,
        '\x07' => KeyPress::CTRL_G,
        '\x08' => KeyPress::CTRL_H,
        '\x09' => KeyPress::TAB,
        '\x0a' => KeyPress::CTRL_J,
        '\x0b' => KeyPress::CTRL_K,
        '\x0c' => KeyPress::CTRL_L,
        '\x0d' => KeyPress::ENTER,
        '\x0e' => KeyPress::CTRL_N,
        '\x10' => KeyPress::CTRL_P,
        '\x12' => KeyPress::CTRL_R,
        '\x13' => KeyPress::CTRL_S,
        '\x14' => KeyPress::CTRL_T,
        '\x15' => KeyPress::CTRL_U,
        '\x17' => KeyPress::CTRL_W,
        '\x19' => KeyPress::CTRL_Y,
        '\x1b' => KeyPress::ESC,
        '\x7f' => KeyPress::BACKSPACE,
        _ => KeyPress::NULL,
    }
}
