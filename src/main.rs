use serialport::*;

fn main() {
    match available_ports() {
       Ok(ports) => {
            for port in ports {
                println!("PORTS {:?}", port);
                println!("----------------");
                println!("PORTS {:?}", port.port_type);
            }
       },
       Err(err_var) => println!("Err {:?}", err_var)
    }
}
