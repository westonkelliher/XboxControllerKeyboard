extern crate libusb;
mod xboxController;

static XBOX_CONTROLLER_ID: u16 = 654;

fn main() {
    let context = libusb::Context::new().unwrap();
    let timeout = std::time::Duration::from_secs(1);

    let xbox_device_optn = get_device_at_address(XBOX_CONTROLLER_ID, &context);
    match xbox_device_optn {
        Some(device) => {
            println!("Xbox controller found at usb address {:03}", device.address());
            match device.open() {
                Ok(mut handle) => {

                    // make sure something else doesnt have control
                    // of the interface (like the kernel)
                    match handle.kernel_driver_active(0) {
                        Ok(true) => {
                            println!("found a kernel driver attached");
                            match handle.detach_kernel_driver(0) {
                                Ok(_) => println!("Detached kernel driver"),
                                Err(error) => println!("failed to detach kernel driver: {}", error),
                            }
                        }
                        Ok(false) =>
                            println!("no kernel driver attached"),
                        Err(error) =>
                            println!("failed to chec for active kernel: {}", error),
                    }

                    // claim the interface
                    match handle.claim_interface(0) {
                        Ok(_) => println!("Claimed interface 0"),
                        Err(error) => println!("Failed to claim interface 0: {}", error),
                    }

                    // attempt to read from controller
                    let mut buffer: [u8; 128] = [0; 128];
                    match handle.read_interrupt(0x81, &mut buffer, timeout) {
                        Ok(n) => {
                            println!("{} bytes recieved", n);
                            print_buffer_contents(n, &buffer);
                            let mut cont_data = xboxController::init_controller_data();
                            cont_data.load_from_bytes(&buffer[..20]);
                            match cont_data.joystick_L_pos() {
                                (x, y) => println!("{}, {}", x, y),
                            }
                            
                        }
                        Err(error) =>
                            println!("problem with read_interrupt: {}", error),
                    }
                }
                Err(error) =>
                    println!("Failed to get handle: {}", error),
            }
        }
        None =>
            println!("No Xbox controller found (XBOX_CONTROLLER_ID \
                      must match the product ID of the xbox controller you are using"),
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

