extern crate libusb;

//const INPUT_BUFFER_SIZE:usize = 20;
const XBOX_CONTROLLER_ID: u16 = 654;
const DATAFLOW_INTERFACE: u8 = 0;


fn abs(x: i16) -> i16 {
    if x < 0 {
        -1*(x+1) // add 1 as dirty way to avoid overflow
    } else {
        x
    }
}


//TODO: propper errors instead of String as Error
pub fn get_controller_handle(context: &libusb::Context) -> Result<libusb::DeviceHandle, std::string::String> {
    let xbox_device_optn = get_device_at_address(XBOX_CONTROLLER_ID, &context);
    match xbox_device_optn {
        Some(device) => {
            match device.open() {
                Ok(mut handle) => {

                    // make sure the kernel doesn't have control
                    // of the interface
                    match handle.kernel_driver_active(DATAFLOW_INTERFACE) {
                        Ok(true) => {
                            match handle.detach_kernel_driver(DATAFLOW_INTERFACE) {
                                Err(error) =>
                                    return Err(format!("failed to detach kernel driver:\
                                                        {}", error)),
                                _ => println!("detached kernel"),
                            }
                        }
                        Err(error) => {
                            return Err(format!("failed to check for active kernel: {}", error));
                        }
                        _ => println!("-no kernel attached"),
                    }
                    
                    // claim the interface
                    match handle.claim_interface(DATAFLOW_INTERFACE) {
                        Err(error) =>
                            return Err(format!("failed to claim interface {}: {}",
                                               DATAFLOW_INTERFACE, error)),
                        _ => {},
                    }
                    
                    return Ok(handle);
                }
                Err(error) =>
                    return Err(format!("failed to get handle: {}", error)),
            }
        }
        None =>
            return Err(format!("No Xbox controller found. (XBOX_CONTROLLER_ID \
                 may not match the product ID of the xbox controller you are using, \
                 or controller may not be plugged in.)")),
    }
}

fn get_device_at_address<'a>(id: u16, context: &'a libusb::Context)
                             -> Option<libusb::Device> {
    let mut res: Option<libusb::Device> = None;
    for device in context.devices().unwrap().iter() {
        if device.device_descriptor().unwrap().product_id() == id {
            res = Some(device);
            break;
        }
    }
    res
}   








pub struct ControllerData {
    button_xbox: bool,
    button_back: bool,
    button_start: bool,
    button_a: bool,
    button_b: bool,
    button_x: bool,
    button_y: bool,
    button_dpad_left: bool,
    button_dpad_right: bool,
    button_dpad_up: bool,
    button_dpad_down: bool,
    button_l_bumper: bool,
    button_r_bumper: bool,
    button_l_stick: bool,
    button_r_stick: bool,
    trigger_l: TriggerData,
    trigger_r: TriggerData,
    stick_l: StickData,
    stick_r: StickData,
    //
    changed: bool,
    last_state: ControllerState,
}

#[derive(PartialEq)]
enum DPadState {
    Neutral, // none pressed
    Confused, //multiple pressed
    Down,
    Left,
    Up,
    Right,
}

#[derive(PartialEq)]
enum RButtonState {
    Neutral, // none pressed
    Confused, // multiple pressed
    Down,
    Left,
    Up,
    Right,
    Bumper,
    Trigger,
    Back,
    Start,
    Xbox,
    Stick,
}

#[derive(PartialEq)]
enum ModifierState {
    Neutral,
    Shift,
    Ctrl,
    Alt,
    AltShift,
    AltCtrl,
    Super,
    SuperCtrl,
}

use ControllerState::{ModableReady, ExactReady, MacroReady};
impl ControllerData {

    pub fn state(&self) -> ControllerState {
        if self.last_state == ControllerState::Neutral {
            return self.raw_state();
        } else {
            
            // macros
            match (&self.last_state, self.raw_state()) {
                (ModableReady('i'), ModableReady('v')) =>
                    return MacroReady(Macro::If),
                _ =>
                    return self.raw_state(),
            }
        }
    }
    
