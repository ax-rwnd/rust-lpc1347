#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO0_17 {
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
#[doc = "Possible values of the field `FUNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUNCR {
    #[doc = "PIO0_17."]
    PIO0_17_,
    #[doc = "RTS."]
    RTS_,
    #[doc = "CT32B0_CAP0."]
    CT32B0_CAP0_,
    #[doc = "SCLK (UART synchronous clock)."]
    SCLK_UART_SYNCHRONO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::PIO0_17_ => 0,
            FUNCR::RTS_ => 1,
            FUNCR::CT32B0_CAP0_ => 2,
            FUNCR::SCLK_UART_SYNCHRONO => 3,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::PIO0_17_,
            1 => FUNCR::RTS_,
            2 => FUNCR::CT32B0_CAP0_,
            3 => FUNCR::SCLK_UART_SYNCHRONO,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIO0_17_`"]
    #[inline]
    pub fn is_pio0_17_(&self) -> bool {
        *self == FUNCR::PIO0_17_
    }
    #[doc = "Checks if the value of the field is `RTS_`"]
    #[inline]
    pub fn is_rts_(&self) -> bool {
        *self == FUNCR::RTS_
    }
    #[doc = "Checks if the value of the field is `CT32B0_CAP0_`"]
    #[inline]
    pub fn is_ct32b0_cap0_(&self) -> bool {
        *self == FUNCR::CT32B0_CAP0_
    }
    #[doc = "Checks if the value of the field is `SCLK_UART_SYNCHRONO`"]
    #[inline]
    pub fn is_sclk_uart_synchrono(&self) -> bool {
        *self == FUNCR::SCLK_UART_SYNCHRONO
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE_NO_PULL_DO,
    #[doc = "Pull-down resistor enabled."]
    PULL_DOWN_RESISTOR_E,
    #[doc = "Pull-up resistor enabled."]
    PULL_UP_RESISTOR_ENA,
    #[doc = "Repeater mode."]
    REPEATER_MODE_,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::INACTIVE_NO_PULL_DO => 0,
            MODER::PULL_DOWN_RESISTOR_E => 1,
            MODER::PULL_UP_RESISTOR_ENA => 2,
            MODER::REPEATER_MODE_ => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::INACTIVE_NO_PULL_DO,
            1 => MODER::PULL_DOWN_RESISTOR_E,
            2 => MODER::PULL_UP_RESISTOR_ENA,
            3 => MODER::REPEATER_MODE_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE_NO_PULL_DO`"]
    #[inline]
    pub fn is_inactive_no_pull_do(&self) -> bool {
        *self == MODER::INACTIVE_NO_PULL_DO
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN_RESISTOR_E`"]
    #[inline]
    pub fn is_pull_down_resistor_e(&self) -> bool {
        *self == MODER::PULL_DOWN_RESISTOR_E
    }
    #[doc = "Checks if the value of the field is `PULL_UP_RESISTOR_ENA`"]
    #[inline]
    pub fn is_pull_up_resistor_ena(&self) -> bool {
        *self == MODER::PULL_UP_RESISTOR_ENA
    }
    #[doc = "Checks if the value of the field is `REPEATER_MODE_`"]
    #[inline]
    pub fn is_repeater_mode_(&self) -> bool {
        *self == MODER::REPEATER_MODE_
    }
}
#[doc = "Possible values of the field `HYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSR {
    #[doc = "Disable."]
    DISABLE_,
    #[doc = "Enable."]
    ENABLE_,
}
impl HYSR {
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
            HYSR::DISABLE_ => false,
            HYSR::ENABLE_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HYSR {
        match value {
            false => HYSR::DISABLE_,
            true => HYSR::ENABLE_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_`"]
    #[inline]
    pub fn is_disable_(&self) -> bool {
        *self == HYSR::DISABLE_
    }
    #[doc = "Checks if the value of the field is `ENABLE_`"]
    #[inline]
    pub fn is_enable_(&self) -> bool {
        *self == HYSR::ENABLE_
    }
}
#[doc = "Possible values of the field `INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVR {
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    INPUT_NOT_INVERTED_,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INPUT_INVERTED_HIGH,
}
impl INVR {
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
            INVR::INPUT_NOT_INVERTED_ => false,
            INVR::INPUT_INVERTED_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVR {
        match value {
            false => INVR::INPUT_NOT_INVERTED_,
            true => INVR::INPUT_INVERTED_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_NOT_INVERTED_`"]
    #[inline]
    pub fn is_input_not_inverted_(&self) -> bool {
        *self == INVR::INPUT_NOT_INVERTED_
    }
    #[doc = "Checks if the value of the field is `INPUT_INVERTED_HIGH`"]
    #[inline]
    pub fn is_input_inverted_high(&self) -> bool {
        *self == INVR::INPUT_INVERTED_HIGH
    }
}
#[doc = "Possible values of the field `OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODR {
    #[doc = "Disable."]
    DISABLE_,
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    OPEN_DRAIN_MODE_ENAB,
}
impl ODR {
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
            ODR::DISABLE_ => false,
            ODR::OPEN_DRAIN_MODE_ENAB => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ODR {
        match value {
            false => ODR::DISABLE_,
            true => ODR::OPEN_DRAIN_MODE_ENAB,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_`"]
    #[inline]
    pub fn is_disable_(&self) -> bool {
        *self == ODR::DISABLE_
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN_MODE_ENAB`"]
    #[inline]
    pub fn is_open_drain_mode_enab(&self) -> bool {
        *self == ODR::OPEN_DRAIN_MODE_ENAB
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "PIO0_17."]
    PIO0_17_,
    #[doc = "RTS."]
    RTS_,
    #[doc = "CT32B0_CAP0."]
    CT32B0_CAP0_,
    #[doc = "SCLK (UART synchronous clock)."]
    SCLK_UART_SYNCHRONO,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::PIO0_17_ => 0,
            FUNCW::RTS_ => 1,
            FUNCW::CT32B0_CAP0_ => 2,
            FUNCW::SCLK_UART_SYNCHRONO => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FUNCW<'a> {
    w: &'a mut W,
}
impl<'a> _FUNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FUNCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PIO0_17."]
    #[inline]
    pub fn pio0_17_(self) -> &'a mut W {
        self.variant(FUNCW::PIO0_17_)
    }
    #[doc = "RTS."]
    #[inline]
    pub fn rts_(self) -> &'a mut W {
        self.variant(FUNCW::RTS_)
    }
    #[doc = "CT32B0_CAP0."]
    #[inline]
    pub fn ct32b0_cap0_(self) -> &'a mut W {
        self.variant(FUNCW::CT32B0_CAP0_)
    }
    #[doc = "SCLK (UART synchronous clock)."]
    #[inline]
    pub fn sclk_uart_synchrono(self) -> &'a mut W {
        self.variant(FUNCW::SCLK_UART_SYNCHRONO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE_NO_PULL_DO,
    #[doc = "Pull-down resistor enabled."]
    PULL_DOWN_RESISTOR_E,
    #[doc = "Pull-up resistor enabled."]
    PULL_UP_RESISTOR_ENA,
    #[doc = "Repeater mode."]
    REPEATER_MODE_,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::INACTIVE_NO_PULL_DO => 0,
            MODEW::PULL_DOWN_RESISTOR_E => 1,
            MODEW::PULL_UP_RESISTOR_ENA => 2,
            MODEW::REPEATER_MODE_ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline]
    pub fn inactive_no_pull_do(self) -> &'a mut W {
        self.variant(MODEW::INACTIVE_NO_PULL_DO)
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline]
    pub fn pull_down_resistor_e(self) -> &'a mut W {
        self.variant(MODEW::PULL_DOWN_RESISTOR_E)
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline]
    pub fn pull_up_resistor_ena(self) -> &'a mut W {
        self.variant(MODEW::PULL_UP_RESISTOR_ENA)
    }
    #[doc = "Repeater mode."]
    #[inline]
    pub fn repeater_mode_(self) -> &'a mut W {
        self.variant(MODEW::REPEATER_MODE_)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HYS`"]
pub enum HYSW {
    #[doc = "Disable."]
    DISABLE_,
    #[doc = "Enable."]
    ENABLE_,
}
impl HYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HYSW::DISABLE_ => false,
            HYSW::ENABLE_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HYSW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HYSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable."]
    #[inline]
    pub fn disable_(self) -> &'a mut W {
        self.variant(HYSW::DISABLE_)
    }
    #[doc = "Enable."]
    #[inline]
    pub fn enable_(self) -> &'a mut W {
        self.variant(HYSW::ENABLE_)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INV`"]
pub enum INVW {
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    INPUT_NOT_INVERTED_,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INPUT_INVERTED_HIGH,
}
impl INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW::INPUT_NOT_INVERTED_ => false,
            INVW::INPUT_INVERTED_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVW<'a> {
    w: &'a mut W,
}
impl<'a> _INVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    #[inline]
    pub fn input_not_inverted_(self) -> &'a mut W {
        self.variant(INVW::INPUT_NOT_INVERTED_)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline]
    pub fn input_inverted_high(self) -> &'a mut W {
        self.variant(INVW::INPUT_INVERTED_HIGH)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OD`"]
pub enum ODW {
    #[doc = "Disable."]
    DISABLE_,
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    OPEN_DRAIN_MODE_ENAB,
}
impl ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ODW::DISABLE_ => false,
            ODW::OPEN_DRAIN_MODE_ENAB => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ODW<'a> {
    w: &'a mut W,
}
impl<'a> _ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable."]
    #[inline]
    pub fn disable_(self) -> &'a mut W {
        self.variant(ODW::DISABLE_)
    }
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    #[inline]
    pub fn open_drain_mode_enab(self) -> &'a mut W {
        self.variant(ODW::OPEN_DRAIN_MODE_ENAB)
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
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:2 - Selects pin function. Values 0x4 to 0x7 are reserved."]
    #[inline]
    pub fn func(&self) -> FUNCR {
        FUNCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline]
    pub fn hys(&self) -> HYSR {
        HYSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline]
    pub fn inv(&self) -> INVR {
        INVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline]
    pub fn od(&self) -> ODR {
        ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 144 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Selects pin function. Values 0x4 to 0x7 are reserved."]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline]
    pub fn hys(&mut self) -> _HYSW {
        _HYSW { w: self }
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline]
    pub fn inv(&mut self) -> _INVW {
        _INVW { w: self }
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline]
    pub fn od(&mut self) -> _ODW {
        _ODW { w: self }
    }
}
