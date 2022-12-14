#[doc = "Register `LCDANMCTL` reader"]
pub struct R(crate::R<LCDANMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDANMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDANMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDANMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDANMCTL` writer"]
pub struct W(crate::W<LCDANMCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDANMCTL_SPEC>;
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
impl From<crate::W<LCDANMCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDANMCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDANMEN` reader - Enable Animation"]
pub type LCDANMEN_R = crate::BitReader<LCDANMEN_A>;
#[doc = "Enable Animation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDANMEN_A {
    #[doc = "0: Animation disabled"]
    LCDANMEN_0 = 0,
    #[doc = "1: Animation enabled"]
    LCDANMEN_1 = 1,
}
impl From<LCDANMEN_A> for bool {
    #[inline(always)]
    fn from(variant: LCDANMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDANMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDANMEN_A {
        match self.bits {
            false => LCDANMEN_A::LCDANMEN_0,
            true => LCDANMEN_A::LCDANMEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDANMEN_0`"]
    #[inline(always)]
    pub fn is_lcdanmen_0(&self) -> bool {
        *self == LCDANMEN_A::LCDANMEN_0
    }
    #[doc = "Checks if the value of the field is `LCDANMEN_1`"]
    #[inline(always)]
    pub fn is_lcdanmen_1(&self) -> bool {
        *self == LCDANMEN_A::LCDANMEN_1
    }
}
#[doc = "Field `LCDANMEN` writer - Enable Animation"]
pub type LCDANMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDANMCTL_SPEC, LCDANMEN_A, O>;
impl<'a, const O: u8> LCDANMEN_W<'a, O> {
    #[doc = "Animation disabled"]
    #[inline(always)]
    pub fn lcdanmen_0(self) -> &'a mut W {
        self.variant(LCDANMEN_A::LCDANMEN_0)
    }
    #[doc = "Animation enabled"]
    #[inline(always)]
    pub fn lcdanmen_1(self) -> &'a mut W {
        self.variant(LCDANMEN_A::LCDANMEN_1)
    }
}
#[doc = "Field `LCDANMSTP` reader - Number of Amimation frames"]
pub type LCDANMSTP_R = crate::FieldReader<u8, LCDANMSTP_A>;
#[doc = "Number of Amimation frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDANMSTP_A {
    #[doc = "0: T0"]
    LCDANMSTP_0 = 0,
    #[doc = "1: T0 to T1"]
    LCDANMSTP_1 = 1,
    #[doc = "2: T0 to T2"]
    LCDANMSTP_2 = 2,
    #[doc = "3: T0 to T3"]
    LCDANMSTP_3 = 3,
    #[doc = "4: T0 to T4"]
    LCDANMSTP_4 = 4,
    #[doc = "5: T0 to T5"]
    LCDANMSTP_5 = 5,
    #[doc = "6: T0 to T6"]
    LCDANMSTP_6 = 6,
    #[doc = "7: T0 to T7"]
    LCDANMSTP_7 = 7,
}
impl From<LCDANMSTP_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDANMSTP_A) -> Self {
        variant as _
    }
}
impl LCDANMSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDANMSTP_A {
        match self.bits {
            0 => LCDANMSTP_A::LCDANMSTP_0,
            1 => LCDANMSTP_A::LCDANMSTP_1,
            2 => LCDANMSTP_A::LCDANMSTP_2,
            3 => LCDANMSTP_A::LCDANMSTP_3,
            4 => LCDANMSTP_A::LCDANMSTP_4,
            5 => LCDANMSTP_A::LCDANMSTP_5,
            6 => LCDANMSTP_A::LCDANMSTP_6,
            7 => LCDANMSTP_A::LCDANMSTP_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDANMSTP_0`"]
    #[inline(always)]
    pub fn is_lcdanmstp_0(&self) -> bool {
        *self == LCDANMSTP_A::LCDANMSTP_0
    }
    #[doc = "Checks if the value of the field is `LCDANMSTP_1`"]
    #[inline(always)]
    pub fn is_lcdanmstp_1(&self) -> bool {
        *self == LCDANMSTP_A::LCDANMSTP_1
    }
    #[doc = "Checks if the value of the field is `LCDANMSTP_2`"]
    #[inline(always)]
    pub fn is_lcdanmstp_2(&self) -> bool {
        *self == LCDANMSTP_A::LCDANMSTP_2
    }
    #[doc = "Checks if the value of the field is `LCDANMSTP_3`"]
    #[inline(always)]
    pub fn is_lcdanmstp_3(&self) -> bool {
        *self == LCDANMSTP_A::LCDANMSTP_3
    }
    #[doc = "Checks if the value of the field is `LCDANMSTP_4`"]
    #[inline(always)]
    pub fn is_lcdanmstp_4(&self) -> bool {
        *self == LCDANMSTP_A::LCDANMSTP_4
    }
    #[doc = "Checks if the value of the field is `LCDANMSTP_5`"]
    #[inline(always)]
    pub fn is_lcdanmstp_5(&self) -> bool {
        *self == LCDANMSTP_A::LCDANMSTP_5
    }
    #[doc = "Checks if the value of the field is `LCDANMSTP_6`"]
    #[inline(always)]
    pub fn is_lcdanmstp_6(&self) -> bool {
        *self == LCDANMSTP_A::LCDANMSTP_6
    }
    #[doc = "Checks if the value of the field is `LCDANMSTP_7`"]
    #[inline(always)]
    pub fn is_lcdanmstp_7(&self) -> bool {
        *self == LCDANMSTP_A::LCDANMSTP_7
    }
}
#[doc = "Field `LCDANMSTP` writer - Number of Amimation frames"]
pub type LCDANMSTP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LCDANMCTL_SPEC, u8, LCDANMSTP_A, 3, O>;
impl<'a, const O: u8> LCDANMSTP_W<'a, O> {
    #[doc = "T0"]
    #[inline(always)]
    pub fn lcdanmstp_0(self) -> &'a mut W {
        self.variant(LCDANMSTP_A::LCDANMSTP_0)
    }
    #[doc = "T0 to T1"]
    #[inline(always)]
    pub fn lcdanmstp_1(self) -> &'a mut W {
        self.variant(LCDANMSTP_A::LCDANMSTP_1)
    }
    #[doc = "T0 to T2"]
    #[inline(always)]
    pub fn lcdanmstp_2(self) -> &'a mut W {
        self.variant(LCDANMSTP_A::LCDANMSTP_2)
    }
    #[doc = "T0 to T3"]
    #[inline(always)]
    pub fn lcdanmstp_3(self) -> &'a mut W {
        self.variant(LCDANMSTP_A::LCDANMSTP_3)
    }
    #[doc = "T0 to T4"]
    #[inline(always)]
    pub fn lcdanmstp_4(self) -> &'a mut W {
        self.variant(LCDANMSTP_A::LCDANMSTP_4)
    }
    #[doc = "T0 to T5"]
    #[inline(always)]
    pub fn lcdanmstp_5(self) -> &'a mut W {
        self.variant(LCDANMSTP_A::LCDANMSTP_5)
    }
    #[doc = "T0 to T6"]
    #[inline(always)]
    pub fn lcdanmstp_6(self) -> &'a mut W {
        self.variant(LCDANMSTP_A::LCDANMSTP_6)
    }
    #[doc = "T0 to T7"]
    #[inline(always)]
    pub fn lcdanmstp_7(self) -> &'a mut W {
        self.variant(LCDANMSTP_A::LCDANMSTP_7)
    }
}
#[doc = "Field `LCDANMCLR` reader - Clear Animation Memory"]
pub type LCDANMCLR_R = crate::BitReader<LCDANMCLR_A>;
#[doc = "Clear Animation Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDANMCLR_A {
    #[doc = "0: Contents of animation memory registers LCDANMx remain unchanged"]
    LCDANMCLR_0 = 0,
    #[doc = "1: Clear content of all animation memory registers LCDANMx"]
    LCDANMCLR_1 = 1,
}
impl From<LCDANMCLR_A> for bool {
    #[inline(always)]
    fn from(variant: LCDANMCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDANMCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDANMCLR_A {
        match self.bits {
            false => LCDANMCLR_A::LCDANMCLR_0,
            true => LCDANMCLR_A::LCDANMCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDANMCLR_0`"]
    #[inline(always)]
    pub fn is_lcdanmclr_0(&self) -> bool {
        *self == LCDANMCLR_A::LCDANMCLR_0
    }
    #[doc = "Checks if the value of the field is `LCDANMCLR_1`"]
    #[inline(always)]
    pub fn is_lcdanmclr_1(&self) -> bool {
        *self == LCDANMCLR_A::LCDANMCLR_1
    }
}
#[doc = "Field `LCDANMCLR` writer - Clear Animation Memory"]
pub type LCDANMCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDANMCTL_SPEC, LCDANMCLR_A, O>;
impl<'a, const O: u8> LCDANMCLR_W<'a, O> {
    #[doc = "Contents of animation memory registers LCDANMx remain unchanged"]
    #[inline(always)]
    pub fn lcdanmclr_0(self) -> &'a mut W {
        self.variant(LCDANMCLR_A::LCDANMCLR_0)
    }
    #[doc = "Clear content of all animation memory registers LCDANMx"]
    #[inline(always)]
    pub fn lcdanmclr_1(self) -> &'a mut W {
        self.variant(LCDANMCLR_A::LCDANMCLR_1)
    }
}
#[doc = "Field `LCDANMPREx` reader - Clock pre-scaler for animation frequency"]
pub type LCDANMPREX_R = crate::FieldReader<u8, LCDANMPREX_A>;
#[doc = "Clock pre-scaler for animation frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDANMPREX_A {
    #[doc = "0: Divide by 512"]
    LCDANMPREX_0 = 0,
    #[doc = "1: Divide by 1024"]
    LCDANMPREX_1 = 1,
    #[doc = "2: Divide by 2048"]
    LCDANMPREX_2 = 2,
    #[doc = "3: Divide by 4096"]
    LCDANMPREX_3 = 3,
    #[doc = "4: Divide by 8162"]
    LCDANMPREX_4 = 4,
    #[doc = "5: Divide by 16384"]
    LCDANMPREX_5 = 5,
    #[doc = "6: Divide by 32768"]
    LCDANMPREX_6 = 6,
    #[doc = "7: Divide by 65536"]
    LCDANMPREX_7 = 7,
}
impl From<LCDANMPREX_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDANMPREX_A) -> Self {
        variant as _
    }
}
impl LCDANMPREX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDANMPREX_A {
        match self.bits {
            0 => LCDANMPREX_A::LCDANMPREX_0,
            1 => LCDANMPREX_A::LCDANMPREX_1,
            2 => LCDANMPREX_A::LCDANMPREX_2,
            3 => LCDANMPREX_A::LCDANMPREX_3,
            4 => LCDANMPREX_A::LCDANMPREX_4,
            5 => LCDANMPREX_A::LCDANMPREX_5,
            6 => LCDANMPREX_A::LCDANMPREX_6,
            7 => LCDANMPREX_A::LCDANMPREX_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDANMPREX_0`"]
    #[inline(always)]
    pub fn is_lcdanmprex_0(&self) -> bool {
        *self == LCDANMPREX_A::LCDANMPREX_0
    }
    #[doc = "Checks if the value of the field is `LCDANMPREX_1`"]
    #[inline(always)]
    pub fn is_lcdanmprex_1(&self) -> bool {
        *self == LCDANMPREX_A::LCDANMPREX_1
    }
    #[doc = "Checks if the value of the field is `LCDANMPREX_2`"]
    #[inline(always)]
    pub fn is_lcdanmprex_2(&self) -> bool {
        *self == LCDANMPREX_A::LCDANMPREX_2
    }
    #[doc = "Checks if the value of the field is `LCDANMPREX_3`"]
    #[inline(always)]
    pub fn is_lcdanmprex_3(&self) -> bool {
        *self == LCDANMPREX_A::LCDANMPREX_3
    }
    #[doc = "Checks if the value of the field is `LCDANMPREX_4`"]
    #[inline(always)]
    pub fn is_lcdanmprex_4(&self) -> bool {
        *self == LCDANMPREX_A::LCDANMPREX_4
    }
    #[doc = "Checks if the value of the field is `LCDANMPREX_5`"]
    #[inline(always)]
    pub fn is_lcdanmprex_5(&self) -> bool {
        *self == LCDANMPREX_A::LCDANMPREX_5
    }
    #[doc = "Checks if the value of the field is `LCDANMPREX_6`"]
    #[inline(always)]
    pub fn is_lcdanmprex_6(&self) -> bool {
        *self == LCDANMPREX_A::LCDANMPREX_6
    }
    #[doc = "Checks if the value of the field is `LCDANMPREX_7`"]
    #[inline(always)]
    pub fn is_lcdanmprex_7(&self) -> bool {
        *self == LCDANMPREX_A::LCDANMPREX_7
    }
}
#[doc = "Field `LCDANMPREx` writer - Clock pre-scaler for animation frequency"]
pub type LCDANMPREX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LCDANMCTL_SPEC, u8, LCDANMPREX_A, 3, O>;
impl<'a, const O: u8> LCDANMPREX_W<'a, O> {
    #[doc = "Divide by 512"]
    #[inline(always)]
    pub fn lcdanmprex_0(self) -> &'a mut W {
        self.variant(LCDANMPREX_A::LCDANMPREX_0)
    }
    #[doc = "Divide by 1024"]
    #[inline(always)]
    pub fn lcdanmprex_1(self) -> &'a mut W {
        self.variant(LCDANMPREX_A::LCDANMPREX_1)
    }
    #[doc = "Divide by 2048"]
    #[inline(always)]
    pub fn lcdanmprex_2(self) -> &'a mut W {
        self.variant(LCDANMPREX_A::LCDANMPREX_2)
    }
    #[doc = "Divide by 4096"]
    #[inline(always)]
    pub fn lcdanmprex_3(self) -> &'a mut W {
        self.variant(LCDANMPREX_A::LCDANMPREX_3)
    }
    #[doc = "Divide by 8162"]
    #[inline(always)]
    pub fn lcdanmprex_4(self) -> &'a mut W {
        self.variant(LCDANMPREX_A::LCDANMPREX_4)
    }
    #[doc = "Divide by 16384"]
    #[inline(always)]
    pub fn lcdanmprex_5(self) -> &'a mut W {
        self.variant(LCDANMPREX_A::LCDANMPREX_5)
    }
    #[doc = "Divide by 32768"]
    #[inline(always)]
    pub fn lcdanmprex_6(self) -> &'a mut W {
        self.variant(LCDANMPREX_A::LCDANMPREX_6)
    }
    #[doc = "Divide by 65536"]
    #[inline(always)]
    pub fn lcdanmprex_7(self) -> &'a mut W {
        self.variant(LCDANMPREX_A::LCDANMPREX_7)
    }
}
#[doc = "Field `LCDANMDIVx` reader - Clock divider for animation frequency"]
pub type LCDANMDIVX_R = crate::FieldReader<u8, LCDANMDIVX_A>;
#[doc = "Clock divider for animation frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDANMDIVX_A {
    #[doc = "0: Divide by 1"]
    LCDANMDIVX_0 = 0,
    #[doc = "1: Divide by 2"]
    LCDANMDIVX_1 = 1,
    #[doc = "2: Divide by 3"]
    LCDANMDIVX_2 = 2,
    #[doc = "3: Divide by 4"]
    LCDANMDIVX_3 = 3,
    #[doc = "4: Divide by 5"]
    LCDANMDIVX_4 = 4,
    #[doc = "5: Divide by 6"]
    LCDANMDIVX_5 = 5,
    #[doc = "6: Divide by 7"]
    LCDANMDIVX_6 = 6,
    #[doc = "7: Divide by 8"]
    LCDANMDIVX_7 = 7,
}
impl From<LCDANMDIVX_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDANMDIVX_A) -> Self {
        variant as _
    }
}
impl LCDANMDIVX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDANMDIVX_A {
        match self.bits {
            0 => LCDANMDIVX_A::LCDANMDIVX_0,
            1 => LCDANMDIVX_A::LCDANMDIVX_1,
            2 => LCDANMDIVX_A::LCDANMDIVX_2,
            3 => LCDANMDIVX_A::LCDANMDIVX_3,
            4 => LCDANMDIVX_A::LCDANMDIVX_4,
            5 => LCDANMDIVX_A::LCDANMDIVX_5,
            6 => LCDANMDIVX_A::LCDANMDIVX_6,
            7 => LCDANMDIVX_A::LCDANMDIVX_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDANMDIVX_0`"]
    #[inline(always)]
    pub fn is_lcdanmdivx_0(&self) -> bool {
        *self == LCDANMDIVX_A::LCDANMDIVX_0
    }
    #[doc = "Checks if the value of the field is `LCDANMDIVX_1`"]
    #[inline(always)]
    pub fn is_lcdanmdivx_1(&self) -> bool {
        *self == LCDANMDIVX_A::LCDANMDIVX_1
    }
    #[doc = "Checks if the value of the field is `LCDANMDIVX_2`"]
    #[inline(always)]
    pub fn is_lcdanmdivx_2(&self) -> bool {
        *self == LCDANMDIVX_A::LCDANMDIVX_2
    }
    #[doc = "Checks if the value of the field is `LCDANMDIVX_3`"]
    #[inline(always)]
    pub fn is_lcdanmdivx_3(&self) -> bool {
        *self == LCDANMDIVX_A::LCDANMDIVX_3
    }
    #[doc = "Checks if the value of the field is `LCDANMDIVX_4`"]
    #[inline(always)]
    pub fn is_lcdanmdivx_4(&self) -> bool {
        *self == LCDANMDIVX_A::LCDANMDIVX_4
    }
    #[doc = "Checks if the value of the field is `LCDANMDIVX_5`"]
    #[inline(always)]
    pub fn is_lcdanmdivx_5(&self) -> bool {
        *self == LCDANMDIVX_A::LCDANMDIVX_5
    }
    #[doc = "Checks if the value of the field is `LCDANMDIVX_6`"]
    #[inline(always)]
    pub fn is_lcdanmdivx_6(&self) -> bool {
        *self == LCDANMDIVX_A::LCDANMDIVX_6
    }
    #[doc = "Checks if the value of the field is `LCDANMDIVX_7`"]
    #[inline(always)]
    pub fn is_lcdanmdivx_7(&self) -> bool {
        *self == LCDANMDIVX_A::LCDANMDIVX_7
    }
}
#[doc = "Field `LCDANMDIVx` writer - Clock divider for animation frequency"]
pub type LCDANMDIVX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LCDANMCTL_SPEC, u8, LCDANMDIVX_A, 3, O>;
impl<'a, const O: u8> LCDANMDIVX_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn lcdanmdivx_0(self) -> &'a mut W {
        self.variant(LCDANMDIVX_A::LCDANMDIVX_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn lcdanmdivx_1(self) -> &'a mut W {
        self.variant(LCDANMDIVX_A::LCDANMDIVX_1)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn lcdanmdivx_2(self) -> &'a mut W {
        self.variant(LCDANMDIVX_A::LCDANMDIVX_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn lcdanmdivx_3(self) -> &'a mut W {
        self.variant(LCDANMDIVX_A::LCDANMDIVX_3)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn lcdanmdivx_4(self) -> &'a mut W {
        self.variant(LCDANMDIVX_A::LCDANMDIVX_4)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn lcdanmdivx_5(self) -> &'a mut W {
        self.variant(LCDANMDIVX_A::LCDANMDIVX_5)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn lcdanmdivx_6(self) -> &'a mut W {
        self.variant(LCDANMDIVX_A::LCDANMDIVX_6)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn lcdanmdivx_7(self) -> &'a mut W {
        self.variant(LCDANMDIVX_A::LCDANMDIVX_7)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Animation"]
    #[inline(always)]
    pub fn lcdanmen(&self) -> LCDANMEN_R {
        LCDANMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Number of Amimation frames"]
    #[inline(always)]
    pub fn lcdanmstp(&self) -> LCDANMSTP_R {
        LCDANMSTP_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 7 - Clear Animation Memory"]
    #[inline(always)]
    pub fn lcdanmclr(&self) -> LCDANMCLR_R {
        LCDANMCLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Clock pre-scaler for animation frequency"]
    #[inline(always)]
    pub fn lcdanmprex(&self) -> LCDANMPREX_R {
        LCDANMPREX_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Clock divider for animation frequency"]
    #[inline(always)]
    pub fn lcdanmdivx(&self) -> LCDANMDIVX_R {
        LCDANMDIVX_R::new(((self.bits >> 19) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Animation"]
    #[inline(always)]
    pub fn lcdanmen(&mut self) -> LCDANMEN_W<0> {
        LCDANMEN_W::new(self)
    }
    #[doc = "Bits 1:3 - Number of Amimation frames"]
    #[inline(always)]
    pub fn lcdanmstp(&mut self) -> LCDANMSTP_W<1> {
        LCDANMSTP_W::new(self)
    }
    #[doc = "Bit 7 - Clear Animation Memory"]
    #[inline(always)]
    pub fn lcdanmclr(&mut self) -> LCDANMCLR_W<7> {
        LCDANMCLR_W::new(self)
    }
    #[doc = "Bits 16:18 - Clock pre-scaler for animation frequency"]
    #[inline(always)]
    pub fn lcdanmprex(&mut self) -> LCDANMPREX_W<16> {
        LCDANMPREX_W::new(self)
    }
    #[doc = "Bits 19:21 - Clock divider for animation frequency"]
    #[inline(always)]
    pub fn lcdanmdivx(&mut self) -> LCDANMDIVX_W<19> {
        LCDANMDIVX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_F Animation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdanmctl](index.html) module"]
pub struct LCDANMCTL_SPEC;
impl crate::RegisterSpec for LCDANMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdanmctl::R](R) reader structure"]
impl crate::Readable for LCDANMCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdanmctl::W](W) writer structure"]
impl crate::Writable for LCDANMCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDANMCTL to value 0"]
impl crate::Resettable for LCDANMCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