    fn raw_state(&self) -> ControllerState {   //TODO: change pub's in this file that need not be pub

        if self.rbutton_state() == RButtonState::Neutral &&
            self.stick_r.state() == StickState::Neutral {
            return ControllerState::Neutral;
        }
        
        // alphabetic/modable
        if (self.dpad_state() == DPadState::Neutral &&
            self.stick_r.state() == StickState::Neutral) {
            match (self.stick_l.state(), self.rbutton_state()) {
                
                // vowels
                (StickState::Neutral, RButtonState::Down) =>
                    return ModableReady('a'),
                (StickState::Neutral, RButtonState::Left) =>
                    return ModableReady('e'),
                (StickState::Neutral, RButtonState::Up) =>
                    return ModableReady('i'),
                (StickState::Neutral, RButtonState::Right) =>
                    return ModableReady('o'),
                (StickState::Neutral, RButtonState::Bumper) =>
                    return ModableReady('u'),
                (StickState::Neutral, RButtonState::Trigger) =>
                    return ModableReady('y'),

                // mid-buttons
                (StickState::Neutral, RButtonState::Back) =>
                    return MacroReady(Macro::Undo),
                (StickState::Neutral, RButtonState::Start) =>
                    return MacroReady(Macro::Tab),
                (StickState::Neutral, RButtonState::Xbox) =>
                    return MacroReady(Macro::Super),
                
                // non-voiced
                (StickState::Up(_), RButtonState::Down) =>
                    return ModableReady('p'),
                (StickState::Up(_), RButtonState::Left) =>
                    return ModableReady('t'),
                (StickState::Up(_), RButtonState::Up) =>
                    return ModableReady('f'),
                (StickState::Up(_), RButtonState::Right) =>
                    return ModableReady('s'),
                (StickState::Up(_), RButtonState::Bumper) =>
                    return ModableReady('c'),
                (StickState::Up(_), RButtonState::Trigger) =>
                    return ModableReady('k'),

                // voiced
                (StickState::Down(_), RButtonState::Down) =>
                    return ModableReady('b'),
                (StickState::Down(_), RButtonState::Left) =>
                    return ModableReady('d'),
                (StickState::Down(_), RButtonState::Up) =>
                    return ModableReady('v'),
                (StickState::Down(_), RButtonState::Right) =>
                    return ModableReady('z'),
                (StickState::Down(_), RButtonState::Bumper) =>
                    return ModableReady('g'),
                (StickState::Down(_), RButtonState::Trigger) =>
                    return ModableReady('j'),

                // slides
                (StickState::Right(_), RButtonState::Down) =>
                    return ModableReady('h'),
                (StickState::Right(_), RButtonState::Left) =>
                    return ModableReady('l'),
                (StickState::Right(_), RButtonState::Up) =>
                    return ModableReady('r'),
                (StickState::Right(_), RButtonState::Right) =>
                    return ModableReady('w'),

                // odds
                (StickState::Left(_), RButtonState::Down) =>
                    return ModableReady('m'),
                (StickState::Left(_), RButtonState::Left) =>
                    return ModableReady('n'),
                (StickState::Left(_), RButtonState::Up) =>
                    return ModableReady('q'),
                (StickState::Left(_), RButtonState::Right) =>
                    return ModableReady('x'),

                // white space
                (StickState::Neutral, RButtonState::Stick) => {
                    if self.modifier_state() == ModifierState::Shift {
                        return MacroReady(Macro::Enter);
                    }
                    return ExactReady(' ');
                }

                // else move on
                _ => (),
            }
        }
        
        else if (self.dpad_state() != DPadState::Neutral &&
                 self.stick_r.state() == StickState::Neutral) {
            match (self.dpad_state(), self.rbutton_state(),
                   self.modifier_state()) {
                
                // numbers (low)
                (DPadState::Down, RButtonState::Down, _) =>
                    return ModableReady('1'),
                (DPadState::Down, RButtonState::Left, _) =>
                    return ModableReady('2'),
                (DPadState::Down, RButtonState::Up, _) =>
                    return ModableReady('3'),
                (DPadState::Down, RButtonState::Right, _) =>
                    return ModableReady('4'),
                (DPadState::Down, RButtonState::Bumper, _) =>
                    return ModableReady('5'),
                (DPadState::Down, RButtonState::Trigger, _) =>
                    return ModableReady('6'),
                
                // numbers (high)
                (DPadState::Up, RButtonState::Down, _) =>
                    return ModableReady('7'),
                (DPadState::Up, RButtonState::Left, _) =>
                    return ModableReady('8'),
                (DPadState::Up, RButtonState::Up, _) =>
                    return ModableReady('9'),
                (DPadState::Up, RButtonState::Right, _) =>
                    return ModableReady('0'),
                
                // (+/- configuration on the keyboard is dumb so i changed it)
                (DPadState::Up, RButtonState::Bumper, ModifierState::Neutral) =>
                    return ExactReady('-'),
                (DPadState::Up, RButtonState::Trigger, ModifierState::Neutral) =>
                    return ExactReady('+'),
                (DPadState::Up, RButtonState::Bumper, ModifierState::Shift) =>
                    return ExactReady('_'),
                (DPadState::Up, RButtonState::Trigger, ModifierState::Shift) =>
                    return ExactReady('='),
                
                // scoping (note: RButtonState::Up/Down used for macros)
                (DPadState::Right, RButtonState::Left, ModifierState::Neutral) =>
                    return ExactReady('('),
                (DPadState::Right, RButtonState::Right, ModifierState::Neutral) =>
                    return ExactReady(')'),
                (DPadState::Right, RButtonState::Left, ModifierState::Shift) =>
                    return ExactReady('['),
                (DPadState::Right, RButtonState::Right, ModifierState::Shift) =>
                    return ExactReady(']'),
                (DPadState::Right, RButtonState::Left, ModifierState::Ctrl) =>
                    return ExactReady('{'),
                (DPadState::Right, RButtonState::Right, ModifierState::Ctrl) =>
                    return ExactReady('}'),
                (DPadState::Right, RButtonState::Down, ModifierState::Neutral) =>
                    return ExactReady('<'),
                (DPadState::Right, RButtonState::Up, ModifierState::Neutral) =>
                    return ExactReady('>'),
                
                // quotes('Y\"')
                (DPadState::Right, RButtonState::Bumper, ModifierState::Neutral) =>
                    return ExactReady('\''),
                (DPadState::Right, RButtonState::Bumper, ModifierState::Shift) =>
                    return ExactReady('\"'),
                (DPadState::Right, RButtonState::Bumper, ModifierState::Ctrl) =>
                    return ExactReady('`'),
                (DPadState::Right, RButtonState::Bumper, ModifierState::Alt) =>
                    return ExactReady('~'),
                
                // slashes
                (DPadState::Right, RButtonState::Trigger, ModifierState::Neutral) =>
                    return ExactReady('/'),
                (DPadState::Right, RButtonState::Trigger, ModifierState::Shift) =>
                    return ExactReady('\\'),
                (DPadState::Right, RButtonState::Trigger, ModifierState::Ctrl) =>
                    return ExactReady('|'),
                
                // punctuation
                (DPadState::Left, RButtonState::Down, ModifierState::Neutral) =>
                    return ExactReady('.'),
                (DPadState::Left, RButtonState::Left, ModifierState::Neutral) =>
                    return ExactReady(','),
                (DPadState::Left, RButtonState::Up, ModifierState::Neutral) =>
                    return ExactReady(':'),
                (DPadState::Left, RButtonState::Right, ModifierState::Neutral) =>
                    return ExactReady(';'),
                (DPadState::Left, RButtonState::Bumper, ModifierState::Neutral) =>
                    return ExactReady('?'),
                (DPadState::Left, RButtonState::Trigger, ModifierState::Neutral) =>
                    return ExactReady('!'),
                
                // else move on
                _ => (),
            }
        }
        
        else if self.stick_r.state() != StickState::Neutral {
            match (self.stick_r.state(), self.dpad_state(), self.modifier_state()) {

                // navigation
                (StickState::Left(_), DPadState::Neutral, ModifierState::Neutral) =>
                    return MacroReady(Macro::CharBack),
                (StickState::Right(_), DPadState::Neutral, ModifierState::Neutral) =>
                    return MacroReady(Macro::CharFor),
                (StickState::Up(_), DPadState::Neutral, ModifierState::Neutral) =>
                    return MacroReady(Macro::LineUp),
                (StickState::Down(_), DPadState::Neutral, ModifierState::Neutral) =>
                    return MacroReady(Macro::LineDown),

                // deletion
                (StickState::Right(_), DPadState::Down, ModifierState::Neutral) =>
                    return MacroReady(Macro::DelCharFor),
                (StickState::Left(_), DPadState::Down, ModifierState::Neutral) =>
                    return MacroReady(Macro::DelCharBack),
                (StickState::Right(_), DPadState::Down, ModifierState::Shift) =>
                    return MacroReady(Macro::DelWordFor),
                (StickState::Left(_), DPadState::Down, ModifierState::Shift) =>
                    return MacroReady(Macro::DelWordBack),
                (StickState::Down(_), DPadState::Down, ModifierState::Neutral) =>
                    return MacroReady(Macro::DelLineFor),
                (StickState::Up(_), DPadState::Down, ModifierState::Neutral) =>
                    return MacroReady(Macro::DelLineBack),
                (StickState::Down(_), DPadState::Down, ModifierState::Shift) =>
                    return MacroReady(Macro::DelParDown),
                (StickState::Up(_), DPadState::Down, ModifierState::Shift) =>
                    return MacroReady(Macro::DelParUp),

                // else move on
                _ => (),
            }
        }
                 
                 
                 
        return ControllerState::Confused;
    }
    
