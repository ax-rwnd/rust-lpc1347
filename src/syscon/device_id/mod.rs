#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DEVICE_ID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DEVICEIDR {
    bits: u32,
}
impl DEVICEIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - LPC1345FHN33 = 0x2801 0541 LPC1345FBD48 = 0x2801 0541 LPC1346FHN33 = 0x0801 8542 LPC1346FBD48 = 0x0801 8542 LPC1347FHN33 = 0x0802 0543 LPC1347FBD48 = 0x0802 0543 LPC1347FBD64 = 0x0802 0543 LPC1315FHN33 = 0x3A01 0523 LPC1315FBD48 = 0x3A01 0523 LPC1316FHN33 = 0x1A01 8524 LPC1316FBD48 = 0x1A01 8524 LPC1317FHN33 = 0x1A02 0525 LPC1317FBD48 = 0x1A02 0525 LPC1317FBD64 = 0x1A02 0525"]
    #[inline]
    pub fn deviceid(&self) -> DEVICEIDR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DEVICEIDR { bits }
    }
}
