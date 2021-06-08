use x86_64::instructions::interrupts::without_interrupts;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts::{self, Us104Key}};
use lazy_static::lazy_static;
use spin::Mutex;
use tinix_fs::api::{FileReader, FileInteractor, File};



pub struct RingBuffer32B {
    read_index  : usize  ,
    write_index : usize  ,
    buffer      : [u8;32],

    _private:()
}

impl RingBuffer32B {
    pub fn new() -> RingBuffer32B {
        RingBuffer32B {buffer : [0;32], read_index : 0, write_index : 1, _private : () }
    }

    pub fn peek(&self) -> u8 {
        self.buffer[self.read_index]
    }

    pub fn read(&mut self) -> Option<u8> {
        if !self.is_empty() {
            crate::serial_println!("Reading | Read_index: {}, Write_index: {}", self.read_index, self.write_index);
            let item = self.buffer[self.read_index];
            self.read_index += 1;
            self.read_index %= self.buffer.len();
            Some(item)
        } else {
            None
        }
    }

    pub fn write(&mut self, data : u8) {
        crate::serial_println!("Attempting To Write {} To Scancode Buffer...",data);
        if !self.is_full() {
            crate::serial_println!("Writing | Read_index: {}, Write_index: {}", self.read_index, self.write_index);
            self.buffer[self.write_index] = data;
            self.write_index += 1;
            self.write_index %= self.buffer.len();
        } else {
            crate::serial_println!("Unable To Write To Buffer (Buffer Full) | Read_index: {}, Write_index: {}", self.read_index, self.write_index);
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.write_index > 0 {
            self.read_index == self.write_index - 1
        } else {
            self.read_index == self.write_index
        }
    }

    pub fn is_full(&self) -> bool {
        if self.read_index > 0  {
            self.write_index == self.read_index - 1
        } else {
            false
        }
    }
 
}


static mut SCANCODE_BUFFER : RingBuffer32B = RingBuffer32B {buffer : [0;32], read_index : 0, write_index : 1, _private : () }; 

fn get_scancode() -> Option<u8> {
    without_interrupts(|| {
        unsafe {
            SCANCODE_BUFFER.read()
        }
    })
}

fn peek_scancode() -> u8 {
    unsafe {SCANCODE_BUFFER.peek()}
}

pub(crate) fn add_scancode(scancode : u8) {
    without_interrupts(|| {
        unsafe {
            SCANCODE_BUFFER.write(scancode)
        }
    });
}

pub fn get_decoded_key() -> Option<DecodedKey> {
    if let Some(key) = get_scancode() {
        let mut keyboard = KEYBOARD.lock();
        if let Ok(Some(key_event)) = keyboard.add_byte(key) {
            keyboard.process_keyevent(key_event)
        } else {
            None
        }
    
    } else {
        None
    }
}

pub fn get_ascii_key() -> Option<char> {
    if let Some(key) = get_scancode() {
        let mut keyboard = KEYBOARD.lock();
        if let Ok(Some(key_event)) = keyboard.add_byte(key) {
            if let Some(dk) = keyboard.process_keyevent(key_event) {
                match dk {
                    DecodedKey::Unicode(chr) => Some(chr),
                    DecodedKey::RawKey(_rk) => None
                }
            } else {
                None
            }
        } else {
            None
        }
    
    } else {
        None
    }
}

pub fn get_keycode() -> Option<KeyCode> {
    if let Some(key) = get_scancode() {
        let mut keyboard = KEYBOARD.lock();
        if let Ok(Some(key_event)) = keyboard.add_byte(key) {
            if let Some(dk) = keyboard.process_keyevent(key_event) {
                Some(KeyCode::from_dec_key(dk))
            } else {
                None
            }
        } else {
            None
        }
    
    } else {
        None
    }
}


lazy_static! {
    static ref KEYBOARD : Mutex<Keyboard<Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(
        layouts::Us104Key, 
        ScancodeSet1,
        HandleControl::Ignore
        )
    );
}


pub struct StandardIn {_private : ()}

impl StandardIn {
    pub fn get() -> StandardIn { StandardIn {_private : ()} }
}

impl FileInteractor for StandardIn {
    fn at_end(&self) -> bool {
        (peek_scancode() == 0x1A) |
        (to_ascii_key(peek_scancode()).unwrap_or_default() == '\n')
    }

    fn close(_file : File) {
        
    }

    fn get_pos(_file : File) -> usize {
        0
    }