    pub fn is_shift(&self) -> bool {
        if let TriggerState::Pressed(_) = self.trigger_l.state() {
            return true;
        }
        false
    }
    
    pub fn is_ctrl(&self) -> bool {
        self.button_l_bumper
    }
    
    pub fn is_alt(&self) -> bool {
        self.button_l_stick
    }

    pub fn is_super(&self) -> bool {
        self.button_xbox
    }

    pub fn changed(&mut self) -> bool {
        if self.changed {
            self.changed = false;
            return true;
        }
        false
    }

    fn dpad_state(&self) -> DPadState {
        match (self.button_dpad_down, self.button_dpad_left,
               self.button_dpad_up, self.button_dpad_right) {
            (false, false, false, false) =>
                return DPadState::Neutral,
            (true, false, false, false) =>
                return DPadState::Down,
            (false, true, false, false) =>
                return DPadState::Left,
            (false, false, true, false) =>
                return DPadState::Up,
            (false, false, false, true) =>
                return DPadState::Right,
            _ =>
                return DPadState::Confused,
        }
    }

    fn rbutton_state(&self) -> RButtonState {
        match (self.button_a, self.button_x, self.button_y,
               self.button_b, self.button_r_bumper,
               self.button_r_stick, self.trigger_r.state(),
               self.button_start, self.button_back) {
            (false, false, false, false, false, false, TriggerState::Neutral, false, false) =>
                return RButtonState::Neutral,
            (true, false, false, false, false, false, TriggerState::Neutral, false, false) =>
                return RButtonState::Down,
            (false, true, false, false, false, false, TriggerState::Neutral, false, false) =>
                return RButtonState::Left,
            (false, false, true, false, false, false, TriggerState::Neutral, false, false) =>
                return RButtonState::Up,
            (false, false, false, true, false, false, TriggerState::Neutral, false, false) =>
                return RButtonState::Right,
            (false, false, false, false, true, false, TriggerState::Neutral, false, false) =>
                return RButtonState::Bumper,
            (false, false, false, false, false, true, TriggerState::Neutral, false, false) =>
                return RButtonState::Stick,
            (false, false, false, false, false, false, TriggerState::Pressed(_), false, false) =>
                return RButtonState::Trigger,
            (false, false, false, false, false, false, TriggerState::Neutral, true, false) =>
                return RButtonState::Start,
            (false, false, false, false, false, false, TriggerState::Neutral, false, true) =>
                return RButtonState::Back,
            _ =>
                return RButtonState::Confused,
        }
    }

