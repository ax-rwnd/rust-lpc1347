#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Compare value LSB register. Holds the 32 LSBs of the compare value."]
    pub compval: COMPVAL,
    #[doc = "0x04 - Mask LSB register. This register holds the 32 LSB s of the mask value. A one written to any bit will force a compare on the corresponding bit of the counter and compare register."]
    pub mask: MASK,
    #[doc = "0x08 - Control register."]
    pub ctrl: CTRL,
    #[doc = "0x0c - Counter LSB register. 32 LSBs of the counter."]
    pub counter: COUNTER,
    #[doc = "0x10 - Compare value MSB register. Holds the 16 MSBs of the compare value."]
    pub compval_h: COMPVAL_H,
    #[doc = "0x14 - Mask MSB register. This register holds the 16 MSBs of the mask value. A one written to any bit will force a compare on the corresponding bit of the counter and compare register."]
    pub mask_h: MASK_H,
    #[doc = "0x18 - Counter MSB register. 16 MSBs of the counter."]
    pub counter_h: COUNTER_H,
}
#[doc = "Compare value LSB register. Holds the 32 LSBs of the compare value."]
pub struct COMPVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare value LSB register. Holds the 32 LSBs of the compare value."]
pub mod compval;
#[doc = "Mask LSB register. This register holds the 32 LSB s of the mask value. A one written to any bit will force a compare on the corresponding bit of the counter and compare register."]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask LSB register. This register holds the 32 LSB s of the mask value. A one written to any bit will force a compare on the corresponding bit of the counter and compare register."]
pub mod mask;
#[doc = "Control register."]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register."]
pub mod ctrl;
#[doc = "Counter LSB register. 32 LSBs of the counter."]
pub struct COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter LSB register. 32 LSBs of the counter."]
pub mod counter;
#[doc = "Compare value MSB register. Holds the 16 MSBs of the compare value."]
pub struct COMPVAL_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare value MSB register. Holds the 16 MSBs of the compare value."]
pub mod compval_h;
#[doc = "Mask MSB register. This register holds the 16 MSBs of the mask value. A one written to any bit will force a compare on the corresponding bit of the counter and compare register."]
pub struct MASK_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask MSB register. This register holds the 16 MSBs of the mask value. A one written to any bit will force a compare on the corresponding bit of the counter and compare register."]
pub mod mask_h;
#[doc = "Counter MSB register. 16 MSBs of the counter."]
pub struct COUNTER_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter MSB register. 16 MSBs of the counter."]
pub mod counter_h;
