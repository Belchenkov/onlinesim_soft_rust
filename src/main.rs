use serialport::*;

fn main() {
    match available_ports() {
        Ok(ok_var) => println!("PORTS {:?}", ok_var),
       _ => println!("Err")
    }
}