    fn modifier_state(&self) -> ModifierState {
        match (self.trigger_l.state(), self.button_l_bumper,
               self.button_l_stick, self.button_xbox) {
            (TriggerState::Neutral, false, false, false) =>
                return ModifierState::Neutral,
            (TriggerState::Pressed(_), false, false, false) =>
                return ModifierState::Shift,
            (TriggerState::Neutral, true, false, false) =>
                return ModifierState::Ctrl,
            (TriggerState::Neutral, false, true, false) =>
                return ModifierState::Alt,
            (TriggerState::Pressed(_), false, true, false) =>
                return ModifierState::AltShift,
            (TriggerState::Neutral, true, true, false) =>
                return ModifierState::AltCtrl,
            (TriggerState::Neutral, false, false, true) =>
                return ModifierState::Super,
            (TriggerState::Neutral, false, true, true) =>
                return ModifierState::SuperCtrl,
            _ =>
                return ModifierState::Neutral,
        }
    }

}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ControllerState { // returning to Neutral from a signicant state should incure output
    Confused,
    Neutral,
    ModableReady(char), // modable as in modifiable as in applying shift, ctrl, and al
    //                     works as it would on a keyboard
    ExactReady(char), // alt, shift, and ctrl, modify the output differently than expected
    MacroReady(Macro),
}

