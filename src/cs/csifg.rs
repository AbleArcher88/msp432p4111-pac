#[doc = "Register `CSIFG` reader"]
pub struct R(crate::R<CSIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LFXTIFG` reader - LFXT oscillator fault flag"]
pub type LFXTIFG_R = crate::BitReader<LFXTIFG_ENUM_READ_A>;
#[doc = "LFXT oscillator fault flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFXTIFG_ENUM_READ_A {
    #[doc = "0: No fault condition occurred after the last reset"]
    LFXTIFG_0 = 0,
    #[doc = "1: LFXT fault. A LFXT fault occurred after the last reset"]
    LFXTIFG_1 = 1,
}
impl From<LFXTIFG_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: LFXTIFG_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl LFXTIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTIFG_ENUM_READ_A {
        match self.bits {
            false => LFXTIFG_ENUM_READ_A::LFXTIFG_0,
            true => LFXTIFG_ENUM_READ_A::LFXTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXTIFG_0`"]
    #[inline(always)]
    pub fn is_lfxtifg_0(&self) -> bool {
        *self == LFXTIFG_ENUM_READ_A::LFXTIFG_0
    }
    #[doc = "Checks if the value of the field is `LFXTIFG_1`"]
    #[inline(always)]
    pub fn is_lfxtifg_1(&self) -> bool {
        *self == LFXTIFG_ENUM_READ_A::LFXTIFG_1
    }
}
#[doc = "Field `HFXTIFG` reader - HFXT oscillator fault flag"]
pub type HFXTIFG_R = crate::BitReader<HFXTIFG_ENUM_READ_A>;
#[doc = "HFXT oscillator fault flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFXTIFG_ENUM_READ_A {
    #[doc = "0: No fault condition occurred after the last reset"]
    HFXTIFG_0 = 0,
    #[doc = "1: HFXT fault. A HFXT fault occurred after the last reset"]
    HFXTIFG_1 = 1,
}
impl From<HFXTIFG_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: HFXTIFG_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl HFXTIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXTIFG_ENUM_READ_A {
        match self.bits {
            false => HFXTIFG_ENUM_READ_A::HFXTIFG_0,
            true => HFXTIFG_ENUM_READ_A::HFXTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXTIFG_0`"]
    #[inline(always)]
    pub fn is_hfxtifg_0(&self) -> bool {
        *self == HFXTIFG_ENUM_READ_A::HFXTIFG_0
    }
    #[doc = "Checks if the value of the field is `HFXTIFG_1`"]
    #[inline(always)]
    pub fn is_hfxtifg_1(&self) -> bool {
        *self == HFXTIFG_ENUM_READ_A::HFXTIFG_1
    }
}
#[doc = "Field `HFXT2IFG` reader - HFXT2 oscillator fault flag"]
pub type HFXT2IFG_R = crate::BitReader<HFXT2IFG_ENUM_READ_A>;
#[doc = "HFXT2 oscillator fault flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFXT2IFG_ENUM_READ_A {
    #[doc = "0: No fault condition occurred after the last reset"]
    HFXT2IFG_0 = 0,
    #[doc = "1: HFXT2 fault. A HFXT2 fault occurred after the last reset"]
    HFXT2IFG_1 = 1,
}
impl From<HFXT2IFG_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: HFXT2IFG_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl HFXT2IFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXT2IFG_ENUM_READ_A {
        match self.bits {
            false => HFXT2IFG_ENUM_READ_A::HFXT2IFG_0,
            true => HFXT2IFG_ENUM_READ_A::HFXT2IFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXT2IFG_0`"]
    #[inline(always)]
    pub fn is_hfxt2ifg_0(&self) -> bool {
        *self == HFXT2IFG_ENUM_READ_A::HFXT2IFG_0
    }
    #[doc = "Checks if the value of the field is `HFXT2IFG_1`"]
    #[inline(always)]
    pub fn is_hfxt2ifg_1(&self) -> bool {
        *self == HFXT2IFG_ENUM_READ_A::HFXT2IFG_1
    }
}
#[doc = "Field `DCOR_SHTIFG` reader - DCO external resistor short circuit fault flag."]
pub type DCOR_SHTIFG_R = crate::BitReader<DCOR_SHTIFG_ENUM_READ_A>;
#[doc = "DCO external resistor short circuit fault flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCOR_SHTIFG_ENUM_READ_A {
    #[doc = "0: DCO external resistor present"]
    DCOR_SHTIFG_0 = 0,
    #[doc = "1: DCO external resistor short circuit fault"]
    DCOR_SHTIFG_1 = 1,
}
impl From<DCOR_SHTIFG_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: DCOR_SHTIFG_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl DCOR_SHTIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOR_SHTIFG_ENUM_READ_A {
        match self.bits {
            false => DCOR_SHTIFG_ENUM_READ_A::DCOR_SHTIFG_0,
            true => DCOR_SHTIFG_ENUM_READ_A::DCOR_SHTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCOR_SHTIFG_0`"]
    #[inline(always)]
    pub fn is_dcor_shtifg_0(&self) -> bool {
        *self == DCOR_SHTIFG_ENUM_READ_A::DCOR_SHTIFG_0
    }
    #[doc = "Checks if the value of the field is `DCOR_SHTIFG_1`"]
    #[inline(always)]
    pub fn is_dcor_shtifg_1(&self) -> bool {
        *self == DCOR_SHTIFG_ENUM_READ_A::DCOR_SHTIFG_1
    }
}
#[doc = "Field `DCOR_OPNIFG` reader - DCO external resistor open circuit fault flag."]
pub type DCOR_OPNIFG_R = crate::BitReader<DCOR_OPNIFG_ENUM_READ_A>;
#[doc = "DCO external resistor open circuit fault flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCOR_OPNIFG_ENUM_READ_A {
    #[doc = "0: DCO external resistor present"]
    DCOR_OPNIFG_0 = 0,
    #[doc = "1: DCO external resistor open circuit fault"]
    DCOR_OPNIFG_1 = 1,
}
impl From<DCOR_OPNIFG_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: DCOR_OPNIFG_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl DCOR_OPNIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOR_OPNIFG_ENUM_READ_A {
        match self.bits {
            false => DCOR_OPNIFG_ENUM_READ_A::DCOR_OPNIFG_0,
            true => DCOR_OPNIFG_ENUM_READ_A::DCOR_OPNIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCOR_OPNIFG_0`"]
    #[inline(always)]
    pub fn is_dcor_opnifg_0(&self) -> bool {
        *self == DCOR_OPNIFG_ENUM_READ_A::DCOR_OPNIFG_0
    }
    #[doc = "Checks if the value of the field is `DCOR_OPNIFG_1`"]
    #[inline(always)]
    pub fn is_dcor_opnifg_1(&self) -> bool {
        *self == DCOR_OPNIFG_ENUM_READ_A::DCOR_OPNIFG_1
    }
}
#[doc = "Field `FCNTLFIFG` reader - Start fault counter interrupt flag LFXT"]
pub type FCNTLFIFG_R = crate::BitReader<FCNTLFIFG_ENUM_READ_A>;
#[doc = "Start fault counter interrupt flag LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCNTLFIFG_ENUM_READ_A {
    #[doc = "0: Start counter not expired"]
    FCNTLFIFG_0 = 0,
    #[doc = "1: Start counter expired"]
    FCNTLFIFG_1 = 1,
}
impl From<FCNTLFIFG_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTLFIFG_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl FCNTLFIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTLFIFG_ENUM_READ_A {
        match self.bits {
            false => FCNTLFIFG_ENUM_READ_A::FCNTLFIFG_0,
            true => FCNTLFIFG_ENUM_READ_A::FCNTLFIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTLFIFG_0`"]
    #[inline(always)]
    pub fn is_fcntlfifg_0(&self) -> bool {
        *self == FCNTLFIFG_ENUM_READ_A::FCNTLFIFG_0
    }
    #[doc = "Checks if the value of the field is `FCNTLFIFG_1`"]
    #[inline(always)]
    pub fn is_fcntlfifg_1(&self) -> bool {
        *self == FCNTLFIFG_ENUM_READ_A::FCNTLFIFG_1
    }
}
#[doc = "Field `FCNTHFIFG` reader - Start fault counter interrupt flag HFXT"]
pub type FCNTHFIFG_R = crate::BitReader<FCNTHFIFG_ENUM_READ_A>;
#[doc = "Start fault counter interrupt flag HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCNTHFIFG_ENUM_READ_A {
    #[doc = "0: Start counter not expired"]
    FCNTHFIFG_0 = 0,
    #[doc = "1: Start counter expired"]
    FCNTHFIFG_1 = 1,
}
impl From<FCNTHFIFG_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTHFIFG_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl FCNTHFIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHFIFG_ENUM_READ_A {
        match self.bits {
            false => FCNTHFIFG_ENUM_READ_A::FCNTHFIFG_0,
            true => FCNTHFIFG_ENUM_READ_A::FCNTHFIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHFIFG_0`"]
    #[inline(always)]
    pub fn is_fcnthfifg_0(&self) -> bool {
        *self == FCNTHFIFG_ENUM_READ_A::FCNTHFIFG_0
    }
    #[doc = "Checks if the value of the field is `FCNTHFIFG_1`"]
    #[inline(always)]
    pub fn is_fcnthfifg_1(&self) -> bool {
        *self == FCNTHFIFG_ENUM_READ_A::FCNTHFIFG_1
    }
}
#[doc = "Field `FCNTHF2IFG` reader - Start fault counter interrupt flag HFXT2"]
pub type FCNTHF2IFG_R = crate::BitReader<FCNTHF2IFG_ENUM_READ_A>;
#[doc = "Start fault counter interrupt flag HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCNTHF2IFG_ENUM_READ_A {
    #[doc = "0: Start counter not expired"]
    FCNTHF2IFG_0 = 0,
    #[doc = "1: Start counter expired"]
    FCNTHF2IFG_1 = 1,
}
impl From<FCNTHF2IFG_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTHF2IFG_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl FCNTHF2IFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHF2IFG_ENUM_READ_A {
        match self.bits {
            false => FCNTHF2IFG_ENUM_READ_A::FCNTHF2IFG_0,
            true => FCNTHF2IFG_ENUM_READ_A::FCNTHF2IFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHF2IFG_0`"]
    #[inline(always)]
    pub fn is_fcnthf2ifg_0(&self) -> bool {
        *self == FCNTHF2IFG_ENUM_READ_A::FCNTHF2IFG_0
    }
    #[doc = "Checks if the value of the field is `FCNTHF2IFG_1`"]
    #[inline(always)]
    pub fn is_fcnthf2ifg_1(&self) -> bool {
        *self == FCNTHF2IFG_ENUM_READ_A::FCNTHF2IFG_1
    }
}
#[doc = "Field `PLLOOLIFG` reader - PLL out-of-lock interrupt flag"]
pub type PLLOOLIFG_R = crate::BitReader<PLLOOLIFG_ENUM_READ_A>;
#[doc = "PLL out-of-lock interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLOOLIFG_ENUM_READ_A {
    #[doc = "0: No interrupt pending"]
    PLLOOLIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    PLLOOLIFG_1 = 1,
}
impl From<PLLOOLIFG_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: PLLOOLIFG_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLOOLIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLOOLIFG_ENUM_READ_A {
        match self.bits {
            false => PLLOOLIFG_ENUM_READ_A::PLLOOLIFG_0,
            true => PLLOOLIFG_ENUM_READ_A::PLLOOLIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLLOOLIFG_0`"]
    #[inline(always)]
    pub fn is_plloolifg_0(&self) -> bool {
        *self == PLLOOLIFG_ENUM_READ_A::PLLOOLIFG_0
    }
    #[doc = "Checks if the value of the field is `PLLOOLIFG_1`"]
    #[inline(always)]
    pub fn is_plloolifg_1(&self) -> bool {
        *self == PLLOOLIFG_ENUM_READ_A::PLLOOLIFG_1
    }
}
#[doc = "Field `PLLLOSIFG` reader - PLL loss-of-signal interrupt flag"]
pub type PLLLOSIFG_R = crate::BitReader<PLLLOSIFG_ENUM_READ_A>;
#[doc = "PLL loss-of-signal interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLLOSIFG_ENUM_READ_A {
    #[doc = "0: No interrupt pending"]
    PLLLOSIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    PLLLOSIFG_1 = 1,
}
impl From<PLLLOSIFG_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: PLLLOSIFG_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLLOSIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLLOSIFG_ENUM_READ_A {
        match self.bits {
            false => PLLLOSIFG_ENUM_READ_A::PLLLOSIFG_0,
            true => PLLLOSIFG_ENUM_READ_A::PLLLOSIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLLLOSIFG_0`"]
    #[inline(always)]
    pub fn is_plllosifg_0(&self) -> bool {
        *self == PLLLOSIFG_ENUM_READ_A::PLLLOSIFG_0
    }
    #[doc = "Checks if the value of the field is `PLLLOSIFG_1`"]
    #[inline(always)]
    pub fn is_plllosifg_1(&self) -> bool {
        *self == PLLLOSIFG_ENUM_READ_A::PLLLOSIFG_1
    }
}
#[doc = "Field `PLLOORIFG` reader - PLL out-of-range interrupt flag"]
pub type PLLOORIFG_R = crate::BitReader<PLLOORIFG_ENUM_READ_A>;
#[doc = "PLL out-of-range interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLOORIFG_ENUM_READ_A {
    #[doc = "0: No interrupt pending"]
    PLLOORIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    PLLOORIFG_1 = 1,
}
impl From<PLLOORIFG_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: PLLOORIFG_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLOORIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLOORIFG_ENUM_READ_A {
        match self.bits {
            false => PLLOORIFG_ENUM_READ_A::PLLOORIFG_0,
            true => PLLOORIFG_ENUM_READ_A::PLLOORIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLLOORIFG_0`"]
    #[inline(always)]
    pub fn is_plloorifg_0(&self) -> bool {
        *self == PLLOORIFG_ENUM_READ_A::PLLOORIFG_0
    }
    #[doc = "Checks if the value of the field is `PLLOORIFG_1`"]
    #[inline(always)]
    pub fn is_plloorifg_1(&self) -> bool {
        *self == PLLOORIFG_ENUM_READ_A::PLLOORIFG_1
    }
}
#[doc = "Field `CALIFG` reader - REFCNT period counter expired"]
pub type CALIFG_R = crate::BitReader<CALIFG_ENUM_READ_A>;
#[doc = "REFCNT period counter expired\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALIFG_ENUM_READ_A {
    #[doc = "0: REFCNT period counter not expired"]
    CALIFG_0 = 0,
    #[doc = "1: REFCNT period counter expired"]
    CALIFG_1 = 1,
}
impl From<CALIFG_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: CALIFG_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl CALIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALIFG_ENUM_READ_A {
        match self.bits {
            false => CALIFG_ENUM_READ_A::CALIFG_0,
            true => CALIFG_ENUM_READ_A::CALIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CALIFG_0`"]
    #[inline(always)]
    pub fn is_califg_0(&self) -> bool {
        *self == CALIFG_ENUM_READ_A::CALIFG_0
    }
    #[doc = "Checks if the value of the field is `CALIFG_1`"]
    #[inline(always)]
    pub fn is_califg_1(&self) -> bool {
        *self == CALIFG_ENUM_READ_A::CALIFG_1
    }
}
impl R {
    #[doc = "Bit 0 - LFXT oscillator fault flag"]
    #[inline(always)]
    pub fn lfxtifg(&self) -> LFXTIFG_R {
        LFXTIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXT oscillator fault flag"]
    #[inline(always)]
    pub fn hfxtifg(&self) -> HFXTIFG_R {
        HFXTIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HFXT2 oscillator fault flag"]
    #[inline(always)]
    pub fn hfxt2ifg(&self) -> HFXT2IFG_R {
        HFXT2IFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - DCO external resistor short circuit fault flag."]
    #[inline(always)]
    pub fn dcor_shtifg(&self) -> DCOR_SHTIFG_R {
        DCOR_SHTIFG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DCO external resistor open circuit fault flag."]
    #[inline(always)]
    pub fn dcor_opnifg(&self) -> DCOR_OPNIFG_R {
        DCOR_OPNIFG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Start fault counter interrupt flag LFXT"]
    #[inline(always)]
    pub fn fcntlfifg(&self) -> FCNTLFIFG_R {
        FCNTLFIFG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start fault counter interrupt flag HFXT"]
    #[inline(always)]
    pub fn fcnthfifg(&self) -> FCNTHFIFG_R {
        FCNTHFIFG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Start fault counter interrupt flag HFXT2"]
    #[inline(always)]
    pub fn fcnthf2ifg(&self) -> FCNTHF2IFG_R {
        FCNTHF2IFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL out-of-lock interrupt flag"]
    #[inline(always)]
    pub fn plloolifg(&self) -> PLLOOLIFG_R {
        PLLOOLIFG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PLL loss-of-signal interrupt flag"]
    #[inline(always)]
    pub fn plllosifg(&self) -> PLLLOSIFG_R {
        PLLLOSIFG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PLL out-of-range interrupt flag"]
    #[inline(always)]
    pub fn plloorifg(&self) -> PLLOORIFG_R {
        PLLOORIFG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - REFCNT period counter expired"]
    #[inline(always)]
    pub fn califg(&self) -> CALIFG_R {
        CALIFG_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csifg](index.html) module"]
pub struct CSIFG_SPEC;
impl crate::RegisterSpec for CSIFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csifg::R](R) reader structure"]
impl crate::Readable for CSIFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSIFG to value 0x01"]
impl crate::Resettable for CSIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
