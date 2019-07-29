extern crate libusb;

const INPUT_BUFFER_SIZE:usize = 20;
const XBOX_CONTROLLER_ID: u16 = 654;
const DATAFLOW_INTERFACE: u8 = 0;

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
    button_A: bool,
    button_B: bool,
    button_X: bool,
    button_Y: bool,
    button_Dpad_left: bool,
    button_Dpad_right: bool,
    button_Dpad_up: bool,
    button_Dpad_down: bool,
    button_Lbumper: bool,
    button_Rbumper: bool,
    button_Lstick: bool,
    button_Rstick: bool,
    trigger_L: u8,
    trigger_R: u8,
        joystick_L: JoyStickData,
    joystick_R: JoyStickData,
}

struct JoyStickData {
    x_pos: i16,
    y_pos: i16,
}

pub fn init_controller_data() -> ControllerData {
    ControllerData {
        button_xbox: false,
        button_back: false,
        button_start: false,
        button_A: false,
        button_B: false,
        button_X: false,
        button_Y: false,
        button_Dpad_left: false,
        button_Dpad_right: false,
        button_Dpad_up: false,
        button_Dpad_down: false,
        button_Lbumper: false,
        button_Rbumper: false,
        button_Lstick: false,
        button_Rstick: false,
        trigger_L: 0,
        trigger_R: 0,
        joystick_L: JoyStickData {x_pos: 0, y_pos: 0},
        joystick_R: JoyStickData {x_pos: 0, y_pos: 0},
    }
}

impl ControllerData {
    pub fn load_from_bytes(&mut self, buf: &[u8]) { //TODO: ensure slice has 20 elements
        // byte 2
        self.button_Rstick =     0b10000000 & buf[2] != 0;
        self.button_Lstick =     0b01000000 & buf[2] != 0;
        self.button_back =       0b00100000 & buf[2] != 0;
        self.button_start =      0b00010000 & buf[2] != 0;
        self.button_Dpad_right = 0b00001000 & buf[2] != 0;
        self.button_Dpad_left =  0b00000100 & buf[2] != 0;
        self.button_Dpad_down =  0b00000010 & buf[2] != 0;
        self.button_Dpad_up =    0b00000001 & buf[2] != 0;
        // byte 3
        self.button_Y =       0b10000000 & buf[3] != 0;
        self.button_X =       0b01000000 & buf[3] != 0;
        self.button_B =       0b00100000 & buf[3] != 0;
        self.button_A =       0b00010000 & buf[3] != 0;
        //self.button_ = 0b00001000 & buf[3] != 0; //no data encoded by this bit
        self.button_start =   0b00000100 & buf[3] != 0;
        self.button_Lbumper = 0b00000010 & buf[3] != 0;
        self.button_Rbumper = 0b00000001 & buf[3] != 0;
        // bytes 4 and 5
        self.trigger_L = buf[4];
        self.trigger_R = buf[5];
        // bytes 6, 7, 8, and 9
        self.joystick_L.x_pos = (buf[6] as i16) + ((buf[7] as i16) << 8);
        self.joystick_L.y_pos = (buf[8] as i16) + ((buf[9] as i16) << 8);
        // bytes 10, 11, 12, 13
        self.joystick_R.x_pos = (buf[10] as i16) + ((buf[11] as i16) << 8);
        self.joystick_R.y_pos = (buf[12] as i16) + ((buf[13] as i16) << 8);
        
    }
    

    pub fn button_back_pressed(&self) -> bool {
        self.button_back
    }

    pub fn joystick_L_pos(&self) -> (i16, i16) {
        (self.joystick_L.x_pos, self.joystick_L.y_pos)
    }
}