#[derive(Debug, Clone, PartialEq, Hash, Copy)]
pub enum Macro {
    //navigation
    LineDown,
    LineUp,
    CharFor,
    CharBack,
    WordFor,
    WordBack,
    ParDown,
    ParUp,

    //deletion
    DelLineFor,
    DelLineBack,
    DelCharFor,
    DelCharBack,
    DelWordFor,
    DelWordBack,
    DelParDown,
    DelParUp,

    // other
    Undo,
    
    //special char
    Enter,
    Super,
    Tab,
    
    //key words
    ExternCrate,
    Mod,
    Use,
    Const,
    Pub,
    Let,

    //expansive
    Fn,
    Struct,
    Enum,
    Impl,
    ForIn,
    Match,
    If,

    //TODO: the rest
} //TODO: Map to Vec<OutPart> in configuration.rs
impl Eq for Macro {}

// Stick
struct StickData {
    x_pos: i16,
    y_pos: i16,
}
const TILT_THRESHHOLD: i16 = 6000;
const SMASH_THRESHHOLD: i16 = 31000;
impl StickData {
    pub fn state(&self) -> StickState { //TODO: use math.abs() when you have wifi for syntx
        match (self.x_pos, self.y_pos) {
            (x,y) if x < TILT_THRESHHOLD && x > -1*TILT_THRESHHOLD 
                && y < TILT_THRESHHOLD && y > -1* TILT_THRESHHOLD => {
                    return StickState::Neutral;
                }
            (x,y) if abs(x) >= abs(y) => {
                if x > 0 {
                    return StickState::Right(x > SMASH_THRESHHOLD);
                } else {
                    return StickState::Left(x < -1*SMASH_THRESHHOLD);
                } 
            }
            (x,y) if abs(x) < abs(y) => {
                if y > 0 {
                    return StickState::Up(y > SMASH_THRESHHOLD);
                } else {
                    return StickState::Down(y < -1*SMASH_THRESHHOLD);
                } 
            }
            (x,y) => panic!("Broken values of stick x,y: {},{}", x, y),
        }
    }
}
#[derive(PartialEq)]
pub enum StickState {
    Neutral,
    Left(bool), // false means tilt, true means full (smash)
    Right(bool),
    Up(bool),
    Down(bool),
}

