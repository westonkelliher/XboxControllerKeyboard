extern crate libusb;
extern crate enigo;
use std::collections::HashMap;
mod xboxcontroller;
use xboxcontroller::{ControllerData, ControllerState, Macro}; //? JoyStickData? ControllerState?
use enigo::{Enigo, KeyboardControllable, Key};

const INPUT_CHANNEL: u8 = 0x81;
const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(30);


pub fn init_xkey() -> XKey {
    XKey {
        controller_data: xboxcontroller::init_controller_data(),
        controller_state: ControllerState::Neutral,
        enigo: Enigo::new(),
        usb_buffer: [0; 20],
        macro_map: [

            // navigation
            (Macro::CharBack, vec!(SpecialKey(Key::LeftArrow)) ),
            (Macro::CharFor, vec!(SpecialKey(Key::RightArrow)) ),
            (Macro::LineDown, vec!(SpecialKey(Key::DownArrow)) ),
            (Macro::LineUp, vec!(SpecialKey(Key::UpArrow)) ),
            (Macro::WordFor, vec!(ModifierDown(Key::Alt), KeySequence("f".to_string())) ),
            (Macro::WordBack, vec!(ModifierDown(Key::Alt), KeySequence("b".to_string())) ),
            (Macro::ParDown, vec!(ModifierDown(Key::Alt), KeySequence("n".to_string())) ),
            (Macro::ParUp, vec!(ModifierDown(Key::Alt), KeySequence("p".to_string())) ),

            // deletion
            (Macro::DelCharFor, vec!(ModifierDown(Key::Control),
                                     KeySequence("d".to_string())) ),
            (Macro::DelCharBack, vec!(ModifierDown(Key::Control),
                                     SpecialKey(Key::Backspace)) ),
            (Macro::DelWordFor, vec!(ModifierDown(Key::Alt),
                                     KeySequence("d".to_string())) ),
            (Macro::DelWordBack, vec!(ModifierDown(Key::Alt),
                                     SpecialKey(Key::Backspace)) ),
            (Macro::DelLineFor, vec!(ModifierDown(Key::Control),
                                     KeySequence("k".to_string())) ),
            (Macro::DelLineBack, vec!(ModifierDown(Key::Control),
                                      KeySequence(" aw".to_string())) ),
            (Macro::DelParDown, vec!(ModifierDown(Key::Alt),
                                      KeySequence("k".to_string())) ),
            (Macro::DelParUp, vec!(ModifierDown(Key::Control),
                                   KeySequence(" ".to_string()),
                                   ModifierUp(Key::Control),
                                   ModifierDown(Key::Alt),
                                   KeySequence("p".to_string()),
                                   ModifierUp(Key::Alt),
                                   ModifierDown(Key::Control),
                                   KeySequence("w".to_string())) ),
            
             // specials
            (Macro::Enter, vec!(SpecialKey(Key::Return)) ),
            (Macro::Tab, vec!(SpecialKey(Key::Tab)) ),
            (Macro::Super, vec!(SpecialKey(Key::Super)) ),

            // other
            (Macro::Undo, vec!(ModifierDown(Key::Control), KeySequence("x".to_string()),
                               ModifierUp(Key::Control), KeySequence("u".to_string())) ),

            // expansions
            (Macro::If, vec!(KeySequence("if  {\n\n}".to_string()), ModifierDown(Key::Control),
                             KeySequence("ppa".to_string()), ModifierUp(Key::Control),
                             ModifierDown(Key::Alt), KeySequence("f".to_string()),
                             ModifierUp(Key::Alt), ModifierDown(Key::Control),
                             KeySequence("f".to_string())) ),
            
        ].iter().cloned().collect(),
    }
    
}

pub struct XKey {
    controller_data: ControllerData,
    controller_state: ControllerState,
    enigo: Enigo,
    usb_buffer: [u8; 20],
    macro_map: HashMap<Macro, Vec<OutPart>>,
}

