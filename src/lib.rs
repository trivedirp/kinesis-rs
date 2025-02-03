#![allow(unused)]
mod utils;
pub use utils::*;
use serialport::{SerialPort, SerialPortType};
use std::{
    thread::sleep,
    time::{Duration, Instant},
    error::Error,
    thread,
};

pub struct KinesisDevice {
    port: Box<dyn SerialPort>,
    device_id: u8,
    // vel_degps: f32,
    // acc_degpss: f32,
}

impl KinesisDevice {
    pub fn new(port_name: &str, device_id: u8) -> Self {
        /* let settings = SerialPortSettings {
            baud_rate: 115_200,
            data_bits: DataBits::Eight,
            parity: Parity::None,
            stop_bits: StopBits::One,
            flow_control: FlowControl::None,
            timeout: Duration::from_millis(500),
        }; */
        let mut port = serialport::new(port_name, 115_200)
            .timeout(Duration::from_millis(100))
            .open().expect("Failed to open port");
        println!("Opened port {} successfully.", port_name);

        KinesisDevice {
            port,
            device_id,
        }
    }

    pub fn get_status(&mut self) -> u32 {
        let command = vec![
            utils::MGMSG_REQ_STATUS_BITS,
            0x04,
            self.device_id,
            0x00,
            0x50,
            0x01,
        ];
        
        self.port.write_all(&command);
        
        let mut response = [0u8; 12];
        self.port.read_exact(&mut response);
        
        // Status bits are bytes 8-11 
        let status = u32::from_le_bytes([
            response[8],
            response[9],
            response[10],
            response[11],
        ]);
        // println!("dest: {:8x}", response[4]);
        // println!("source: {:8x}", response[5]);
        // println!("chan ident1: {:8x}", response[6]);
        // println!("chan ident2: {:8x}", response[7]);

        
        status
    }

    pub fn set_vel_params(&mut self, min_vel: f32, acc: f32, max_vel: f32) -> Result<(), Box<dyn Error>> {
        let mut command = vec![
            utils::MGMSG_SET_VELOCITY_PARAMS,
            0x04,
            0x0E, // 14 byte data packet
            0x00,
            0xD0, // dest || 0x80
            0x01,
            0x01, // chan ident
            0x00, // chan ident
        ];
        let min_vel_int = vel_to_counts(min_vel);
        let max_vel_int = vel_to_counts(max_vel);
        let acc_int = acc_to_counts(acc);
        command.extend_from_slice(&min_vel_int.to_le_bytes());  
        command.extend_from_slice(&acc_int.to_le_bytes());  
        command.extend_from_slice(&max_vel_int.to_le_bytes());  
        // println!("command length : {}", command.len());
        self.port.write_all(&command)?;   

        Ok(())
    }

    pub fn home(&mut self) -> Result<(), Box<dyn Error>> {
        let mut command = vec![
            utils::MGMSG_MOVE_HOME,
            0x04,
            self.device_id,
            0x00,
            0x50, // dest
            0x01, // source
        ];        
        self.port.write_all(&command)?;   
        self.wait_home_complete()?;
        Ok(())
    }

    pub fn move_rel(&mut self, deg: f32) -> Result<(), Box<dyn Error>> { 
        let mut command = vec![
            utils::MGMSG_MOVE_RELATIVE,
            0x04,
            0x06, // 6 byte data packet
            0x00,
            0xD0, // dest || 0x80
            0x01,
            0x01, // chan ident
            0x00, // chan ident
        ];        
        let deg_int = deg_to_counts(deg);
        command.extend_from_slice(&deg_int.to_le_bytes());  
        self.port.write_all(&command)?; 
        self.wait_move_complete()?;
        Ok(())
    }

    pub fn move_abs(&mut self, deg: f32) -> Result<(), Box<dyn Error>> { 
        let mut command = vec![
            utils::MGMSG_MOVE_ABSOLUTE,
            0x04,
            0x06, // 6 byte data packet
            0x00,
            0xD0, // dest || 0x80
            0x01,
            0x01, // chan ident
            0x00, // chan ident
        ];        
        let deg_int = deg_to_counts(deg);
        command.extend_from_slice(&deg_int.to_le_bytes());  
        self.port.write_all(&command)?; 
        self.wait_move_complete()?;
        Ok(())
    }

    pub fn wait_home_complete(&mut self) -> Result<(), Box<dyn Error>> {   
        let start_time = std::time::Instant::now();
        let timeout = Duration::from_millis(15000);
        loop {
            let status = self.get_status();
            // println!("Device status: {:32x}", status);
            let is_homing = status & P_MOT_SB_HOMING != 0;  

            if !is_homing {
                return Ok(());
            }
            if start_time.elapsed() > timeout {
                return Err("Movement timeout".into());
            }
            thread::sleep(Duration::from_millis(100));
        }  
    }

    pub fn wait_move_complete(&mut self) -> Result<(), Box<dyn Error>> {   
        let start_time = std::time::Instant::now();
        let timeout = Duration::from_millis(5000);
        loop {
            let status = self.get_status();
            println!("Device status: {:32x}", status);
            // Check if the stage is still moving in either direction
            let is_moving = (status & P_MOT_SB_INMOTIONCW != 0) || 
            (status & P_MOT_SB_INMOTIONCCW != 0);  
            if !is_moving {
                return Ok(());
            }
            if start_time.elapsed() > timeout {
                return Err("Movement timeout".into());
            }
            thread::sleep(Duration::from_millis(10));
        }  
    }
}

#[cfg(test)]
mod tests { 
    use std::iter::{repeat, zip};
    use super::*;
    #[test]
    fn home() {
        let mut device = KinesisDevice::new("/dev/ttyUSB0", 0x01);
        device.home();
        thread::sleep(Duration::from_millis(50));
        let status= device.get_status();
        println!("Device homed stauts: {:32x}", status);
    }

    #[test]
    fn move_rel() {
        let mut device = KinesisDevice::new("/dev/ttyUSB0", 0x01);
        device.set_vel_params(0.0, 10.0, 10.0);
        thread::sleep(Duration::from_millis(50));
        device.move_rel(10.0);
        thread::sleep(Duration::from_millis(50));
        let status= device.get_status();
        println!("Device moved status: {:32x}", status);
    }
    
    #[test]
    fn move_abs() {
        let mut device = KinesisDevice::new("/dev/ttyUSB0", 0x01);
        device.set_vel_params(0.0, 10.0, 10.0);
        thread::sleep(Duration::from_millis(50));
        device.move_abs(10.0);
        thread::sleep(Duration::from_millis(50));
        let status= device.get_status();
        println!("Device moved status: {:32x}", status);
    }
}
