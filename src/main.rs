use serialport::*;

fn main() {
    let mut last_port_count = 0;

    match available_ports() {
       Ok(port_infos) => {
            let curr_port_count = port_infos.len();

            if curr_port_count != last_port_count {
                last_port_count = curr_port_count;
                println!("last_port_count: {}", last_port_count);

                for port_info in port_infos {
                    println!("Type: {:?}", port_info.port_type);
                    println!("Name: {:?}", port_info.port_name);
                }

            }
       },
       Err(err_var) => println!("Err {:?}", err_var)
    }
}
