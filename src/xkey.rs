extern crate libusb;
extern crate enigo;
mod xboxcontroller;
use xboxcontroller::{ControllerData, ControllerState}; //? JoyStickData? ControllerState?
use enigo::{Enigo, KeyboardControllable, Key};

const INPUT_CHANNEL: u8 = 0x81;
const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(30);


pub fn init_xkey() -> XKey {
    XKey {
        controller_data: xboxcontroller::init_controller_data(),
        controller_state: ControllerState::Neutral,
        enigo: Enigo::new(),
        libusb_context: libusb::Context::new().unwrap(),
        usb_buffer: [0; 20],
    }
    
}

pub struct XKey {
    controller_data: ControllerData,
    controller_state: ControllerState,
    enigo: Enigo,
    libusb_context: libusb::Context,
    usb_buffer: [u8; 20],
}
impl XKey {

    fn get_output_string(&self, last_state: &ControllerState) -> std::string::String {
        if let ControllerState::PoisedChar(c) = last_state {
            if let ControllerState::Neutral = self.controller_state {
                /*let shift = if self.controller_data.is_shift() { "{+SHIFT}" } else { "" };
                let ctrl = if self.controller_data.is_ctrl() { "{+CTRL}" } else { "" };
                let alt = if self.controller_data.is_alt() { "{+ALT}" } else { "" };
                let shift_ = if self.controller_data.is_shift() { "{-SHIFT}" } else { "" };
                let ctrl_ = if self.controller_data.is_ctrl() { "{-CTRL}" } else { "" };
                let alt_ = if self.controller_data.is_alt() { "{-ALT}" } else { "" };
                */
                return format!("{}", c);
            }
        }

        //if let ConrtollerState::Macro //TODO: MAcros
        std::string::String::from("")
    }

    pub fn begin(mut self) {
        let mut handle_option: Option<libusb::DeviceHandle> = None;
        match xboxcontroller::get_controller_handle(&self.libusb_context) {
            Ok(handle) => {
                //rename_this_fn(&handle);
                handle_option = Some(handle);
            }
            Err(msg) => {
                panic!(msg);
            }
        };

        let handle = handle_option.unwrap();
        loop {
            match handle.read_interrupt(INPUT_CHANNEL, &mut self.usb_buffer, TIMEOUT) {
                Ok(_n) => {
                    let last_state = self.controller_state;
                    self.controller_data.load_from_bytes(&self.usb_buffer);
                    self.controller_state = self.controller_data.state();
                    if self.controller_data.changed() {
                        if self.controller_data.is_shift() { self.enigo.key_down(Key::Shift); }
                        if self.controller_data.is_ctrl() { self.enigo.key_down(Key::Control); }
                        if self.controller_data.is_alt() { self.enigo.key_down(Key::Alt); }
                        self.enigo.key_sequence_parse(&self.get_output_string(&last_state));
                        self.enigo.key_up(Key::Shift);
                        self.enigo.key_up(Key::Control);
                        self.enigo.key_up(Key::Alt);
                    }
                }
                Err(_) => { //only do this for timeout errors
                    continue;
                }
                //Err(e) => panic!("{}", e),
            }
        }
    }
}
