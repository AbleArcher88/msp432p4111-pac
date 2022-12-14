#[doc = "Register `LCDBMCTL` reader"]
pub struct R(crate::R<LCDBMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDBMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDBMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDBMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDBMCTL` writer"]
pub struct W(crate::W<LCDBMCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDBMCTL_SPEC>;
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
impl From<crate::W<LCDBMCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDBMCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDBLKMODx` reader - Blinking mode"]
pub type LCDBLKMODX_R = crate::FieldReader<u8, LCDBLKMODX_A>;
#[doc = "Blinking mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDBLKMODX_A {
    #[doc = "0: Blinking disabled"]
    LCDBLKMODX_0 = 0,
    #[doc = "1: Blinking of individual segments as enabled in blinking memory register LCDBMx."]
    LCDBLKMODX_1 = 1,
    #[doc = "2: Blinking of all segments"]
    LCDBLKMODX_2 = 2,
    #[doc = "3: Switching between display contents as stored in LCDMx and LCDBMx memory registers."]
    LCDBLKMODX_3 = 3,
}
impl From<LCDBLKMODX_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDBLKMODX_A) -> Self {
        variant as _
    }
}
impl LCDBLKMODX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDBLKMODX_A {
        match self.bits {
            0 => LCDBLKMODX_A::LCDBLKMODX_0,
            1 => LCDBLKMODX_A::LCDBLKMODX_1,
            2 => LCDBLKMODX_A::LCDBLKMODX_2,
            3 => LCDBLKMODX_A::LCDBLKMODX_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDBLKMODX_0`"]
    #[inline(always)]
    pub fn is_lcdblkmodx_0(&self) -> bool {
        *self == LCDBLKMODX_A::LCDBLKMODX_0
    }
    #[doc = "Checks if the value of the field is `LCDBLKMODX_1`"]
    #[inline(always)]
    pub fn is_lcdblkmodx_1(&self) -> bool {
        *self == LCDBLKMODX_A::LCDBLKMODX_1
    }
    #[doc = "Checks if the value of the field is `LCDBLKMODX_2`"]
    #[inline(always)]
    pub fn is_lcdblkmodx_2(&self) -> bool {
        *self == LCDBLKMODX_A::LCDBLKMODX_2
    }
    #[doc = "Checks if the value of the field is `LCDBLKMODX_3`"]
    #[inline(always)]
    pub fn is_lcdblkmodx_3(&self) -> bool {
        *self == LCDBLKMODX_A::LCDBLKMODX_3
    }
}
#[doc = "Field `LCDBLKMODx` writer - Blinking mode"]
pub type LCDBLKMODX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LCDBMCTL_SPEC, u8, LCDBLKMODX_A, 2, O>;
impl<'a, const O: u8> LCDBLKMODX_W<'a, O> {
    #[doc = "Blinking disabled"]
    #[inline(always)]
    pub fn lcdblkmodx_0(self) -> &'a mut W {
        self.variant(LCDBLKMODX_A::LCDBLKMODX_0)
    }
    #[doc = "Blinking of individual segments as enabled in blinking memory register LCDBMx."]
    #[inline(always)]
    pub fn lcdblkmodx_1(self) -> &'a mut W {
        self.variant(LCDBLKMODX_A::LCDBLKMODX_1)
    }
    #[doc = "Blinking of all segments"]
    #[inline(always)]
    pub fn lcdblkmodx_2(self) -> &'a mut W {
        self.variant(LCDBLKMODX_A::LCDBLKMODX_2)
    }
    #[doc = "Switching between display contents as stored in LCDMx and LCDBMx memory registers."]
    #[inline(always)]
    pub fn lcdblkmodx_3(self) -> &'a mut W {
        self.variant(LCDBLKMODX_A::LCDBLKMODX_3)
    }
}
#[doc = "Field `LCDBLKPREx` reader - Clock pre-scaler for blinking frequency"]
pub type LCDBLKPREX_R = crate::FieldReader<u8, LCDBLKPREX_A>;
#[doc = "Clock pre-scaler for blinking frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDBLKPREX_A {
    #[doc = "0: Divide by 512"]
    LCDBLKPREX_0 = 0,
    #[doc = "1: Divide by 1024"]
    LCDBLKPREX_1 = 1,
    #[doc = "2: Divide by 2048"]
    LCDBLKPREX_2 = 2,
    #[doc = "3: Divide by 4096"]
    LCDBLKPREX_3 = 3,
    #[doc = "4: Divide by 8162"]
    LCDBLKPREX_4 = 4,
    #[doc = "5: Divide by 16384"]
    LCDBLKPREX_5 = 5,
    #[doc = "6: Divide by 32768"]
    LCDBLKPREX_6 = 6,
    #[doc = "7: Divide by 65536"]
    LCDBLKPREX_7 = 7,
}
impl From<LCDBLKPREX_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDBLKPREX_A) -> Self {
        variant as _
    }
}
impl LCDBLKPREX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDBLKPREX_A {
        match self.bits {
            0 => LCDBLKPREX_A::LCDBLKPREX_0,
            1 => LCDBLKPREX_A::LCDBLKPREX_1,
            2 => LCDBLKPREX_A::LCDBLKPREX_2,
            3 => LCDBLKPREX_A::LCDBLKPREX_3,
            4 => LCDBLKPREX_A::LCDBLKPREX_4,
            5 => LCDBLKPREX_A::LCDBLKPREX_5,
            6 => LCDBLKPREX_A::LCDBLKPREX_6,
            7 => LCDBLKPREX_A::LCDBLKPREX_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDBLKPREX_0`"]
    #[inline(always)]
    pub fn is_lcdblkprex_0(&self) -> bool {
        *self == LCDBLKPREX_A::LCDBLKPREX_0
    }
    #[doc = "Checks if the value of the field is `LCDBLKPREX_1`"]
    #[inline(always)]
    pub fn is_lcdblkprex_1(&self) -> bool {
        *self == LCDBLKPREX_A::LCDBLKPREX_1
    }
    #[doc = "Checks if the value of the field is `LCDBLKPREX_2`"]
    #[inline(always)]
    pub fn is_lcdblkprex_2(&self) -> bool {
        *self == LCDBLKPREX_A::LCDBLKPREX_2
    }
    #[doc = "Checks if the value of the field is `LCDBLKPREX_3`"]
    #[inline(always)]
    pub fn is_lcdblkprex_3(&self) -> bool {
        *self == LCDBLKPREX_A::LCDBLKPREX_3
    }
    #[doc = "Checks if the value of the field is `LCDBLKPREX_4`"]
    #[inline(always)]
    pub fn is_lcdblkprex_4(&self) -> bool {
        *self == LCDBLKPREX_A::LCDBLKPREX_4
    }
    #[doc = "Checks if the value of the field is `LCDBLKPREX_5`"]
    #[inline(always)]
    pub fn is_lcdblkprex_5(&self) -> bool {
        *self == LCDBLKPREX_A::LCDBLKPREX_5
    }
    #[doc = "Checks if the value of the field is `LCDBLKPREX_6`"]
    #[inline(always)]
    pub fn is_lcdblkprex_6(&self) -> bool {
        *self == LCDBLKPREX_A::LCDBLKPREX_6
    }
    #[doc = "Checks if the value of the field is `LCDBLKPREX_7`"]
    #[inline(always)]
    pub fn is_lcdblkprex_7(&self) -> bool {
        *self == LCDBLKPREX_A::LCDBLKPREX_7
    }
}
#[doc = "Field `LCDBLKPREx` writer - Clock pre-scaler for blinking frequency"]
pub type LCDBLKPREX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LCDBMCTL_SPEC, u8, LCDBLKPREX_A, 3, O>;
impl<'a, const O: u8> LCDBLKPREX_W<'a, O> {
    #[doc = "Divide by 512"]
    #[inline(always)]
    pub fn lcdblkprex_0(self) -> &'a mut W {
        self.variant(LCDBLKPREX_A::LCDBLKPREX_0)
    }
    #[doc = "Divide by 1024"]
    #[inline(always)]
    pub fn lcdblkprex_1(self) -> &'a mut W {
        self.variant(LCDBLKPREX_A::LCDBLKPREX_1)
    }
    #[doc = "Divide by 2048"]
    #[inline(always)]
    pub fn lcdblkprex_2(self) -> &'a mut W {
        self.variant(LCDBLKPREX_A::LCDBLKPREX_2)
    }
    #[doc = "Divide by 4096"]
    #[inline(always)]
    pub fn lcdblkprex_3(self) -> &'a mut W {
        self.variant(LCDBLKPREX_A::LCDBLKPREX_3)
    }
    #[doc = "Divide by 8162"]
    #[inline(always)]
    pub fn lcdblkprex_4(self) -> &'a mut W {
        self.variant(LCDBLKPREX_A::LCDBLKPREX_4)
    }
    #[doc = "Divide by 16384"]
    #[inline(always)]
    pub fn lcdblkprex_5(self) -> &'a mut W {
        self.variant(LCDBLKPREX_A::LCDBLKPREX_5)
    }
    #[doc = "Divide by 32768"]
    #[inline(always)]
    pub fn lcdblkprex_6(self) -> &'a mut W {
        self.variant(LCDBLKPREX_A::LCDBLKPREX_6)
    }
    #[doc = "Divide by 65536"]
    #[inline(always)]
    pub fn lcdblkprex_7(self) -> &'a mut W {
        self.variant(LCDBLKPREX_A::LCDBLKPREX_7)
    }
}
#[doc = "Field `LCDBLKDIVx` reader - Clock divider for blinking frequency"]
pub type LCDBLKDIVX_R = crate::FieldReader<u8, LCDBLKDIVX_A>;
#[doc = "Clock divider for blinking frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCDBLKDIVX_A {
    #[doc = "0: Divide by 1"]
    LCDBLKDIVX_0 = 0,
    #[doc = "1: Divide by 2"]
    LCDBLKDIVX_1 = 1,
    #[doc = "2: Divide by 3"]
    LCDBLKDIVX_2 = 2,
    #[doc = "3: Divide by 4"]
    LCDBLKDIVX_3 = 3,
    #[doc = "4: Divide by 5"]
    LCDBLKDIVX_4 = 4,
    #[doc = "5: Divide by 6"]
    LCDBLKDIVX_5 = 5,
    #[doc = "6: Divide by 7"]
    LCDBLKDIVX_6 = 6,
    #[doc = "7: Divide by 8"]
    LCDBLKDIVX_7 = 7,
}
impl From<LCDBLKDIVX_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDBLKDIVX_A) -> Self {
        variant as _
    }
}
impl LCDBLKDIVX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDBLKDIVX_A {
        match self.bits {
            0 => LCDBLKDIVX_A::LCDBLKDIVX_0,
            1 => LCDBLKDIVX_A::LCDBLKDIVX_1,
            2 => LCDBLKDIVX_A::LCDBLKDIVX_2,
            3 => LCDBLKDIVX_A::LCDBLKDIVX_3,
            4 => LCDBLKDIVX_A::LCDBLKDIVX_4,
            5 => LCDBLKDIVX_A::LCDBLKDIVX_5,
            6 => LCDBLKDIVX_A::LCDBLKDIVX_6,
            7 => LCDBLKDIVX_A::LCDBLKDIVX_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIVX_0`"]
    #[inline(always)]
    pub fn is_lcdblkdivx_0(&self) -> bool {
        *self == LCDBLKDIVX_A::LCDBLKDIVX_0
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIVX_1`"]
    #[inline(always)]
    pub fn is_lcdblkdivx_1(&self) -> bool {
        *self == LCDBLKDIVX_A::LCDBLKDIVX_1
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIVX_2`"]
    #[inline(always)]
    pub fn is_lcdblkdivx_2(&self) -> bool {
        *self == LCDBLKDIVX_A::LCDBLKDIVX_2
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIVX_3`"]
    #[inline(always)]
    pub fn is_lcdblkdivx_3(&self) -> bool {
        *self == LCDBLKDIVX_A::LCDBLKDIVX_3
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIVX_4`"]
    #[inline(always)]
    pub fn is_lcdblkdivx_4(&self) -> bool {
        *self == LCDBLKDIVX_A::LCDBLKDIVX_4
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIVX_5`"]
    #[inline(always)]
    pub fn is_lcdblkdivx_5(&self) -> bool {
        *self == LCDBLKDIVX_A::LCDBLKDIVX_5
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIVX_6`"]
    #[inline(always)]
    pub fn is_lcdblkdivx_6(&self) -> bool {
        *self == LCDBLKDIVX_A::LCDBLKDIVX_6
    }
    #[doc = "Checks if the value of the field is `LCDBLKDIVX_7`"]
    #[inline(always)]
    pub fn is_lcdblkdivx_7(&self) -> bool {
        *self == LCDBLKDIVX_A::LCDBLKDIVX_7
    }
}
#[doc = "Field `LCDBLKDIVx` writer - Clock divider for blinking frequency"]
pub type LCDBLKDIVX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LCDBMCTL_SPEC, u8, LCDBLKDIVX_A, 3, O>;
impl<'a, const O: u8> LCDBLKDIVX_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn lcdblkdivx_0(self) -> &'a mut W {
        self.variant(LCDBLKDIVX_A::LCDBLKDIVX_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn lcdblkdivx_1(self) -> &'a mut W {
        self.variant(LCDBLKDIVX_A::LCDBLKDIVX_1)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn lcdblkdivx_2(self) -> &'a mut W {
        self.variant(LCDBLKDIVX_A::LCDBLKDIVX_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn lcdblkdivx_3(self) -> &'a mut W {
        self.variant(LCDBLKDIVX_A::LCDBLKDIVX_3)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn lcdblkdivx_4(self) -> &'a mut W {
        self.variant(LCDBLKDIVX_A::LCDBLKDIVX_4)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn lcdblkdivx_5(self) -> &'a mut W {
        self.variant(LCDBLKDIVX_A::LCDBLKDIVX_5)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn lcdblkdivx_6(self) -> &'a mut W {
        self.variant(LCDBLKDIVX_A::LCDBLKDIVX_6)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn lcdblkdivx_7(self) -> &'a mut W {
        self.variant(LCDBLKDIVX_A::LCDBLKDIVX_7)
    }
}
#[doc = "Field `LCDDISP` reader - Select LCD memory registers for display"]
pub type LCDDISP_R = crate::BitReader<LCDDISP_A>;
#[doc = "Select LCD memory registers for display\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDDISP_A {
    #[doc = "0: Display content of LCD memory registers LCDMx"]
    LCDDISP_0 = 0,
    #[doc = "1: Display content of LCD blinking memory registers LCDBMx"]
    LCDDISP_1 = 1,
}
impl From<LCDDISP_A> for bool {
    #[inline(always)]
    fn from(variant: LCDDISP_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDDISP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDDISP_A {
        match self.bits {
            false => LCDDISP_A::LCDDISP_0,
            true => LCDDISP_A::LCDDISP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDDISP_0`"]
    #[inline(always)]
    pub fn is_lcddisp_0(&self) -> bool {
        *self == LCDDISP_A::LCDDISP_0
    }
    #[doc = "Checks if the value of the field is `LCDDISP_1`"]
    #[inline(always)]
    pub fn is_lcddisp_1(&self) -> bool {
        *self == LCDDISP_A::LCDDISP_1
    }
}
#[doc = "Field `LCDDISP` writer - Select LCD memory registers for display"]
pub type LCDDISP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDBMCTL_SPEC, LCDDISP_A, O>;
impl<'a, const O: u8> LCDDISP_W<'a, O> {
    #[doc = "Display content of LCD memory registers LCDMx"]
    #[inline(always)]
    pub fn lcddisp_0(self) -> &'a mut W {
        self.variant(LCDDISP_A::LCDDISP_0)
    }
    #[doc = "Display content of LCD blinking memory registers LCDBMx"]
    #[inline(always)]
    pub fn lcddisp_1(self) -> &'a mut W {
        self.variant(LCDDISP_A::LCDDISP_1)
    }
}
#[doc = "Field `LCDCLRM` reader - Clear LCD memory"]
pub type LCDCLRM_R = crate::BitReader<LCDCLRM_A>;
#[doc = "Clear LCD memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCLRM_A {
    #[doc = "0: Contents of LCD memory registers LCDMx remain unchanged"]
    LCDCLRM_0 = 0,
    #[doc = "1: Clear content of all LCD memory registers LCDMx"]
    LCDCLRM_1 = 1,
}
impl From<LCDCLRM_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCLRM_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCLRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCLRM_A {
        match self.bits {
            false => LCDCLRM_A::LCDCLRM_0,
            true => LCDCLRM_A::LCDCLRM_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCLRM_0`"]
    #[inline(always)]
    pub fn is_lcdclrm_0(&self) -> bool {
        *self == LCDCLRM_A::LCDCLRM_0
    }
    #[doc = "Checks if the value of the field is `LCDCLRM_1`"]
    #[inline(always)]
    pub fn is_lcdclrm_1(&self) -> bool {
        *self == LCDCLRM_A::LCDCLRM_1
    }
}
#[doc = "Field `LCDCLRM` writer - Clear LCD memory"]
pub type LCDCLRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDBMCTL_SPEC, LCDCLRM_A, O>;
impl<'a, const O: u8> LCDCLRM_W<'a, O> {
    #[doc = "Contents of LCD memory registers LCDMx remain unchanged"]
    #[inline(always)]
    pub fn lcdclrm_0(self) -> &'a mut W {
        self.variant(LCDCLRM_A::LCDCLRM_0)
    }
    #[doc = "Clear content of all LCD memory registers LCDMx"]
    #[inline(always)]
    pub fn lcdclrm_1(self) -> &'a mut W {
        self.variant(LCDCLRM_A::LCDCLRM_1)
    }
}
#[doc = "Field `LCDCLRBM` reader - Clear LCD blinking memory"]
pub type LCDCLRBM_R = crate::BitReader<LCDCLRBM_A>;
#[doc = "Clear LCD blinking memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCLRBM_A {
    #[doc = "0: Contents of blinking memory registers LCDBMx remain unchanged"]
    LCDCLRBM_0 = 0,
    #[doc = "1: Clear content of all blinking memory registers LCDBMx"]
    LCDCLRBM_1 = 1,
}
impl From<LCDCLRBM_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCLRBM_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCLRBM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCLRBM_A {
        match self.bits {
            false => LCDCLRBM_A::LCDCLRBM_0,
            true => LCDCLRBM_A::LCDCLRBM_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCLRBM_0`"]
    #[inline(always)]
    pub fn is_lcdclrbm_0(&self) -> bool {
        *self == LCDCLRBM_A::LCDCLRBM_0
    }
    #[doc = "Checks if the value of the field is `LCDCLRBM_1`"]
    #[inline(always)]
    pub fn is_lcdclrbm_1(&self) -> bool {
        *self == LCDCLRBM_A::LCDCLRBM_1
    }
}
#[doc = "Field `LCDCLRBM` writer - Clear LCD blinking memory"]
pub type LCDCLRBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDBMCTL_SPEC, LCDCLRBM_A, O>;
impl<'a, const O: u8> LCDCLRBM_W<'a, O> {
    #[doc = "Contents of blinking memory registers LCDBMx remain unchanged"]
    #[inline(always)]
    pub fn lcdclrbm_0(self) -> &'a mut W {
        self.variant(LCDCLRBM_A::LCDCLRBM_0)
    }
    #[doc = "Clear content of all blinking memory registers LCDBMx"]
    #[inline(always)]
    pub fn lcdclrbm_1(self) -> &'a mut W {
        self.variant(LCDCLRBM_A::LCDCLRBM_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Blinking mode"]
    #[inline(always)]
    pub fn lcdblkmodx(&self) -> LCDBLKMODX_R {
        LCDBLKMODX_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Clock pre-scaler for blinking frequency"]
    #[inline(always)]
    pub fn lcdblkprex(&self) -> LCDBLKPREX_R {
        LCDBLKPREX_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - Clock divider for blinking frequency"]
    #[inline(always)]
    pub fn lcdblkdivx(&self) -> LCDBLKDIVX_R {
        LCDBLKDIVX_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 16 - Select LCD memory registers for display"]
    #[inline(always)]
    pub fn lcddisp(&self) -> LCDDISP_R {
        LCDDISP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clear LCD memory"]
    #[inline(always)]
    pub fn lcdclrm(&self) -> LCDCLRM_R {
        LCDCLRM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clear LCD blinking memory"]
    #[inline(always)]
    pub fn lcdclrbm(&self) -> LCDCLRBM_R {
        LCDCLRBM_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Blinking mode"]
    #[inline(always)]
    pub fn lcdblkmodx(&mut self) -> LCDBLKMODX_W<0> {
        LCDBLKMODX_W::new(self)
    }
    #[doc = "Bits 2:4 - Clock pre-scaler for blinking frequency"]
    #[inline(always)]
    pub fn lcdblkprex(&mut self) -> LCDBLKPREX_W<2> {
        LCDBLKPREX_W::new(self)
    }
    #[doc = "Bits 5:7 - Clock divider for blinking frequency"]
    #[inline(always)]
    pub fn lcdblkdivx(&mut self) -> LCDBLKDIVX_W<5> {
        LCDBLKDIVX_W::new(self)
    }
    #[doc = "Bit 16 - Select LCD memory registers for display"]
    #[inline(always)]
    pub fn lcddisp(&mut self) -> LCDDISP_W<16> {
        LCDDISP_W::new(self)
    }
    #[doc = "Bit 17 - Clear LCD memory"]
    #[inline(always)]
    pub fn lcdclrm(&mut self) -> LCDCLRM_W<17> {
        LCDCLRM_W::new(self)
    }
    #[doc = "Bit 18 - Clear LCD blinking memory"]
    #[inline(always)]
    pub fn lcdclrbm(&mut self) -> LCDCLRBM_W<18> {
        LCDCLRBM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_F blinking and memory control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbmctl](index.html) module"]
pub struct LCDBMCTL_SPEC;
impl crate::RegisterSpec for LCDBMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdbmctl::R](R) reader structure"]
impl crate::Readable for LCDBMCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdbmctl::W](W) writer structure"]
impl crate::Writable for LCDBMCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDBMCTL to value 0"]
impl crate::Resettable for LCDBMCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