//TODO: create vec of these for input to execute
#[derive(Clone)]
enum OutPart {
    ModifierDown(Key), // (enigo::Key)
    ModifierUp(Key),
    KeySequence(std::string::String),
    SpecialKey(Key),
}

use OutPart::{ModifierDown, ModifierUp, KeySequence, SpecialKey};



impl XKey {

    fn press_mods(&mut self) {
        if self.controller_data.is_shift() { self.enigo.key_down(Key::Shift); }
        if self.controller_data.is_ctrl() { self.enigo.key_down(Key::Control); }
        if self.controller_data.is_alt() { self.enigo.key_down(Key::Alt); }
        if self.controller_data.is_super() { self.enigo.key_down(Key::Super); }
    }
    fn release_mods(&mut self) {
        self.enigo.key_up(Key::Shift);
        self.enigo.key_up(Key::Control);
        self.enigo.key_up(Key::Alt);
        self.enigo.key_up(Key::Super);
    }
    
    fn execute_output(&mut self, cont_state:ControllerState) {
        match cont_state {
            ControllerState::ModableReady(c) => {
                self.press_mods();
                self.enigo.key_sequence_parse(&format!("{}",c));
                self.release_mods();
            }
            ControllerState::ExactReady(c) => {
                self.execute_char(c);
            }
            ControllerState::MacroReady(m) => 
                self.execute_macro(self.macro_map.get(&m).unwrap().to_vec()),
            _ =>
                (),
        }
        self.release_mods();
    }
    
    fn execute_macro(&mut self, parts: Vec<OutPart>) {
        for part in &parts {
            match part {
                KeySequence(s) =>
                    self.execute_string(s),
                ModifierDown(k) =>
                    self.enigo.key_down(*k),
                ModifierUp(k) =>
                    self.enigo.key_up(*k),
                SpecialKey(k) =>
                    self.enigo.key_click(*k),
            }
        }
    }

    fn execute_char(&mut self, c: char) {
        if c == '\n' {
            self.enigo.key_click(Key::Return);
        } else if  c == '}' {
            self.enigo.key_down(Key::Shift);
            self.enigo.key_sequence_parse("]");
            self.enigo.key_up(Key::Shift);
        } else if c =='{' {
            self.enigo.key_down(Key::Shift);
            self.enigo.key_sequence_parse("[");
            self.enigo.key_up(Key::Shift);            
        } else {
            self.enigo.key_sequence_parse(&format!("{}",c));
        }             
    }

    fn execute_string(&mut self, s: &str) {
        for c in s.chars() {
            self.execute_char(c);
        }
    }

    /*fn get_output_string(&self, last_state: &ControllerState) -> std::string::String {
        if let ControllerState::PoisedChar(c) = last_state {
            if let ControllerState::Neutral = self.controller_state {
                return format!("{}", c);
            }
        }

        //if let ConrtollerState::Macro //TODO: MAcros
        std::string::String::from("")
    }
     */
    
    pub fn begin(mut self) {

        let libusb_context = libusb::Context::new().unwrap();

        let handle: libusb::DeviceHandle;
        let temp = xboxcontroller::get_controller_handle(&libusb_context);
        match temp {
            Ok(h) => {
                handle = h;
            }
            Err(msg) => {
                panic!(msg);
            }
        };

        loop {
            match handle.read_interrupt(INPUT_CHANNEL, &mut self.usb_buffer, TIMEOUT) {
                Ok(_n) => {
                    
                    self.controller_data.load_from_bytes(&self.usb_buffer);
                    self.controller_state = self.controller_data.state();
                    let last_state = self.controller_data.last_state();
                    if self.controller_data.changed() {
                        //println!("{:?}  {:?}", self.controller_data.last_state(), self.controller_data.state());
                        if self.controller_state == ControllerState::Neutral {
                            self.execute_output(last_state);
                        }
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
