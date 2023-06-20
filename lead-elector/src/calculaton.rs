use std::env;
use std::net::{TcpListener, TcpStream};
use std::result::Result::{Err, Ok};

struct CalculationResult {
    result: i32,
}

impl CalculationResult {
    fn new() -> Self {
        Self { result: 0 }
    }

    fn calc_emc2(&mut self, mass: i32) -> i32 {
        let c = 300;
        self.result = mass * c * c;
        self.result
    }

    //todo - complete tcp for calc result to be sent over network

    // fn handle_connection() -> Result<Ok, Err> {
    //     println!("address of connection: {:?}", conn.peer_addr().unwrap());
    //     let mut buffer = [0u8; 512];
    //     let rbytes = conn.read(&mut buffer).unwrap();
    //     if rbytes == 0 {
    //         return Ok(());
    //     }
    //     conn.write(&buffer[..rbytes]);
    // }
}



#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    #[test]
    fn test_given_mass_return_emc2() {
        let mut calc = CalculationResult::new();
        let result = calc.calc_emc2(100);
        assert_eq!(result, 9000000i32);
    }

    #[test]
    #[ignore = "to be continued"]
    fn test_given_result_send_over_network() {
        let result = 9000000i32;
        
        let mut conn = TcpStream::connect("0.0.0.0:8888".to_string()).unwrap();

    }
}