// Trigger
struct TriggerData {
    pos: u8,
}
impl TriggerData {
    pub fn state(&self) -> TriggerState {
        match self.pos {
            0 => return TriggerState::Neutral,
            255 => return TriggerState::Pressed(true),
            _ => return TriggerState::Pressed(false),
        }
    }
}
#[derive(PartialEq)]
pub enum TriggerState {
    Neutral,
    Pressed(bool), //false means partially pressed, true means fully squeezed down
}

pub fn init_controller_data() -> ControllerData {
    ControllerData {
        button_xbox: false,
        button_back: false,
        button_start: false,
        button_a: false,
        button_b: false,
        button_x: false,
        button_y: false,
        button_dpad_left: false,
        button_dpad_right: false,
        button_dpad_up: false,
        button_dpad_down: false,
        button_l_bumper: false,
        button_r_bumper: false,
        button_l_stick: false,
        button_r_stick: false,
        trigger_l: TriggerData{pos: 0},
        trigger_r: TriggerData{pos: 0},
        stick_l: StickData {x_pos: 0, y_pos: 0},
        stick_r: StickData {x_pos: 0, y_pos: 0},
        //
        changed: true,
        last_state: ControllerState::Neutral,
    }
}

impl ControllerData {
    pub fn load_from_bytes(&mut self, buf: &[u8]) { //TODO: ensure slice has 20 elements
        let last_frame_state = self.state();
        // byte 2
        self.button_r_stick =     0b10000000 & buf[2] != 0;
        self.button_l_stick =     0b01000000 & buf[2] != 0;
        self.button_back =       0b00100000 & buf[2] != 0;
        self.button_start =      0b00010000 & buf[2] != 0;
        self.button_dpad_right = 0b00001000 & buf[2] != 0;
        self.button_dpad_left =  0b00000100 & buf[2] != 0;
        self.button_dpad_down =  0b00000010 & buf[2] != 0;
        self.button_dpad_up =    0b00000001 & buf[2] != 0;
        // byte 3
        self.button_y =       0b10000000 & buf[3] != 0;
        self.button_x =       0b01000000 & buf[3] != 0;
        self.button_b =       0b00100000 & buf[3] != 0;
        self.button_a =       0b00010000 & buf[3] != 0;
        //self.button_ = 0b00001000 & buf[3] != 0; //no data encoded by this bit
        self.button_xbox =   0b00000100 & buf[3] != 0;
        self.button_r_bumper = 0b00000010 & buf[3] != 0;
        self.button_l_bumper = 0b00000001 & buf[3] != 0;
        // bytes 4 and 5
        self.trigger_l.pos = buf[4];
        self.trigger_r.pos = buf[5];
        // bytes 6, 7, 8, and 9
        self.stick_l.x_pos = (buf[6] as i16) + ((buf[7] as i16) << 8);
        self.stick_l.y_pos = (buf[8] as i16) + ((buf[9] as i16) << 8);
        // bytes 10, 11, 12, 13
        self.stick_r.x_pos = (buf[10] as i16) + ((buf[11] as i16) << 8);
        self.stick_r.y_pos = (buf[12] as i16) + ((buf[13] as i16) << 8);

        // reduce cost of figuring out output
        if self.state() != last_frame_state {
            self.last_state = last_frame_state;
            self.changed = true;
        }
    }

    pub fn last_state(&self) -> ControllerState {
       self.last_state
    }
}
