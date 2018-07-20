#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct SELR {
    bits: u8,
}
impl SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLKDIVR {
    bits: u8,
}
impl CLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTR {
    #[doc = "Software-controlled mode: Conversions are software-controlled and require 31 clocks."]
    SOFTWARE_CONTROLLED_,
    #[doc = "Hardware scan mode: The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by ones in the SEL field. The first conversion after the start corresponds to the least-significant bit set to 1 in the SEL field, then the next higher  bits (pins) set to one are scanned if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1, or conversions will not start."]
    HARDWARE_SCAN_MODE_,
}
impl BURSTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BURSTR::SOFTWARE_CONTROLLED_ => false,
            BURSTR::HARDWARE_SCAN_MODE_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BURSTR {
        match value {
            false => BURSTR::SOFTWARE_CONTROLLED_,
            true => BURSTR::HARDWARE_SCAN_MODE_,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE_CONTROLLED_`"]
    #[inline]
    pub fn is_software_controlled_(&self) -> bool {
        *self == BURSTR::SOFTWARE_CONTROLLED_
    }
    #[doc = "Checks if the value of the field is `HARDWARE_SCAN_MODE_`"]
    #[inline]
    pub fn is_hardware_scan_mode_(&self) -> bool {
        *self == BURSTR::HARDWARE_SCAN_MODE_
    }
}
#[doc = "Possible values of the field `LPWRMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPWRMODER {
    #[doc = "Disable the low-power ADC mode. The analog circuitry remains activated when no conversions are requested."]
    DISABLE_THE_LOW_POWE,
    #[doc = "Enable the low-power ADC mode.  The analog circuitry is automatically powered-down when no conversions are taking  place. When any (hardware or software) triggering event is detected, the analog circuitry  is enabled. After the required start-up time, the requested conversion will be launched.  Once the conversion completes, the analog-circuitry will again be powered-down provided  no further conversions are pending.  This mode will NOT power-up the A/D if the ADC is powered down (ADC_PD bit in the PDRUNCFG register is HIGH) or if the part is in Deep-sleep, Power-down, or Deep power-down mode."]
    ENABLE_THE_LOW_POWER,
}
impl LPWRMODER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LPWRMODER::DISABLE_THE_LOW_POWE => false,
            LPWRMODER::ENABLE_THE_LOW_POWER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPWRMODER {
        match value {
            false => LPWRMODER::DISABLE_THE_LOW_POWE,
            true => LPWRMODER::ENABLE_THE_LOW_POWER,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_LOW_POWE`"]
    #[inline]
    pub fn is_disable_the_low_powe(&self) -> bool {
        *self == LPWRMODER::DISABLE_THE_LOW_POWE
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_LOW_POWER`"]
    #[inline]
    pub fn is_enable_the_low_power(&self) -> bool {
        *self == LPWRMODER::ENABLE_THE_LOW_POWER
    }
}
#[doc = "Possible values of the field `MODE10BIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE10BITR {
    #[doc = "Disable the 10-bit conversion rate mode."]
    DISABLE_THE_10_BIT_C,
    #[doc = "Enable the 10-bit conversion rate mode with high conversion rate.The A/D resolution is reduced to 10 bits (the two LSB of the conversion result will be forced  to 0). The clock rate (set via the CLKDIV field) can be doubled to up to 31 MHz to  achieve a conversion rate of up to one million samples per second."]
    ENABLE_THE_10_BIT_CO,
}
impl MODE10BITR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MODE10BITR::DISABLE_THE_10_BIT_C => false,
            MODE10BITR::ENABLE_THE_10_BIT_CO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODE10BITR {
        match value {
            false => MODE10BITR::DISABLE_THE_10_BIT_C,
            true => MODE10BITR::ENABLE_THE_10_BIT_CO,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_10_BIT_C`"]
    #[inline]
    pub fn is_disable_the_10_bit_c(&self) -> bool {
        *self == MODE10BITR::DISABLE_THE_10_BIT_C
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_10_BIT_CO`"]
    #[inline]
    pub fn is_enable_the_10_bit_co(&self) -> bool {
        *self == MODE10BITR::ENABLE_THE_10_BIT_CO
    }
}
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    NO_START_THIS_VALUE,
    #[doc = "Start conversion now."]
    START_CONVERSION_NOW,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO0_2/SSEL/CT16B0_CAP0."]
    PIO0_2,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO1_5/DIR/CT32B0_CAP0."]
    PIO1_5,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT0\\[1\\]."]
    CT32B0_MAT0,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT1\\[1\\]."]
    CT32B0_MAT1,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT0\\[1\\]."]
    CT16B0_MAT0,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT1\\[1\\]."]
    CT16B0_MAT1,
}
impl STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STARTR::NO_START_THIS_VALUE => 0,
            STARTR::START_CONVERSION_NOW => 1,
            STARTR::PIO0_2 => 2,
            STARTR::PIO1_5 => 3,
            STARTR::CT32B0_MAT0 => 4,
            STARTR::CT32B0_MAT1 => 5,
            STARTR::CT16B0_MAT0 => 6,
            STARTR::CT16B0_MAT1 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STARTR {
        match value {
            0 => STARTR::NO_START_THIS_VALUE,
            1 => STARTR::START_CONVERSION_NOW,
            2 => STARTR::PIO0_2,
            3 => STARTR::PIO1_5,
            4 => STARTR::CT32B0_MAT0,
            5 => STARTR::CT32B0_MAT1,
            6 => STARTR::CT16B0_MAT0,
            7 => STARTR::CT16B0_MAT1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_START_THIS_VALUE`"]
    #[inline]
    pub fn is_no_start_this_value(&self) -> bool {
        *self == STARTR::NO_START_THIS_VALUE
    }
    #[doc = "Checks if the value of the field is `START_CONVERSION_NOW`"]
    #[inline]
    pub fn is_start_conversion_now(&self) -> bool {
        *self == STARTR::START_CONVERSION_NOW
    }
    #[doc = "Checks if the value of the field is `PIO0_2`"]
    #[inline]
    pub fn is_pio0_2(&self) -> bool {
        *self == STARTR::PIO0_2
    }
    #[doc = "Checks if the value of the field is `PIO1_5`"]
    #[inline]
    pub fn is_pio1_5(&self) -> bool {
        *self == STARTR::PIO1_5
    }
    #[doc = "Checks if the value of the field is `CT32B0_MAT0`"]
    #[inline]
    pub fn is_ct32b0_mat0(&self) -> bool {
        *self == STARTR::CT32B0_MAT0
    }
    #[doc = "Checks if the value of the field is `CT32B0_MAT1`"]
    #[inline]
    pub fn is_ct32b0_mat1(&self) -> bool {
        *self == STARTR::CT32B0_MAT1
    }
    #[doc = "Checks if the value of the field is `CT16B0_MAT0`"]
    #[inline]
    pub fn is_ct16b0_mat0(&self) -> bool {
        *self == STARTR::CT16B0_MAT0
    }
    #[doc = "Checks if the value of the field is `CT16B0_MAT1`"]
    #[inline]
    pub fn is_ct16b0_mat1(&self) -> bool {
        *self == STARTR::CT16B0_MAT1
    }
}
#[doc = "Possible values of the field `EDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGER {
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    RISING,
    #[doc = "Start conversion on a falling edge on the selected CAP/MAT signal."]
    FALLING,
}
impl EDGER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EDGER::RISING => false,
            EDGER::FALLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDGER {
        match value {
            false => EDGER::RISING,
            true => EDGER::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == EDGER::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == EDGER::FALLING
    }
}
#[doc = r" Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BURST`"]
pub enum BURSTW {
    #[doc = "Software-controlled mode: Conversions are software-controlled and require 31 clocks."]
    SOFTWARE_CONTROLLED_,
    #[doc = "Hardware scan mode: The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by ones in the SEL field. The first conversion after the start corresponds to the least-significant bit set to 1 in the SEL field, then the next higher  bits (pins) set to one are scanned if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1, or conversions will not start."]
    HARDWARE_SCAN_MODE_,
}
impl BURSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BURSTW::SOFTWARE_CONTROLLED_ => false,
            BURSTW::HARDWARE_SCAN_MODE_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software-controlled mode: Conversions are software-controlled and require 31 clocks."]
    #[inline]
    pub fn software_controlled_(self) -> &'a mut W {
        self.variant(BURSTW::SOFTWARE_CONTROLLED_)
    }
    #[doc = "Hardware scan mode: The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by ones in the SEL field. The first conversion after the start corresponds to the least-significant bit set to 1 in the SEL field, then the next higher bits (pins) set to one are scanned if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1, or conversions will not start."]
    #[inline]
    pub fn hardware_scan_mode_(self) -> &'a mut W {
        self.variant(BURSTW::HARDWARE_SCAN_MODE_)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPWRMODE`"]
pub enum LPWRMODEW {
    #[doc = "Disable the low-power ADC mode. The analog circuitry remains activated when no conversions are requested."]
    DISABLE_THE_LOW_POWE,
    #[doc = "Enable the low-power ADC mode.  The analog circuitry is automatically powered-down when no conversions are taking  place. When any (hardware or software) triggering event is detected, the analog circuitry  is enabled. After the required start-up time, the requested conversion will be launched.  Once the conversion completes, the analog-circuitry will again be powered-down provided  no further conversions are pending.  This mode will NOT power-up the A/D if the ADC is powered down (ADC_PD bit in the PDRUNCFG register is HIGH) or if the part is in Deep-sleep, Power-down, or Deep power-down mode."]
    ENABLE_THE_LOW_POWER,
}
impl LPWRMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPWRMODEW::DISABLE_THE_LOW_POWE => false,
            LPWRMODEW::ENABLE_THE_LOW_POWER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPWRMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPWRMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPWRMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the low-power ADC mode. The analog circuitry remains activated when no conversions are requested."]
    #[inline]
    pub fn disable_the_low_powe(self) -> &'a mut W {
        self.variant(LPWRMODEW::DISABLE_THE_LOW_POWE)
    }
    #[doc = "Enable the low-power ADC mode. The analog circuitry is automatically powered-down when no conversions are taking place. When any (hardware or software) triggering event is detected, the analog circuitry is enabled. After the required start-up time, the requested conversion will be launched. Once the conversion completes, the analog-circuitry will again be powered-down provided no further conversions are pending. This mode will NOT power-up the A/D if the ADC is powered down (ADC_PD bit in the PDRUNCFG register is HIGH) or if the part is in Deep-sleep, Power-down, or Deep power-down mode."]
    #[inline]
    pub fn enable_the_low_power(self) -> &'a mut W {
        self.variant(LPWRMODEW::ENABLE_THE_LOW_POWER)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE10BIT`"]
pub enum MODE10BITW {
    #[doc = "Disable the 10-bit conversion rate mode."]
    DISABLE_THE_10_BIT_C,
    #[doc = "Enable the 10-bit conversion rate mode with high conversion rate.The A/D resolution is reduced to 10 bits (the two LSB of the conversion result will be forced  to 0). The clock rate (set via the CLKDIV field) can be doubled to up to 31 MHz to  achieve a conversion rate of up to one million samples per second."]
    ENABLE_THE_10_BIT_CO,
}
impl MODE10BITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODE10BITW::DISABLE_THE_10_BIT_C => false,
            MODE10BITW::ENABLE_THE_10_BIT_CO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE10BITW<'a> {
    w: &'a mut W,
}
impl<'a> _MODE10BITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE10BITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the 10-bit conversion rate mode."]
    #[inline]
    pub fn disable_the_10_bit_c(self) -> &'a mut W {
        self.variant(MODE10BITW::DISABLE_THE_10_BIT_C)
    }
    #[doc = "Enable the 10-bit conversion rate mode with high conversion rate.The A/D resolution is reduced to 10 bits (the two LSB of the conversion result will be forced to 0). The clock rate (set via the CLKDIV field) can be doubled to up to 31 MHz to achieve a conversion rate of up to one million samples per second."]
    #[inline]
    pub fn enable_the_10_bit_co(self) -> &'a mut W {
        self.variant(MODE10BITW::ENABLE_THE_10_BIT_CO)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `START`"]
pub enum STARTW {
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    NO_START_THIS_VALUE,
    #[doc = "Start conversion now."]
    START_CONVERSION_NOW,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO0_2/SSEL/CT16B0_CAP0."]
    PIO0_2,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO1_5/DIR/CT32B0_CAP0."]
    PIO1_5,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT0\\[1\\]."]
    CT32B0_MAT0,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT1\\[1\\]."]
    CT32B0_MAT1,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT0\\[1\\]."]
    CT16B0_MAT0,
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT1\\[1\\]."]
    CT16B0_MAT1,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STARTW::NO_START_THIS_VALUE => 0,
            STARTW::START_CONVERSION_NOW => 1,
            STARTW::PIO0_2 => 2,
            STARTW::PIO1_5 => 3,
            STARTW::CT32B0_MAT0 => 4,
            STARTW::CT32B0_MAT1 => 5,
            STARTW::CT16B0_MAT0 => 6,
            STARTW::CT16B0_MAT1 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    #[inline]
    pub fn no_start_this_value(self) -> &'a mut W {
        self.variant(STARTW::NO_START_THIS_VALUE)
    }
    #[doc = "Start conversion now."]
    #[inline]
    pub fn start_conversion_now(self) -> &'a mut W {
        self.variant(STARTW::START_CONVERSION_NOW)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO0_2/SSEL/CT16B0_CAP0."]
    #[inline]
    pub fn pio0_2(self) -> &'a mut W {
        self.variant(STARTW::PIO0_2)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO1_5/DIR/CT32B0_CAP0."]
    #[inline]
    pub fn pio1_5(self) -> &'a mut W {
        self.variant(STARTW::PIO1_5)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT0\\[1\\]."]
    #[inline]
    pub fn ct32b0_mat0(self) -> &'a mut W {
        self.variant(STARTW::CT32B0_MAT0)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT1\\[1\\]."]
    #[inline]
    pub fn ct32b0_mat1(self) -> &'a mut W {
        self.variant(STARTW::CT32B0_MAT1)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT0\\[1\\]."]
    #[inline]
    pub fn ct16b0_mat0(self) -> &'a mut W {
        self.variant(STARTW::CT16B0_MAT0)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT1\\[1\\]."]
    #[inline]
    pub fn ct16b0_mat1(self) -> &'a mut W {
        self.variant(STARTW::CT16B0_MAT1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDGE`"]
pub enum EDGEW {
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    RISING,
    #[doc = "Start conversion on a falling edge on the selected CAP/MAT signal."]
    FALLING,
}
impl EDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDGEW::RISING => false,
            EDGEW::FALLING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGEW::RISING)
    }
    #[doc = "Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(EDGEW::FALLING)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Selects which of the AD7:0 pins is (are) to be sampled and converted. Bit 0 selects Pin AD0, bit 1 selects pin AD1,..., and bit 7 selects pin AD7. In software-controlled mode (BURST = 0), only one channel can be selected, i.e. only one of these bits should be 1. In hardware scan mode (BURST = 1), any numbers of channels can be selected, i.e any or all bits can be set to 1. If all bits are set to 0, channel 0 is selected automatically (SEL = 0x01)."]
    #[inline]
    pub fn sel(&self) -> SELR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SELR { bits }
    }
    #[doc = "Bits 8:15 - The main clock (PCLK_ADC) is divided by (this value plus one) to produce the clock for the A/D converter. The clock should be less than or equal to 15.5 MHz(12-bit mode) or 31 MHz (10-bit mode) in software-controlled mode (BURST bit = 0).. Typically, software should program the smallest value in this field that yields a clock of 15.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline]
    pub fn clkdiv(&self) -> CLKDIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLKDIVR { bits }
    }
    #[doc = "Bit 16 - Burst mode If BURST is set to 1, the ADGINTEN bit in the INTEN register (Table 327) must be set to 0."]
    #[inline]
    pub fn burst(&self) -> BURSTR {
        BURSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Low-power mode"]
    #[inline]
    pub fn lpwrmode(&self) -> LPWRMODER {
        LPWRMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - 10-bit conversion rate mode"]
    #[inline]
    pub fn mode10bit(&self) -> MODE10BITR {
        MODE10BITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
    #[inline]
    pub fn start(&self) -> STARTR {
        STARTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 27 - Edge control. This bit is significant only when the START field contains 010-111."]
    #[inline]
    pub fn edge(&self) -> EDGER {
        EDGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Selects which of the AD7:0 pins is (are) to be sampled and converted. Bit 0 selects Pin AD0, bit 1 selects pin AD1,..., and bit 7 selects pin AD7. In software-controlled mode (BURST = 0), only one channel can be selected, i.e. only one of these bits should be 1. In hardware scan mode (BURST = 1), any numbers of channels can be selected, i.e any or all bits can be set to 1. If all bits are set to 0, channel 0 is selected automatically (SEL = 0x01)."]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
    #[doc = "Bits 8:15 - The main clock (PCLK_ADC) is divided by (this value plus one) to produce the clock for the A/D converter. The clock should be less than or equal to 15.5 MHz(12-bit mode) or 31 MHz (10-bit mode) in software-controlled mode (BURST bit = 0).. Typically, software should program the smallest value in this field that yields a clock of 15.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline]
    pub fn clkdiv(&mut self) -> _CLKDIVW {
        _CLKDIVW { w: self }
    }
    #[doc = "Bit 16 - Burst mode If BURST is set to 1, the ADGINTEN bit in the INTEN register (Table 327) must be set to 0."]
    #[inline]
    pub fn burst(&mut self) -> _BURSTW {
        _BURSTW { w: self }
    }
    #[doc = "Bit 22 - Low-power mode"]
    #[inline]
    pub fn lpwrmode(&mut self) -> _LPWRMODEW {
        _LPWRMODEW { w: self }
    }
    #[doc = "Bit 23 - 10-bit conversion rate mode"]
    #[inline]
    pub fn mode10bit(&mut self) -> _MODE10BITW {
        _MODE10BITW { w: self }
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 27 - Edge control. This bit is significant only when the START field contains 010-111."]
    #[inline]
    pub fn edge(&mut self) -> _EDGEW {
        _EDGEW { w: self }
    }
}
