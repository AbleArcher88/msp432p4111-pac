#[doc = "Register `SYS_SRAM_BANKEN_CTL0` reader"]
pub struct R(crate::R<SYS_SRAM_BANKEN_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BANKEN_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_BANKEN_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_BANKEN_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_BANKEN_CTL0` writer"]
pub struct W(crate::W<SYS_SRAM_BANKEN_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_BANKEN_CTL0_SPEC>;
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
impl From<crate::W<SYS_SRAM_BANKEN_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SRAM_BANKEN_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNK0_EN` reader - When 1, enables Bank0 of the SRAM"]
pub type BNK0_EN_R = crate::BitReader<bool>;
#[doc = "Field `BNK1_EN` reader - When 1, enables Bank1 of the SRAM"]
pub type BNK1_EN_R = crate::BitReader<BNK1_EN_A>;
#[doc = "When 1, enables Bank1 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK1_EN_A {
    #[doc = "0: Disables Bank1 of the SRAM"]
    BNK1_EN_0 = 0,
    #[doc = "1: Enables Bank1 of the SRAM"]
    BNK1_EN_1 = 1,
}
impl From<BNK1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK1_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK1_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK1_EN_A {
        match self.bits {
            false => BNK1_EN_A::BNK1_EN_0,
            true => BNK1_EN_A::BNK1_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK1_EN_0`"]
    #[inline(always)]
    pub fn is_bnk1_en_0(&self) -> bool {
        *self == BNK1_EN_A::BNK1_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK1_EN_1`"]
    #[inline(always)]
    pub fn is_bnk1_en_1(&self) -> bool {
        *self == BNK1_EN_A::BNK1_EN_1
    }
}
#[doc = "Field `BNK1_EN` writer - When 1, enables Bank1 of the SRAM"]
pub type BNK1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK1_EN_A, O>;
impl<'a, const O: u8> BNK1_EN_W<'a, O> {
    #[doc = "Disables Bank1 of the SRAM"]
    #[inline(always)]
    pub fn bnk1_en_0(self) -> &'a mut W {
        self.variant(BNK1_EN_A::BNK1_EN_0)
    }
    #[doc = "Enables Bank1 of the SRAM"]
    #[inline(always)]
    pub fn bnk1_en_1(self) -> &'a mut W {
        self.variant(BNK1_EN_A::BNK1_EN_1)
    }
}
#[doc = "Field `BNK2_EN` reader - When 1, enables Bank2 of the SRAM"]
pub type BNK2_EN_R = crate::BitReader<BNK2_EN_A>;
#[doc = "When 1, enables Bank2 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK2_EN_A {
    #[doc = "0: Disables Bank2 of the SRAM"]
    BNK2_EN_0 = 0,
    #[doc = "1: Enables Bank2 of the SRAM"]
    BNK2_EN_1 = 1,
}
impl From<BNK2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK2_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK2_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK2_EN_A {
        match self.bits {
            false => BNK2_EN_A::BNK2_EN_0,
            true => BNK2_EN_A::BNK2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK2_EN_0`"]
    #[inline(always)]
    pub fn is_bnk2_en_0(&self) -> bool {
        *self == BNK2_EN_A::BNK2_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK2_EN_1`"]
    #[inline(always)]
    pub fn is_bnk2_en_1(&self) -> bool {
        *self == BNK2_EN_A::BNK2_EN_1
    }
}
#[doc = "Field `BNK2_EN` writer - When 1, enables Bank2 of the SRAM"]
pub type BNK2_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK2_EN_A, O>;
impl<'a, const O: u8> BNK2_EN_W<'a, O> {
    #[doc = "Disables Bank2 of the SRAM"]
    #[inline(always)]
    pub fn bnk2_en_0(self) -> &'a mut W {
        self.variant(BNK2_EN_A::BNK2_EN_0)
    }
    #[doc = "Enables Bank2 of the SRAM"]
    #[inline(always)]
    pub fn bnk2_en_1(self) -> &'a mut W {
        self.variant(BNK2_EN_A::BNK2_EN_1)
    }
}
#[doc = "Field `BNK3_EN` reader - When 1, enables Bank3 of the SRAM"]
pub type BNK3_EN_R = crate::BitReader<BNK3_EN_A>;
#[doc = "When 1, enables Bank3 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK3_EN_A {
    #[doc = "0: Disables Bank3 of the SRAM"]
    BNK3_EN_0 = 0,
    #[doc = "1: Enables Bank3 of the SRAM"]
    BNK3_EN_1 = 1,
}
impl From<BNK3_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK3_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK3_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK3_EN_A {
        match self.bits {
            false => BNK3_EN_A::BNK3_EN_0,
            true => BNK3_EN_A::BNK3_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK3_EN_0`"]
    #[inline(always)]
    pub fn is_bnk3_en_0(&self) -> bool {
        *self == BNK3_EN_A::BNK3_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK3_EN_1`"]
    #[inline(always)]
    pub fn is_bnk3_en_1(&self) -> bool {
        *self == BNK3_EN_A::BNK3_EN_1
    }
}
#[doc = "Field `BNK3_EN` writer - When 1, enables Bank3 of the SRAM"]
pub type BNK3_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK3_EN_A, O>;
impl<'a, const O: u8> BNK3_EN_W<'a, O> {
    #[doc = "Disables Bank3 of the SRAM"]
    #[inline(always)]
    pub fn bnk3_en_0(self) -> &'a mut W {
        self.variant(BNK3_EN_A::BNK3_EN_0)
    }
    #[doc = "Enables Bank3 of the SRAM"]
    #[inline(always)]
    pub fn bnk3_en_1(self) -> &'a mut W {
        self.variant(BNK3_EN_A::BNK3_EN_1)
    }
}
#[doc = "Field `BNK4_EN` reader - When 1, enables Bank4 of the SRAM"]
pub type BNK4_EN_R = crate::BitReader<BNK4_EN_A>;
#[doc = "When 1, enables Bank4 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK4_EN_A {
    #[doc = "0: Disables Bank4 of the SRAM"]
    BNK4_EN_0 = 0,
    #[doc = "1: Enables Bank4 of the SRAM"]
    BNK4_EN_1 = 1,
}
impl From<BNK4_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK4_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK4_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK4_EN_A {
        match self.bits {
            false => BNK4_EN_A::BNK4_EN_0,
            true => BNK4_EN_A::BNK4_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK4_EN_0`"]
    #[inline(always)]
    pub fn is_bnk4_en_0(&self) -> bool {
        *self == BNK4_EN_A::BNK4_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK4_EN_1`"]
    #[inline(always)]
    pub fn is_bnk4_en_1(&self) -> bool {
        *self == BNK4_EN_A::BNK4_EN_1
    }
}
#[doc = "Field `BNK4_EN` writer - When 1, enables Bank4 of the SRAM"]
pub type BNK4_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK4_EN_A, O>;
impl<'a, const O: u8> BNK4_EN_W<'a, O> {
    #[doc = "Disables Bank4 of the SRAM"]
    #[inline(always)]
    pub fn bnk4_en_0(self) -> &'a mut W {
        self.variant(BNK4_EN_A::BNK4_EN_0)
    }
    #[doc = "Enables Bank4 of the SRAM"]
    #[inline(always)]
    pub fn bnk4_en_1(self) -> &'a mut W {
        self.variant(BNK4_EN_A::BNK4_EN_1)
    }
}
#[doc = "Field `BNK5_EN` reader - When 1, enables Bank5 of the SRAM"]
pub type BNK5_EN_R = crate::BitReader<BNK5_EN_A>;
#[doc = "When 1, enables Bank5 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK5_EN_A {
    #[doc = "0: Disables Bank5 of the SRAM"]
    BNK5_EN_0 = 0,
    #[doc = "1: Enables Bank5 of the SRAM"]
    BNK5_EN_1 = 1,
}
impl From<BNK5_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK5_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK5_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK5_EN_A {
        match self.bits {
            false => BNK5_EN_A::BNK5_EN_0,
            true => BNK5_EN_A::BNK5_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK5_EN_0`"]
    #[inline(always)]
    pub fn is_bnk5_en_0(&self) -> bool {
        *self == BNK5_EN_A::BNK5_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK5_EN_1`"]
    #[inline(always)]
    pub fn is_bnk5_en_1(&self) -> bool {
        *self == BNK5_EN_A::BNK5_EN_1
    }
}
#[doc = "Field `BNK5_EN` writer - When 1, enables Bank5 of the SRAM"]
pub type BNK5_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK5_EN_A, O>;
impl<'a, const O: u8> BNK5_EN_W<'a, O> {
    #[doc = "Disables Bank5 of the SRAM"]
    #[inline(always)]
    pub fn bnk5_en_0(self) -> &'a mut W {
        self.variant(BNK5_EN_A::BNK5_EN_0)
    }
    #[doc = "Enables Bank5 of the SRAM"]
    #[inline(always)]
    pub fn bnk5_en_1(self) -> &'a mut W {
        self.variant(BNK5_EN_A::BNK5_EN_1)
    }
}
#[doc = "Field `BNK6_EN` reader - When 1, enables Bank6 of the SRAM"]
pub type BNK6_EN_R = crate::BitReader<BNK6_EN_A>;
#[doc = "When 1, enables Bank6 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK6_EN_A {
    #[doc = "0: Disables Bank6 of the SRAM"]
    BNK6_EN_0 = 0,
    #[doc = "1: Enables Bank6 of the SRAM"]
    BNK6_EN_1 = 1,
}
impl From<BNK6_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK6_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK6_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK6_EN_A {
        match self.bits {
            false => BNK6_EN_A::BNK6_EN_0,
            true => BNK6_EN_A::BNK6_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK6_EN_0`"]
    #[inline(always)]
    pub fn is_bnk6_en_0(&self) -> bool {
        *self == BNK6_EN_A::BNK6_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK6_EN_1`"]
    #[inline(always)]
    pub fn is_bnk6_en_1(&self) -> bool {
        *self == BNK6_EN_A::BNK6_EN_1
    }
}
#[doc = "Field `BNK6_EN` writer - When 1, enables Bank6 of the SRAM"]
pub type BNK6_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK6_EN_A, O>;
impl<'a, const O: u8> BNK6_EN_W<'a, O> {
    #[doc = "Disables Bank6 of the SRAM"]
    #[inline(always)]
    pub fn bnk6_en_0(self) -> &'a mut W {
        self.variant(BNK6_EN_A::BNK6_EN_0)
    }
    #[doc = "Enables Bank6 of the SRAM"]
    #[inline(always)]
    pub fn bnk6_en_1(self) -> &'a mut W {
        self.variant(BNK6_EN_A::BNK6_EN_1)
    }
}
#[doc = "Field `BNK7_EN` reader - When 1, enables Bank7 of the SRAM"]
pub type BNK7_EN_R = crate::BitReader<BNK7_EN_A>;
#[doc = "When 1, enables Bank7 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK7_EN_A {
    #[doc = "0: Disables Bank7 of the SRAM"]
    BNK7_EN_0 = 0,
    #[doc = "1: Enables Bank7 of the SRAM"]
    BNK7_EN_1 = 1,
}
impl From<BNK7_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK7_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK7_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK7_EN_A {
        match self.bits {
            false => BNK7_EN_A::BNK7_EN_0,
            true => BNK7_EN_A::BNK7_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK7_EN_0`"]
    #[inline(always)]
    pub fn is_bnk7_en_0(&self) -> bool {
        *self == BNK7_EN_A::BNK7_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK7_EN_1`"]
    #[inline(always)]
    pub fn is_bnk7_en_1(&self) -> bool {
        *self == BNK7_EN_A::BNK7_EN_1
    }
}
#[doc = "Field `BNK7_EN` writer - When 1, enables Bank7 of the SRAM"]
pub type BNK7_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK7_EN_A, O>;
impl<'a, const O: u8> BNK7_EN_W<'a, O> {
    #[doc = "Disables Bank7 of the SRAM"]
    #[inline(always)]
    pub fn bnk7_en_0(self) -> &'a mut W {
        self.variant(BNK7_EN_A::BNK7_EN_0)
    }
    #[doc = "Enables Bank7 of the SRAM"]
    #[inline(always)]
    pub fn bnk7_en_1(self) -> &'a mut W {
        self.variant(BNK7_EN_A::BNK7_EN_1)
    }
}
#[doc = "Field `BNK8_EN` reader - When 1, enables Bank8 of the SRAM"]
pub type BNK8_EN_R = crate::BitReader<BNK8_EN_A>;
#[doc = "When 1, enables Bank8 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK8_EN_A {
    #[doc = "0: Disables Bank8 of the SRAM"]
    BNK8_EN_0 = 0,
    #[doc = "1: Enables Bank8 of the SRAM"]
    BNK8_EN_1 = 1,
}
impl From<BNK8_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK8_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK8_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK8_EN_A {
        match self.bits {
            false => BNK8_EN_A::BNK8_EN_0,
            true => BNK8_EN_A::BNK8_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK8_EN_0`"]
    #[inline(always)]
    pub fn is_bnk8_en_0(&self) -> bool {
        *self == BNK8_EN_A::BNK8_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK8_EN_1`"]
    #[inline(always)]
    pub fn is_bnk8_en_1(&self) -> bool {
        *self == BNK8_EN_A::BNK8_EN_1
    }
}
#[doc = "Field `BNK8_EN` writer - When 1, enables Bank8 of the SRAM"]
pub type BNK8_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK8_EN_A, O>;
impl<'a, const O: u8> BNK8_EN_W<'a, O> {
    #[doc = "Disables Bank8 of the SRAM"]
    #[inline(always)]
    pub fn bnk8_en_0(self) -> &'a mut W {
        self.variant(BNK8_EN_A::BNK8_EN_0)
    }
    #[doc = "Enables Bank8 of the SRAM"]
    #[inline(always)]
    pub fn bnk8_en_1(self) -> &'a mut W {
        self.variant(BNK8_EN_A::BNK8_EN_1)
    }
}
#[doc = "Field `BNK9_EN` reader - When 1, enables Bank9 of the SRAM"]
pub type BNK9_EN_R = crate::BitReader<BNK9_EN_A>;
#[doc = "When 1, enables Bank9 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK9_EN_A {
    #[doc = "0: Disables Bank9 of the SRAM"]
    BNK9_EN_0 = 0,
    #[doc = "1: Enables Bank9 of the SRAM"]
    BNK9_EN_1 = 1,
}
impl From<BNK9_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK9_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK9_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK9_EN_A {
        match self.bits {
            false => BNK9_EN_A::BNK9_EN_0,
            true => BNK9_EN_A::BNK9_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK9_EN_0`"]
    #[inline(always)]
    pub fn is_bnk9_en_0(&self) -> bool {
        *self == BNK9_EN_A::BNK9_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK9_EN_1`"]
    #[inline(always)]
    pub fn is_bnk9_en_1(&self) -> bool {
        *self == BNK9_EN_A::BNK9_EN_1
    }
}
#[doc = "Field `BNK9_EN` writer - When 1, enables Bank9 of the SRAM"]
pub type BNK9_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK9_EN_A, O>;
impl<'a, const O: u8> BNK9_EN_W<'a, O> {
    #[doc = "Disables Bank9 of the SRAM"]
    #[inline(always)]
    pub fn bnk9_en_0(self) -> &'a mut W {
        self.variant(BNK9_EN_A::BNK9_EN_0)
    }
    #[doc = "Enables Bank9 of the SRAM"]
    #[inline(always)]
    pub fn bnk9_en_1(self) -> &'a mut W {
        self.variant(BNK9_EN_A::BNK9_EN_1)
    }
}
#[doc = "Field `BNK10_EN` reader - When 1, enables Bank10 of the SRAM"]
pub type BNK10_EN_R = crate::BitReader<BNK10_EN_A>;
#[doc = "When 1, enables Bank10 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK10_EN_A {
    #[doc = "0: Disables Bank10 of the SRAM"]
    BNK10_EN_0 = 0,
    #[doc = "1: Enables Bank10 of the SRAM"]
    BNK10_EN_1 = 1,
}
impl From<BNK10_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK10_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK10_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK10_EN_A {
        match self.bits {
            false => BNK10_EN_A::BNK10_EN_0,
            true => BNK10_EN_A::BNK10_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK10_EN_0`"]
    #[inline(always)]
    pub fn is_bnk10_en_0(&self) -> bool {
        *self == BNK10_EN_A::BNK10_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK10_EN_1`"]
    #[inline(always)]
    pub fn is_bnk10_en_1(&self) -> bool {
        *self == BNK10_EN_A::BNK10_EN_1
    }
}
#[doc = "Field `BNK10_EN` writer - When 1, enables Bank10 of the SRAM"]
pub type BNK10_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK10_EN_A, O>;
impl<'a, const O: u8> BNK10_EN_W<'a, O> {
    #[doc = "Disables Bank10 of the SRAM"]
    #[inline(always)]
    pub fn bnk10_en_0(self) -> &'a mut W {
        self.variant(BNK10_EN_A::BNK10_EN_0)
    }
    #[doc = "Enables Bank10 of the SRAM"]
    #[inline(always)]
    pub fn bnk10_en_1(self) -> &'a mut W {
        self.variant(BNK10_EN_A::BNK10_EN_1)
    }
}
#[doc = "Field `BNK11_EN` reader - When 1, enables Bank11 of the SRAM"]
pub type BNK11_EN_R = crate::BitReader<BNK11_EN_A>;
#[doc = "When 1, enables Bank11 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK11_EN_A {
    #[doc = "0: Disables Bank11 of the SRAM"]
    BNK11_EN_0 = 0,
    #[doc = "1: Enables Bank11 of the SRAM"]
    BNK11_EN_1 = 1,
}
impl From<BNK11_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK11_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK11_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK11_EN_A {
        match self.bits {
            false => BNK11_EN_A::BNK11_EN_0,
            true => BNK11_EN_A::BNK11_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK11_EN_0`"]
    #[inline(always)]
    pub fn is_bnk11_en_0(&self) -> bool {
        *self == BNK11_EN_A::BNK11_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK11_EN_1`"]
    #[inline(always)]
    pub fn is_bnk11_en_1(&self) -> bool {
        *self == BNK11_EN_A::BNK11_EN_1
    }
}
#[doc = "Field `BNK11_EN` writer - When 1, enables Bank11 of the SRAM"]
pub type BNK11_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK11_EN_A, O>;
impl<'a, const O: u8> BNK11_EN_W<'a, O> {
    #[doc = "Disables Bank11 of the SRAM"]
    #[inline(always)]
    pub fn bnk11_en_0(self) -> &'a mut W {
        self.variant(BNK11_EN_A::BNK11_EN_0)
    }
    #[doc = "Enables Bank11 of the SRAM"]
    #[inline(always)]
    pub fn bnk11_en_1(self) -> &'a mut W {
        self.variant(BNK11_EN_A::BNK11_EN_1)
    }
}
#[doc = "Field `BNK12_EN` reader - When 1, enables Bank12 of the SRAM"]
pub type BNK12_EN_R = crate::BitReader<BNK12_EN_A>;
#[doc = "When 1, enables Bank12 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK12_EN_A {
    #[doc = "0: Disables Bank12 of the SRAM"]
    BNK12_EN_0 = 0,
    #[doc = "1: Enables Bank12 of the SRAM"]
    BNK12_EN_1 = 1,
}
impl From<BNK12_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK12_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK12_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK12_EN_A {
        match self.bits {
            false => BNK12_EN_A::BNK12_EN_0,
            true => BNK12_EN_A::BNK12_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK12_EN_0`"]
    #[inline(always)]
    pub fn is_bnk12_en_0(&self) -> bool {
        *self == BNK12_EN_A::BNK12_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK12_EN_1`"]
    #[inline(always)]
    pub fn is_bnk12_en_1(&self) -> bool {
        *self == BNK12_EN_A::BNK12_EN_1
    }
}
#[doc = "Field `BNK12_EN` writer - When 1, enables Bank12 of the SRAM"]
pub type BNK12_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK12_EN_A, O>;
impl<'a, const O: u8> BNK12_EN_W<'a, O> {
    #[doc = "Disables Bank12 of the SRAM"]
    #[inline(always)]
    pub fn bnk12_en_0(self) -> &'a mut W {
        self.variant(BNK12_EN_A::BNK12_EN_0)
    }
    #[doc = "Enables Bank12 of the SRAM"]
    #[inline(always)]
    pub fn bnk12_en_1(self) -> &'a mut W {
        self.variant(BNK12_EN_A::BNK12_EN_1)
    }
}
#[doc = "Field `BNK13_EN` reader - When 1, enables Bank13 of the SRAM"]
pub type BNK13_EN_R = crate::BitReader<BNK13_EN_A>;
#[doc = "When 1, enables Bank13 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK13_EN_A {
    #[doc = "0: Disables Bank13 of the SRAM"]
    BNK13_EN_0 = 0,
    #[doc = "1: Enables Bank13 of the SRAM"]
    BNK13_EN_1 = 1,
}
impl From<BNK13_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK13_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK13_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK13_EN_A {
        match self.bits {
            false => BNK13_EN_A::BNK13_EN_0,
            true => BNK13_EN_A::BNK13_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK13_EN_0`"]
    #[inline(always)]
    pub fn is_bnk13_en_0(&self) -> bool {
        *self == BNK13_EN_A::BNK13_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK13_EN_1`"]
    #[inline(always)]
    pub fn is_bnk13_en_1(&self) -> bool {
        *self == BNK13_EN_A::BNK13_EN_1
    }
}
#[doc = "Field `BNK13_EN` writer - When 1, enables Bank13 of the SRAM"]
pub type BNK13_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK13_EN_A, O>;
impl<'a, const O: u8> BNK13_EN_W<'a, O> {
    #[doc = "Disables Bank13 of the SRAM"]
    #[inline(always)]
    pub fn bnk13_en_0(self) -> &'a mut W {
        self.variant(BNK13_EN_A::BNK13_EN_0)
    }
    #[doc = "Enables Bank13 of the SRAM"]
    #[inline(always)]
    pub fn bnk13_en_1(self) -> &'a mut W {
        self.variant(BNK13_EN_A::BNK13_EN_1)
    }
}
#[doc = "Field `BNK14_EN` reader - When 1, enables Bank14 of the SRAM"]
pub type BNK14_EN_R = crate::BitReader<BNK14_EN_A>;
#[doc = "When 1, enables Bank14 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK14_EN_A {
    #[doc = "0: Disables Bank14 of the SRAM"]
    BNK14_EN_0 = 0,
    #[doc = "1: Enables Bank14 of the SRAM"]
    BNK14_EN_1 = 1,
}
impl From<BNK14_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK14_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK14_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK14_EN_A {
        match self.bits {
            false => BNK14_EN_A::BNK14_EN_0,
            true => BNK14_EN_A::BNK14_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK14_EN_0`"]
    #[inline(always)]
    pub fn is_bnk14_en_0(&self) -> bool {
        *self == BNK14_EN_A::BNK14_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK14_EN_1`"]
    #[inline(always)]
    pub fn is_bnk14_en_1(&self) -> bool {
        *self == BNK14_EN_A::BNK14_EN_1
    }
}
#[doc = "Field `BNK14_EN` writer - When 1, enables Bank14 of the SRAM"]
pub type BNK14_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK14_EN_A, O>;
impl<'a, const O: u8> BNK14_EN_W<'a, O> {
    #[doc = "Disables Bank14 of the SRAM"]
    #[inline(always)]
    pub fn bnk14_en_0(self) -> &'a mut W {
        self.variant(BNK14_EN_A::BNK14_EN_0)
    }
    #[doc = "Enables Bank14 of the SRAM"]
    #[inline(always)]
    pub fn bnk14_en_1(self) -> &'a mut W {
        self.variant(BNK14_EN_A::BNK14_EN_1)
    }
}
#[doc = "Field `BNK15_EN` reader - When 1, enables Bank15 of the SRAM"]
pub type BNK15_EN_R = crate::BitReader<BNK15_EN_A>;
#[doc = "When 1, enables Bank15 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK15_EN_A {
    #[doc = "0: Disables Bank15 of the SRAM"]
    BNK15_EN_0 = 0,
    #[doc = "1: Enables Bank15 of the SRAM"]
    BNK15_EN_1 = 1,
}
impl From<BNK15_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK15_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK15_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK15_EN_A {
        match self.bits {
            false => BNK15_EN_A::BNK15_EN_0,
            true => BNK15_EN_A::BNK15_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK15_EN_0`"]
    #[inline(always)]
    pub fn is_bnk15_en_0(&self) -> bool {
        *self == BNK15_EN_A::BNK15_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK15_EN_1`"]
    #[inline(always)]
    pub fn is_bnk15_en_1(&self) -> bool {
        *self == BNK15_EN_A::BNK15_EN_1
    }
}
#[doc = "Field `BNK15_EN` writer - When 1, enables Bank15 of the SRAM"]
pub type BNK15_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK15_EN_A, O>;
impl<'a, const O: u8> BNK15_EN_W<'a, O> {
    #[doc = "Disables Bank15 of the SRAM"]
    #[inline(always)]
    pub fn bnk15_en_0(self) -> &'a mut W {
        self.variant(BNK15_EN_A::BNK15_EN_0)
    }
    #[doc = "Enables Bank15 of the SRAM"]
    #[inline(always)]
    pub fn bnk15_en_1(self) -> &'a mut W {
        self.variant(BNK15_EN_A::BNK15_EN_1)
    }
}
#[doc = "Field `BNK16_EN` reader - When 1, enables Bank16 of the SRAM"]
pub type BNK16_EN_R = crate::BitReader<BNK16_EN_A>;
#[doc = "When 1, enables Bank16 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK16_EN_A {
    #[doc = "0: Disables Bank16 of the SRAM"]
    BNK16_EN_0 = 0,
    #[doc = "1: Enables Bank16 of the SRAM"]
    BNK16_EN_1 = 1,
}
impl From<BNK16_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK16_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK16_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK16_EN_A {
        match self.bits {
            false => BNK16_EN_A::BNK16_EN_0,
            true => BNK16_EN_A::BNK16_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK16_EN_0`"]
    #[inline(always)]
    pub fn is_bnk16_en_0(&self) -> bool {
        *self == BNK16_EN_A::BNK16_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK16_EN_1`"]
    #[inline(always)]
    pub fn is_bnk16_en_1(&self) -> bool {
        *self == BNK16_EN_A::BNK16_EN_1
    }
}
#[doc = "Field `BNK16_EN` writer - When 1, enables Bank16 of the SRAM"]
pub type BNK16_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK16_EN_A, O>;
impl<'a, const O: u8> BNK16_EN_W<'a, O> {
    #[doc = "Disables Bank16 of the SRAM"]
    #[inline(always)]
    pub fn bnk16_en_0(self) -> &'a mut W {
        self.variant(BNK16_EN_A::BNK16_EN_0)
    }
    #[doc = "Enables Bank16 of the SRAM"]
    #[inline(always)]
    pub fn bnk16_en_1(self) -> &'a mut W {
        self.variant(BNK16_EN_A::BNK16_EN_1)
    }
}
#[doc = "Field `BNK17_EN` reader - When 1, enables Bank17 of the SRAM"]
pub type BNK17_EN_R = crate::BitReader<BNK17_EN_A>;
#[doc = "When 1, enables Bank17 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK17_EN_A {
    #[doc = "0: Disables Bank17 of the SRAM"]
    BNK17_EN_0 = 0,
    #[doc = "1: Enables Bank17 of the SRAM"]
    BNK17_EN_1 = 1,
}
impl From<BNK17_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK17_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK17_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK17_EN_A {
        match self.bits {
            false => BNK17_EN_A::BNK17_EN_0,
            true => BNK17_EN_A::BNK17_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK17_EN_0`"]
    #[inline(always)]
    pub fn is_bnk17_en_0(&self) -> bool {
        *self == BNK17_EN_A::BNK17_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK17_EN_1`"]
    #[inline(always)]
    pub fn is_bnk17_en_1(&self) -> bool {
        *self == BNK17_EN_A::BNK17_EN_1
    }
}
#[doc = "Field `BNK17_EN` writer - When 1, enables Bank17 of the SRAM"]
pub type BNK17_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK17_EN_A, O>;
impl<'a, const O: u8> BNK17_EN_W<'a, O> {
    #[doc = "Disables Bank17 of the SRAM"]
    #[inline(always)]
    pub fn bnk17_en_0(self) -> &'a mut W {
        self.variant(BNK17_EN_A::BNK17_EN_0)
    }
    #[doc = "Enables Bank17 of the SRAM"]
    #[inline(always)]
    pub fn bnk17_en_1(self) -> &'a mut W {
        self.variant(BNK17_EN_A::BNK17_EN_1)
    }
}
#[doc = "Field `BNK18_EN` reader - When 1, enables Bank18 of the SRAM"]
pub type BNK18_EN_R = crate::BitReader<BNK18_EN_A>;
#[doc = "When 1, enables Bank18 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK18_EN_A {
    #[doc = "0: Disables Bank18 of the SRAM"]
    BNK18_EN_0 = 0,
    #[doc = "1: Enables Bank18 of the SRAM"]
    BNK18_EN_1 = 1,
}
impl From<BNK18_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK18_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK18_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK18_EN_A {
        match self.bits {
            false => BNK18_EN_A::BNK18_EN_0,
            true => BNK18_EN_A::BNK18_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK18_EN_0`"]
    #[inline(always)]
    pub fn is_bnk18_en_0(&self) -> bool {
        *self == BNK18_EN_A::BNK18_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK18_EN_1`"]
    #[inline(always)]
    pub fn is_bnk18_en_1(&self) -> bool {
        *self == BNK18_EN_A::BNK18_EN_1
    }
}
#[doc = "Field `BNK18_EN` writer - When 1, enables Bank18 of the SRAM"]
pub type BNK18_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK18_EN_A, O>;
impl<'a, const O: u8> BNK18_EN_W<'a, O> {
    #[doc = "Disables Bank18 of the SRAM"]
    #[inline(always)]
    pub fn bnk18_en_0(self) -> &'a mut W {
        self.variant(BNK18_EN_A::BNK18_EN_0)
    }
    #[doc = "Enables Bank18 of the SRAM"]
    #[inline(always)]
    pub fn bnk18_en_1(self) -> &'a mut W {
        self.variant(BNK18_EN_A::BNK18_EN_1)
    }
}
#[doc = "Field `BNK19_EN` reader - When 1, enables Bank19 of the SRAM"]
pub type BNK19_EN_R = crate::BitReader<BNK19_EN_A>;
#[doc = "When 1, enables Bank19 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK19_EN_A {
    #[doc = "0: Disables Bank19 of the SRAM"]
    BNK19_EN_0 = 0,
    #[doc = "1: Enables Bank19 of the SRAM"]
    BNK19_EN_1 = 1,
}
impl From<BNK19_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK19_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK19_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK19_EN_A {
        match self.bits {
            false => BNK19_EN_A::BNK19_EN_0,
            true => BNK19_EN_A::BNK19_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK19_EN_0`"]
    #[inline(always)]
    pub fn is_bnk19_en_0(&self) -> bool {
        *self == BNK19_EN_A::BNK19_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK19_EN_1`"]
    #[inline(always)]
    pub fn is_bnk19_en_1(&self) -> bool {
        *self == BNK19_EN_A::BNK19_EN_1
    }
}
#[doc = "Field `BNK19_EN` writer - When 1, enables Bank19 of the SRAM"]
pub type BNK19_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK19_EN_A, O>;
impl<'a, const O: u8> BNK19_EN_W<'a, O> {
    #[doc = "Disables Bank19 of the SRAM"]
    #[inline(always)]
    pub fn bnk19_en_0(self) -> &'a mut W {
        self.variant(BNK19_EN_A::BNK19_EN_0)
    }
    #[doc = "Enables Bank19 of the SRAM"]
    #[inline(always)]
    pub fn bnk19_en_1(self) -> &'a mut W {
        self.variant(BNK19_EN_A::BNK19_EN_1)
    }
}
#[doc = "Field `BNK20_EN` reader - When 1, enables Bank20 of the SRAM"]
pub type BNK20_EN_R = crate::BitReader<BNK20_EN_A>;
#[doc = "When 1, enables Bank20 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK20_EN_A {
    #[doc = "0: Disables Bank20 of the SRAM"]
    BNK20_EN_0 = 0,
    #[doc = "1: Enables Bank20 of the SRAM"]
    BNK20_EN_1 = 1,
}
impl From<BNK20_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK20_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK20_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK20_EN_A {
        match self.bits {
            false => BNK20_EN_A::BNK20_EN_0,
            true => BNK20_EN_A::BNK20_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK20_EN_0`"]
    #[inline(always)]
    pub fn is_bnk20_en_0(&self) -> bool {
        *self == BNK20_EN_A::BNK20_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK20_EN_1`"]
    #[inline(always)]
    pub fn is_bnk20_en_1(&self) -> bool {
        *self == BNK20_EN_A::BNK20_EN_1
    }
}
#[doc = "Field `BNK20_EN` writer - When 1, enables Bank20 of the SRAM"]
pub type BNK20_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK20_EN_A, O>;
impl<'a, const O: u8> BNK20_EN_W<'a, O> {
    #[doc = "Disables Bank20 of the SRAM"]
    #[inline(always)]
    pub fn bnk20_en_0(self) -> &'a mut W {
        self.variant(BNK20_EN_A::BNK20_EN_0)
    }
    #[doc = "Enables Bank20 of the SRAM"]
    #[inline(always)]
    pub fn bnk20_en_1(self) -> &'a mut W {
        self.variant(BNK20_EN_A::BNK20_EN_1)
    }
}
#[doc = "Field `BNK21_EN` reader - When 1, enables Bank21 of the SRAM"]
pub type BNK21_EN_R = crate::BitReader<BNK21_EN_A>;
#[doc = "When 1, enables Bank21 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK21_EN_A {
    #[doc = "0: Disables Bank21 of the SRAM"]
    BNK21_EN_0 = 0,
    #[doc = "1: Enables Bank21 of the SRAM"]
    BNK21_EN_1 = 1,
}
impl From<BNK21_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK21_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK21_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK21_EN_A {
        match self.bits {
            false => BNK21_EN_A::BNK21_EN_0,
            true => BNK21_EN_A::BNK21_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK21_EN_0`"]
    #[inline(always)]
    pub fn is_bnk21_en_0(&self) -> bool {
        *self == BNK21_EN_A::BNK21_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK21_EN_1`"]
    #[inline(always)]
    pub fn is_bnk21_en_1(&self) -> bool {
        *self == BNK21_EN_A::BNK21_EN_1
    }
}
#[doc = "Field `BNK21_EN` writer - When 1, enables Bank21 of the SRAM"]
pub type BNK21_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK21_EN_A, O>;
impl<'a, const O: u8> BNK21_EN_W<'a, O> {
    #[doc = "Disables Bank21 of the SRAM"]
    #[inline(always)]
    pub fn bnk21_en_0(self) -> &'a mut W {
        self.variant(BNK21_EN_A::BNK21_EN_0)
    }
    #[doc = "Enables Bank21 of the SRAM"]
    #[inline(always)]
    pub fn bnk21_en_1(self) -> &'a mut W {
        self.variant(BNK21_EN_A::BNK21_EN_1)
    }
}
#[doc = "Field `BNK22_EN` reader - When 1, enables Bank22 of the SRAM"]
pub type BNK22_EN_R = crate::BitReader<BNK22_EN_A>;
#[doc = "When 1, enables Bank22 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK22_EN_A {
    #[doc = "0: Disables Bank22 of the SRAM"]
    BNK22_EN_0 = 0,
    #[doc = "1: Enables Bank22 of the SRAM"]
    BNK22_EN_1 = 1,
}
impl From<BNK22_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK22_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK22_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK22_EN_A {
        match self.bits {
            false => BNK22_EN_A::BNK22_EN_0,
            true => BNK22_EN_A::BNK22_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK22_EN_0`"]
    #[inline(always)]
    pub fn is_bnk22_en_0(&self) -> bool {
        *self == BNK22_EN_A::BNK22_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK22_EN_1`"]
    #[inline(always)]
    pub fn is_bnk22_en_1(&self) -> bool {
        *self == BNK22_EN_A::BNK22_EN_1
    }
}
#[doc = "Field `BNK22_EN` writer - When 1, enables Bank22 of the SRAM"]
pub type BNK22_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK22_EN_A, O>;
impl<'a, const O: u8> BNK22_EN_W<'a, O> {
    #[doc = "Disables Bank22 of the SRAM"]
    #[inline(always)]
    pub fn bnk22_en_0(self) -> &'a mut W {
        self.variant(BNK22_EN_A::BNK22_EN_0)
    }
    #[doc = "Enables Bank22 of the SRAM"]
    #[inline(always)]
    pub fn bnk22_en_1(self) -> &'a mut W {
        self.variant(BNK22_EN_A::BNK22_EN_1)
    }
}
#[doc = "Field `BNK23_EN` reader - When 1, enables Bank23 of the SRAM"]
pub type BNK23_EN_R = crate::BitReader<BNK23_EN_A>;
#[doc = "When 1, enables Bank23 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK23_EN_A {
    #[doc = "0: Disables Bank23 of the SRAM"]
    BNK23_EN_0 = 0,
    #[doc = "1: Enables Bank23 of the SRAM"]
    BNK23_EN_1 = 1,
}
impl From<BNK23_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK23_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK23_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK23_EN_A {
        match self.bits {
            false => BNK23_EN_A::BNK23_EN_0,
            true => BNK23_EN_A::BNK23_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK23_EN_0`"]
    #[inline(always)]
    pub fn is_bnk23_en_0(&self) -> bool {
        *self == BNK23_EN_A::BNK23_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK23_EN_1`"]
    #[inline(always)]
    pub fn is_bnk23_en_1(&self) -> bool {
        *self == BNK23_EN_A::BNK23_EN_1
    }
}
#[doc = "Field `BNK23_EN` writer - When 1, enables Bank23 of the SRAM"]
pub type BNK23_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK23_EN_A, O>;
impl<'a, const O: u8> BNK23_EN_W<'a, O> {
    #[doc = "Disables Bank23 of the SRAM"]
    #[inline(always)]
    pub fn bnk23_en_0(self) -> &'a mut W {
        self.variant(BNK23_EN_A::BNK23_EN_0)
    }
    #[doc = "Enables Bank23 of the SRAM"]
    #[inline(always)]
    pub fn bnk23_en_1(self) -> &'a mut W {
        self.variant(BNK23_EN_A::BNK23_EN_1)
    }
}
#[doc = "Field `BNK24_EN` reader - When 1, enables Bank24 of the SRAM"]
pub type BNK24_EN_R = crate::BitReader<BNK24_EN_A>;
#[doc = "When 1, enables Bank24 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK24_EN_A {
    #[doc = "0: Disables Bank24 of the SRAM"]
    BNK24_EN_0 = 0,
    #[doc = "1: Enables Bank24 of the SRAM"]
    BNK24_EN_1 = 1,
}
impl From<BNK24_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK24_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK24_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK24_EN_A {
        match self.bits {
            false => BNK24_EN_A::BNK24_EN_0,
            true => BNK24_EN_A::BNK24_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK24_EN_0`"]
    #[inline(always)]
    pub fn is_bnk24_en_0(&self) -> bool {
        *self == BNK24_EN_A::BNK24_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK24_EN_1`"]
    #[inline(always)]
    pub fn is_bnk24_en_1(&self) -> bool {
        *self == BNK24_EN_A::BNK24_EN_1
    }
}
#[doc = "Field `BNK24_EN` writer - When 1, enables Bank24 of the SRAM"]
pub type BNK24_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK24_EN_A, O>;
impl<'a, const O: u8> BNK24_EN_W<'a, O> {
    #[doc = "Disables Bank24 of the SRAM"]
    #[inline(always)]
    pub fn bnk24_en_0(self) -> &'a mut W {
        self.variant(BNK24_EN_A::BNK24_EN_0)
    }
    #[doc = "Enables Bank24 of the SRAM"]
    #[inline(always)]
    pub fn bnk24_en_1(self) -> &'a mut W {
        self.variant(BNK24_EN_A::BNK24_EN_1)
    }
}
#[doc = "Field `BNK25_EN` reader - When 1, enables Bank25 of the SRAM"]
pub type BNK25_EN_R = crate::BitReader<BNK25_EN_A>;
#[doc = "When 1, enables Bank25 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK25_EN_A {
    #[doc = "0: Disables Bank25 of the SRAM"]
    BNK25_EN_0 = 0,
    #[doc = "1: Enables Bank25 of the SRAM"]
    BNK25_EN_1 = 1,
}
impl From<BNK25_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK25_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK25_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK25_EN_A {
        match self.bits {
            false => BNK25_EN_A::BNK25_EN_0,
            true => BNK25_EN_A::BNK25_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK25_EN_0`"]
    #[inline(always)]
    pub fn is_bnk25_en_0(&self) -> bool {
        *self == BNK25_EN_A::BNK25_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK25_EN_1`"]
    #[inline(always)]
    pub fn is_bnk25_en_1(&self) -> bool {
        *self == BNK25_EN_A::BNK25_EN_1
    }
}
#[doc = "Field `BNK25_EN` writer - When 1, enables Bank25 of the SRAM"]
pub type BNK25_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK25_EN_A, O>;
impl<'a, const O: u8> BNK25_EN_W<'a, O> {
    #[doc = "Disables Bank25 of the SRAM"]
    #[inline(always)]
    pub fn bnk25_en_0(self) -> &'a mut W {
        self.variant(BNK25_EN_A::BNK25_EN_0)
    }
    #[doc = "Enables Bank25 of the SRAM"]
    #[inline(always)]
    pub fn bnk25_en_1(self) -> &'a mut W {
        self.variant(BNK25_EN_A::BNK25_EN_1)
    }
}
#[doc = "Field `BNK26_EN` reader - When 1, enables Bank26 of the SRAM"]
pub type BNK26_EN_R = crate::BitReader<BNK26_EN_A>;
#[doc = "When 1, enables Bank26 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK26_EN_A {
    #[doc = "0: Disables Bank26 of the SRAM"]
    BNK26_EN_0 = 0,
    #[doc = "1: Enables Bank26 of the SRAM"]
    BNK26_EN_1 = 1,
}
impl From<BNK26_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK26_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK26_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK26_EN_A {
        match self.bits {
            false => BNK26_EN_A::BNK26_EN_0,
            true => BNK26_EN_A::BNK26_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK26_EN_0`"]
    #[inline(always)]
    pub fn is_bnk26_en_0(&self) -> bool {
        *self == BNK26_EN_A::BNK26_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK26_EN_1`"]
    #[inline(always)]
    pub fn is_bnk26_en_1(&self) -> bool {
        *self == BNK26_EN_A::BNK26_EN_1
    }
}
#[doc = "Field `BNK26_EN` writer - When 1, enables Bank26 of the SRAM"]
pub type BNK26_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK26_EN_A, O>;
impl<'a, const O: u8> BNK26_EN_W<'a, O> {
    #[doc = "Disables Bank26 of the SRAM"]
    #[inline(always)]
    pub fn bnk26_en_0(self) -> &'a mut W {
        self.variant(BNK26_EN_A::BNK26_EN_0)
    }
    #[doc = "Enables Bank26 of the SRAM"]
    #[inline(always)]
    pub fn bnk26_en_1(self) -> &'a mut W {
        self.variant(BNK26_EN_A::BNK26_EN_1)
    }
}
#[doc = "Field `BNK27_EN` reader - When 1, enables Bank27 of the SRAM"]
pub type BNK27_EN_R = crate::BitReader<BNK27_EN_A>;
#[doc = "When 1, enables Bank27 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK27_EN_A {
    #[doc = "0: Disables Bank27 of the SRAM"]
    BNK27_EN_0 = 0,
    #[doc = "1: Enables Bank27 of the SRAM"]
    BNK27_EN_1 = 1,
}
impl From<BNK27_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK27_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK27_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK27_EN_A {
        match self.bits {
            false => BNK27_EN_A::BNK27_EN_0,
            true => BNK27_EN_A::BNK27_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK27_EN_0`"]
    #[inline(always)]
    pub fn is_bnk27_en_0(&self) -> bool {
        *self == BNK27_EN_A::BNK27_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK27_EN_1`"]
    #[inline(always)]
    pub fn is_bnk27_en_1(&self) -> bool {
        *self == BNK27_EN_A::BNK27_EN_1
    }
}
#[doc = "Field `BNK27_EN` writer - When 1, enables Bank27 of the SRAM"]
pub type BNK27_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK27_EN_A, O>;
impl<'a, const O: u8> BNK27_EN_W<'a, O> {
    #[doc = "Disables Bank27 of the SRAM"]
    #[inline(always)]
    pub fn bnk27_en_0(self) -> &'a mut W {
        self.variant(BNK27_EN_A::BNK27_EN_0)
    }
    #[doc = "Enables Bank27 of the SRAM"]
    #[inline(always)]
    pub fn bnk27_en_1(self) -> &'a mut W {
        self.variant(BNK27_EN_A::BNK27_EN_1)
    }
}
#[doc = "Field `BNK28_EN` reader - When 1, enables Bank28 of the SRAM"]
pub type BNK28_EN_R = crate::BitReader<BNK28_EN_A>;
#[doc = "When 1, enables Bank28 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK28_EN_A {
    #[doc = "0: Disables Bank28 of the SRAM"]
    BNK28_EN_0 = 0,
    #[doc = "1: Enables Bank28 of the SRAM"]
    BNK28_EN_1 = 1,
}
impl From<BNK28_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK28_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK28_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK28_EN_A {
        match self.bits {
            false => BNK28_EN_A::BNK28_EN_0,
            true => BNK28_EN_A::BNK28_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK28_EN_0`"]
    #[inline(always)]
    pub fn is_bnk28_en_0(&self) -> bool {
        *self == BNK28_EN_A::BNK28_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK28_EN_1`"]
    #[inline(always)]
    pub fn is_bnk28_en_1(&self) -> bool {
        *self == BNK28_EN_A::BNK28_EN_1
    }
}
#[doc = "Field `BNK28_EN` writer - When 1, enables Bank28 of the SRAM"]
pub type BNK28_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK28_EN_A, O>;
impl<'a, const O: u8> BNK28_EN_W<'a, O> {
    #[doc = "Disables Bank28 of the SRAM"]
    #[inline(always)]
    pub fn bnk28_en_0(self) -> &'a mut W {
        self.variant(BNK28_EN_A::BNK28_EN_0)
    }
    #[doc = "Enables Bank28 of the SRAM"]
    #[inline(always)]
    pub fn bnk28_en_1(self) -> &'a mut W {
        self.variant(BNK28_EN_A::BNK28_EN_1)
    }
}
#[doc = "Field `BNK29_EN` reader - When 1, enables Bank29 of the SRAM"]
pub type BNK29_EN_R = crate::BitReader<BNK29_EN_A>;
#[doc = "When 1, enables Bank29 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK29_EN_A {
    #[doc = "0: Disables Bank29 of the SRAM"]
    BNK29_EN_0 = 0,
    #[doc = "1: Enables Bank29 of the SRAM"]
    BNK29_EN_1 = 1,
}
impl From<BNK29_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK29_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK29_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK29_EN_A {
        match self.bits {
            false => BNK29_EN_A::BNK29_EN_0,
            true => BNK29_EN_A::BNK29_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK29_EN_0`"]
    #[inline(always)]
    pub fn is_bnk29_en_0(&self) -> bool {
        *self == BNK29_EN_A::BNK29_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK29_EN_1`"]
    #[inline(always)]
    pub fn is_bnk29_en_1(&self) -> bool {
        *self == BNK29_EN_A::BNK29_EN_1
    }
}
#[doc = "Field `BNK29_EN` writer - When 1, enables Bank29 of the SRAM"]
pub type BNK29_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK29_EN_A, O>;
impl<'a, const O: u8> BNK29_EN_W<'a, O> {
    #[doc = "Disables Bank29 of the SRAM"]
    #[inline(always)]
    pub fn bnk29_en_0(self) -> &'a mut W {
        self.variant(BNK29_EN_A::BNK29_EN_0)
    }
    #[doc = "Enables Bank29 of the SRAM"]
    #[inline(always)]
    pub fn bnk29_en_1(self) -> &'a mut W {
        self.variant(BNK29_EN_A::BNK29_EN_1)
    }
}
#[doc = "Field `BNK30_EN` reader - When 1, enables Bank30 of the SRAM"]
pub type BNK30_EN_R = crate::BitReader<BNK30_EN_A>;
#[doc = "When 1, enables Bank30 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK30_EN_A {
    #[doc = "0: Disables Bank30 of the SRAM"]
    BNK30_EN_0 = 0,
    #[doc = "1: Enables Bank30 of the SRAM"]
    BNK30_EN_1 = 1,
}
impl From<BNK30_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK30_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK30_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK30_EN_A {
        match self.bits {
            false => BNK30_EN_A::BNK30_EN_0,
            true => BNK30_EN_A::BNK30_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK30_EN_0`"]
    #[inline(always)]
    pub fn is_bnk30_en_0(&self) -> bool {
        *self == BNK30_EN_A::BNK30_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK30_EN_1`"]
    #[inline(always)]
    pub fn is_bnk30_en_1(&self) -> bool {
        *self == BNK30_EN_A::BNK30_EN_1
    }
}
#[doc = "Field `BNK30_EN` writer - When 1, enables Bank30 of the SRAM"]
pub type BNK30_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK30_EN_A, O>;
impl<'a, const O: u8> BNK30_EN_W<'a, O> {
    #[doc = "Disables Bank30 of the SRAM"]
    #[inline(always)]
    pub fn bnk30_en_0(self) -> &'a mut W {
        self.variant(BNK30_EN_A::BNK30_EN_0)
    }
    #[doc = "Enables Bank30 of the SRAM"]
    #[inline(always)]
    pub fn bnk30_en_1(self) -> &'a mut W {
        self.variant(BNK30_EN_A::BNK30_EN_1)
    }
}
#[doc = "Field `BNK31_EN` reader - When 1, enables Bank31 of the SRAM"]
pub type BNK31_EN_R = crate::BitReader<BNK31_EN_A>;
#[doc = "When 1, enables Bank31 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK31_EN_A {
    #[doc = "0: Disables Bank31 of the SRAM"]
    BNK31_EN_0 = 0,
    #[doc = "1: Enables Bank31 of the SRAM"]
    BNK31_EN_1 = 1,
}
impl From<BNK31_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK31_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK31_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK31_EN_A {
        match self.bits {
            false => BNK31_EN_A::BNK31_EN_0,
            true => BNK31_EN_A::BNK31_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK31_EN_0`"]
    #[inline(always)]
    pub fn is_bnk31_en_0(&self) -> bool {
        *self == BNK31_EN_A::BNK31_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK31_EN_1`"]
    #[inline(always)]
    pub fn is_bnk31_en_1(&self) -> bool {
        *self == BNK31_EN_A::BNK31_EN_1
    }
}
#[doc = "Field `BNK31_EN` writer - When 1, enables Bank31 of the SRAM"]
pub type BNK31_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL0_SPEC, BNK31_EN_A, O>;
impl<'a, const O: u8> BNK31_EN_W<'a, O> {
    #[doc = "Disables Bank31 of the SRAM"]
    #[inline(always)]
    pub fn bnk31_en_0(self) -> &'a mut W {
        self.variant(BNK31_EN_A::BNK31_EN_0)
    }
    #[doc = "Enables Bank31 of the SRAM"]
    #[inline(always)]
    pub fn bnk31_en_1(self) -> &'a mut W {
        self.variant(BNK31_EN_A::BNK31_EN_1)
    }
}
impl R {
    #[doc = "Bit 0 - When 1, enables Bank0 of the SRAM"]
    #[inline(always)]
    pub fn bnk0_en(&self) -> BNK0_EN_R {
        BNK0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, enables Bank1 of the SRAM"]
    #[inline(always)]
    pub fn bnk1_en(&self) -> BNK1_EN_R {
        BNK1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, enables Bank2 of the SRAM"]
    #[inline(always)]
    pub fn bnk2_en(&self) -> BNK2_EN_R {
        BNK2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, enables Bank3 of the SRAM"]
    #[inline(always)]
    pub fn bnk3_en(&self) -> BNK3_EN_R {
        BNK3_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, enables Bank4 of the SRAM"]
    #[inline(always)]
    pub fn bnk4_en(&self) -> BNK4_EN_R {
        BNK4_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, enables Bank5 of the SRAM"]
    #[inline(always)]
    pub fn bnk5_en(&self) -> BNK5_EN_R {
        BNK5_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, enables Bank6 of the SRAM"]
    #[inline(always)]
    pub fn bnk6_en(&self) -> BNK6_EN_R {
        BNK6_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, enables Bank7 of the SRAM"]
    #[inline(always)]
    pub fn bnk7_en(&self) -> BNK7_EN_R {
        BNK7_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, enables Bank8 of the SRAM"]
    #[inline(always)]
    pub fn bnk8_en(&self) -> BNK8_EN_R {
        BNK8_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, enables Bank9 of the SRAM"]
    #[inline(always)]
    pub fn bnk9_en(&self) -> BNK9_EN_R {
        BNK9_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When 1, enables Bank10 of the SRAM"]
    #[inline(always)]
    pub fn bnk10_en(&self) -> BNK10_EN_R {
        BNK10_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When 1, enables Bank11 of the SRAM"]
    #[inline(always)]
    pub fn bnk11_en(&self) -> BNK11_EN_R {
        BNK11_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When 1, enables Bank12 of the SRAM"]
    #[inline(always)]
    pub fn bnk12_en(&self) -> BNK12_EN_R {
        BNK12_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When 1, enables Bank13 of the SRAM"]
    #[inline(always)]
    pub fn bnk13_en(&self) -> BNK13_EN_R {
        BNK13_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When 1, enables Bank14 of the SRAM"]
    #[inline(always)]
    pub fn bnk14_en(&self) -> BNK14_EN_R {
        BNK14_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When 1, enables Bank15 of the SRAM"]
    #[inline(always)]
    pub fn bnk15_en(&self) -> BNK15_EN_R {
        BNK15_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When 1, enables Bank16 of the SRAM"]
    #[inline(always)]
    pub fn bnk16_en(&self) -> BNK16_EN_R {
        BNK16_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When 1, enables Bank17 of the SRAM"]
    #[inline(always)]
    pub fn bnk17_en(&self) -> BNK17_EN_R {
        BNK17_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When 1, enables Bank18 of the SRAM"]
    #[inline(always)]
    pub fn bnk18_en(&self) -> BNK18_EN_R {
        BNK18_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When 1, enables Bank19 of the SRAM"]
    #[inline(always)]
    pub fn bnk19_en(&self) -> BNK19_EN_R {
        BNK19_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When 1, enables Bank20 of the SRAM"]
    #[inline(always)]
    pub fn bnk20_en(&self) -> BNK20_EN_R {
        BNK20_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When 1, enables Bank21 of the SRAM"]
    #[inline(always)]
    pub fn bnk21_en(&self) -> BNK21_EN_R {
        BNK21_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When 1, enables Bank22 of the SRAM"]
    #[inline(always)]
    pub fn bnk22_en(&self) -> BNK22_EN_R {
        BNK22_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When 1, enables Bank23 of the SRAM"]
    #[inline(always)]
    pub fn bnk23_en(&self) -> BNK23_EN_R {
        BNK23_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When 1, enables Bank24 of the SRAM"]
    #[inline(always)]
    pub fn bnk24_en(&self) -> BNK24_EN_R {
        BNK24_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When 1, enables Bank25 of the SRAM"]
    #[inline(always)]
    pub fn bnk25_en(&self) -> BNK25_EN_R {
        BNK25_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When 1, enables Bank26 of the SRAM"]
    #[inline(always)]
    pub fn bnk26_en(&self) -> BNK26_EN_R {
        BNK26_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When 1, enables Bank27 of the SRAM"]
    #[inline(always)]
    pub fn bnk27_en(&self) -> BNK27_EN_R {
        BNK27_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When 1, enables Bank28 of the SRAM"]
    #[inline(always)]
    pub fn bnk28_en(&self) -> BNK28_EN_R {
        BNK28_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When 1, enables Bank29 of the SRAM"]
    #[inline(always)]
    pub fn bnk29_en(&self) -> BNK29_EN_R {
        BNK29_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When 1, enables Bank30 of the SRAM"]
    #[inline(always)]
    pub fn bnk30_en(&self) -> BNK30_EN_R {
        BNK30_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When 1, enables Bank31 of the SRAM"]
    #[inline(always)]
    pub fn bnk31_en(&self) -> BNK31_EN_R {
        BNK31_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - When 1, enables Bank1 of the SRAM"]
    #[inline(always)]
    pub fn bnk1_en(&mut self) -> BNK1_EN_W<1> {
        BNK1_EN_W::new(self)
    }
    #[doc = "Bit 2 - When 1, enables Bank2 of the SRAM"]
    #[inline(always)]
    pub fn bnk2_en(&mut self) -> BNK2_EN_W<2> {
        BNK2_EN_W::new(self)
    }
    #[doc = "Bit 3 - When 1, enables Bank3 of the SRAM"]
    #[inline(always)]
    pub fn bnk3_en(&mut self) -> BNK3_EN_W<3> {
        BNK3_EN_W::new(self)
    }
    #[doc = "Bit 4 - When 1, enables Bank4 of the SRAM"]
    #[inline(always)]
    pub fn bnk4_en(&mut self) -> BNK4_EN_W<4> {
        BNK4_EN_W::new(self)
    }
    #[doc = "Bit 5 - When 1, enables Bank5 of the SRAM"]
    #[inline(always)]
    pub fn bnk5_en(&mut self) -> BNK5_EN_W<5> {
        BNK5_EN_W::new(self)
    }
    #[doc = "Bit 6 - When 1, enables Bank6 of the SRAM"]
    #[inline(always)]
    pub fn bnk6_en(&mut self) -> BNK6_EN_W<6> {
        BNK6_EN_W::new(self)
    }
    #[doc = "Bit 7 - When 1, enables Bank7 of the SRAM"]
    #[inline(always)]
    pub fn bnk7_en(&mut self) -> BNK7_EN_W<7> {
        BNK7_EN_W::new(self)
    }
    #[doc = "Bit 8 - When 1, enables Bank8 of the SRAM"]
    #[inline(always)]
    pub fn bnk8_en(&mut self) -> BNK8_EN_W<8> {
        BNK8_EN_W::new(self)
    }
    #[doc = "Bit 9 - When 1, enables Bank9 of the SRAM"]
    #[inline(always)]
    pub fn bnk9_en(&mut self) -> BNK9_EN_W<9> {
        BNK9_EN_W::new(self)
    }
    #[doc = "Bit 10 - When 1, enables Bank10 of the SRAM"]
    #[inline(always)]
    pub fn bnk10_en(&mut self) -> BNK10_EN_W<10> {
        BNK10_EN_W::new(self)
    }
    #[doc = "Bit 11 - When 1, enables Bank11 of the SRAM"]
    #[inline(always)]
    pub fn bnk11_en(&mut self) -> BNK11_EN_W<11> {
        BNK11_EN_W::new(self)
    }
    #[doc = "Bit 12 - When 1, enables Bank12 of the SRAM"]
    #[inline(always)]
    pub fn bnk12_en(&mut self) -> BNK12_EN_W<12> {
        BNK12_EN_W::new(self)
    }
    #[doc = "Bit 13 - When 1, enables Bank13 of the SRAM"]
    #[inline(always)]
    pub fn bnk13_en(&mut self) -> BNK13_EN_W<13> {
        BNK13_EN_W::new(self)
    }
    #[doc = "Bit 14 - When 1, enables Bank14 of the SRAM"]
    #[inline(always)]
    pub fn bnk14_en(&mut self) -> BNK14_EN_W<14> {
        BNK14_EN_W::new(self)
    }
    #[doc = "Bit 15 - When 1, enables Bank15 of the SRAM"]
    #[inline(always)]
    pub fn bnk15_en(&mut self) -> BNK15_EN_W<15> {
        BNK15_EN_W::new(self)
    }
    #[doc = "Bit 16 - When 1, enables Bank16 of the SRAM"]
    #[inline(always)]
    pub fn bnk16_en(&mut self) -> BNK16_EN_W<16> {
        BNK16_EN_W::new(self)
    }
    #[doc = "Bit 17 - When 1, enables Bank17 of the SRAM"]
    #[inline(always)]
    pub fn bnk17_en(&mut self) -> BNK17_EN_W<17> {
        BNK17_EN_W::new(self)
    }
    #[doc = "Bit 18 - When 1, enables Bank18 of the SRAM"]
    #[inline(always)]
    pub fn bnk18_en(&mut self) -> BNK18_EN_W<18> {
        BNK18_EN_W::new(self)
    }
    #[doc = "Bit 19 - When 1, enables Bank19 of the SRAM"]
    #[inline(always)]
    pub fn bnk19_en(&mut self) -> BNK19_EN_W<19> {
        BNK19_EN_W::new(self)
    }
    #[doc = "Bit 20 - When 1, enables Bank20 of the SRAM"]
    #[inline(always)]
    pub fn bnk20_en(&mut self) -> BNK20_EN_W<20> {
        BNK20_EN_W::new(self)
    }
    #[doc = "Bit 21 - When 1, enables Bank21 of the SRAM"]
    #[inline(always)]
    pub fn bnk21_en(&mut self) -> BNK21_EN_W<21> {
        BNK21_EN_W::new(self)
    }
    #[doc = "Bit 22 - When 1, enables Bank22 of the SRAM"]
    #[inline(always)]
    pub fn bnk22_en(&mut self) -> BNK22_EN_W<22> {
        BNK22_EN_W::new(self)
    }
    #[doc = "Bit 23 - When 1, enables Bank23 of the SRAM"]
    #[inline(always)]
    pub fn bnk23_en(&mut self) -> BNK23_EN_W<23> {
        BNK23_EN_W::new(self)
    }
    #[doc = "Bit 24 - When 1, enables Bank24 of the SRAM"]
    #[inline(always)]
    pub fn bnk24_en(&mut self) -> BNK24_EN_W<24> {
        BNK24_EN_W::new(self)
    }
    #[doc = "Bit 25 - When 1, enables Bank25 of the SRAM"]
    #[inline(always)]
    pub fn bnk25_en(&mut self) -> BNK25_EN_W<25> {
        BNK25_EN_W::new(self)
    }
    #[doc = "Bit 26 - When 1, enables Bank26 of the SRAM"]
    #[inline(always)]
    pub fn bnk26_en(&mut self) -> BNK26_EN_W<26> {
        BNK26_EN_W::new(self)
    }
    #[doc = "Bit 27 - When 1, enables Bank27 of the SRAM"]
    #[inline(always)]
    pub fn bnk27_en(&mut self) -> BNK27_EN_W<27> {
        BNK27_EN_W::new(self)
    }
    #[doc = "Bit 28 - When 1, enables Bank28 of the SRAM"]
    #[inline(always)]
    pub fn bnk28_en(&mut self) -> BNK28_EN_W<28> {
        BNK28_EN_W::new(self)
    }
    #[doc = "Bit 29 - When 1, enables Bank29 of the SRAM"]
    #[inline(always)]
    pub fn bnk29_en(&mut self) -> BNK29_EN_W<29> {
        BNK29_EN_W::new(self)
    }
    #[doc = "Bit 30 - When 1, enables Bank30 of the SRAM"]
    #[inline(always)]
    pub fn bnk30_en(&mut self) -> BNK30_EN_W<30> {
        BNK30_EN_W::new(self)
    }
    #[doc = "Bit 31 - When 1, enables Bank31 of the SRAM"]
    #[inline(always)]
    pub fn bnk31_en(&mut self) -> BNK31_EN_W<31> {
        BNK31_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Bank Enable Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_banken_ctl0](index.html) module"]
pub struct SYS_SRAM_BANKEN_CTL0_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BANKEN_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_banken_ctl0::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BANKEN_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_banken_ctl0::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_BANKEN_CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_BANKEN_CTL0 to value 0xffff_ffff"]
impl crate::Resettable for SYS_SRAM_BANKEN_CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
