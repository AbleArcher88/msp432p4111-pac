#[doc = "Register `LCDPCTL0` reader"]
pub struct R(crate::R<LCDPCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDPCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDPCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDPCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDPCTL0` writer"]
pub struct W(crate::W<LCDPCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDPCTL0_SPEC>;
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
impl From<crate::W<LCDPCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDPCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDS0` reader - LCD pin 0 enable"]
pub type LCDS0_R = crate::BitReader<LCDS0_A>;
#[doc = "LCD pin 0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS0_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS0_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS0_1 = 1,
}
impl From<LCDS0_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS0_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS0_A {
        match self.bits {
            false => LCDS0_A::LCDS0_0,
            true => LCDS0_A::LCDS0_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS0_0`"]
    #[inline(always)]
    pub fn is_lcds0_0(&self) -> bool {
        *self == LCDS0_A::LCDS0_0
    }
    #[doc = "Checks if the value of the field is `LCDS0_1`"]
    #[inline(always)]
    pub fn is_lcds0_1(&self) -> bool {
        *self == LCDS0_A::LCDS0_1
    }
}
#[doc = "Field `LCDS0` writer - LCD pin 0 enable"]
pub type LCDS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS0_A, O>;
impl<'a, const O: u8> LCDS0_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds0_0(self) -> &'a mut W {
        self.variant(LCDS0_A::LCDS0_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds0_1(self) -> &'a mut W {
        self.variant(LCDS0_A::LCDS0_1)
    }
}
#[doc = "Field `LCDS1` reader - LCD pin 1 enable"]
pub type LCDS1_R = crate::BitReader<LCDS1_A>;
#[doc = "LCD pin 1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS1_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS1_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS1_1 = 1,
}
impl From<LCDS1_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS1_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS1_A {
        match self.bits {
            false => LCDS1_A::LCDS1_0,
            true => LCDS1_A::LCDS1_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS1_0`"]
    #[inline(always)]
    pub fn is_lcds1_0(&self) -> bool {
        *self == LCDS1_A::LCDS1_0
    }
    #[doc = "Checks if the value of the field is `LCDS1_1`"]
    #[inline(always)]
    pub fn is_lcds1_1(&self) -> bool {
        *self == LCDS1_A::LCDS1_1
    }
}
#[doc = "Field `LCDS1` writer - LCD pin 1 enable"]
pub type LCDS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS1_A, O>;
impl<'a, const O: u8> LCDS1_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds1_0(self) -> &'a mut W {
        self.variant(LCDS1_A::LCDS1_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds1_1(self) -> &'a mut W {
        self.variant(LCDS1_A::LCDS1_1)
    }
}
#[doc = "Field `LCDS2` reader - LCD pin 2 enable"]
pub type LCDS2_R = crate::BitReader<LCDS2_A>;
#[doc = "LCD pin 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS2_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS2_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS2_1 = 1,
}
impl From<LCDS2_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS2_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS2_A {
        match self.bits {
            false => LCDS2_A::LCDS2_0,
            true => LCDS2_A::LCDS2_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS2_0`"]
    #[inline(always)]
    pub fn is_lcds2_0(&self) -> bool {
        *self == LCDS2_A::LCDS2_0
    }
    #[doc = "Checks if the value of the field is `LCDS2_1`"]
    #[inline(always)]
    pub fn is_lcds2_1(&self) -> bool {
        *self == LCDS2_A::LCDS2_1
    }
}
#[doc = "Field `LCDS2` writer - LCD pin 2 enable"]
pub type LCDS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS2_A, O>;
impl<'a, const O: u8> LCDS2_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds2_0(self) -> &'a mut W {
        self.variant(LCDS2_A::LCDS2_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds2_1(self) -> &'a mut W {
        self.variant(LCDS2_A::LCDS2_1)
    }
}
#[doc = "Field `LCDS3` reader - LCD pin 3 enable"]
pub type LCDS3_R = crate::BitReader<LCDS3_A>;
#[doc = "LCD pin 3 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS3_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS3_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS3_1 = 1,
}
impl From<LCDS3_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS3_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS3_A {
        match self.bits {
            false => LCDS3_A::LCDS3_0,
            true => LCDS3_A::LCDS3_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS3_0`"]
    #[inline(always)]
    pub fn is_lcds3_0(&self) -> bool {
        *self == LCDS3_A::LCDS3_0
    }
    #[doc = "Checks if the value of the field is `LCDS3_1`"]
    #[inline(always)]
    pub fn is_lcds3_1(&self) -> bool {
        *self == LCDS3_A::LCDS3_1
    }
}
#[doc = "Field `LCDS3` writer - LCD pin 3 enable"]
pub type LCDS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS3_A, O>;
impl<'a, const O: u8> LCDS3_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds3_0(self) -> &'a mut W {
        self.variant(LCDS3_A::LCDS3_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds3_1(self) -> &'a mut W {
        self.variant(LCDS3_A::LCDS3_1)
    }
}
#[doc = "Field `LCDS4` reader - LCD pin 4 enable"]
pub type LCDS4_R = crate::BitReader<LCDS4_A>;
#[doc = "LCD pin 4 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS4_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS4_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS4_1 = 1,
}
impl From<LCDS4_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS4_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS4_A {
        match self.bits {
            false => LCDS4_A::LCDS4_0,
            true => LCDS4_A::LCDS4_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS4_0`"]
    #[inline(always)]
    pub fn is_lcds4_0(&self) -> bool {
        *self == LCDS4_A::LCDS4_0
    }
    #[doc = "Checks if the value of the field is `LCDS4_1`"]
    #[inline(always)]
    pub fn is_lcds4_1(&self) -> bool {
        *self == LCDS4_A::LCDS4_1
    }
}
#[doc = "Field `LCDS4` writer - LCD pin 4 enable"]
pub type LCDS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS4_A, O>;
impl<'a, const O: u8> LCDS4_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds4_0(self) -> &'a mut W {
        self.variant(LCDS4_A::LCDS4_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds4_1(self) -> &'a mut W {
        self.variant(LCDS4_A::LCDS4_1)
    }
}
#[doc = "Field `LCDS5` reader - LCD pin 5 enable"]
pub type LCDS5_R = crate::BitReader<LCDS5_A>;
#[doc = "LCD pin 5 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS5_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS5_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS5_1 = 1,
}
impl From<LCDS5_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS5_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS5_A {
        match self.bits {
            false => LCDS5_A::LCDS5_0,
            true => LCDS5_A::LCDS5_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS5_0`"]
    #[inline(always)]
    pub fn is_lcds5_0(&self) -> bool {
        *self == LCDS5_A::LCDS5_0
    }
    #[doc = "Checks if the value of the field is `LCDS5_1`"]
    #[inline(always)]
    pub fn is_lcds5_1(&self) -> bool {
        *self == LCDS5_A::LCDS5_1
    }
}
#[doc = "Field `LCDS5` writer - LCD pin 5 enable"]
pub type LCDS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS5_A, O>;
impl<'a, const O: u8> LCDS5_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds5_0(self) -> &'a mut W {
        self.variant(LCDS5_A::LCDS5_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds5_1(self) -> &'a mut W {
        self.variant(LCDS5_A::LCDS5_1)
    }
}
#[doc = "Field `LCDS6` reader - LCD pin 6 enable"]
pub type LCDS6_R = crate::BitReader<LCDS6_A>;
#[doc = "LCD pin 6 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS6_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS6_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS6_1 = 1,
}
impl From<LCDS6_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS6_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS6_A {
        match self.bits {
            false => LCDS6_A::LCDS6_0,
            true => LCDS6_A::LCDS6_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS6_0`"]
    #[inline(always)]
    pub fn is_lcds6_0(&self) -> bool {
        *self == LCDS6_A::LCDS6_0
    }
    #[doc = "Checks if the value of the field is `LCDS6_1`"]
    #[inline(always)]
    pub fn is_lcds6_1(&self) -> bool {
        *self == LCDS6_A::LCDS6_1
    }
}
#[doc = "Field `LCDS6` writer - LCD pin 6 enable"]
pub type LCDS6_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS6_A, O>;
impl<'a, const O: u8> LCDS6_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds6_0(self) -> &'a mut W {
        self.variant(LCDS6_A::LCDS6_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds6_1(self) -> &'a mut W {
        self.variant(LCDS6_A::LCDS6_1)
    }
}
#[doc = "Field `LCDS7` reader - LCD pin 7 enable"]
pub type LCDS7_R = crate::BitReader<LCDS7_A>;
#[doc = "LCD pin 7 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS7_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS7_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS7_1 = 1,
}
impl From<LCDS7_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS7_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS7_A {
        match self.bits {
            false => LCDS7_A::LCDS7_0,
            true => LCDS7_A::LCDS7_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS7_0`"]
    #[inline(always)]
    pub fn is_lcds7_0(&self) -> bool {
        *self == LCDS7_A::LCDS7_0
    }
    #[doc = "Checks if the value of the field is `LCDS7_1`"]
    #[inline(always)]
    pub fn is_lcds7_1(&self) -> bool {
        *self == LCDS7_A::LCDS7_1
    }
}
#[doc = "Field `LCDS7` writer - LCD pin 7 enable"]
pub type LCDS7_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS7_A, O>;
impl<'a, const O: u8> LCDS7_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds7_0(self) -> &'a mut W {
        self.variant(LCDS7_A::LCDS7_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds7_1(self) -> &'a mut W {
        self.variant(LCDS7_A::LCDS7_1)
    }
}
#[doc = "Field `LCDS8` reader - LCD pin 8 enable"]
pub type LCDS8_R = crate::BitReader<LCDS8_A>;
#[doc = "LCD pin 8 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS8_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS8_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS8_1 = 1,
}
impl From<LCDS8_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS8_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS8_A {
        match self.bits {
            false => LCDS8_A::LCDS8_0,
            true => LCDS8_A::LCDS8_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS8_0`"]
    #[inline(always)]
    pub fn is_lcds8_0(&self) -> bool {
        *self == LCDS8_A::LCDS8_0
    }
    #[doc = "Checks if the value of the field is `LCDS8_1`"]
    #[inline(always)]
    pub fn is_lcds8_1(&self) -> bool {
        *self == LCDS8_A::LCDS8_1
    }
}
#[doc = "Field `LCDS8` writer - LCD pin 8 enable"]
pub type LCDS8_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS8_A, O>;
impl<'a, const O: u8> LCDS8_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds8_0(self) -> &'a mut W {
        self.variant(LCDS8_A::LCDS8_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds8_1(self) -> &'a mut W {
        self.variant(LCDS8_A::LCDS8_1)
    }
}
#[doc = "Field `LCDS9` reader - LCD pin 9 enable"]
pub type LCDS9_R = crate::BitReader<LCDS9_A>;
#[doc = "LCD pin 9 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS9_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS9_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS9_1 = 1,
}
impl From<LCDS9_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS9_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS9_A {
        match self.bits {
            false => LCDS9_A::LCDS9_0,
            true => LCDS9_A::LCDS9_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS9_0`"]
    #[inline(always)]
    pub fn is_lcds9_0(&self) -> bool {
        *self == LCDS9_A::LCDS9_0
    }
    #[doc = "Checks if the value of the field is `LCDS9_1`"]
    #[inline(always)]
    pub fn is_lcds9_1(&self) -> bool {
        *self == LCDS9_A::LCDS9_1
    }
}
#[doc = "Field `LCDS9` writer - LCD pin 9 enable"]
pub type LCDS9_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS9_A, O>;
impl<'a, const O: u8> LCDS9_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds9_0(self) -> &'a mut W {
        self.variant(LCDS9_A::LCDS9_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds9_1(self) -> &'a mut W {
        self.variant(LCDS9_A::LCDS9_1)
    }
}
#[doc = "Field `LCDS10` reader - LCD pin 10 enable"]
pub type LCDS10_R = crate::BitReader<LCDS10_A>;
#[doc = "LCD pin 10 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS10_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS10_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS10_1 = 1,
}
impl From<LCDS10_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS10_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS10_A {
        match self.bits {
            false => LCDS10_A::LCDS10_0,
            true => LCDS10_A::LCDS10_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS10_0`"]
    #[inline(always)]
    pub fn is_lcds10_0(&self) -> bool {
        *self == LCDS10_A::LCDS10_0
    }
    #[doc = "Checks if the value of the field is `LCDS10_1`"]
    #[inline(always)]
    pub fn is_lcds10_1(&self) -> bool {
        *self == LCDS10_A::LCDS10_1
    }
}
#[doc = "Field `LCDS10` writer - LCD pin 10 enable"]
pub type LCDS10_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS10_A, O>;
impl<'a, const O: u8> LCDS10_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds10_0(self) -> &'a mut W {
        self.variant(LCDS10_A::LCDS10_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds10_1(self) -> &'a mut W {
        self.variant(LCDS10_A::LCDS10_1)
    }
}
#[doc = "Field `LCDS11` reader - LCD pin 11 enable"]
pub type LCDS11_R = crate::BitReader<LCDS11_A>;
#[doc = "LCD pin 11 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS11_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS11_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS11_1 = 1,
}
impl From<LCDS11_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS11_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS11_A {
        match self.bits {
            false => LCDS11_A::LCDS11_0,
            true => LCDS11_A::LCDS11_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS11_0`"]
    #[inline(always)]
    pub fn is_lcds11_0(&self) -> bool {
        *self == LCDS11_A::LCDS11_0
    }
    #[doc = "Checks if the value of the field is `LCDS11_1`"]
    #[inline(always)]
    pub fn is_lcds11_1(&self) -> bool {
        *self == LCDS11_A::LCDS11_1
    }
}
#[doc = "Field `LCDS11` writer - LCD pin 11 enable"]
pub type LCDS11_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS11_A, O>;
impl<'a, const O: u8> LCDS11_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds11_0(self) -> &'a mut W {
        self.variant(LCDS11_A::LCDS11_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds11_1(self) -> &'a mut W {
        self.variant(LCDS11_A::LCDS11_1)
    }
}
#[doc = "Field `LCDS12` reader - LCD pin 12 enable"]
pub type LCDS12_R = crate::BitReader<LCDS12_A>;
#[doc = "LCD pin 12 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS12_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS12_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS12_1 = 1,
}
impl From<LCDS12_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS12_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS12_A {
        match self.bits {
            false => LCDS12_A::LCDS12_0,
            true => LCDS12_A::LCDS12_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS12_0`"]
    #[inline(always)]
    pub fn is_lcds12_0(&self) -> bool {
        *self == LCDS12_A::LCDS12_0
    }
    #[doc = "Checks if the value of the field is `LCDS12_1`"]
    #[inline(always)]
    pub fn is_lcds12_1(&self) -> bool {
        *self == LCDS12_A::LCDS12_1
    }
}
#[doc = "Field `LCDS12` writer - LCD pin 12 enable"]
pub type LCDS12_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS12_A, O>;
impl<'a, const O: u8> LCDS12_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds12_0(self) -> &'a mut W {
        self.variant(LCDS12_A::LCDS12_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds12_1(self) -> &'a mut W {
        self.variant(LCDS12_A::LCDS12_1)
    }
}
#[doc = "Field `LCDS13` reader - LCD pin 13 enable"]
pub type LCDS13_R = crate::BitReader<LCDS13_A>;
#[doc = "LCD pin 13 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS13_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS13_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS13_1 = 1,
}
impl From<LCDS13_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS13_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS13_A {
        match self.bits {
            false => LCDS13_A::LCDS13_0,
            true => LCDS13_A::LCDS13_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS13_0`"]
    #[inline(always)]
    pub fn is_lcds13_0(&self) -> bool {
        *self == LCDS13_A::LCDS13_0
    }
    #[doc = "Checks if the value of the field is `LCDS13_1`"]
    #[inline(always)]
    pub fn is_lcds13_1(&self) -> bool {
        *self == LCDS13_A::LCDS13_1
    }
}
#[doc = "Field `LCDS13` writer - LCD pin 13 enable"]
pub type LCDS13_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS13_A, O>;
impl<'a, const O: u8> LCDS13_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds13_0(self) -> &'a mut W {
        self.variant(LCDS13_A::LCDS13_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds13_1(self) -> &'a mut W {
        self.variant(LCDS13_A::LCDS13_1)
    }
}
#[doc = "Field `LCDS14` reader - LCD pin 14 enable"]
pub type LCDS14_R = crate::BitReader<LCDS14_A>;
#[doc = "LCD pin 14 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS14_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS14_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS14_1 = 1,
}
impl From<LCDS14_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS14_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS14_A {
        match self.bits {
            false => LCDS14_A::LCDS14_0,
            true => LCDS14_A::LCDS14_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS14_0`"]
    #[inline(always)]
    pub fn is_lcds14_0(&self) -> bool {
        *self == LCDS14_A::LCDS14_0
    }
    #[doc = "Checks if the value of the field is `LCDS14_1`"]
    #[inline(always)]
    pub fn is_lcds14_1(&self) -> bool {
        *self == LCDS14_A::LCDS14_1
    }
}
#[doc = "Field `LCDS14` writer - LCD pin 14 enable"]
pub type LCDS14_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS14_A, O>;
impl<'a, const O: u8> LCDS14_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds14_0(self) -> &'a mut W {
        self.variant(LCDS14_A::LCDS14_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds14_1(self) -> &'a mut W {
        self.variant(LCDS14_A::LCDS14_1)
    }
}
#[doc = "Field `LCDS15` reader - LCD pin 15 enable"]
pub type LCDS15_R = crate::BitReader<LCDS15_A>;
#[doc = "LCD pin 15 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS15_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS15_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS15_1 = 1,
}
impl From<LCDS15_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS15_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS15_A {
        match self.bits {
            false => LCDS15_A::LCDS15_0,
            true => LCDS15_A::LCDS15_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS15_0`"]
    #[inline(always)]
    pub fn is_lcds15_0(&self) -> bool {
        *self == LCDS15_A::LCDS15_0
    }
    #[doc = "Checks if the value of the field is `LCDS15_1`"]
    #[inline(always)]
    pub fn is_lcds15_1(&self) -> bool {
        *self == LCDS15_A::LCDS15_1
    }
}
#[doc = "Field `LCDS15` writer - LCD pin 15 enable"]
pub type LCDS15_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS15_A, O>;
impl<'a, const O: u8> LCDS15_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds15_0(self) -> &'a mut W {
        self.variant(LCDS15_A::LCDS15_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds15_1(self) -> &'a mut W {
        self.variant(LCDS15_A::LCDS15_1)
    }
}
#[doc = "Field `LCDS16` reader - LCD pin 16 enable"]
pub type LCDS16_R = crate::BitReader<LCDS16_A>;
#[doc = "LCD pin 16 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS16_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS16_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS16_1 = 1,
}
impl From<LCDS16_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS16_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS16_A {
        match self.bits {
            false => LCDS16_A::LCDS16_0,
            true => LCDS16_A::LCDS16_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS16_0`"]
    #[inline(always)]
    pub fn is_lcds16_0(&self) -> bool {
        *self == LCDS16_A::LCDS16_0
    }
    #[doc = "Checks if the value of the field is `LCDS16_1`"]
    #[inline(always)]
    pub fn is_lcds16_1(&self) -> bool {
        *self == LCDS16_A::LCDS16_1
    }
}
#[doc = "Field `LCDS16` writer - LCD pin 16 enable"]
pub type LCDS16_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS16_A, O>;
impl<'a, const O: u8> LCDS16_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds16_0(self) -> &'a mut W {
        self.variant(LCDS16_A::LCDS16_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds16_1(self) -> &'a mut W {
        self.variant(LCDS16_A::LCDS16_1)
    }
}
#[doc = "Field `LCDS17` reader - LCD pin 17 enable"]
pub type LCDS17_R = crate::BitReader<LCDS17_A>;
#[doc = "LCD pin 17 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS17_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS17_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS17_1 = 1,
}
impl From<LCDS17_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS17_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS17_A {
        match self.bits {
            false => LCDS17_A::LCDS17_0,
            true => LCDS17_A::LCDS17_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS17_0`"]
    #[inline(always)]
    pub fn is_lcds17_0(&self) -> bool {
        *self == LCDS17_A::LCDS17_0
    }
    #[doc = "Checks if the value of the field is `LCDS17_1`"]
    #[inline(always)]
    pub fn is_lcds17_1(&self) -> bool {
        *self == LCDS17_A::LCDS17_1
    }
}
#[doc = "Field `LCDS17` writer - LCD pin 17 enable"]
pub type LCDS17_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS17_A, O>;
impl<'a, const O: u8> LCDS17_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds17_0(self) -> &'a mut W {
        self.variant(LCDS17_A::LCDS17_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds17_1(self) -> &'a mut W {
        self.variant(LCDS17_A::LCDS17_1)
    }
}
#[doc = "Field `LCDS18` reader - LCD pin 18 enable"]
pub type LCDS18_R = crate::BitReader<LCDS18_A>;
#[doc = "LCD pin 18 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS18_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS18_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS18_1 = 1,
}
impl From<LCDS18_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS18_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS18_A {
        match self.bits {
            false => LCDS18_A::LCDS18_0,
            true => LCDS18_A::LCDS18_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS18_0`"]
    #[inline(always)]
    pub fn is_lcds18_0(&self) -> bool {
        *self == LCDS18_A::LCDS18_0
    }
    #[doc = "Checks if the value of the field is `LCDS18_1`"]
    #[inline(always)]
    pub fn is_lcds18_1(&self) -> bool {
        *self == LCDS18_A::LCDS18_1
    }
}
#[doc = "Field `LCDS18` writer - LCD pin 18 enable"]
pub type LCDS18_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS18_A, O>;
impl<'a, const O: u8> LCDS18_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds18_0(self) -> &'a mut W {
        self.variant(LCDS18_A::LCDS18_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds18_1(self) -> &'a mut W {
        self.variant(LCDS18_A::LCDS18_1)
    }
}
#[doc = "Field `LCDS19` reader - LCD pin 19 enable"]
pub type LCDS19_R = crate::BitReader<LCDS19_A>;
#[doc = "LCD pin 19 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS19_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS19_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS19_1 = 1,
}
impl From<LCDS19_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS19_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS19_A {
        match self.bits {
            false => LCDS19_A::LCDS19_0,
            true => LCDS19_A::LCDS19_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS19_0`"]
    #[inline(always)]
    pub fn is_lcds19_0(&self) -> bool {
        *self == LCDS19_A::LCDS19_0
    }
    #[doc = "Checks if the value of the field is `LCDS19_1`"]
    #[inline(always)]
    pub fn is_lcds19_1(&self) -> bool {
        *self == LCDS19_A::LCDS19_1
    }
}
#[doc = "Field `LCDS19` writer - LCD pin 19 enable"]
pub type LCDS19_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS19_A, O>;
impl<'a, const O: u8> LCDS19_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds19_0(self) -> &'a mut W {
        self.variant(LCDS19_A::LCDS19_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds19_1(self) -> &'a mut W {
        self.variant(LCDS19_A::LCDS19_1)
    }
}
#[doc = "Field `LCDS20` reader - LCD pin 20 enable"]
pub type LCDS20_R = crate::BitReader<LCDS20_A>;
#[doc = "LCD pin 20 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS20_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS20_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS20_1 = 1,
}
impl From<LCDS20_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS20_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS20_A {
        match self.bits {
            false => LCDS20_A::LCDS20_0,
            true => LCDS20_A::LCDS20_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS20_0`"]
    #[inline(always)]
    pub fn is_lcds20_0(&self) -> bool {
        *self == LCDS20_A::LCDS20_0
    }
    #[doc = "Checks if the value of the field is `LCDS20_1`"]
    #[inline(always)]
    pub fn is_lcds20_1(&self) -> bool {
        *self == LCDS20_A::LCDS20_1
    }
}
#[doc = "Field `LCDS20` writer - LCD pin 20 enable"]
pub type LCDS20_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS20_A, O>;
impl<'a, const O: u8> LCDS20_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds20_0(self) -> &'a mut W {
        self.variant(LCDS20_A::LCDS20_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds20_1(self) -> &'a mut W {
        self.variant(LCDS20_A::LCDS20_1)
    }
}
#[doc = "Field `LCDS21` reader - LCD pin 21 enable"]
pub type LCDS21_R = crate::BitReader<LCDS21_A>;
#[doc = "LCD pin 21 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS21_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS21_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS21_1 = 1,
}
impl From<LCDS21_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS21_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS21_A {
        match self.bits {
            false => LCDS21_A::LCDS21_0,
            true => LCDS21_A::LCDS21_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS21_0`"]
    #[inline(always)]
    pub fn is_lcds21_0(&self) -> bool {
        *self == LCDS21_A::LCDS21_0
    }
    #[doc = "Checks if the value of the field is `LCDS21_1`"]
    #[inline(always)]
    pub fn is_lcds21_1(&self) -> bool {
        *self == LCDS21_A::LCDS21_1
    }
}
#[doc = "Field `LCDS21` writer - LCD pin 21 enable"]
pub type LCDS21_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS21_A, O>;
impl<'a, const O: u8> LCDS21_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds21_0(self) -> &'a mut W {
        self.variant(LCDS21_A::LCDS21_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds21_1(self) -> &'a mut W {
        self.variant(LCDS21_A::LCDS21_1)
    }
}
#[doc = "Field `LCDS22` reader - LCD pin 22 enable"]
pub type LCDS22_R = crate::BitReader<LCDS22_A>;
#[doc = "LCD pin 22 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS22_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS22_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS22_1 = 1,
}
impl From<LCDS22_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS22_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS22_A {
        match self.bits {
            false => LCDS22_A::LCDS22_0,
            true => LCDS22_A::LCDS22_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS22_0`"]
    #[inline(always)]
    pub fn is_lcds22_0(&self) -> bool {
        *self == LCDS22_A::LCDS22_0
    }
    #[doc = "Checks if the value of the field is `LCDS22_1`"]
    #[inline(always)]
    pub fn is_lcds22_1(&self) -> bool {
        *self == LCDS22_A::LCDS22_1
    }
}
#[doc = "Field `LCDS22` writer - LCD pin 22 enable"]
pub type LCDS22_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS22_A, O>;
impl<'a, const O: u8> LCDS22_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds22_0(self) -> &'a mut W {
        self.variant(LCDS22_A::LCDS22_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds22_1(self) -> &'a mut W {
        self.variant(LCDS22_A::LCDS22_1)
    }
}
#[doc = "Field `LCDS23` reader - LCD pin 23 enable"]
pub type LCDS23_R = crate::BitReader<LCDS23_A>;
#[doc = "LCD pin 23 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS23_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS23_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS23_1 = 1,
}
impl From<LCDS23_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS23_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS23_A {
        match self.bits {
            false => LCDS23_A::LCDS23_0,
            true => LCDS23_A::LCDS23_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS23_0`"]
    #[inline(always)]
    pub fn is_lcds23_0(&self) -> bool {
        *self == LCDS23_A::LCDS23_0
    }
    #[doc = "Checks if the value of the field is `LCDS23_1`"]
    #[inline(always)]
    pub fn is_lcds23_1(&self) -> bool {
        *self == LCDS23_A::LCDS23_1
    }
}
#[doc = "Field `LCDS23` writer - LCD pin 23 enable"]
pub type LCDS23_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS23_A, O>;
impl<'a, const O: u8> LCDS23_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds23_0(self) -> &'a mut W {
        self.variant(LCDS23_A::LCDS23_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds23_1(self) -> &'a mut W {
        self.variant(LCDS23_A::LCDS23_1)
    }
}
#[doc = "Field `LCDS24` reader - LCD pin 24 enable"]
pub type LCDS24_R = crate::BitReader<LCDS24_A>;
#[doc = "LCD pin 24 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS24_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS24_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS24_1 = 1,
}
impl From<LCDS24_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS24_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS24_A {
        match self.bits {
            false => LCDS24_A::LCDS24_0,
            true => LCDS24_A::LCDS24_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS24_0`"]
    #[inline(always)]
    pub fn is_lcds24_0(&self) -> bool {
        *self == LCDS24_A::LCDS24_0
    }
    #[doc = "Checks if the value of the field is `LCDS24_1`"]
    #[inline(always)]
    pub fn is_lcds24_1(&self) -> bool {
        *self == LCDS24_A::LCDS24_1
    }
}
#[doc = "Field `LCDS24` writer - LCD pin 24 enable"]
pub type LCDS24_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS24_A, O>;
impl<'a, const O: u8> LCDS24_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds24_0(self) -> &'a mut W {
        self.variant(LCDS24_A::LCDS24_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds24_1(self) -> &'a mut W {
        self.variant(LCDS24_A::LCDS24_1)
    }
}
#[doc = "Field `LCDS25` reader - LCD pin 25 enable"]
pub type LCDS25_R = crate::BitReader<LCDS25_A>;
#[doc = "LCD pin 25 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS25_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS25_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS25_1 = 1,
}
impl From<LCDS25_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS25_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS25_A {
        match self.bits {
            false => LCDS25_A::LCDS25_0,
            true => LCDS25_A::LCDS25_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS25_0`"]
    #[inline(always)]
    pub fn is_lcds25_0(&self) -> bool {
        *self == LCDS25_A::LCDS25_0
    }
    #[doc = "Checks if the value of the field is `LCDS25_1`"]
    #[inline(always)]
    pub fn is_lcds25_1(&self) -> bool {
        *self == LCDS25_A::LCDS25_1
    }
}
#[doc = "Field `LCDS25` writer - LCD pin 25 enable"]
pub type LCDS25_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS25_A, O>;
impl<'a, const O: u8> LCDS25_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds25_0(self) -> &'a mut W {
        self.variant(LCDS25_A::LCDS25_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds25_1(self) -> &'a mut W {
        self.variant(LCDS25_A::LCDS25_1)
    }
}
#[doc = "Field `LCDS26` reader - LCD pin 26 enable"]
pub type LCDS26_R = crate::BitReader<LCDS26_A>;
#[doc = "LCD pin 26 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS26_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS26_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS26_1 = 1,
}
impl From<LCDS26_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS26_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS26_A {
        match self.bits {
            false => LCDS26_A::LCDS26_0,
            true => LCDS26_A::LCDS26_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS26_0`"]
    #[inline(always)]
    pub fn is_lcds26_0(&self) -> bool {
        *self == LCDS26_A::LCDS26_0
    }
    #[doc = "Checks if the value of the field is `LCDS26_1`"]
    #[inline(always)]
    pub fn is_lcds26_1(&self) -> bool {
        *self == LCDS26_A::LCDS26_1
    }
}
#[doc = "Field `LCDS26` writer - LCD pin 26 enable"]
pub type LCDS26_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS26_A, O>;
impl<'a, const O: u8> LCDS26_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds26_0(self) -> &'a mut W {
        self.variant(LCDS26_A::LCDS26_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds26_1(self) -> &'a mut W {
        self.variant(LCDS26_A::LCDS26_1)
    }
}
#[doc = "Field `LCDS27` reader - LCD pin 27 enable"]
pub type LCDS27_R = crate::BitReader<LCDS27_A>;
#[doc = "LCD pin 27 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS27_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS27_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS27_1 = 1,
}
impl From<LCDS27_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS27_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS27_A {
        match self.bits {
            false => LCDS27_A::LCDS27_0,
            true => LCDS27_A::LCDS27_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS27_0`"]
    #[inline(always)]
    pub fn is_lcds27_0(&self) -> bool {
        *self == LCDS27_A::LCDS27_0
    }
    #[doc = "Checks if the value of the field is `LCDS27_1`"]
    #[inline(always)]
    pub fn is_lcds27_1(&self) -> bool {
        *self == LCDS27_A::LCDS27_1
    }
}
#[doc = "Field `LCDS27` writer - LCD pin 27 enable"]
pub type LCDS27_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS27_A, O>;
impl<'a, const O: u8> LCDS27_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds27_0(self) -> &'a mut W {
        self.variant(LCDS27_A::LCDS27_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds27_1(self) -> &'a mut W {
        self.variant(LCDS27_A::LCDS27_1)
    }
}
#[doc = "Field `LCDS28` reader - LCD pin 28 enable"]
pub type LCDS28_R = crate::BitReader<LCDS28_A>;
#[doc = "LCD pin 28 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS28_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS28_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS28_1 = 1,
}
impl From<LCDS28_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS28_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS28_A {
        match self.bits {
            false => LCDS28_A::LCDS28_0,
            true => LCDS28_A::LCDS28_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS28_0`"]
    #[inline(always)]
    pub fn is_lcds28_0(&self) -> bool {
        *self == LCDS28_A::LCDS28_0
    }
    #[doc = "Checks if the value of the field is `LCDS28_1`"]
    #[inline(always)]
    pub fn is_lcds28_1(&self) -> bool {
        *self == LCDS28_A::LCDS28_1
    }
}
#[doc = "Field `LCDS28` writer - LCD pin 28 enable"]
pub type LCDS28_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS28_A, O>;
impl<'a, const O: u8> LCDS28_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds28_0(self) -> &'a mut W {
        self.variant(LCDS28_A::LCDS28_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds28_1(self) -> &'a mut W {
        self.variant(LCDS28_A::LCDS28_1)
    }
}
#[doc = "Field `LCDS29` reader - LCD pin 29 enable"]
pub type LCDS29_R = crate::BitReader<LCDS29_A>;
#[doc = "LCD pin 29 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS29_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS29_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS29_1 = 1,
}
impl From<LCDS29_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS29_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS29_A {
        match self.bits {
            false => LCDS29_A::LCDS29_0,
            true => LCDS29_A::LCDS29_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS29_0`"]
    #[inline(always)]
    pub fn is_lcds29_0(&self) -> bool {
        *self == LCDS29_A::LCDS29_0
    }
    #[doc = "Checks if the value of the field is `LCDS29_1`"]
    #[inline(always)]
    pub fn is_lcds29_1(&self) -> bool {
        *self == LCDS29_A::LCDS29_1
    }
}
#[doc = "Field `LCDS29` writer - LCD pin 29 enable"]
pub type LCDS29_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS29_A, O>;
impl<'a, const O: u8> LCDS29_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds29_0(self) -> &'a mut W {
        self.variant(LCDS29_A::LCDS29_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds29_1(self) -> &'a mut W {
        self.variant(LCDS29_A::LCDS29_1)
    }
}
#[doc = "Field `LCDS30` reader - LCD pin 30 enable"]
pub type LCDS30_R = crate::BitReader<LCDS30_A>;
#[doc = "LCD pin 30 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS30_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS30_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS30_1 = 1,
}
impl From<LCDS30_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS30_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS30_A {
        match self.bits {
            false => LCDS30_A::LCDS30_0,
            true => LCDS30_A::LCDS30_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS30_0`"]
    #[inline(always)]
    pub fn is_lcds30_0(&self) -> bool {
        *self == LCDS30_A::LCDS30_0
    }
    #[doc = "Checks if the value of the field is `LCDS30_1`"]
    #[inline(always)]
    pub fn is_lcds30_1(&self) -> bool {
        *self == LCDS30_A::LCDS30_1
    }
}
#[doc = "Field `LCDS30` writer - LCD pin 30 enable"]
pub type LCDS30_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS30_A, O>;
impl<'a, const O: u8> LCDS30_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds30_0(self) -> &'a mut W {
        self.variant(LCDS30_A::LCDS30_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds30_1(self) -> &'a mut W {
        self.variant(LCDS30_A::LCDS30_1)
    }
}
#[doc = "Field `LCDS31` reader - LCD pin 31 enable"]
pub type LCDS31_R = crate::BitReader<LCDS31_A>;
#[doc = "LCD pin 31 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS31_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS31_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS31_1 = 1,
}
impl From<LCDS31_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS31_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS31_A {
        match self.bits {
            false => LCDS31_A::LCDS31_0,
            true => LCDS31_A::LCDS31_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS31_0`"]
    #[inline(always)]
    pub fn is_lcds31_0(&self) -> bool {
        *self == LCDS31_A::LCDS31_0
    }
    #[doc = "Checks if the value of the field is `LCDS31_1`"]
    #[inline(always)]
    pub fn is_lcds31_1(&self) -> bool {
        *self == LCDS31_A::LCDS31_1
    }
}
#[doc = "Field `LCDS31` writer - LCD pin 31 enable"]
pub type LCDS31_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL0_SPEC, LCDS31_A, O>;
impl<'a, const O: u8> LCDS31_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds31_0(self) -> &'a mut W {
        self.variant(LCDS31_A::LCDS31_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds31_1(self) -> &'a mut W {
        self.variant(LCDS31_A::LCDS31_1)
    }
}
impl R {
    #[doc = "Bit 0 - LCD pin 0 enable"]
    #[inline(always)]
    pub fn lcds0(&self) -> LCDS0_R {
        LCDS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD pin 1 enable"]
    #[inline(always)]
    pub fn lcds1(&self) -> LCDS1_R {
        LCDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD pin 2 enable"]
    #[inline(always)]
    pub fn lcds2(&self) -> LCDS2_R {
        LCDS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD pin 3 enable"]
    #[inline(always)]
    pub fn lcds3(&self) -> LCDS3_R {
        LCDS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD pin 4 enable"]
    #[inline(always)]
    pub fn lcds4(&self) -> LCDS4_R {
        LCDS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD pin 5 enable"]
    #[inline(always)]
    pub fn lcds5(&self) -> LCDS5_R {
        LCDS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD pin 6 enable"]
    #[inline(always)]
    pub fn lcds6(&self) -> LCDS6_R {
        LCDS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD pin 7 enable"]
    #[inline(always)]
    pub fn lcds7(&self) -> LCDS7_R {
        LCDS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD pin 8 enable"]
    #[inline(always)]
    pub fn lcds8(&self) -> LCDS8_R {
        LCDS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD pin 9 enable"]
    #[inline(always)]
    pub fn lcds9(&self) -> LCDS9_R {
        LCDS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD pin 10 enable"]
    #[inline(always)]
    pub fn lcds10(&self) -> LCDS10_R {
        LCDS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCD pin 11 enable"]
    #[inline(always)]
    pub fn lcds11(&self) -> LCDS11_R {
        LCDS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCD pin 12 enable"]
    #[inline(always)]
    pub fn lcds12(&self) -> LCDS12_R {
        LCDS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCD pin 13 enable"]
    #[inline(always)]
    pub fn lcds13(&self) -> LCDS13_R {
        LCDS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCD pin 14 enable"]
    #[inline(always)]
    pub fn lcds14(&self) -> LCDS14_R {
        LCDS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCD pin 15 enable"]
    #[inline(always)]
    pub fn lcds15(&self) -> LCDS15_R {
        LCDS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LCD pin 16 enable"]
    #[inline(always)]
    pub fn lcds16(&self) -> LCDS16_R {
        LCDS16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LCD pin 17 enable"]
    #[inline(always)]
    pub fn lcds17(&self) -> LCDS17_R {
        LCDS17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LCD pin 18 enable"]
    #[inline(always)]
    pub fn lcds18(&self) -> LCDS18_R {
        LCDS18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LCD pin 19 enable"]
    #[inline(always)]
    pub fn lcds19(&self) -> LCDS19_R {
        LCDS19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LCD pin 20 enable"]
    #[inline(always)]
    pub fn lcds20(&self) -> LCDS20_R {
        LCDS20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LCD pin 21 enable"]
    #[inline(always)]
    pub fn lcds21(&self) -> LCDS21_R {
        LCDS21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LCD pin 22 enable"]
    #[inline(always)]
    pub fn lcds22(&self) -> LCDS22_R {
        LCDS22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LCD pin 23 enable"]
    #[inline(always)]
    pub fn lcds23(&self) -> LCDS23_R {
        LCDS23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - LCD pin 24 enable"]
    #[inline(always)]
    pub fn lcds24(&self) -> LCDS24_R {
        LCDS24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LCD pin 25 enable"]
    #[inline(always)]
    pub fn lcds25(&self) -> LCDS25_R {
        LCDS25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LCD pin 26 enable"]
    #[inline(always)]
    pub fn lcds26(&self) -> LCDS26_R {
        LCDS26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LCD pin 27 enable"]
    #[inline(always)]
    pub fn lcds27(&self) -> LCDS27_R {
        LCDS27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LCD pin 28 enable"]
    #[inline(always)]
    pub fn lcds28(&self) -> LCDS28_R {
        LCDS28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LCD pin 29 enable"]
    #[inline(always)]
    pub fn lcds29(&self) -> LCDS29_R {
        LCDS29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LCD pin 30 enable"]
    #[inline(always)]
    pub fn lcds30(&self) -> LCDS30_R {
        LCDS30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LCD pin 31 enable"]
    #[inline(always)]
    pub fn lcds31(&self) -> LCDS31_R {
        LCDS31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD pin 0 enable"]
    #[inline(always)]
    pub fn lcds0(&mut self) -> LCDS0_W<0> {
        LCDS0_W::new(self)
    }
    #[doc = "Bit 1 - LCD pin 1 enable"]
    #[inline(always)]
    pub fn lcds1(&mut self) -> LCDS1_W<1> {
        LCDS1_W::new(self)
    }
    #[doc = "Bit 2 - LCD pin 2 enable"]
    #[inline(always)]
    pub fn lcds2(&mut self) -> LCDS2_W<2> {
        LCDS2_W::new(self)
    }
    #[doc = "Bit 3 - LCD pin 3 enable"]
    #[inline(always)]
    pub fn lcds3(&mut self) -> LCDS3_W<3> {
        LCDS3_W::new(self)
    }
    #[doc = "Bit 4 - LCD pin 4 enable"]
    #[inline(always)]
    pub fn lcds4(&mut self) -> LCDS4_W<4> {
        LCDS4_W::new(self)
    }
    #[doc = "Bit 5 - LCD pin 5 enable"]
    #[inline(always)]
    pub fn lcds5(&mut self) -> LCDS5_W<5> {
        LCDS5_W::new(self)
    }
    #[doc = "Bit 6 - LCD pin 6 enable"]
    #[inline(always)]
    pub fn lcds6(&mut self) -> LCDS6_W<6> {
        LCDS6_W::new(self)
    }
    #[doc = "Bit 7 - LCD pin 7 enable"]
    #[inline(always)]
    pub fn lcds7(&mut self) -> LCDS7_W<7> {
        LCDS7_W::new(self)
    }
    #[doc = "Bit 8 - LCD pin 8 enable"]
    #[inline(always)]
    pub fn lcds8(&mut self) -> LCDS8_W<8> {
        LCDS8_W::new(self)
    }
    #[doc = "Bit 9 - LCD pin 9 enable"]
    #[inline(always)]
    pub fn lcds9(&mut self) -> LCDS9_W<9> {
        LCDS9_W::new(self)
    }
    #[doc = "Bit 10 - LCD pin 10 enable"]
    #[inline(always)]
    pub fn lcds10(&mut self) -> LCDS10_W<10> {
        LCDS10_W::new(self)
    }
    #[doc = "Bit 11 - LCD pin 11 enable"]
    #[inline(always)]
    pub fn lcds11(&mut self) -> LCDS11_W<11> {
        LCDS11_W::new(self)
    }
    #[doc = "Bit 12 - LCD pin 12 enable"]
    #[inline(always)]
    pub fn lcds12(&mut self) -> LCDS12_W<12> {
        LCDS12_W::new(self)
    }
    #[doc = "Bit 13 - LCD pin 13 enable"]
    #[inline(always)]
    pub fn lcds13(&mut self) -> LCDS13_W<13> {
        LCDS13_W::new(self)
    }
    #[doc = "Bit 14 - LCD pin 14 enable"]
    #[inline(always)]
    pub fn lcds14(&mut self) -> LCDS14_W<14> {
        LCDS14_W::new(self)
    }
    #[doc = "Bit 15 - LCD pin 15 enable"]
    #[inline(always)]
    pub fn lcds15(&mut self) -> LCDS15_W<15> {
        LCDS15_W::new(self)
    }
    #[doc = "Bit 16 - LCD pin 16 enable"]
    #[inline(always)]
    pub fn lcds16(&mut self) -> LCDS16_W<16> {
        LCDS16_W::new(self)
    }
    #[doc = "Bit 17 - LCD pin 17 enable"]
    #[inline(always)]
    pub fn lcds17(&mut self) -> LCDS17_W<17> {
        LCDS17_W::new(self)
    }
    #[doc = "Bit 18 - LCD pin 18 enable"]
    #[inline(always)]
    pub fn lcds18(&mut self) -> LCDS18_W<18> {
        LCDS18_W::new(self)
    }
    #[doc = "Bit 19 - LCD pin 19 enable"]
    #[inline(always)]
    pub fn lcds19(&mut self) -> LCDS19_W<19> {
        LCDS19_W::new(self)
    }
    #[doc = "Bit 20 - LCD pin 20 enable"]
    #[inline(always)]
    pub fn lcds20(&mut self) -> LCDS20_W<20> {
        LCDS20_W::new(self)
    }
    #[doc = "Bit 21 - LCD pin 21 enable"]
    #[inline(always)]
    pub fn lcds21(&mut self) -> LCDS21_W<21> {
        LCDS21_W::new(self)
    }
    #[doc = "Bit 22 - LCD pin 22 enable"]
    #[inline(always)]
    pub fn lcds22(&mut self) -> LCDS22_W<22> {
        LCDS22_W::new(self)
    }
    #[doc = "Bit 23 - LCD pin 23 enable"]
    #[inline(always)]
    pub fn lcds23(&mut self) -> LCDS23_W<23> {
        LCDS23_W::new(self)
    }
    #[doc = "Bit 24 - LCD pin 24 enable"]
    #[inline(always)]
    pub fn lcds24(&mut self) -> LCDS24_W<24> {
        LCDS24_W::new(self)
    }
    #[doc = "Bit 25 - LCD pin 25 enable"]
    #[inline(always)]
    pub fn lcds25(&mut self) -> LCDS25_W<25> {
        LCDS25_W::new(self)
    }
    #[doc = "Bit 26 - LCD pin 26 enable"]
    #[inline(always)]
    pub fn lcds26(&mut self) -> LCDS26_W<26> {
        LCDS26_W::new(self)
    }
    #[doc = "Bit 27 - LCD pin 27 enable"]
    #[inline(always)]
    pub fn lcds27(&mut self) -> LCDS27_W<27> {
        LCDS27_W::new(self)
    }
    #[doc = "Bit 28 - LCD pin 28 enable"]
    #[inline(always)]
    pub fn lcds28(&mut self) -> LCDS28_W<28> {
        LCDS28_W::new(self)
    }
    #[doc = "Bit 29 - LCD pin 29 enable"]
    #[inline(always)]
    pub fn lcds29(&mut self) -> LCDS29_W<29> {
        LCDS29_W::new(self)
    }
    #[doc = "Bit 30 - LCD pin 30 enable"]
    #[inline(always)]
    pub fn lcds30(&mut self) -> LCDS30_W<30> {
        LCDS30_W::new(self)
    }
    #[doc = "Bit 31 - LCD pin 31 enable"]
    #[inline(always)]
    pub fn lcds31(&mut self) -> LCDS31_W<31> {
        LCDS31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_F port control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdpctl0](index.html) module"]
pub struct LCDPCTL0_SPEC;
impl crate::RegisterSpec for LCDPCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdpctl0::R](R) reader structure"]
impl crate::Readable for LCDPCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdpctl0::W](W) writer structure"]
impl crate::Writable for LCDPCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDPCTL0 to value 0"]
impl crate::Resettable for LCDPCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
