#[doc = "Register `LCDCTL` reader"]
pub struct R(crate::R<LCDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCTL` writer"]
pub struct W(crate::W<LCDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LCDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDON` reader - LCD on"]
pub type LCDON_R = crate::BitReader<LCDON_A>;
#[doc = "LCD on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDON_A {
    #[doc = "0: LCD module off"]
    LCDON_0 = 0,
    #[doc = "1: LCD module on"]
    LCDON_1 = 1,
}
impl From<LCDON_A> for bool {
    #[inline(always)]
    fn from(variant: LCDON_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDON_A {
        match self.bits {
            false => LCDON_A::LCDON_0,
            true => LCDON_A::LCDON_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDON_0`"]
    #[inline(always)]
    pub fn is_lcdon_0(&self) -> bool {
        *self == LCDON_A::LCDON_0
    }
    #[doc = "Checks if the value of the field is `LCDON_1`"]
    #[inline(always)]
    pub fn is_lcdon_1(&self) -> bool {
        *self == LCDON_A::LCDON_1
    }
}
#[doc = "Field `LCDON` writer - LCD on"]
pub type LCDON_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCTL_SPEC, LCDON_A, O>;
impl<'a, const O: u8> LCDON_W<'a, O> {
    #[doc = "LCD module off"]
    #[inline(always)]
    pub fn lcdon_0(self) -> &'a mut W {
        self.variant(LCDON_A::LCDON_0)
    }
    #[doc = "LCD module on"]
    #[inline(always)]
    pub fn lcdon_1(self) -> &'a mut W {
        self.variant(LCDON_A::LCDON_1)
    }
}
#[doc = "Field `LCDLP` reader - LCD Low-power Waveform"]
pub type LCDLP_R = crate::BitReader<LCDLP_A>;
#[doc = "LCD Low-power Waveform\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDLP_A {
    #[doc = "0: Standard LCD waveforms on segment and common lines selected"]
    LCDLP_0 = 0,
    #[doc = "1: Low-power LCD waveforms on segment and common lines selected"]
    LCDLP_1 = 1,
}
impl From<LCDLP_A> for bool {
    #[inline(always)]
    fn from(variant: LCDLP_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDLP_A {
        match self.bits {
            false => LCDLP_A::LCDLP_0,
            true => LCDLP_A::LCDLP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDLP_0`"]
    #[inline(always)]
    pub fn is_lcdlp_0(&self) -> bool {
        *self == LCDLP_A::LCDLP_0
    }
    #[doc = "Checks if the value of the field is `LCDLP_1`"]
    #[inline(always)]
    pub fn is_lcdlp_1(&self) -> bool {
        *self == LCDLP_A::LCDLP_1
    }
}
#[doc = "Field `LCDLP` writer - LCD Low-power Waveform"]
pub type LCDLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCTL_SPEC, LCDLP_A, O>;
impl<'a, const O: u8> LCDLP_W<'a, O> {
    #[doc = "Standard LCD waveforms on segment and common lines selected"]
    #[inline(always)]
    pub fn lcdlp_0(self) -> &'a mut W {
        self.variant(LCDLP_A::LCDLP_0)
    }
    #[doc = "Low-power LCD waveforms on segment and common lines selected"]
    #[inline(always)]
    pub fn lcdlp_1(self) -> &'a mut W {
        self.variant(LCDLP_A::LCDLP_1)
    }
}
#[doc = "Field `LCDSON` reader - LCD segments on"]
pub type LCDSON_R = crate::BitReader<LCDSON_A>;
#[doc = "LCD segments on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDSON_A {
    #[doc = "0: All LCD segments are off"]
    LCDSON_0 = 0,
    #[doc = "1: All LCD segments are enabled and on or off according to their corresponding memory location"]
    LCDSON_1 = 1,
}
impl From<LCDSON_A> for bool {
    #[inline(always)]
    fn from(variant: LCDSON_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDSON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDSON_A {
        match self.bits {
            false => LCDSON_A::LCDSON_0,
            true => LCDSON_A::LCDSON_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDSON_0`"]
    #[inline(always)]
    pub fn is_lcdson_0(&self) -> bool {
        *self == LCDSON_A::LCDSON_0
    }
    #[doc = "Checks if the value of the field is `LCDSON_1`"]
    #[inline(always)]
    pub fn is_lcdson_1(&self) -> bool {
        *self == LCDSON_A::LCDSON_1
    }
}
#[doc = "Field `LCDSON` writer - LCD segments on"]
pub type LCDSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCTL_SPEC, LCDSON_A, O>;
impl<'a, const O: u8> LCDSON_W<'a, O> {
    #[doc = "All LCD segments are off"]
    #[inline(always)]
    pub fn lcdson_0(self) -> &'a mut W {
        self.variant(LCDSON_A::LCDSON_0)
    }
    #[doc = "All LCD segments are enabled and on or off according to their corresponding memory location"]
    #[inline(always)]
    pub fn lcdson_1(self) -> &'a mut W {
        self.variant(LCDSON_A::LCDSON_1)
    }
}
#[doc = "Field `LCDMXx` reader - LCD mux rate"]
pub type LCDMXX_R = crate::FieldReader<u8, LCDMXX_A>;
#[doc = "LCD mux rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDMXX_A {
    #[doc = "0: Static"]
    LCDMXX_0 = 0,
    #[doc = "1: 2-mux"]
    LCDMXX_1 = 1,
    #[doc = "2: 3-mux"]
    LCDMXX_2 = 2,
    #[doc = "3: 4-mux"]
    LCDMXX_3 = 3,
    #[doc = "4: 5-mux"]
    LCDMXX_4 = 4,
    #[doc = "5: 6-mux"]
    LCDMXX_5 = 5,
    #[doc = "6: 7-mux"]
    LCDMXX_6 = 6,
    #[doc = "7: 8-mux"]
    LCDMXX_7 = 7,
}
impl From<LCDMXX_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDMXX_A) -> Self {
        variant as _
    }
}
impl LCDMXX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDMXX_A {
        match self.bits {
            0 => LCDMXX_A::LCDMXX_0,
            1 => LCDMXX_A::LCDMXX_1,
            2 => LCDMXX_A::LCDMXX_2,
            3 => LCDMXX_A::LCDMXX_3,
            4 => LCDMXX_A::LCDMXX_4,
            5 => LCDMXX_A::LCDMXX_5,
            6 => LCDMXX_A::LCDMXX_6,
            7 => LCDMXX_A::LCDMXX_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDMXX_0`"]
    #[inline(always)]
    pub fn is_lcdmxx_0(&self) -> bool {
        *self == LCDMXX_A::LCDMXX_0
    }
    #[doc = "Checks if the value of the field is `LCDMXX_1`"]
    #[inline(always)]
    pub fn is_lcdmxx_1(&self) -> bool {
        *self == LCDMXX_A::LCDMXX_1
    }
    #[doc = "Checks if the value of the field is `LCDMXX_2`"]
    #[inline(always)]
    pub fn is_lcdmxx_2(&self) -> bool {
        *self == LCDMXX_A::LCDMXX_2
    }
    #[doc = "Checks if the value of the field is `LCDMXX_3`"]
    #[inline(always)]
    pub fn is_lcdmxx_3(&self) -> bool {
        *self == LCDMXX_A::LCDMXX_3
    }
    #[doc = "Checks if the value of the field is `LCDMXX_4`"]
    #[inline(always)]
    pub fn is_lcdmxx_4(&self) -> bool {
        *self == LCDMXX_A::LCDMXX_4
    }
    #[doc = "Checks if the value of the field is `LCDMXX_5`"]
    #[inline(always)]
    pub fn is_lcdmxx_5(&self) -> bool {
        *self == LCDMXX_A::LCDMXX_5
    }
    #[doc = "Checks if the value of the field is `LCDMXX_6`"]
    #[inline(always)]
    pub fn is_lcdmxx_6(&self) -> bool {
        *self == LCDMXX_A::LCDMXX_6
    }
    #[doc = "Checks if the value of the field is `LCDMXX_7`"]
    #[inline(always)]
    pub fn is_lcdmxx_7(&self) -> bool {
        *self == LCDMXX_A::LCDMXX_7
    }
}
#[doc = "Field `LCDMXx` writer - LCD mux rate"]
pub type LCDMXX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LCDCTL_SPEC, u8, LCDMXX_A, 3, O>;
impl<'a, const O: u8> LCDMXX_W<'a, O> {
    #[doc = "Static"]
    #[inline(always)]
    pub fn lcdmxx_0(self) -> &'a mut W {
        self.variant(LCDMXX_A::LCDMXX_0)
    }
    #[doc = "2-mux"]
    #[inline(always)]
    pub fn lcdmxx_1(self) -> &'a mut W {
        self.variant(LCDMXX_A::LCDMXX_1)
    }
    #[doc = "3-mux"]
    #[inline(always)]
    pub fn lcdmxx_2(self) -> &'a mut W {
        self.variant(LCDMXX_A::LCDMXX_2)
    }
    #[doc = "4-mux"]
    #[inline(always)]
    pub fn lcdmxx_3(self) -> &'a mut W {
        self.variant(LCDMXX_A::LCDMXX_3)
    }
    #[doc = "5-mux"]
    #[inline(always)]
    pub fn lcdmxx_4(self) -> &'a mut W {
        self.variant(LCDMXX_A::LCDMXX_4)
    }
    #[doc = "6-mux"]
    #[inline(always)]
    pub fn lcdmxx_5(self) -> &'a mut W {
        self.variant(LCDMXX_A::LCDMXX_5)
    }
    #[doc = "7-mux"]
    #[inline(always)]
    pub fn lcdmxx_6(self) -> &'a mut W {
        self.variant(LCDMXX_A::LCDMXX_6)
    }
    #[doc = "8-mux"]
    #[inline(always)]
    pub fn lcdmxx_7(self) -> &'a mut W {
        self.variant(LCDMXX_A::LCDMXX_7)
    }
}
#[doc = "Field `LCDPREx` reader - LCD frequency pre-scaler"]
pub type LCDPREX_R = crate::FieldReader<u8, LCDPREX_A>;
#[doc = "LCD frequency pre-scaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDPREX_A {
    #[doc = "0: Divide by 1"]
    LCDPREX_0 = 0,
    #[doc = "1: Divide by 2"]
    LCDPREX_1 = 1,
    #[doc = "2: Divide by 4"]
    LCDPREX_2 = 2,
    #[doc = "3: Divide by 8"]
    LCDPREX_3 = 3,
    #[doc = "4: Divide by 16"]
    LCDPREX_4 = 4,
    #[doc = "5: Divide by 32"]
    LCDPREX_5 = 5,
}
impl From<LCDPREX_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDPREX_A) -> Self {
        variant as _
    }
}
impl LCDPREX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LCDPREX_A> {
        match self.bits {
            0 => Some(LCDPREX_A::LCDPREX_0),
            1 => Some(LCDPREX_A::LCDPREX_1),
            2 => Some(LCDPREX_A::LCDPREX_2),
            3 => Some(LCDPREX_A::LCDPREX_3),
            4 => Some(LCDPREX_A::LCDPREX_4),
            5 => Some(LCDPREX_A::LCDPREX_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LCDPREX_0`"]
    #[inline(always)]
    pub fn is_lcdprex_0(&self) -> bool {
        *self == LCDPREX_A::LCDPREX_0
    }
    #[doc = "Checks if the value of the field is `LCDPREX_1`"]
    #[inline(always)]
    pub fn is_lcdprex_1(&self) -> bool {
        *self == LCDPREX_A::LCDPREX_1
    }
    #[doc = "Checks if the value of the field is `LCDPREX_2`"]
    #[inline(always)]
    pub fn is_lcdprex_2(&self) -> bool {
        *self == LCDPREX_A::LCDPREX_2
    }
    #[doc = "Checks if the value of the field is `LCDPREX_3`"]
    #[inline(always)]
    pub fn is_lcdprex_3(&self) -> bool {
        *self == LCDPREX_A::LCDPREX_3
    }
    #[doc = "Checks if the value of the field is `LCDPREX_4`"]
    #[inline(always)]
    pub fn is_lcdprex_4(&self) -> bool {
        *self == LCDPREX_A::LCDPREX_4
    }
    #[doc = "Checks if the value of the field is `LCDPREX_5`"]
    #[inline(always)]
    pub fn is_lcdprex_5(&self) -> bool {
        *self == LCDPREX_A::LCDPREX_5
    }
}
#[doc = "Field `LCDPREx` writer - LCD frequency pre-scaler"]
pub type LCDPREX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCDCTL_SPEC, u8, LCDPREX_A, 3, O>;
impl<'a, const O: u8> LCDPREX_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn lcdprex_0(self) -> &'a mut W {
        self.variant(LCDPREX_A::LCDPREX_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn lcdprex_1(self) -> &'a mut W {
        self.variant(LCDPREX_A::LCDPREX_1)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn lcdprex_2(self) -> &'a mut W {
        self.variant(LCDPREX_A::LCDPREX_2)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn lcdprex_3(self) -> &'a mut W {
        self.variant(LCDPREX_A::LCDPREX_3)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn lcdprex_4(self) -> &'a mut W {
        self.variant(LCDPREX_A::LCDPREX_4)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn lcdprex_5(self) -> &'a mut W {
        self.variant(LCDPREX_A::LCDPREX_5)
    }
}
#[doc = "Field `LCDDIVx` reader - LCD frequency divider"]
pub type LCDDIVX_R = crate::FieldReader<u8, LCDDIVX_A>;
#[doc = "LCD frequency divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDDIVX_A {
    #[doc = "0: Divide by 1"]
    LCDDIVX_0 = 0,
    #[doc = "1: Divide by 2"]
    LCDDIVX_1 = 1,
    #[doc = "2: Divide by 3"]
    LCDDIVX_2 = 2,
    #[doc = "3: Divide by 4"]
    LCDDIVX_3 = 3,
    #[doc = "4: Divide by 5"]
    LCDDIVX_4 = 4,
    #[doc = "5: Divide by 6"]
    LCDDIVX_5 = 5,
    #[doc = "6: Divide by 7"]
    LCDDIVX_6 = 6,
    #[doc = "7: Divide by 8"]
    LCDDIVX_7 = 7,
    #[doc = "8: Divide by 9"]
    LCDDIVX_8 = 8,
    #[doc = "9: Divide by 10"]
    LCDDIVX_9 = 9,
    #[doc = "10: Divide by 11"]
    LCDDIVX_10 = 10,
    #[doc = "11: Divide by 12"]
    LCDDIVX_11 = 11,
    #[doc = "12: Divide by 13"]
    LCDDIVX_12 = 12,
    #[doc = "13: Divide by 14"]
    LCDDIVX_13 = 13,
    #[doc = "14: Divide by 15"]
    LCDDIVX_14 = 14,
    #[doc = "15: Divide by 16"]
    LCDDIVX_15 = 15,
    #[doc = "16: Divide by 17"]
    LCDDIVX_16 = 16,
    #[doc = "17: Divide by 18"]
    LCDDIVX_17 = 17,
    #[doc = "18: Divide by 19"]
    LCDDIVX_18 = 18,
    #[doc = "19: Divide by 20"]
    LCDDIVX_19 = 19,
    #[doc = "20: Divide by 21"]
    LCDDIVX_20 = 20,
    #[doc = "21: Divide by 22"]
    LCDDIVX_21 = 21,
    #[doc = "22: Divide by 23"]
    LCDDIVX_22 = 22,
    #[doc = "23: Divide by 24"]
    LCDDIVX_23 = 23,
    #[doc = "24: Divide by 25"]
    LCDDIVX_24 = 24,
    #[doc = "25: Divide by 26"]
    LCDDIVX_25 = 25,
    #[doc = "26: Divide by 27"]
    LCDDIVX_26 = 26,
    #[doc = "27: Divide by 28"]
    LCDDIVX_27 = 27,
    #[doc = "28: Divide by 29"]
    LCDDIVX_28 = 28,
    #[doc = "29: Divide by 30"]
    LCDDIVX_29 = 29,
    #[doc = "30: Divide by 31"]
    LCDDIVX_30 = 30,
    #[doc = "31: Divide by 32"]
    LCDDIVX_31 = 31,
}
impl From<LCDDIVX_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDDIVX_A) -> Self {
        variant as _
    }
}
impl LCDDIVX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDDIVX_A {
        match self.bits {
            0 => LCDDIVX_A::LCDDIVX_0,
            1 => LCDDIVX_A::LCDDIVX_1,
            2 => LCDDIVX_A::LCDDIVX_2,
            3 => LCDDIVX_A::LCDDIVX_3,
            4 => LCDDIVX_A::LCDDIVX_4,
            5 => LCDDIVX_A::LCDDIVX_5,
            6 => LCDDIVX_A::LCDDIVX_6,
            7 => LCDDIVX_A::LCDDIVX_7,
            8 => LCDDIVX_A::LCDDIVX_8,
            9 => LCDDIVX_A::LCDDIVX_9,
            10 => LCDDIVX_A::LCDDIVX_10,
            11 => LCDDIVX_A::LCDDIVX_11,
            12 => LCDDIVX_A::LCDDIVX_12,
            13 => LCDDIVX_A::LCDDIVX_13,
            14 => LCDDIVX_A::LCDDIVX_14,
            15 => LCDDIVX_A::LCDDIVX_15,
            16 => LCDDIVX_A::LCDDIVX_16,
            17 => LCDDIVX_A::LCDDIVX_17,
            18 => LCDDIVX_A::LCDDIVX_18,
            19 => LCDDIVX_A::LCDDIVX_19,
            20 => LCDDIVX_A::LCDDIVX_20,
            21 => LCDDIVX_A::LCDDIVX_21,
            22 => LCDDIVX_A::LCDDIVX_22,
            23 => LCDDIVX_A::LCDDIVX_23,
            24 => LCDDIVX_A::LCDDIVX_24,
            25 => LCDDIVX_A::LCDDIVX_25,
            26 => LCDDIVX_A::LCDDIVX_26,
            27 => LCDDIVX_A::LCDDIVX_27,
            28 => LCDDIVX_A::LCDDIVX_28,
            29 => LCDDIVX_A::LCDDIVX_29,
            30 => LCDDIVX_A::LCDDIVX_30,
            31 => LCDDIVX_A::LCDDIVX_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_0`"]
    #[inline(always)]
    pub fn is_lcddivx_0(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_0
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_1`"]
    #[inline(always)]
    pub fn is_lcddivx_1(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_1
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_2`"]
    #[inline(always)]
    pub fn is_lcddivx_2(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_2
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_3`"]
    #[inline(always)]
    pub fn is_lcddivx_3(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_3
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_4`"]
    #[inline(always)]
    pub fn is_lcddivx_4(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_4
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_5`"]
    #[inline(always)]
    pub fn is_lcddivx_5(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_5
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_6`"]
    #[inline(always)]
    pub fn is_lcddivx_6(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_6
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_7`"]
    #[inline(always)]
    pub fn is_lcddivx_7(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_7
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_8`"]
    #[inline(always)]
    pub fn is_lcddivx_8(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_8
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_9`"]
    #[inline(always)]
    pub fn is_lcddivx_9(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_9
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_10`"]
    #[inline(always)]
    pub fn is_lcddivx_10(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_10
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_11`"]
    #[inline(always)]
    pub fn is_lcddivx_11(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_11
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_12`"]
    #[inline(always)]
    pub fn is_lcddivx_12(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_12
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_13`"]
    #[inline(always)]
    pub fn is_lcddivx_13(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_13
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_14`"]
    #[inline(always)]
    pub fn is_lcddivx_14(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_14
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_15`"]
    #[inline(always)]
    pub fn is_lcddivx_15(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_15
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_16`"]
    #[inline(always)]
    pub fn is_lcddivx_16(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_16
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_17`"]
    #[inline(always)]
    pub fn is_lcddivx_17(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_17
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_18`"]
    #[inline(always)]
    pub fn is_lcddivx_18(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_18
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_19`"]
    #[inline(always)]
    pub fn is_lcddivx_19(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_19
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_20`"]
    #[inline(always)]
    pub fn is_lcddivx_20(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_20
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_21`"]
    #[inline(always)]
    pub fn is_lcddivx_21(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_21
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_22`"]
    #[inline(always)]
    pub fn is_lcddivx_22(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_22
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_23`"]
    #[inline(always)]
    pub fn is_lcddivx_23(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_23
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_24`"]
    #[inline(always)]
    pub fn is_lcddivx_24(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_24
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_25`"]
    #[inline(always)]
    pub fn is_lcddivx_25(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_25
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_26`"]
    #[inline(always)]
    pub fn is_lcddivx_26(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_26
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_27`"]
    #[inline(always)]
    pub fn is_lcddivx_27(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_27
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_28`"]
    #[inline(always)]
    pub fn is_lcddivx_28(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_28
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_29`"]
    #[inline(always)]
    pub fn is_lcddivx_29(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_29
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_30`"]
    #[inline(always)]
    pub fn is_lcddivx_30(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_30
    }
    #[doc = "Checks if the value of the field is `LCDDIVX_31`"]
    #[inline(always)]
    pub fn is_lcddivx_31(&self) -> bool {
        *self == LCDDIVX_A::LCDDIVX_31
    }
}
#[doc = "Field `LCDDIVx` writer - LCD frequency divider"]
pub type LCDDIVX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LCDCTL_SPEC, u8, LCDDIVX_A, 5, O>;
impl<'a, const O: u8> LCDDIVX_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn lcddivx_0(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn lcddivx_1(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_1)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn lcddivx_2(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn lcddivx_3(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_3)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn lcddivx_4(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_4)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn lcddivx_5(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_5)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn lcddivx_6(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_6)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn lcddivx_7(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_7)
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn lcddivx_8(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_8)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn lcddivx_9(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_9)
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn lcddivx_10(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_10)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn lcddivx_11(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_11)
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn lcddivx_12(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_12)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn lcddivx_13(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_13)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn lcddivx_14(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_14)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn lcddivx_15(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_15)
    }
    #[doc = "Divide by 17"]
    #[inline(always)]
    pub fn lcddivx_16(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_16)
    }
    #[doc = "Divide by 18"]
    #[inline(always)]
    pub fn lcddivx_17(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_17)
    }
    #[doc = "Divide by 19"]
    #[inline(always)]
    pub fn lcddivx_18(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_18)
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn lcddivx_19(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_19)
    }
    #[doc = "Divide by 21"]
    #[inline(always)]
    pub fn lcddivx_20(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_20)
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn lcddivx_21(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_21)
    }
    #[doc = "Divide by 23"]
    #[inline(always)]
    pub fn lcddivx_22(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_22)
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn lcddivx_23(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_23)
    }
    #[doc = "Divide by 25"]
    #[inline(always)]
    pub fn lcddivx_24(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_24)
    }
    #[doc = "Divide by 26"]
    #[inline(always)]
    pub fn lcddivx_25(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_25)
    }
    #[doc = "Divide by 27"]
    #[inline(always)]
    pub fn lcddivx_26(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_26)
    }
    #[doc = "Divide by 28"]
    #[inline(always)]
    pub fn lcddivx_27(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_27)
    }
    #[doc = "Divide by 29"]
    #[inline(always)]
    pub fn lcddivx_28(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_28)
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn lcddivx_29(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_29)
    }
    #[doc = "Divide by 31"]
    #[inline(always)]
    pub fn lcddivx_30(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_30)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn lcddivx_31(self) -> &'a mut W {
        self.variant(LCDDIVX_A::LCDDIVX_31)
    }
}
#[doc = "Field `LCDSSEL` reader - Clock source select"]
pub type LCDSSEL_R = crate::FieldReader<u8, LCDSSEL_A>;
#[doc = "Clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDSSEL_A {
    #[doc = "0: ACLK"]
    LCDSSEL_0 = 0,
    #[doc = "1: VLOCLK"]
    LCDSSEL_1 = 1,
    #[doc = "2: REFOCLK"]
    LCDSSEL_2 = 2,
    #[doc = "3: LFXTCLK"]
    LCDSSEL_3 = 3,
}
impl From<LCDSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDSSEL_A) -> Self {
        variant as _
    }
}
impl LCDSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDSSEL_A {
        match self.bits {
            0 => LCDSSEL_A::LCDSSEL_0,
            1 => LCDSSEL_A::LCDSSEL_1,
            2 => LCDSSEL_A::LCDSSEL_2,
            3 => LCDSSEL_A::LCDSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDSSEL_0`"]
    #[inline(always)]
    pub fn is_lcdssel_0(&self) -> bool {
        *self == LCDSSEL_A::LCDSSEL_0
    }
    #[doc = "Checks if the value of the field is `LCDSSEL_1`"]
    #[inline(always)]
    pub fn is_lcdssel_1(&self) -> bool {
        *self == LCDSSEL_A::LCDSSEL_1
    }
    #[doc = "Checks if the value of the field is `LCDSSEL_2`"]
    #[inline(always)]
    pub fn is_lcdssel_2(&self) -> bool {
        *self == LCDSSEL_A::LCDSSEL_2
    }
    #[doc = "Checks if the value of the field is `LCDSSEL_3`"]
    #[inline(always)]
    pub fn is_lcdssel_3(&self) -> bool {
        *self == LCDSSEL_A::LCDSSEL_3
    }
}
#[doc = "Field `LCDSSEL` writer - Clock source select"]
pub type LCDSSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LCDCTL_SPEC, u8, LCDSSEL_A, 2, O>;
impl<'a, const O: u8> LCDSSEL_W<'a, O> {
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn lcdssel_0(self) -> &'a mut W {
        self.variant(LCDSSEL_A::LCDSSEL_0)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn lcdssel_1(self) -> &'a mut W {
        self.variant(LCDSSEL_A::LCDSSEL_1)
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn lcdssel_2(self) -> &'a mut W {
        self.variant(LCDSSEL_A::LCDSSEL_2)
    }
    #[doc = "LFXTCLK"]
    #[inline(always)]
    pub fn lcdssel_3(self) -> &'a mut W {
        self.variant(LCDSSEL_A::LCDSSEL_3)
    }
}
impl R {
    #[doc = "Bit 0 - LCD on"]
    #[inline(always)]
    pub fn lcdon(&self) -> LCDON_R {
        LCDON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD Low-power Waveform"]
    #[inline(always)]
    pub fn lcdlp(&self) -> LCDLP_R {
        LCDLP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD segments on"]
    #[inline(always)]
    pub fn lcdson(&self) -> LCDSON_R {
        LCDSON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - LCD mux rate"]
    #[inline(always)]
    pub fn lcdmxx(&self) -> LCDMXX_R {
        LCDMXX_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 8:10 - LCD frequency pre-scaler"]
    #[inline(always)]
    pub fn lcdprex(&self) -> LCDPREX_R {
        LCDPREX_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - LCD frequency divider"]
    #[inline(always)]
    pub fn lcddivx(&self) -> LCDDIVX_R {
        LCDDIVX_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Clock source select"]
    #[inline(always)]
    pub fn lcdssel(&self) -> LCDSSEL_R {
        LCDSSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LCD on"]
    #[inline(always)]
    pub fn lcdon(&mut self) -> LCDON_W<0> {
        LCDON_W::new(self)
    }
    #[doc = "Bit 1 - LCD Low-power Waveform"]
    #[inline(always)]
    pub fn lcdlp(&mut self) -> LCDLP_W<1> {
        LCDLP_W::new(self)
    }
    #[doc = "Bit 2 - LCD segments on"]
    #[inline(always)]
    pub fn lcdson(&mut self) -> LCDSON_W<2> {
        LCDSON_W::new(self)
    }
    #[doc = "Bits 3:5 - LCD mux rate"]
    #[inline(always)]
    pub fn lcdmxx(&mut self) -> LCDMXX_W<3> {
        LCDMXX_W::new(self)
    }
    #[doc = "Bits 8:10 - LCD frequency pre-scaler"]
    #[inline(always)]
    pub fn lcdprex(&mut self) -> LCDPREX_W<8> {
        LCDPREX_W::new(self)
    }
    #[doc = "Bits 11:15 - LCD frequency divider"]
    #[inline(always)]
    pub fn lcddivx(&mut self) -> LCDDIVX_W<11> {
        LCDDIVX_W::new(self)
    }
    #[doc = "Bits 16:17 - Clock source select"]
    #[inline(always)]
    pub fn lcdssel(&mut self) -> LCDSSEL_W<16> {
        LCDSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_F control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdctl](index.html) module"]
pub struct LCDCTL_SPEC;
impl crate::RegisterSpec for LCDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdctl::R](R) reader structure"]
impl crate::Readable for LCDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdctl::W](W) writer structure"]
impl crate::Writable for LCDCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCTL to value 0"]
impl crate::Resettable for LCDCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
