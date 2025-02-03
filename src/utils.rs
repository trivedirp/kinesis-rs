/// device setting
const COUNTS_PER_DEG: f32 = 1919.64;
const COUNTS_PER_VEL: f32 = 42941.66;
const COUNTS_PER_ACC: f32 = 14.66;

/// device port info 
pub const DESTINATION: u8 = 0x50;
pub const SOURCE: u8 = 0x01;

/// message cocdes
pub const MGMSG_MOVE_HOME: u8 = 0x43;
pub const MGMSG_MOVE_ABSOLUTE: u8 = 0x53;
pub const MGMSG_MOVE_RELATIVE: u8 = 0x48;
pub const MGMSG_MOVE_STOP: u8 = 0x65;
pub const MGMSG_REQ_STATUS_BITS: u8 = 0x29;
pub const MGMSG_SET_VELOCITY_PARAMS: u8 = 0x13;
pub const MGMSG_MOT_SET_MOVERELPARAMS: u8 = 0x45; 
pub const MGMSG_MOT_SET_MOVEABSPARAMS: u8 = 0x50; 
pub const MGMSG_MOT_SET_VELPARAMS: u8 = 0x13;

/// status bits
pub const P_MOT_SB_INMOTIONCW: u32 = 0x00000010;
pub const P_MOT_SB_INMOTIONCCW: u32 = 0x00000020;
pub const P_MOT_SB_SETTLED: u32 = 0x00002000;
pub const P_MOT_SB_HOMING: u32 = 0x00000200;
pub const P_MOT_SB_HOMED: u32 = 0x00000400;


pub fn deg_to_counts(deg: f32) -> i32 {
    (deg * COUNTS_PER_DEG).round() as i32
}
pub fn vel_to_counts(vel: f32) -> i32 {
    (vel * COUNTS_PER_VEL).round() as i32
}
pub fn acc_to_counts(acc: f32) -> i32 {
    (acc * COUNTS_PER_ACC).round() as i32
}
