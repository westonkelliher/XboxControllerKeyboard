extern crate libusb;

const INPUT_BUFFER_SIZE:usize = 20;
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
    joystick_l: JoyStickData,
    joystick_r: JoyStickData,
    //
    changed: bool,
}
impl ControllerData {

    //TODO: DPad enum. RButton enum (along with dpad() and rbutton() functions)
    
    pub fn state(&self) -> ControllerState {   //TODO: change all pub's in this file that need not be pub
        match (self.button_dpad_down, self.button_dpad_left,
               self.button_dpad_up, self.button_dpad_right,
               self.button_a, self.button_x, self.button_y, self.button_b,
               self.button_r_bumper, self.trigger_r.state()) {

            // numbers / top row
            (true, false, false, false,
             true, false, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('1'),
            (true, false, false, false,
             false, true, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('2'),
            (true, false, false, false,
             false, false, true, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('3'),
            (true, false, false, false,
             false, false, false, true, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('4'),
            (true, false, false, false,
             false, false, false, false, true, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('5'),
            (true, false, false, false,
             false, false, false, false, false, TriggerState::Pressed(_)) =>
                return ControllerState::PoisedChar('6'),
            (false, false, true, false,
             true, false, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('7'),
            (false, false, true, false,
             false, true, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('8'),
            (false, false, true, false,
             false, false, true, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('9'),
            (false, false, true, false,
             false, false, false, true, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('0'),
            (false, false, true, false,
             false, false, false, false, true, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('-'),
            (false, false, true, false,
             false, false, false, false, false, TriggerState::Pressed(_)) =>
                return ControllerState::PoisedChar('='),

            // parens and special chars
            (false, true, false, false,
             true, false, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('['),
            (false, true, false, false,
             false, true, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('('),
            (false, true, false, false,
             false, false, true, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar(']'),
            (false, true, false, false,
             false, false, false, true, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar(')'),

            (false, false, false, true,
             true, false, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar(';'),
            (false, false, false, true,
             false, true, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar(','),
            (false, false, false, true,
             false, false, true, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('\''),
            (false, false, false, true,
             false, false, false, true, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('.'),
            (false, false, false, true,
             false, false, false, false, true, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('/'),
            (false, false, false, true,
             false, false, false, false, false, TriggerState::Pressed(_)) =>
                return ControllerState::PoisedChar('\\'),
            
            
            _ =>
                (),
        }
        
        match (self.joystick_l.state(),
               self.button_a, self.button_x, self.button_y, self.button_b,
               self.button_r_bumper, self.trigger_r.state()) {

            // state doeas not corespond to a char
            (_, false, false, false, false, false, TriggerState::Neutral) => {
                if self.button_r_stick {
                    return ControllerState::PoisedChar(' ');
                }
                return ControllerState::Neutral;
            },

            // vowels
            (JoyStickState::Neutral, true, false, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('a'),
            (JoyStickState::Neutral, false, true, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('e'),
            (JoyStickState::Neutral, false, false, true, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('i'),
            (JoyStickState::Neutral, false, false, false, true, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('o'),
            (JoyStickState::Neutral, false, false, false, false, true, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('u'),
            (JoyStickState::Neutral, false, false, false, false, false, TriggerState::Pressed(_)) =>
                return ControllerState::PoisedChar('y'),

            // non-voiced
            (JoyStickState::Up(_), true, false, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('p'),
            (JoyStickState::Up(_), false, true, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('t'),
            (JoyStickState::Up(_), false, false, true, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('f'),
            (JoyStickState::Up(_), false, false, false, true, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('s'),
            (JoyStickState::Up(_), false, false, false, false, true, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('c'),
            (JoyStickState::Up(_), false, false, false, false, false, TriggerState::Pressed(_)) =>
                return ControllerState::PoisedChar('k'),

            // voiced
            (JoyStickState::Down(_), true, false, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('b'),
            (JoyStickState::Down(_), false, true, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('d'),
            (JoyStickState::Down(_), false, false, true, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('v'),
            (JoyStickState::Down(_), false, false, false, true, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('z'),
            (JoyStickState::Down(_), false, false, false, false, true, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('g'),
            (JoyStickState::Down(_), false, false, false, false, false, TriggerState::Pressed(_)) =>
                return ControllerState::PoisedChar('j'),

            // slides
            (JoyStickState::Right(_), true, false, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('h'),
            (JoyStickState::Right(_), false, true, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('l'),
            (JoyStickState::Right(_), false, false, true, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('r'),
            (JoyStickState::Right(_), false, false, false, true, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('w'),

            // oddies
            (JoyStickState::Left(_), true, false, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('m'),
            (JoyStickState::Left(_), false, true, false, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('n'),
            (JoyStickState::Left(_), false, false, true, false, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('q'),
            (JoyStickState::Left(_), false, false, false, true, false, TriggerState::Neutral) =>
                return ControllerState::PoisedChar('x'),

            // convoluted
            (_, _, _, _, _, _, _) =>
                return ControllerState::Confused,
        }
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

    pub fn changed(&mut self) -> bool {
        if self.changed {
            self.changed = false;
            return true;
        }
        false
    }
    
    // TODO: make a struct (called ControllerTracker?) to contain ControllerData with below functionality
    pub fn get_output(&self, last_state: &ControllerState, new_state: &ControllerState) -> Option<std::string::String> {
        if let ControllerState::PoisedChar(c) = last_state {
            if let ControllerState::Neutral = new_state {
                return Some(std::string::String::from(format!("{}",c)));
            }
        }
        None
    }
}
#[derive(Debug)]
pub enum ControllerState {
    Confused,
    Neutral,
    PoisedChar(char),
}
impl ControllerState {
    pub fn copy(&self) -> ControllerState {  //TODO: do this properly (need wifi)
        match self {
            ControllerState::Confused =>
                return ControllerState::Confused,
            ControllerState::Neutral =>
                return ControllerState::Neutral,
            ControllerState::PoisedChar(c) =>
                return ControllerState::PoisedChar(*c),
        }
    }
}


// Joystick
struct JoyStickData {
    x_pos: i16,
    y_pos: i16,
}
const TILT_THRESHHOLD: i16 = 6000;
const SMASH_THRESHHOLD: i16 = 31000;
impl JoyStickData {
    pub fn state(&self) -> JoyStickState { //TODO: use math.abs() when you have wifi for syntx
        match (self.x_pos, self.y_pos) {
            (x,y) if x < TILT_THRESHHOLD && x > -1*TILT_THRESHHOLD 
                && y < TILT_THRESHHOLD && y > -1* TILT_THRESHHOLD => {
                    return JoyStickState::Neutral;
                }
            (x,y) if abs(x) >= abs(y) => {
                if x > 0 {
                    return JoyStickState::Right(x > SMASH_THRESHHOLD);
                } else {
                    return JoyStickState::Left(x < -1*SMASH_THRESHHOLD);
                } 
            }
            (x,y) if abs(x) < abs(y) => {
                if y > 0 {
                    return JoyStickState::Up(y > SMASH_THRESHHOLD);
                } else {
                    return JoyStickState::Down(y < -1*SMASH_THRESHHOLD);
                } 
            }
            (x,y) => panic!("Broken values of joystick x,y: {},{}", x, y),
        }
    }
}
#[derive(Debug)]
pub enum JoyStickState {
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
#[derive(Debug)]
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
        joystick_l: JoyStickData {x_pos: 0, y_pos: 0},
        joystick_r: JoyStickData {x_pos: 0, y_pos: 0},
        //
        changed: true,
    }
}

impl ControllerData {
    pub fn load_from_bytes(&mut self, buf: &[u8]) { //TODO: ensure slice has 20 elements
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
        self.button_start =   0b00000100 & buf[3] != 0;
        self.button_r_bumper = 0b00000010 & buf[3] != 0;
        self.button_l_bumper = 0b00000001 & buf[3] != 0;
        // bytes 4 and 5
        self.trigger_l.pos = buf[4];
        self.trigger_r.pos = buf[5];
        // bytes 6, 7, 8, and 9
        self.joystick_l.x_pos = (buf[6] as i16) + ((buf[7] as i16) << 8);
        self.joystick_l.y_pos = (buf[8] as i16) + ((buf[9] as i16) << 8);
        // bytes 10, 11, 12, 13
        self.joystick_r.x_pos = (buf[10] as i16) + ((buf[11] as i16) << 8);
        self.joystick_r.y_pos = (buf[12] as i16) + ((buf[13] as i16) << 8);

        // reduce cost of figuring out output
        self.changed = true;
    }

}
