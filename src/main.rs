extern crate libusb;
mod XboxController;

const INPUT_CHANNEL: u8 = 0x81;
const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(30);

fn main() {
    let context = libusb::Context::new().unwrap();

    match XboxController::get_controller_handle(&context) {
        Ok(handle) => {
            rename_this_fn(handle);
        }
        Err(msg) => {
            panic!(msg);
        }
    };

    //TODO: check for input ?and update controller_data struct
    //TODO: implement state machine for controller data
    //TODO: implement enum for joystick (neutral, left_tilt, left_full...)
}

fn rename_this_fn(handle: libusb::DeviceHandle) { //TODO: rename
    // attempt to read from controller
    let mut buffer: [u8; 128] = [0; 128];
    match handle.read_interrupt(INPUT_CHANNEL, &mut buffer, TIMEOUT) {
        Ok(n) => {
            println!("{} bytes recieved", n);
            print_buffer_contents(n, &buffer);
            let mut cont_data = XboxController::init_controller_data();
            cont_data.load_from_bytes(&buffer[..20]);
            match cont_data.joystick_L_pos() {
                (x, y) => println!("{}, {}", x, y),
            }
        }
        Err(error) =>
            println!("problem with read_interrupt: {}", error),
    }    
}

fn print_buffer_contents(n: usize, buf: &[u8; 128]) {
    println!("-----");
    let mut i = 0;
    for byte in buf[..n].iter() {
        if i >= n {
            break;
        }
        println!("{:02}> {:X}", i, byte);
        i = i+1;
    }
    println!("-----");
}
