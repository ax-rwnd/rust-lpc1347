#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `RITINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RITINTR {
    #[doc = "This bit is set to 1 by hardware whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. Writing a 1 to this bit will clear it to 0. Writing a 0 has no effect."]
    MASK,
    #[doc = "The counter value does not equal the masked compare value."]
    NO_MASK,
}
impl RITINTR {
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
            RITINTR::MASK => true,
            RITINTR::NO_MASK => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RITINTR {
        match value {
            true => RITINTR::MASK,
            false => RITINTR::NO_MASK,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline]
    pub fn is_mask(&self) -> bool {
        *self == RITINTR::MASK
    }
    #[doc = "Checks if the value of the field is `NO_MASK`"]
    #[inline]
    pub fn is_no_mask(&self) -> bool {
        *self == RITINTR::NO_MASK
    }
}
#[doc = "Possible values of the field `RITENCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RITENCLRR {
    #[doc = "The timer will be cleared to 0 whenever the counter value equals the masked compare value specified by the contents of COMPVAL/COMPVAL_H and MASK/MASK_H registers. This will occur on the same clock that sets the interrupt flag."]
    CLEAR_ON_O,
    #[doc = "The timer will not be cleared to 0."]
    NOT_CLEAR_ON_0,
}
impl RITENCLRR {
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
            RITENCLRR::CLEAR_ON_O => true,
            RITENCLRR::NOT_CLEAR_ON_0 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RITENCLRR {
        match value {
            true => RITENCLRR::CLEAR_ON_O,
            false => RITENCLRR::NOT_CLEAR_ON_0,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_ON_O`"]
    #[inline]
    pub fn is_clear_on_o(&self) -> bool {
        *self == RITENCLRR::CLEAR_ON_O
    }
    #[doc = "Checks if the value of the field is `NOT_CLEAR_ON_0`"]
    #[inline]
    pub fn is_not_clear_on_0(&self) -> bool {
        *self == RITENCLRR::NOT_CLEAR_ON_0
    }
}
#[doc = "Possible values of the field `RITENBR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RITENBRR {
    #[doc = "The timer is halted when the processor is halted for debugging."]
    HALT_ON_DEBUG,
    #[doc = "Debug has no effect on the timer operation."]
    NO_EFFECT_ON_DEBUG,
}
impl RITENBRR {
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
            RITENBRR::HALT_ON_DEBUG => true,
            RITENBRR::NO_EFFECT_ON_DEBUG => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RITENBRR {
        match value {
            true => RITENBRR::HALT_ON_DEBUG,
            false => RITENBRR::NO_EFFECT_ON_DEBUG,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_ON_DEBUG`"]
    #[inline]
    pub fn is_halt_on_debug(&self) -> bool {
        *self == RITENBRR::HALT_ON_DEBUG
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT_ON_DEBUG`"]
    #[inline]
    pub fn is_no_effect_on_debug(&self) -> bool {
        *self == RITENBRR::NO_EFFECT_ON_DEBUG
    }
}
#[doc = "Possible values of the field `RITEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RITENR {
    #[doc = "Timer enabled. This can be overruled by a debug halt if enabled in bit 2."]
    TIMER_ENABLED,
    #[doc = "Timer disabled."]
    TIMER_DISABLED_,
}
impl RITENR {
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
            RITENR::TIMER_ENABLED => true,
            RITENR::TIMER_DISABLED_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RITENR {
        match value {
            true => RITENR::TIMER_ENABLED,
            false => RITENR::TIMER_DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_ENABLED`"]
    #[inline]
    pub fn is_timer_enabled(&self) -> bool {
        *self == RITENR::TIMER_ENABLED
    }
    #[doc = "Checks if the value of the field is `TIMER_DISABLED_`"]
    #[inline]
    pub fn is_timer_disabled_(&self) -> bool {
        *self == RITENR::TIMER_DISABLED_
    }
}
#[doc = "Values that can be written to the field `RITINT`"]
pub enum RITINTW {
    #[doc = "This bit is set to 1 by hardware whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. Writing a 1 to this bit will clear it to 0. Writing a 0 has no effect."]
    MASK,
    #[doc = "The counter value does not equal the masked compare value."]
    NO_MASK,
}
impl RITINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RITINTW::MASK => true,
            RITINTW::NO_MASK => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RITINTW<'a> {
    w: &'a mut W,
}
impl<'a> _RITINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RITINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is set to 1 by hardware whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. Writing a 1 to this bit will clear it to 0. Writing a 0 has no effect."]
    #[inline]
    pub fn mask(self) -> &'a mut W {
        self.variant(RITINTW::MASK)
    }
    #[doc = "The counter value does not equal the masked compare value."]
    #[inline]
    pub fn no_mask(self) -> &'a mut W {
        self.variant(RITINTW::NO_MASK)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RITENCLR`"]
pub enum RITENCLRW {
    #[doc = "The timer will be cleared to 0 whenever the counter value equals the masked compare value specified by the contents of COMPVAL/COMPVAL_H and MASK/MASK_H registers. This will occur on the same clock that sets the interrupt flag."]
    CLEAR_ON_O,
    #[doc = "The timer will not be cleared to 0."]
    NOT_CLEAR_ON_0,
}
impl RITENCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RITENCLRW::CLEAR_ON_O => true,
            RITENCLRW::NOT_CLEAR_ON_0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RITENCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RITENCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RITENCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The timer will be cleared to 0 whenever the counter value equals the masked compare value specified by the contents of COMPVAL/COMPVAL_H and MASK/MASK_H registers. This will occur on the same clock that sets the interrupt flag."]
    #[inline]
    pub fn clear_on_o(self) -> &'a mut W {
        self.variant(RITENCLRW::CLEAR_ON_O)
    }
    #[doc = "The timer will not be cleared to 0."]
    #[inline]
    pub fn not_clear_on_0(self) -> &'a mut W {
        self.variant(RITENCLRW::NOT_CLEAR_ON_0)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RITENBR`"]
pub enum RITENBRW {
    #[doc = "The timer is halted when the processor is halted for debugging."]
    HALT_ON_DEBUG,
    #[doc = "Debug has no effect on the timer operation."]
    NO_EFFECT_ON_DEBUG,
}
impl RITENBRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RITENBRW::HALT_ON_DEBUG => true,
            RITENBRW::NO_EFFECT_ON_DEBUG => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RITENBRW<'a> {
    w: &'a mut W,
}
impl<'a> _RITENBRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RITENBRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The timer is halted when the processor is halted for debugging."]
    #[inline]
    pub fn halt_on_debug(self) -> &'a mut W {
        self.variant(RITENBRW::HALT_ON_DEBUG)
    }
    #[doc = "Debug has no effect on the timer operation."]
    #[inline]
    pub fn no_effect_on_debug(self) -> &'a mut W {
        self.variant(RITENBRW::NO_EFFECT_ON_DEBUG)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RITEN`"]
pub enum RITENW {
    #[doc = "Timer enabled. This can be overruled by a debug halt if enabled in bit 2."]
    TIMER_ENABLED,
    #[doc = "Timer disabled."]
    TIMER_DISABLED_,
}
impl RITENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RITENW::TIMER_ENABLED => true,
            RITENW::TIMER_DISABLED_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RITENW<'a> {
    w: &'a mut W,
}
impl<'a> _RITENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RITENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer enabled. This can be overruled by a debug halt if enabled in bit 2."]
    #[inline]
    pub fn timer_enabled(self) -> &'a mut W {
        self.variant(RITENW::TIMER_ENABLED)
    }
    #[doc = "Timer disabled."]
    #[inline]
    pub fn timer_disabled_(self) -> &'a mut W {
        self.variant(RITENW::TIMER_DISABLED_)
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Interrupt flag"]
    #[inline]
    pub fn ritint(&self) -> RITINTR {
        RITINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Timer enable clear"]
    #[inline]
    pub fn ritenclr(&self) -> RITENCLRR {
        RITENCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Timer enable for debug"]
    #[inline]
    pub fn ritenbr(&self) -> RITENBRR {
        RITENBRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Timer enable."]
    #[inline]
    pub fn riten(&self) -> RITENR {
        RITENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 12 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt flag"]
    #[inline]
    pub fn ritint(&mut self) -> _RITINTW {
        _RITINTW { w: self }
    }
    #[doc = "Bit 1 - Timer enable clear"]
    #[inline]
    pub fn ritenclr(&mut self) -> _RITENCLRW {
        _RITENCLRW { w: self }
    }
    #[doc = "Bit 2 - Timer enable for debug"]
    #[inline]
    pub fn ritenbr(&mut self) -> _RITENBRW {
        _RITENBRW { w: self }
    }
    #[doc = "Bit 3 - Timer enable."]
    #[inline]
    pub fn riten(&mut self) -> _RITENW {
        _RITENW { w: self }
    }
}