    fn get_handle(_file : File) -> usize {
        1
    }

    fn is_eof(_file : File) -> bool {
        peek_scancode() == 0x04
    }

    fn open(_path : &str) -> File {
        File::new(1,1)
    }

    fn set_pos(_pos  : usize, _file : File) {
        
    }
}

impl FileReader<char> for StandardIn {
    fn open(_file : &File) -> StandardIn {
        StandardIn {_private : ()}
    }

    fn read(&mut self) -> Option<char> {
        if false {
            return None
        } else {
            get_ascii_key()
        }
    }
}

fn to_ascii_key(scancode : u8) -> Option<char> {
    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(dk) = keyboard.process_keyevent(key_event) {
            match dk {
                DecodedKey::Unicode(chr) => Some(chr),
                DecodedKey::RawKey(_rk) => None
            }
        } else {
            None
        }
    } else {
        None
    }
}



#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum KeyCode {
    NUL = 0,
    SOH,
    STX,
    ETX,
    EOT,
    ENQ,
    ACK,
    BEL,
    BS,
    HT,
    LF,
    VT,
    FF,
    CR,
    SO,
    SI,
    DLE,
    DC1,
    DC2,
    DC3,
    DC4,
    NAK,
    SYN,
    ETB,
    CAN,
    EM,
    FS,
    GS,
    RS,
    US,
    KEY_SPACE,
    KEY_BANG,
    KEY_DQUOTE,
    KEY_HASH,
    KEY_DOLLAR,
    KEY_PERCENT,
    KEY_AMPERSAND,
    KEY_SQUOTE,
    KEY_RIGHT_BRACKET,
    KEY_LEFT_BRACKET,
    KEY_STAR,
    KEY_PLUS,
    KEY_COMMA,
    KEY_MINUS,
    KEY_DOT,
    KEY_FSLASH,
    KEY_0,
    KEY_1,
    KEY_2,
    KEY_3,
    KEY_4,
    KEY_5,
    KEY_6,
    KEY_7,
    KEY_8,
    KEY_9,
    KEY_COLON,
    KEY_SEMI_COLON,
    KEY_RIGHT_ARROW,
    KEY_EQUAL,
    KEY_LEFT_ARROW,
    KEY_QUESTION,
    KEY_AT,
    KEY_A,
    KEY_B,
    KEY_C,
    KEY_D,
    KEY_E,
    KEY_F,
    KEY_G,
    KEY_H,
    KEY_I,
    KEY_J,
    KEY_K,
    KEY_L,
    KEY_M,
    KEY_N,
    KEY_O,
    KEY_P,
    KEY_Q,
    KEY_R,
    KEY_S,
    KEY_T,
    KEY_U,
    KEY_V,
    KEY_W,
    KEY_X,
    KEY_Y,
    KEY_Z,
    KEY_RIGHT_SQUARE_BRACKET,
    KEY_BSLASH,
    KEY_LEFT_SQUARE_BRACKET,
    KEY_CARET,
    KEY_UNDERSCORE,
    KEY_RIGHT_CURLY_BRACKET = 123,
    KEY_PIPE,
    KEY_LEFT_CURLY_BRACKET,
    KEY_TILDE,
    DEL,

    ARROW_UP, ARROW_DOWN, ARROW_LEFT, ARROW_RIGHT
}

impl KeyCode {
    pub fn from_u8(x : usize) -> KeyCode {
        match x {
            0x00 => Self::NUL,
            0x01 => Self::SOH,
            0x02 => Self::STX,
            0x03 => Self::ETX,
            0x04 => Self::EOT,
            0x05 => Self::ENQ,
            0x06 => Self::BS,

            _ => Self::NUL
        }
    }

    pub fn from_dec_key(k : DecodedKey) -> KeyCode {
        match k {
            DecodedKey::RawKey(c) => {
                match c {
                    pc_keyboard::KeyCode::ArrowUp => {KeyCode::ARROW_UP}
                    pc_keyboard::KeyCode::ArrowDown => {KeyCode::ARROW_DOWN}
                    pc_keyboard::KeyCode::ArrowLeft => {KeyCode::ARROW_LEFT}
                    pc_keyboard::KeyCode::ArrowRight => {KeyCode::ARROW_RIGHT}
                   _ => KeyCode::NUL
                }   
            },

            _ => KeyCode::NUL
        }
    }

    pub fn as_char(self) -> char {
        self.as_u8() as char
    }

    pub fn as_u8(self) -> u8 {
        self as u8
    }
}