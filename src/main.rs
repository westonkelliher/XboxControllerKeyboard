extern crate libusb;
extern crate enigo;
mod xkey;


fn main() {

    let xkey = xkey::init_xkey();
    xkey.begin();

}

/*
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
*/
