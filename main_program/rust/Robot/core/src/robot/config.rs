/// 定数の置き場所
pub const STOP: u32 = 0x1F00FFFF;
pub const EMERGENCY_STOP: u32 = 0x0F00FFF1;
pub const BREAK: u32 = 0x0FFFFFFF;
pub const None: u32 = 0xffffffff;
pub const FRONT:u32 = 0x1FEEFFFF;
pub const TRUN:u32 = 0x1FAEFFFF;

pub type SenderOrders = std::sync::mpsc::Sender<u32>;
