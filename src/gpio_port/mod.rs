#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Byte pin registers port 0/1; pins PIO0/1_0 to PIO0/1_31"]
    pub b: [B; 64],
    _reserved1: [u8; 4032usize],
    #[doc = "0x1000 - Word pin registers port 0/1"]
    pub w: [W; 64],
    _reserved2: [u8; 3840usize],
    #[doc = "0x2000 - Direction registers port 0/1"]
    pub dir: [DIR; 2],
    _reserved3: [u8; 120usize],
    #[doc = "0x2080 - Mask register port 0/1"]
    pub mask: [MASK; 2],
    _reserved4: [u8; 120usize],
    #[doc = "0x2100 - Portpin register port 0"]
    pub pin: [PIN; 2],
    _reserved5: [u8; 120usize],
    #[doc = "0x2180 - Masked port register port 0/1"]
    pub mpin: [MPIN; 2],
    _reserved6: [u8; 120usize],
    #[doc = "0x2200 - Write: Set register for port 0/1 Read: output bits for port 0/1"]
    pub set: [SET; 2],
    _reserved7: [u8; 120usize],
    #[doc = "0x2280 - Clear port 0/1"]
    pub clr: [CLR; 2],
    _reserved8: [u8; 120usize],
    #[doc = "0x2300 - Toggle port 0/1"]
    pub not: [NOT; 2],
}
#[doc = "Byte pin registers port 0/1; pins PIO0/1_0 to PIO0/1_31"]
pub struct B {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Byte pin registers port 0/1; pins PIO0/1_0 to PIO0/1_31"]
pub mod b;
#[doc = "Word pin registers port 0/1"]
pub struct W {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Word pin registers port 0/1"]
pub mod w;
#[doc = "Direction registers port 0/1"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direction registers port 0/1"]
pub mod dir;
#[doc = "Mask register port 0/1"]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask register port 0/1"]
pub mod mask;
#[doc = "Portpin register port 0"]
pub struct PIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Portpin register port 0"]
pub mod pin;
#[doc = "Masked port register port 0/1"]
pub struct MPIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked port register port 0/1"]
pub mod mpin;
#[doc = "Write: Set register for port 0/1 Read: output bits for port 0/1"]
pub struct SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write: Set register for port 0/1 Read: output bits for port 0/1"]
pub mod set;
#[doc = "Clear port 0/1"]
pub struct CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear port 0/1"]
pub mod clr;
#[doc = "Toggle port 0/1"]
pub struct NOT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Toggle port 0/1"]
pub mod not;
