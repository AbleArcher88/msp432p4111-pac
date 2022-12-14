#[doc = "Register `SYS_SRAM_BANKEN_CTL1` reader"]
pub struct R(crate::R<SYS_SRAM_BANKEN_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BANKEN_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_BANKEN_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_BANKEN_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_BANKEN_CTL1` writer"]
pub struct W(crate::W<SYS_SRAM_BANKEN_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_BANKEN_CTL1_SPEC>;
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
impl From<crate::W<SYS_SRAM_BANKEN_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SRAM_BANKEN_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNK32_EN` reader - When 1, enables Bank32 of the SRAM"]
pub type BNK32_EN_R = crate::BitReader<BNK32_EN_A>;
#[doc = "When 1, enables Bank32 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK32_EN_A {
    #[doc = "0: Disables Bank32 of the SRAM"]
    BNK32_EN_0 = 0,
    #[doc = "1: Enables Bank32 of the SRAM"]
    BNK32_EN_1 = 1,
}
impl From<BNK32_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK32_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK32_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK32_EN_A {
        match self.bits {
            false => BNK32_EN_A::BNK32_EN_0,
            true => BNK32_EN_A::BNK32_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK32_EN_0`"]
    #[inline(always)]
    pub fn is_bnk32_en_0(&self) -> bool {
        *self == BNK32_EN_A::BNK32_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK32_EN_1`"]
    #[inline(always)]
    pub fn is_bnk32_en_1(&self) -> bool {
        *self == BNK32_EN_A::BNK32_EN_1
    }
}
#[doc = "Field `BNK32_EN` writer - When 1, enables Bank32 of the SRAM"]
pub type BNK32_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK32_EN_A, O>;
impl<'a, const O: u8> BNK32_EN_W<'a, O> {
    #[doc = "Disables Bank32 of the SRAM"]
    #[inline(always)]
    pub fn bnk32_en_0(self) -> &'a mut W {
        self.variant(BNK32_EN_A::BNK32_EN_0)
    }
    #[doc = "Enables Bank32 of the SRAM"]
    #[inline(always)]
    pub fn bnk32_en_1(self) -> &'a mut W {
        self.variant(BNK32_EN_A::BNK32_EN_1)
    }
}
#[doc = "Field `BNK33_EN` reader - When 1, enables Bank33 of the SRAM"]
pub type BNK33_EN_R = crate::BitReader<BNK33_EN_A>;
#[doc = "When 1, enables Bank33 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK33_EN_A {
    #[doc = "0: Disables Bank33 of the SRAM"]
    BNK33_EN_0 = 0,
    #[doc = "1: Enables Bank33 of the SRAM"]
    BNK33_EN_1 = 1,
}
impl From<BNK33_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK33_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK33_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK33_EN_A {
        match self.bits {
            false => BNK33_EN_A::BNK33_EN_0,
            true => BNK33_EN_A::BNK33_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK33_EN_0`"]
    #[inline(always)]
    pub fn is_bnk33_en_0(&self) -> bool {
        *self == BNK33_EN_A::BNK33_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK33_EN_1`"]
    #[inline(always)]
    pub fn is_bnk33_en_1(&self) -> bool {
        *self == BNK33_EN_A::BNK33_EN_1
    }
}
#[doc = "Field `BNK33_EN` writer - When 1, enables Bank33 of the SRAM"]
pub type BNK33_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK33_EN_A, O>;
impl<'a, const O: u8> BNK33_EN_W<'a, O> {
    #[doc = "Disables Bank33 of the SRAM"]
    #[inline(always)]
    pub fn bnk33_en_0(self) -> &'a mut W {
        self.variant(BNK33_EN_A::BNK33_EN_0)
    }
    #[doc = "Enables Bank33 of the SRAM"]
    #[inline(always)]
    pub fn bnk33_en_1(self) -> &'a mut W {
        self.variant(BNK33_EN_A::BNK33_EN_1)
    }
}
#[doc = "Field `BNK34_EN` reader - When 1, enables Bank34 of the SRAM"]
pub type BNK34_EN_R = crate::BitReader<BNK34_EN_A>;
#[doc = "When 1, enables Bank34 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK34_EN_A {
    #[doc = "0: Disables Bank34 of the SRAM"]
    BNK34_EN_0 = 0,
    #[doc = "1: Enables Bank34 of the SRAM"]
    BNK34_EN_1 = 1,
}
impl From<BNK34_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK34_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK34_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK34_EN_A {
        match self.bits {
            false => BNK34_EN_A::BNK34_EN_0,
            true => BNK34_EN_A::BNK34_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK34_EN_0`"]
    #[inline(always)]
    pub fn is_bnk34_en_0(&self) -> bool {
        *self == BNK34_EN_A::BNK34_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK34_EN_1`"]
    #[inline(always)]
    pub fn is_bnk34_en_1(&self) -> bool {
        *self == BNK34_EN_A::BNK34_EN_1
    }
}
#[doc = "Field `BNK34_EN` writer - When 1, enables Bank34 of the SRAM"]
pub type BNK34_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK34_EN_A, O>;
impl<'a, const O: u8> BNK34_EN_W<'a, O> {
    #[doc = "Disables Bank34 of the SRAM"]
    #[inline(always)]
    pub fn bnk34_en_0(self) -> &'a mut W {
        self.variant(BNK34_EN_A::BNK34_EN_0)
    }
    #[doc = "Enables Bank34 of the SRAM"]
    #[inline(always)]
    pub fn bnk34_en_1(self) -> &'a mut W {
        self.variant(BNK34_EN_A::BNK34_EN_1)
    }
}
#[doc = "Field `BNK35_EN` reader - When 1, enables Bank35 of the SRAM"]
pub type BNK35_EN_R = crate::BitReader<BNK35_EN_A>;
#[doc = "When 1, enables Bank35 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK35_EN_A {
    #[doc = "0: Disables Bank35 of the SRAM"]
    BNK35_EN_0 = 0,
    #[doc = "1: Enables Bank35 of the SRAM"]
    BNK35_EN_1 = 1,
}
impl From<BNK35_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK35_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK35_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK35_EN_A {
        match self.bits {
            false => BNK35_EN_A::BNK35_EN_0,
            true => BNK35_EN_A::BNK35_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK35_EN_0`"]
    #[inline(always)]
    pub fn is_bnk35_en_0(&self) -> bool {
        *self == BNK35_EN_A::BNK35_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK35_EN_1`"]
    #[inline(always)]
    pub fn is_bnk35_en_1(&self) -> bool {
        *self == BNK35_EN_A::BNK35_EN_1
    }
}
#[doc = "Field `BNK35_EN` writer - When 1, enables Bank35 of the SRAM"]
pub type BNK35_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK35_EN_A, O>;
impl<'a, const O: u8> BNK35_EN_W<'a, O> {
    #[doc = "Disables Bank35 of the SRAM"]
    #[inline(always)]
    pub fn bnk35_en_0(self) -> &'a mut W {
        self.variant(BNK35_EN_A::BNK35_EN_0)
    }
    #[doc = "Enables Bank35 of the SRAM"]
    #[inline(always)]
    pub fn bnk35_en_1(self) -> &'a mut W {
        self.variant(BNK35_EN_A::BNK35_EN_1)
    }
}
#[doc = "Field `BNK36_EN` reader - When 1, enables Bank36 of the SRAM"]
pub type BNK36_EN_R = crate::BitReader<BNK36_EN_A>;
#[doc = "When 1, enables Bank36 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK36_EN_A {
    #[doc = "0: Disables Bank36 of the SRAM"]
    BNK36_EN_0 = 0,
    #[doc = "1: Enables Bank36 of the SRAM"]
    BNK36_EN_1 = 1,
}
impl From<BNK36_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK36_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK36_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK36_EN_A {
        match self.bits {
            false => BNK36_EN_A::BNK36_EN_0,
            true => BNK36_EN_A::BNK36_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK36_EN_0`"]
    #[inline(always)]
    pub fn is_bnk36_en_0(&self) -> bool {
        *self == BNK36_EN_A::BNK36_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK36_EN_1`"]
    #[inline(always)]
    pub fn is_bnk36_en_1(&self) -> bool {
        *self == BNK36_EN_A::BNK36_EN_1
    }
}
#[doc = "Field `BNK36_EN` writer - When 1, enables Bank36 of the SRAM"]
pub type BNK36_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK36_EN_A, O>;
impl<'a, const O: u8> BNK36_EN_W<'a, O> {
    #[doc = "Disables Bank36 of the SRAM"]
    #[inline(always)]
    pub fn bnk36_en_0(self) -> &'a mut W {
        self.variant(BNK36_EN_A::BNK36_EN_0)
    }
    #[doc = "Enables Bank36 of the SRAM"]
    #[inline(always)]
    pub fn bnk36_en_1(self) -> &'a mut W {
        self.variant(BNK36_EN_A::BNK36_EN_1)
    }
}
#[doc = "Field `BNK37_EN` reader - When 1, enables Bank37 of the SRAM"]
pub type BNK37_EN_R = crate::BitReader<BNK37_EN_A>;
#[doc = "When 1, enables Bank37 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK37_EN_A {
    #[doc = "0: Disables Bank37 of the SRAM"]
    BNK37_EN_0 = 0,
    #[doc = "1: Enables Bank37 of the SRAM"]
    BNK37_EN_1 = 1,
}
impl From<BNK37_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK37_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK37_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK37_EN_A {
        match self.bits {
            false => BNK37_EN_A::BNK37_EN_0,
            true => BNK37_EN_A::BNK37_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK37_EN_0`"]
    #[inline(always)]
    pub fn is_bnk37_en_0(&self) -> bool {
        *self == BNK37_EN_A::BNK37_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK37_EN_1`"]
    #[inline(always)]
    pub fn is_bnk37_en_1(&self) -> bool {
        *self == BNK37_EN_A::BNK37_EN_1
    }
}
#[doc = "Field `BNK37_EN` writer - When 1, enables Bank37 of the SRAM"]
pub type BNK37_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK37_EN_A, O>;
impl<'a, const O: u8> BNK37_EN_W<'a, O> {
    #[doc = "Disables Bank37 of the SRAM"]
    #[inline(always)]
    pub fn bnk37_en_0(self) -> &'a mut W {
        self.variant(BNK37_EN_A::BNK37_EN_0)
    }
    #[doc = "Enables Bank37 of the SRAM"]
    #[inline(always)]
    pub fn bnk37_en_1(self) -> &'a mut W {
        self.variant(BNK37_EN_A::BNK37_EN_1)
    }
}
#[doc = "Field `BNK38_EN` reader - When 1, enables Bank38 of the SRAM"]
pub type BNK38_EN_R = crate::BitReader<BNK38_EN_A>;
#[doc = "When 1, enables Bank38 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK38_EN_A {
    #[doc = "0: Disables Bank38 of the SRAM"]
    BNK38_EN_0 = 0,
    #[doc = "1: Enables Bank38 of the SRAM"]
    BNK38_EN_1 = 1,
}
impl From<BNK38_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK38_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK38_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK38_EN_A {
        match self.bits {
            false => BNK38_EN_A::BNK38_EN_0,
            true => BNK38_EN_A::BNK38_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK38_EN_0`"]
    #[inline(always)]
    pub fn is_bnk38_en_0(&self) -> bool {
        *self == BNK38_EN_A::BNK38_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK38_EN_1`"]
    #[inline(always)]
    pub fn is_bnk38_en_1(&self) -> bool {
        *self == BNK38_EN_A::BNK38_EN_1
    }
}
#[doc = "Field `BNK38_EN` writer - When 1, enables Bank38 of the SRAM"]
pub type BNK38_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK38_EN_A, O>;
impl<'a, const O: u8> BNK38_EN_W<'a, O> {
    #[doc = "Disables Bank38 of the SRAM"]
    #[inline(always)]
    pub fn bnk38_en_0(self) -> &'a mut W {
        self.variant(BNK38_EN_A::BNK38_EN_0)
    }
    #[doc = "Enables Bank38 of the SRAM"]
    #[inline(always)]
    pub fn bnk38_en_1(self) -> &'a mut W {
        self.variant(BNK38_EN_A::BNK38_EN_1)
    }
}
#[doc = "Field `BNK39_EN` reader - When 1, enables Bank39 of the SRAM"]
pub type BNK39_EN_R = crate::BitReader<BNK39_EN_A>;
#[doc = "When 1, enables Bank39 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK39_EN_A {
    #[doc = "0: Disables Bank39 of the SRAM"]
    BNK39_EN_0 = 0,
    #[doc = "1: Enables Bank39 of the SRAM"]
    BNK39_EN_1 = 1,
}
impl From<BNK39_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK39_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK39_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK39_EN_A {
        match self.bits {
            false => BNK39_EN_A::BNK39_EN_0,
            true => BNK39_EN_A::BNK39_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK39_EN_0`"]
    #[inline(always)]
    pub fn is_bnk39_en_0(&self) -> bool {
        *self == BNK39_EN_A::BNK39_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK39_EN_1`"]
    #[inline(always)]
    pub fn is_bnk39_en_1(&self) -> bool {
        *self == BNK39_EN_A::BNK39_EN_1
    }
}
#[doc = "Field `BNK39_EN` writer - When 1, enables Bank39 of the SRAM"]
pub type BNK39_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK39_EN_A, O>;
impl<'a, const O: u8> BNK39_EN_W<'a, O> {
    #[doc = "Disables Bank39 of the SRAM"]
    #[inline(always)]
    pub fn bnk39_en_0(self) -> &'a mut W {
        self.variant(BNK39_EN_A::BNK39_EN_0)
    }
    #[doc = "Enables Bank39 of the SRAM"]
    #[inline(always)]
    pub fn bnk39_en_1(self) -> &'a mut W {
        self.variant(BNK39_EN_A::BNK39_EN_1)
    }
}
#[doc = "Field `BNK40_EN` reader - When 1, enables Bank40 of the SRAM"]
pub type BNK40_EN_R = crate::BitReader<BNK40_EN_A>;
#[doc = "When 1, enables Bank40 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK40_EN_A {
    #[doc = "0: Disables Bank40 of the SRAM"]
    BNK40_EN_0 = 0,
    #[doc = "1: Enables Bank40 of the SRAM"]
    BNK40_EN_1 = 1,
}
impl From<BNK40_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK40_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK40_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK40_EN_A {
        match self.bits {
            false => BNK40_EN_A::BNK40_EN_0,
            true => BNK40_EN_A::BNK40_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK40_EN_0`"]
    #[inline(always)]
    pub fn is_bnk40_en_0(&self) -> bool {
        *self == BNK40_EN_A::BNK40_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK40_EN_1`"]
    #[inline(always)]
    pub fn is_bnk40_en_1(&self) -> bool {
        *self == BNK40_EN_A::BNK40_EN_1
    }
}
#[doc = "Field `BNK40_EN` writer - When 1, enables Bank40 of the SRAM"]
pub type BNK40_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK40_EN_A, O>;
impl<'a, const O: u8> BNK40_EN_W<'a, O> {
    #[doc = "Disables Bank40 of the SRAM"]
    #[inline(always)]
    pub fn bnk40_en_0(self) -> &'a mut W {
        self.variant(BNK40_EN_A::BNK40_EN_0)
    }
    #[doc = "Enables Bank40 of the SRAM"]
    #[inline(always)]
    pub fn bnk40_en_1(self) -> &'a mut W {
        self.variant(BNK40_EN_A::BNK40_EN_1)
    }
}
#[doc = "Field `BNK41_EN` reader - When 1, enables Bank41 of the SRAM"]
pub type BNK41_EN_R = crate::BitReader<BNK41_EN_A>;
#[doc = "When 1, enables Bank41 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK41_EN_A {
    #[doc = "0: Disables Bank41 of the SRAM"]
    BNK41_EN_0 = 0,
    #[doc = "1: Enables Bank41 of the SRAM"]
    BNK41_EN_1 = 1,
}
impl From<BNK41_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK41_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK41_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK41_EN_A {
        match self.bits {
            false => BNK41_EN_A::BNK41_EN_0,
            true => BNK41_EN_A::BNK41_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK41_EN_0`"]
    #[inline(always)]
    pub fn is_bnk41_en_0(&self) -> bool {
        *self == BNK41_EN_A::BNK41_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK41_EN_1`"]
    #[inline(always)]
    pub fn is_bnk41_en_1(&self) -> bool {
        *self == BNK41_EN_A::BNK41_EN_1
    }
}
#[doc = "Field `BNK41_EN` writer - When 1, enables Bank41 of the SRAM"]
pub type BNK41_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK41_EN_A, O>;
impl<'a, const O: u8> BNK41_EN_W<'a, O> {
    #[doc = "Disables Bank41 of the SRAM"]
    #[inline(always)]
    pub fn bnk41_en_0(self) -> &'a mut W {
        self.variant(BNK41_EN_A::BNK41_EN_0)
    }
    #[doc = "Enables Bank41 of the SRAM"]
    #[inline(always)]
    pub fn bnk41_en_1(self) -> &'a mut W {
        self.variant(BNK41_EN_A::BNK41_EN_1)
    }
}
#[doc = "Field `BNK42_EN` reader - When 1, enables Bank42 of the SRAM"]
pub type BNK42_EN_R = crate::BitReader<BNK42_EN_A>;
#[doc = "When 1, enables Bank42 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK42_EN_A {
    #[doc = "0: Disables Bank42 of the SRAM"]
    BNK42_EN_0 = 0,
    #[doc = "1: Enables Bank42 of the SRAM"]
    BNK42_EN_1 = 1,
}
impl From<BNK42_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK42_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK42_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK42_EN_A {
        match self.bits {
            false => BNK42_EN_A::BNK42_EN_0,
            true => BNK42_EN_A::BNK42_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK42_EN_0`"]
    #[inline(always)]
    pub fn is_bnk42_en_0(&self) -> bool {
        *self == BNK42_EN_A::BNK42_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK42_EN_1`"]
    #[inline(always)]
    pub fn is_bnk42_en_1(&self) -> bool {
        *self == BNK42_EN_A::BNK42_EN_1
    }
}
#[doc = "Field `BNK42_EN` writer - When 1, enables Bank42 of the SRAM"]
pub type BNK42_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK42_EN_A, O>;
impl<'a, const O: u8> BNK42_EN_W<'a, O> {
    #[doc = "Disables Bank42 of the SRAM"]
    #[inline(always)]
    pub fn bnk42_en_0(self) -> &'a mut W {
        self.variant(BNK42_EN_A::BNK42_EN_0)
    }
    #[doc = "Enables Bank42 of the SRAM"]
    #[inline(always)]
    pub fn bnk42_en_1(self) -> &'a mut W {
        self.variant(BNK42_EN_A::BNK42_EN_1)
    }
}
#[doc = "Field `BNK43_EN` reader - When 1, enables Bank43 of the SRAM"]
pub type BNK43_EN_R = crate::BitReader<BNK43_EN_A>;
#[doc = "When 1, enables Bank43 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK43_EN_A {
    #[doc = "0: Disables Bank43 of the SRAM"]
    BNK43_EN_0 = 0,
    #[doc = "1: Enables Bank43 of the SRAM"]
    BNK43_EN_1 = 1,
}
impl From<BNK43_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK43_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK43_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK43_EN_A {
        match self.bits {
            false => BNK43_EN_A::BNK43_EN_0,
            true => BNK43_EN_A::BNK43_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK43_EN_0`"]
    #[inline(always)]
    pub fn is_bnk43_en_0(&self) -> bool {
        *self == BNK43_EN_A::BNK43_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK43_EN_1`"]
    #[inline(always)]
    pub fn is_bnk43_en_1(&self) -> bool {
        *self == BNK43_EN_A::BNK43_EN_1
    }
}
#[doc = "Field `BNK43_EN` writer - When 1, enables Bank43 of the SRAM"]
pub type BNK43_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK43_EN_A, O>;
impl<'a, const O: u8> BNK43_EN_W<'a, O> {
    #[doc = "Disables Bank43 of the SRAM"]
    #[inline(always)]
    pub fn bnk43_en_0(self) -> &'a mut W {
        self.variant(BNK43_EN_A::BNK43_EN_0)
    }
    #[doc = "Enables Bank43 of the SRAM"]
    #[inline(always)]
    pub fn bnk43_en_1(self) -> &'a mut W {
        self.variant(BNK43_EN_A::BNK43_EN_1)
    }
}
#[doc = "Field `BNK44_EN` reader - When 1, enables Bank44 of the SRAM"]
pub type BNK44_EN_R = crate::BitReader<BNK44_EN_A>;
#[doc = "When 1, enables Bank44 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK44_EN_A {
    #[doc = "0: Disables Bank44 of the SRAM"]
    BNK44_EN_0 = 0,
    #[doc = "1: Enables Bank44 of the SRAM"]
    BNK44_EN_1 = 1,
}
impl From<BNK44_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK44_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK44_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK44_EN_A {
        match self.bits {
            false => BNK44_EN_A::BNK44_EN_0,
            true => BNK44_EN_A::BNK44_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK44_EN_0`"]
    #[inline(always)]
    pub fn is_bnk44_en_0(&self) -> bool {
        *self == BNK44_EN_A::BNK44_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK44_EN_1`"]
    #[inline(always)]
    pub fn is_bnk44_en_1(&self) -> bool {
        *self == BNK44_EN_A::BNK44_EN_1
    }
}
#[doc = "Field `BNK44_EN` writer - When 1, enables Bank44 of the SRAM"]
pub type BNK44_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK44_EN_A, O>;
impl<'a, const O: u8> BNK44_EN_W<'a, O> {
    #[doc = "Disables Bank44 of the SRAM"]
    #[inline(always)]
    pub fn bnk44_en_0(self) -> &'a mut W {
        self.variant(BNK44_EN_A::BNK44_EN_0)
    }
    #[doc = "Enables Bank44 of the SRAM"]
    #[inline(always)]
    pub fn bnk44_en_1(self) -> &'a mut W {
        self.variant(BNK44_EN_A::BNK44_EN_1)
    }
}
#[doc = "Field `BNK45_EN` reader - When 1, enables Bank45 of the SRAM"]
pub type BNK45_EN_R = crate::BitReader<BNK45_EN_A>;
#[doc = "When 1, enables Bank45 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK45_EN_A {
    #[doc = "0: Disables Bank45 of the SRAM"]
    BNK45_EN_0 = 0,
    #[doc = "1: Enables Bank45 of the SRAM"]
    BNK45_EN_1 = 1,
}
impl From<BNK45_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK45_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK45_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK45_EN_A {
        match self.bits {
            false => BNK45_EN_A::BNK45_EN_0,
            true => BNK45_EN_A::BNK45_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK45_EN_0`"]
    #[inline(always)]
    pub fn is_bnk45_en_0(&self) -> bool {
        *self == BNK45_EN_A::BNK45_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK45_EN_1`"]
    #[inline(always)]
    pub fn is_bnk45_en_1(&self) -> bool {
        *self == BNK45_EN_A::BNK45_EN_1
    }
}
#[doc = "Field `BNK45_EN` writer - When 1, enables Bank45 of the SRAM"]
pub type BNK45_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK45_EN_A, O>;
impl<'a, const O: u8> BNK45_EN_W<'a, O> {
    #[doc = "Disables Bank45 of the SRAM"]
    #[inline(always)]
    pub fn bnk45_en_0(self) -> &'a mut W {
        self.variant(BNK45_EN_A::BNK45_EN_0)
    }
    #[doc = "Enables Bank45 of the SRAM"]
    #[inline(always)]
    pub fn bnk45_en_1(self) -> &'a mut W {
        self.variant(BNK45_EN_A::BNK45_EN_1)
    }
}
#[doc = "Field `BNK46_EN` reader - When 1, enables Bank46 of the SRAM"]
pub type BNK46_EN_R = crate::BitReader<BNK46_EN_A>;
#[doc = "When 1, enables Bank46 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK46_EN_A {
    #[doc = "0: Disables Bank46 of the SRAM"]
    BNK46_EN_0 = 0,
    #[doc = "1: Enables Bank46 of the SRAM"]
    BNK46_EN_1 = 1,
}
impl From<BNK46_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK46_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK46_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK46_EN_A {
        match self.bits {
            false => BNK46_EN_A::BNK46_EN_0,
            true => BNK46_EN_A::BNK46_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK46_EN_0`"]
    #[inline(always)]
    pub fn is_bnk46_en_0(&self) -> bool {
        *self == BNK46_EN_A::BNK46_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK46_EN_1`"]
    #[inline(always)]
    pub fn is_bnk46_en_1(&self) -> bool {
        *self == BNK46_EN_A::BNK46_EN_1
    }
}
#[doc = "Field `BNK46_EN` writer - When 1, enables Bank46 of the SRAM"]
pub type BNK46_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK46_EN_A, O>;
impl<'a, const O: u8> BNK46_EN_W<'a, O> {
    #[doc = "Disables Bank46 of the SRAM"]
    #[inline(always)]
    pub fn bnk46_en_0(self) -> &'a mut W {
        self.variant(BNK46_EN_A::BNK46_EN_0)
    }
    #[doc = "Enables Bank46 of the SRAM"]
    #[inline(always)]
    pub fn bnk46_en_1(self) -> &'a mut W {
        self.variant(BNK46_EN_A::BNK46_EN_1)
    }
}
#[doc = "Field `BNK47_EN` reader - When 1, enables Bank47 of the SRAM"]
pub type BNK47_EN_R = crate::BitReader<BNK47_EN_A>;
#[doc = "When 1, enables Bank47 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK47_EN_A {
    #[doc = "0: Disables Bank47 of the SRAM"]
    BNK47_EN_0 = 0,
    #[doc = "1: Enables Bank47 of the SRAM"]
    BNK47_EN_1 = 1,
}
impl From<BNK47_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK47_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK47_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK47_EN_A {
        match self.bits {
            false => BNK47_EN_A::BNK47_EN_0,
            true => BNK47_EN_A::BNK47_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK47_EN_0`"]
    #[inline(always)]
    pub fn is_bnk47_en_0(&self) -> bool {
        *self == BNK47_EN_A::BNK47_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK47_EN_1`"]
    #[inline(always)]
    pub fn is_bnk47_en_1(&self) -> bool {
        *self == BNK47_EN_A::BNK47_EN_1
    }
}
#[doc = "Field `BNK47_EN` writer - When 1, enables Bank47 of the SRAM"]
pub type BNK47_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK47_EN_A, O>;
impl<'a, const O: u8> BNK47_EN_W<'a, O> {
    #[doc = "Disables Bank47 of the SRAM"]
    #[inline(always)]
    pub fn bnk47_en_0(self) -> &'a mut W {
        self.variant(BNK47_EN_A::BNK47_EN_0)
    }
    #[doc = "Enables Bank47 of the SRAM"]
    #[inline(always)]
    pub fn bnk47_en_1(self) -> &'a mut W {
        self.variant(BNK47_EN_A::BNK47_EN_1)
    }
}
#[doc = "Field `BNK48_EN` reader - When 1, enables Bank48 of the SRAM"]
pub type BNK48_EN_R = crate::BitReader<BNK48_EN_A>;
#[doc = "When 1, enables Bank48 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK48_EN_A {
    #[doc = "0: Disables Bank48 of the SRAM"]
    BNK48_EN_0 = 0,
    #[doc = "1: Enables Bank48 of the SRAM"]
    BNK48_EN_1 = 1,
}
impl From<BNK48_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK48_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK48_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK48_EN_A {
        match self.bits {
            false => BNK48_EN_A::BNK48_EN_0,
            true => BNK48_EN_A::BNK48_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK48_EN_0`"]
    #[inline(always)]
    pub fn is_bnk48_en_0(&self) -> bool {
        *self == BNK48_EN_A::BNK48_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK48_EN_1`"]
    #[inline(always)]
    pub fn is_bnk48_en_1(&self) -> bool {
        *self == BNK48_EN_A::BNK48_EN_1
    }
}
#[doc = "Field `BNK48_EN` writer - When 1, enables Bank48 of the SRAM"]
pub type BNK48_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK48_EN_A, O>;
impl<'a, const O: u8> BNK48_EN_W<'a, O> {
    #[doc = "Disables Bank48 of the SRAM"]
    #[inline(always)]
    pub fn bnk48_en_0(self) -> &'a mut W {
        self.variant(BNK48_EN_A::BNK48_EN_0)
    }
    #[doc = "Enables Bank48 of the SRAM"]
    #[inline(always)]
    pub fn bnk48_en_1(self) -> &'a mut W {
        self.variant(BNK48_EN_A::BNK48_EN_1)
    }
}
#[doc = "Field `BNK49_EN` reader - When 1, enables Bank49 of the SRAM"]
pub type BNK49_EN_R = crate::BitReader<BNK49_EN_A>;
#[doc = "When 1, enables Bank49 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK49_EN_A {
    #[doc = "0: Disables Bank49 of the SRAM"]
    BNK49_EN_0 = 0,
    #[doc = "1: Enables Bank49 of the SRAM"]
    BNK49_EN_1 = 1,
}
impl From<BNK49_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK49_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK49_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK49_EN_A {
        match self.bits {
            false => BNK49_EN_A::BNK49_EN_0,
            true => BNK49_EN_A::BNK49_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK49_EN_0`"]
    #[inline(always)]
    pub fn is_bnk49_en_0(&self) -> bool {
        *self == BNK49_EN_A::BNK49_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK49_EN_1`"]
    #[inline(always)]
    pub fn is_bnk49_en_1(&self) -> bool {
        *self == BNK49_EN_A::BNK49_EN_1
    }
}
#[doc = "Field `BNK49_EN` writer - When 1, enables Bank49 of the SRAM"]
pub type BNK49_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK49_EN_A, O>;
impl<'a, const O: u8> BNK49_EN_W<'a, O> {
    #[doc = "Disables Bank49 of the SRAM"]
    #[inline(always)]
    pub fn bnk49_en_0(self) -> &'a mut W {
        self.variant(BNK49_EN_A::BNK49_EN_0)
    }
    #[doc = "Enables Bank49 of the SRAM"]
    #[inline(always)]
    pub fn bnk49_en_1(self) -> &'a mut W {
        self.variant(BNK49_EN_A::BNK49_EN_1)
    }
}
#[doc = "Field `BNK50_EN` reader - When 1, enables Bank50 of the SRAM"]
pub type BNK50_EN_R = crate::BitReader<BNK50_EN_A>;
#[doc = "When 1, enables Bank50 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK50_EN_A {
    #[doc = "0: Disables Bank50 of the SRAM"]
    BNK50_EN_0 = 0,
    #[doc = "1: Enables Bank50 of the SRAM"]
    BNK50_EN_1 = 1,
}
impl From<BNK50_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK50_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK50_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK50_EN_A {
        match self.bits {
            false => BNK50_EN_A::BNK50_EN_0,
            true => BNK50_EN_A::BNK50_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK50_EN_0`"]
    #[inline(always)]
    pub fn is_bnk50_en_0(&self) -> bool {
        *self == BNK50_EN_A::BNK50_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK50_EN_1`"]
    #[inline(always)]
    pub fn is_bnk50_en_1(&self) -> bool {
        *self == BNK50_EN_A::BNK50_EN_1
    }
}
#[doc = "Field `BNK50_EN` writer - When 1, enables Bank50 of the SRAM"]
pub type BNK50_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK50_EN_A, O>;
impl<'a, const O: u8> BNK50_EN_W<'a, O> {
    #[doc = "Disables Bank50 of the SRAM"]
    #[inline(always)]
    pub fn bnk50_en_0(self) -> &'a mut W {
        self.variant(BNK50_EN_A::BNK50_EN_0)
    }
    #[doc = "Enables Bank50 of the SRAM"]
    #[inline(always)]
    pub fn bnk50_en_1(self) -> &'a mut W {
        self.variant(BNK50_EN_A::BNK50_EN_1)
    }
}
#[doc = "Field `BNK51_EN` reader - When 1, enables Bank51 of the SRAM"]
pub type BNK51_EN_R = crate::BitReader<BNK51_EN_A>;
#[doc = "When 1, enables Bank51 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK51_EN_A {
    #[doc = "0: Disables Bank51 of the SRAM"]
    BNK51_EN_0 = 0,
    #[doc = "1: Enables Bank51 of the SRAM"]
    BNK51_EN_1 = 1,
}
impl From<BNK51_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK51_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK51_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK51_EN_A {
        match self.bits {
            false => BNK51_EN_A::BNK51_EN_0,
            true => BNK51_EN_A::BNK51_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK51_EN_0`"]
    #[inline(always)]
    pub fn is_bnk51_en_0(&self) -> bool {
        *self == BNK51_EN_A::BNK51_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK51_EN_1`"]
    #[inline(always)]
    pub fn is_bnk51_en_1(&self) -> bool {
        *self == BNK51_EN_A::BNK51_EN_1
    }
}
#[doc = "Field `BNK51_EN` writer - When 1, enables Bank51 of the SRAM"]
pub type BNK51_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK51_EN_A, O>;
impl<'a, const O: u8> BNK51_EN_W<'a, O> {
    #[doc = "Disables Bank51 of the SRAM"]
    #[inline(always)]
    pub fn bnk51_en_0(self) -> &'a mut W {
        self.variant(BNK51_EN_A::BNK51_EN_0)
    }
    #[doc = "Enables Bank51 of the SRAM"]
    #[inline(always)]
    pub fn bnk51_en_1(self) -> &'a mut W {
        self.variant(BNK51_EN_A::BNK51_EN_1)
    }
}
#[doc = "Field `BNK52_EN` reader - When 1, enables Bank52 of the SRAM"]
pub type BNK52_EN_R = crate::BitReader<BNK52_EN_A>;
#[doc = "When 1, enables Bank52 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK52_EN_A {
    #[doc = "0: Disables Bank52 of the SRAM"]
    BNK52_EN_0 = 0,
    #[doc = "1: Enables Bank52 of the SRAM"]
    BNK52_EN_1 = 1,
}
impl From<BNK52_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK52_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK52_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK52_EN_A {
        match self.bits {
            false => BNK52_EN_A::BNK52_EN_0,
            true => BNK52_EN_A::BNK52_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK52_EN_0`"]
    #[inline(always)]
    pub fn is_bnk52_en_0(&self) -> bool {
        *self == BNK52_EN_A::BNK52_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK52_EN_1`"]
    #[inline(always)]
    pub fn is_bnk52_en_1(&self) -> bool {
        *self == BNK52_EN_A::BNK52_EN_1
    }
}
#[doc = "Field `BNK52_EN` writer - When 1, enables Bank52 of the SRAM"]
pub type BNK52_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK52_EN_A, O>;
impl<'a, const O: u8> BNK52_EN_W<'a, O> {
    #[doc = "Disables Bank52 of the SRAM"]
    #[inline(always)]
    pub fn bnk52_en_0(self) -> &'a mut W {
        self.variant(BNK52_EN_A::BNK52_EN_0)
    }
    #[doc = "Enables Bank52 of the SRAM"]
    #[inline(always)]
    pub fn bnk52_en_1(self) -> &'a mut W {
        self.variant(BNK52_EN_A::BNK52_EN_1)
    }
}
#[doc = "Field `BNK53_EN` reader - When 1, enables Bank53 of the SRAM"]
pub type BNK53_EN_R = crate::BitReader<BNK53_EN_A>;
#[doc = "When 1, enables Bank53 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK53_EN_A {
    #[doc = "0: Disables Bank53 of the SRAM"]
    BNK53_EN_0 = 0,
    #[doc = "1: Enables Bank53 of the SRAM"]
    BNK53_EN_1 = 1,
}
impl From<BNK53_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK53_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK53_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK53_EN_A {
        match self.bits {
            false => BNK53_EN_A::BNK53_EN_0,
            true => BNK53_EN_A::BNK53_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK53_EN_0`"]
    #[inline(always)]
    pub fn is_bnk53_en_0(&self) -> bool {
        *self == BNK53_EN_A::BNK53_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK53_EN_1`"]
    #[inline(always)]
    pub fn is_bnk53_en_1(&self) -> bool {
        *self == BNK53_EN_A::BNK53_EN_1
    }
}
#[doc = "Field `BNK53_EN` writer - When 1, enables Bank53 of the SRAM"]
pub type BNK53_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK53_EN_A, O>;
impl<'a, const O: u8> BNK53_EN_W<'a, O> {
    #[doc = "Disables Bank53 of the SRAM"]
    #[inline(always)]
    pub fn bnk53_en_0(self) -> &'a mut W {
        self.variant(BNK53_EN_A::BNK53_EN_0)
    }
    #[doc = "Enables Bank53 of the SRAM"]
    #[inline(always)]
    pub fn bnk53_en_1(self) -> &'a mut W {
        self.variant(BNK53_EN_A::BNK53_EN_1)
    }
}
#[doc = "Field `BNK54_EN` reader - When 1, enables Bank54 of the SRAM"]
pub type BNK54_EN_R = crate::BitReader<BNK54_EN_A>;
#[doc = "When 1, enables Bank54 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK54_EN_A {
    #[doc = "0: Disables Bank54 of the SRAM"]
    BNK54_EN_0 = 0,
    #[doc = "1: Enables Bank54 of the SRAM"]
    BNK54_EN_1 = 1,
}
impl From<BNK54_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK54_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK54_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK54_EN_A {
        match self.bits {
            false => BNK54_EN_A::BNK54_EN_0,
            true => BNK54_EN_A::BNK54_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK54_EN_0`"]
    #[inline(always)]
    pub fn is_bnk54_en_0(&self) -> bool {
        *self == BNK54_EN_A::BNK54_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK54_EN_1`"]
    #[inline(always)]
    pub fn is_bnk54_en_1(&self) -> bool {
        *self == BNK54_EN_A::BNK54_EN_1
    }
}
#[doc = "Field `BNK54_EN` writer - When 1, enables Bank54 of the SRAM"]
pub type BNK54_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK54_EN_A, O>;
impl<'a, const O: u8> BNK54_EN_W<'a, O> {
    #[doc = "Disables Bank54 of the SRAM"]
    #[inline(always)]
    pub fn bnk54_en_0(self) -> &'a mut W {
        self.variant(BNK54_EN_A::BNK54_EN_0)
    }
    #[doc = "Enables Bank54 of the SRAM"]
    #[inline(always)]
    pub fn bnk54_en_1(self) -> &'a mut W {
        self.variant(BNK54_EN_A::BNK54_EN_1)
    }
}
#[doc = "Field `BNK55_EN` reader - When 1, enables Bank55 of the SRAM"]
pub type BNK55_EN_R = crate::BitReader<BNK55_EN_A>;
#[doc = "When 1, enables Bank55 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK55_EN_A {
    #[doc = "0: Disables Bank55 of the SRAM"]
    BNK55_EN_0 = 0,
    #[doc = "1: Enables Bank55 of the SRAM"]
    BNK55_EN_1 = 1,
}
impl From<BNK55_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK55_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK55_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK55_EN_A {
        match self.bits {
            false => BNK55_EN_A::BNK55_EN_0,
            true => BNK55_EN_A::BNK55_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK55_EN_0`"]
    #[inline(always)]
    pub fn is_bnk55_en_0(&self) -> bool {
        *self == BNK55_EN_A::BNK55_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK55_EN_1`"]
    #[inline(always)]
    pub fn is_bnk55_en_1(&self) -> bool {
        *self == BNK55_EN_A::BNK55_EN_1
    }
}
#[doc = "Field `BNK55_EN` writer - When 1, enables Bank55 of the SRAM"]
pub type BNK55_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK55_EN_A, O>;
impl<'a, const O: u8> BNK55_EN_W<'a, O> {
    #[doc = "Disables Bank55 of the SRAM"]
    #[inline(always)]
    pub fn bnk55_en_0(self) -> &'a mut W {
        self.variant(BNK55_EN_A::BNK55_EN_0)
    }
    #[doc = "Enables Bank55 of the SRAM"]
    #[inline(always)]
    pub fn bnk55_en_1(self) -> &'a mut W {
        self.variant(BNK55_EN_A::BNK55_EN_1)
    }
}
#[doc = "Field `BNK56_EN` reader - When 1, enables Bank56 of the SRAM"]
pub type BNK56_EN_R = crate::BitReader<BNK56_EN_A>;
#[doc = "When 1, enables Bank56 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK56_EN_A {
    #[doc = "0: Disables Bank56 of the SRAM"]
    BNK56_EN_0 = 0,
    #[doc = "1: Enables Bank56 of the SRAM"]
    BNK56_EN_1 = 1,
}
impl From<BNK56_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK56_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK56_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK56_EN_A {
        match self.bits {
            false => BNK56_EN_A::BNK56_EN_0,
            true => BNK56_EN_A::BNK56_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK56_EN_0`"]
    #[inline(always)]
    pub fn is_bnk56_en_0(&self) -> bool {
        *self == BNK56_EN_A::BNK56_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK56_EN_1`"]
    #[inline(always)]
    pub fn is_bnk56_en_1(&self) -> bool {
        *self == BNK56_EN_A::BNK56_EN_1
    }
}
#[doc = "Field `BNK56_EN` writer - When 1, enables Bank56 of the SRAM"]
pub type BNK56_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK56_EN_A, O>;
impl<'a, const O: u8> BNK56_EN_W<'a, O> {
    #[doc = "Disables Bank56 of the SRAM"]
    #[inline(always)]
    pub fn bnk56_en_0(self) -> &'a mut W {
        self.variant(BNK56_EN_A::BNK56_EN_0)
    }
    #[doc = "Enables Bank56 of the SRAM"]
    #[inline(always)]
    pub fn bnk56_en_1(self) -> &'a mut W {
        self.variant(BNK56_EN_A::BNK56_EN_1)
    }
}
#[doc = "Field `BNK57_EN` reader - When 1, enables Bank57 of the SRAM"]
pub type BNK57_EN_R = crate::BitReader<BNK57_EN_A>;
#[doc = "When 1, enables Bank57 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK57_EN_A {
    #[doc = "0: Disables Bank57 of the SRAM"]
    BNK57_EN_0 = 0,
    #[doc = "1: Enables Bank57 of the SRAM"]
    BNK57_EN_1 = 1,
}
impl From<BNK57_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK57_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK57_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK57_EN_A {
        match self.bits {
            false => BNK57_EN_A::BNK57_EN_0,
            true => BNK57_EN_A::BNK57_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK57_EN_0`"]
    #[inline(always)]
    pub fn is_bnk57_en_0(&self) -> bool {
        *self == BNK57_EN_A::BNK57_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK57_EN_1`"]
    #[inline(always)]
    pub fn is_bnk57_en_1(&self) -> bool {
        *self == BNK57_EN_A::BNK57_EN_1
    }
}
#[doc = "Field `BNK57_EN` writer - When 1, enables Bank57 of the SRAM"]
pub type BNK57_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK57_EN_A, O>;
impl<'a, const O: u8> BNK57_EN_W<'a, O> {
    #[doc = "Disables Bank57 of the SRAM"]
    #[inline(always)]
    pub fn bnk57_en_0(self) -> &'a mut W {
        self.variant(BNK57_EN_A::BNK57_EN_0)
    }
    #[doc = "Enables Bank57 of the SRAM"]
    #[inline(always)]
    pub fn bnk57_en_1(self) -> &'a mut W {
        self.variant(BNK57_EN_A::BNK57_EN_1)
    }
}
#[doc = "Field `BNK58_EN` reader - When 1, enables Bank58 of the SRAM"]
pub type BNK58_EN_R = crate::BitReader<BNK58_EN_A>;
#[doc = "When 1, enables Bank58 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK58_EN_A {
    #[doc = "0: Disables Bank58 of the SRAM"]
    BNK58_EN_0 = 0,
    #[doc = "1: Enables Bank58 of the SRAM"]
    BNK58_EN_1 = 1,
}
impl From<BNK58_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK58_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK58_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK58_EN_A {
        match self.bits {
            false => BNK58_EN_A::BNK58_EN_0,
            true => BNK58_EN_A::BNK58_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK58_EN_0`"]
    #[inline(always)]
    pub fn is_bnk58_en_0(&self) -> bool {
        *self == BNK58_EN_A::BNK58_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK58_EN_1`"]
    #[inline(always)]
    pub fn is_bnk58_en_1(&self) -> bool {
        *self == BNK58_EN_A::BNK58_EN_1
    }
}
#[doc = "Field `BNK58_EN` writer - When 1, enables Bank58 of the SRAM"]
pub type BNK58_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK58_EN_A, O>;
impl<'a, const O: u8> BNK58_EN_W<'a, O> {
    #[doc = "Disables Bank58 of the SRAM"]
    #[inline(always)]
    pub fn bnk58_en_0(self) -> &'a mut W {
        self.variant(BNK58_EN_A::BNK58_EN_0)
    }
    #[doc = "Enables Bank58 of the SRAM"]
    #[inline(always)]
    pub fn bnk58_en_1(self) -> &'a mut W {
        self.variant(BNK58_EN_A::BNK58_EN_1)
    }
}
#[doc = "Field `BNK59_EN` reader - When 1, enables Bank59 of the SRAM"]
pub type BNK59_EN_R = crate::BitReader<BNK59_EN_A>;
#[doc = "When 1, enables Bank59 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK59_EN_A {
    #[doc = "0: Disables Bank59 of the SRAM"]
    BNK59_EN_0 = 0,
    #[doc = "1: Enables Bank59 of the SRAM"]
    BNK59_EN_1 = 1,
}
impl From<BNK59_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK59_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK59_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK59_EN_A {
        match self.bits {
            false => BNK59_EN_A::BNK59_EN_0,
            true => BNK59_EN_A::BNK59_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK59_EN_0`"]
    #[inline(always)]
    pub fn is_bnk59_en_0(&self) -> bool {
        *self == BNK59_EN_A::BNK59_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK59_EN_1`"]
    #[inline(always)]
    pub fn is_bnk59_en_1(&self) -> bool {
        *self == BNK59_EN_A::BNK59_EN_1
    }
}
#[doc = "Field `BNK59_EN` writer - When 1, enables Bank59 of the SRAM"]
pub type BNK59_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK59_EN_A, O>;
impl<'a, const O: u8> BNK59_EN_W<'a, O> {
    #[doc = "Disables Bank59 of the SRAM"]
    #[inline(always)]
    pub fn bnk59_en_0(self) -> &'a mut W {
        self.variant(BNK59_EN_A::BNK59_EN_0)
    }
    #[doc = "Enables Bank59 of the SRAM"]
    #[inline(always)]
    pub fn bnk59_en_1(self) -> &'a mut W {
        self.variant(BNK59_EN_A::BNK59_EN_1)
    }
}
#[doc = "Field `BNK60_EN` reader - When 1, enables Bank60 of the SRAM"]
pub type BNK60_EN_R = crate::BitReader<BNK60_EN_A>;
#[doc = "When 1, enables Bank60 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK60_EN_A {
    #[doc = "0: Disables Bank60 of the SRAM"]
    BNK60_EN_0 = 0,
    #[doc = "1: Enables Bank60 of the SRAM"]
    BNK60_EN_1 = 1,
}
impl From<BNK60_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK60_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK60_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK60_EN_A {
        match self.bits {
            false => BNK60_EN_A::BNK60_EN_0,
            true => BNK60_EN_A::BNK60_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK60_EN_0`"]
    #[inline(always)]
    pub fn is_bnk60_en_0(&self) -> bool {
        *self == BNK60_EN_A::BNK60_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK60_EN_1`"]
    #[inline(always)]
    pub fn is_bnk60_en_1(&self) -> bool {
        *self == BNK60_EN_A::BNK60_EN_1
    }
}
#[doc = "Field `BNK60_EN` writer - When 1, enables Bank60 of the SRAM"]
pub type BNK60_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK60_EN_A, O>;
impl<'a, const O: u8> BNK60_EN_W<'a, O> {
    #[doc = "Disables Bank60 of the SRAM"]
    #[inline(always)]
    pub fn bnk60_en_0(self) -> &'a mut W {
        self.variant(BNK60_EN_A::BNK60_EN_0)
    }
    #[doc = "Enables Bank60 of the SRAM"]
    #[inline(always)]
    pub fn bnk60_en_1(self) -> &'a mut W {
        self.variant(BNK60_EN_A::BNK60_EN_1)
    }
}
#[doc = "Field `BNK61_EN` reader - When 1, enables Bank61 of the SRAM"]
pub type BNK61_EN_R = crate::BitReader<BNK61_EN_A>;
#[doc = "When 1, enables Bank61 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK61_EN_A {
    #[doc = "0: Disables Bank61 of the SRAM"]
    BNK61_EN_0 = 0,
    #[doc = "1: Enables Bank61 of the SRAM"]
    BNK61_EN_1 = 1,
}
impl From<BNK61_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK61_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK61_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK61_EN_A {
        match self.bits {
            false => BNK61_EN_A::BNK61_EN_0,
            true => BNK61_EN_A::BNK61_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK61_EN_0`"]
    #[inline(always)]
    pub fn is_bnk61_en_0(&self) -> bool {
        *self == BNK61_EN_A::BNK61_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK61_EN_1`"]
    #[inline(always)]
    pub fn is_bnk61_en_1(&self) -> bool {
        *self == BNK61_EN_A::BNK61_EN_1
    }
}
#[doc = "Field `BNK61_EN` writer - When 1, enables Bank61 of the SRAM"]
pub type BNK61_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK61_EN_A, O>;
impl<'a, const O: u8> BNK61_EN_W<'a, O> {
    #[doc = "Disables Bank61 of the SRAM"]
    #[inline(always)]
    pub fn bnk61_en_0(self) -> &'a mut W {
        self.variant(BNK61_EN_A::BNK61_EN_0)
    }
    #[doc = "Enables Bank61 of the SRAM"]
    #[inline(always)]
    pub fn bnk61_en_1(self) -> &'a mut W {
        self.variant(BNK61_EN_A::BNK61_EN_1)
    }
}
#[doc = "Field `BNK62_EN` reader - When 1, enables Bank62 of the SRAM"]
pub type BNK62_EN_R = crate::BitReader<BNK62_EN_A>;
#[doc = "When 1, enables Bank62 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK62_EN_A {
    #[doc = "0: Disables Bank62 of the SRAM"]
    BNK62_EN_0 = 0,
    #[doc = "1: Enables Bank62 of the SRAM"]
    BNK62_EN_1 = 1,
}
impl From<BNK62_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK62_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK62_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK62_EN_A {
        match self.bits {
            false => BNK62_EN_A::BNK62_EN_0,
            true => BNK62_EN_A::BNK62_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK62_EN_0`"]
    #[inline(always)]
    pub fn is_bnk62_en_0(&self) -> bool {
        *self == BNK62_EN_A::BNK62_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK62_EN_1`"]
    #[inline(always)]
    pub fn is_bnk62_en_1(&self) -> bool {
        *self == BNK62_EN_A::BNK62_EN_1
    }
}
#[doc = "Field `BNK62_EN` writer - When 1, enables Bank62 of the SRAM"]
pub type BNK62_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK62_EN_A, O>;
impl<'a, const O: u8> BNK62_EN_W<'a, O> {
    #[doc = "Disables Bank62 of the SRAM"]
    #[inline(always)]
    pub fn bnk62_en_0(self) -> &'a mut W {
        self.variant(BNK62_EN_A::BNK62_EN_0)
    }
    #[doc = "Enables Bank62 of the SRAM"]
    #[inline(always)]
    pub fn bnk62_en_1(self) -> &'a mut W {
        self.variant(BNK62_EN_A::BNK62_EN_1)
    }
}
#[doc = "Field `BNK63_EN` reader - When 1, enables Bank63 of the SRAM"]
pub type BNK63_EN_R = crate::BitReader<BNK63_EN_A>;
#[doc = "When 1, enables Bank63 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK63_EN_A {
    #[doc = "0: Disables Bank63 of the SRAM"]
    BNK63_EN_0 = 0,
    #[doc = "1: Enables Bank63 of the SRAM"]
    BNK63_EN_1 = 1,
}
impl From<BNK63_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK63_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK63_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK63_EN_A {
        match self.bits {
            false => BNK63_EN_A::BNK63_EN_0,
            true => BNK63_EN_A::BNK63_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK63_EN_0`"]
    #[inline(always)]
    pub fn is_bnk63_en_0(&self) -> bool {
        *self == BNK63_EN_A::BNK63_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK63_EN_1`"]
    #[inline(always)]
    pub fn is_bnk63_en_1(&self) -> bool {
        *self == BNK63_EN_A::BNK63_EN_1
    }
}
#[doc = "Field `BNK63_EN` writer - When 1, enables Bank63 of the SRAM"]
pub type BNK63_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL1_SPEC, BNK63_EN_A, O>;
impl<'a, const O: u8> BNK63_EN_W<'a, O> {
    #[doc = "Disables Bank63 of the SRAM"]
    #[inline(always)]
    pub fn bnk63_en_0(self) -> &'a mut W {
        self.variant(BNK63_EN_A::BNK63_EN_0)
    }
    #[doc = "Enables Bank63 of the SRAM"]
    #[inline(always)]
    pub fn bnk63_en_1(self) -> &'a mut W {
        self.variant(BNK63_EN_A::BNK63_EN_1)
    }
}
impl R {
    #[doc = "Bit 0 - When 1, enables Bank32 of the SRAM"]
    #[inline(always)]
    pub fn bnk32_en(&self) -> BNK32_EN_R {
        BNK32_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, enables Bank33 of the SRAM"]
    #[inline(always)]
    pub fn bnk33_en(&self) -> BNK33_EN_R {
        BNK33_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, enables Bank34 of the SRAM"]
    #[inline(always)]
    pub fn bnk34_en(&self) -> BNK34_EN_R {
        BNK34_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, enables Bank35 of the SRAM"]
    #[inline(always)]
    pub fn bnk35_en(&self) -> BNK35_EN_R {
        BNK35_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, enables Bank36 of the SRAM"]
    #[inline(always)]
    pub fn bnk36_en(&self) -> BNK36_EN_R {
        BNK36_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, enables Bank37 of the SRAM"]
    #[inline(always)]
    pub fn bnk37_en(&self) -> BNK37_EN_R {
        BNK37_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, enables Bank38 of the SRAM"]
    #[inline(always)]
    pub fn bnk38_en(&self) -> BNK38_EN_R {
        BNK38_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, enables Bank39 of the SRAM"]
    #[inline(always)]
    pub fn bnk39_en(&self) -> BNK39_EN_R {
        BNK39_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, enables Bank40 of the SRAM"]
    #[inline(always)]
    pub fn bnk40_en(&self) -> BNK40_EN_R {
        BNK40_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, enables Bank41 of the SRAM"]
    #[inline(always)]
    pub fn bnk41_en(&self) -> BNK41_EN_R {
        BNK41_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When 1, enables Bank42 of the SRAM"]
    #[inline(always)]
    pub fn bnk42_en(&self) -> BNK42_EN_R {
        BNK42_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When 1, enables Bank43 of the SRAM"]
    #[inline(always)]
    pub fn bnk43_en(&self) -> BNK43_EN_R {
        BNK43_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When 1, enables Bank44 of the SRAM"]
    #[inline(always)]
    pub fn bnk44_en(&self) -> BNK44_EN_R {
        BNK44_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When 1, enables Bank45 of the SRAM"]
    #[inline(always)]
    pub fn bnk45_en(&self) -> BNK45_EN_R {
        BNK45_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When 1, enables Bank46 of the SRAM"]
    #[inline(always)]
    pub fn bnk46_en(&self) -> BNK46_EN_R {
        BNK46_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When 1, enables Bank47 of the SRAM"]
    #[inline(always)]
    pub fn bnk47_en(&self) -> BNK47_EN_R {
        BNK47_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When 1, enables Bank48 of the SRAM"]
    #[inline(always)]
    pub fn bnk48_en(&self) -> BNK48_EN_R {
        BNK48_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When 1, enables Bank49 of the SRAM"]
    #[inline(always)]
    pub fn bnk49_en(&self) -> BNK49_EN_R {
        BNK49_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When 1, enables Bank50 of the SRAM"]
    #[inline(always)]
    pub fn bnk50_en(&self) -> BNK50_EN_R {
        BNK50_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When 1, enables Bank51 of the SRAM"]
    #[inline(always)]
    pub fn bnk51_en(&self) -> BNK51_EN_R {
        BNK51_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When 1, enables Bank52 of the SRAM"]
    #[inline(always)]
    pub fn bnk52_en(&self) -> BNK52_EN_R {
        BNK52_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When 1, enables Bank53 of the SRAM"]
    #[inline(always)]
    pub fn bnk53_en(&self) -> BNK53_EN_R {
        BNK53_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When 1, enables Bank54 of the SRAM"]
    #[inline(always)]
    pub fn bnk54_en(&self) -> BNK54_EN_R {
        BNK54_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When 1, enables Bank55 of the SRAM"]
    #[inline(always)]
    pub fn bnk55_en(&self) -> BNK55_EN_R {
        BNK55_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When 1, enables Bank56 of the SRAM"]
    #[inline(always)]
    pub fn bnk56_en(&self) -> BNK56_EN_R {
        BNK56_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When 1, enables Bank57 of the SRAM"]
    #[inline(always)]
    pub fn bnk57_en(&self) -> BNK57_EN_R {
        BNK57_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When 1, enables Bank58 of the SRAM"]
    #[inline(always)]
    pub fn bnk58_en(&self) -> BNK58_EN_R {
        BNK58_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When 1, enables Bank59 of the SRAM"]
    #[inline(always)]
    pub fn bnk59_en(&self) -> BNK59_EN_R {
        BNK59_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When 1, enables Bank60 of the SRAM"]
    #[inline(always)]
    pub fn bnk60_en(&self) -> BNK60_EN_R {
        BNK60_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When 1, enables Bank61 of the SRAM"]
    #[inline(always)]
    pub fn bnk61_en(&self) -> BNK61_EN_R {
        BNK61_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When 1, enables Bank62 of the SRAM"]
    #[inline(always)]
    pub fn bnk62_en(&self) -> BNK62_EN_R {
        BNK62_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When 1, enables Bank63 of the SRAM"]
    #[inline(always)]
    pub fn bnk63_en(&self) -> BNK63_EN_R {
        BNK63_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enables Bank32 of the SRAM"]
    #[inline(always)]
    pub fn bnk32_en(&mut self) -> BNK32_EN_W<0> {
        BNK32_EN_W::new(self)
    }
    #[doc = "Bit 1 - When 1, enables Bank33 of the SRAM"]
    #[inline(always)]
    pub fn bnk33_en(&mut self) -> BNK33_EN_W<1> {
        BNK33_EN_W::new(self)
    }
    #[doc = "Bit 2 - When 1, enables Bank34 of the SRAM"]
    #[inline(always)]
    pub fn bnk34_en(&mut self) -> BNK34_EN_W<2> {
        BNK34_EN_W::new(self)
    }
    #[doc = "Bit 3 - When 1, enables Bank35 of the SRAM"]
    #[inline(always)]
    pub fn bnk35_en(&mut self) -> BNK35_EN_W<3> {
        BNK35_EN_W::new(self)
    }
    #[doc = "Bit 4 - When 1, enables Bank36 of the SRAM"]
    #[inline(always)]
    pub fn bnk36_en(&mut self) -> BNK36_EN_W<4> {
        BNK36_EN_W::new(self)
    }
    #[doc = "Bit 5 - When 1, enables Bank37 of the SRAM"]
    #[inline(always)]
    pub fn bnk37_en(&mut self) -> BNK37_EN_W<5> {
        BNK37_EN_W::new(self)
    }
    #[doc = "Bit 6 - When 1, enables Bank38 of the SRAM"]
    #[inline(always)]
    pub fn bnk38_en(&mut self) -> BNK38_EN_W<6> {
        BNK38_EN_W::new(self)
    }
    #[doc = "Bit 7 - When 1, enables Bank39 of the SRAM"]
    #[inline(always)]
    pub fn bnk39_en(&mut self) -> BNK39_EN_W<7> {
        BNK39_EN_W::new(self)
    }
    #[doc = "Bit 8 - When 1, enables Bank40 of the SRAM"]
    #[inline(always)]
    pub fn bnk40_en(&mut self) -> BNK40_EN_W<8> {
        BNK40_EN_W::new(self)
    }
    #[doc = "Bit 9 - When 1, enables Bank41 of the SRAM"]
    #[inline(always)]
    pub fn bnk41_en(&mut self) -> BNK41_EN_W<9> {
        BNK41_EN_W::new(self)
    }
    #[doc = "Bit 10 - When 1, enables Bank42 of the SRAM"]
    #[inline(always)]
    pub fn bnk42_en(&mut self) -> BNK42_EN_W<10> {
        BNK42_EN_W::new(self)
    }
    #[doc = "Bit 11 - When 1, enables Bank43 of the SRAM"]
    #[inline(always)]
    pub fn bnk43_en(&mut self) -> BNK43_EN_W<11> {
        BNK43_EN_W::new(self)
    }
    #[doc = "Bit 12 - When 1, enables Bank44 of the SRAM"]
    #[inline(always)]
    pub fn bnk44_en(&mut self) -> BNK44_EN_W<12> {
        BNK44_EN_W::new(self)
    }
    #[doc = "Bit 13 - When 1, enables Bank45 of the SRAM"]
    #[inline(always)]
    pub fn bnk45_en(&mut self) -> BNK45_EN_W<13> {
        BNK45_EN_W::new(self)
    }
    #[doc = "Bit 14 - When 1, enables Bank46 of the SRAM"]
    #[inline(always)]
    pub fn bnk46_en(&mut self) -> BNK46_EN_W<14> {
        BNK46_EN_W::new(self)
    }
    #[doc = "Bit 15 - When 1, enables Bank47 of the SRAM"]
    #[inline(always)]
    pub fn bnk47_en(&mut self) -> BNK47_EN_W<15> {
        BNK47_EN_W::new(self)
    }
    #[doc = "Bit 16 - When 1, enables Bank48 of the SRAM"]
    #[inline(always)]
    pub fn bnk48_en(&mut self) -> BNK48_EN_W<16> {
        BNK48_EN_W::new(self)
    }
    #[doc = "Bit 17 - When 1, enables Bank49 of the SRAM"]
    #[inline(always)]
    pub fn bnk49_en(&mut self) -> BNK49_EN_W<17> {
        BNK49_EN_W::new(self)
    }
    #[doc = "Bit 18 - When 1, enables Bank50 of the SRAM"]
    #[inline(always)]
    pub fn bnk50_en(&mut self) -> BNK50_EN_W<18> {
        BNK50_EN_W::new(self)
    }
    #[doc = "Bit 19 - When 1, enables Bank51 of the SRAM"]
    #[inline(always)]
    pub fn bnk51_en(&mut self) -> BNK51_EN_W<19> {
        BNK51_EN_W::new(self)
    }
    #[doc = "Bit 20 - When 1, enables Bank52 of the SRAM"]
    #[inline(always)]
    pub fn bnk52_en(&mut self) -> BNK52_EN_W<20> {
        BNK52_EN_W::new(self)
    }
    #[doc = "Bit 21 - When 1, enables Bank53 of the SRAM"]
    #[inline(always)]
    pub fn bnk53_en(&mut self) -> BNK53_EN_W<21> {
        BNK53_EN_W::new(self)
    }
    #[doc = "Bit 22 - When 1, enables Bank54 of the SRAM"]
    #[inline(always)]
    pub fn bnk54_en(&mut self) -> BNK54_EN_W<22> {
        BNK54_EN_W::new(self)
    }
    #[doc = "Bit 23 - When 1, enables Bank55 of the SRAM"]
    #[inline(always)]
    pub fn bnk55_en(&mut self) -> BNK55_EN_W<23> {
        BNK55_EN_W::new(self)
    }
    #[doc = "Bit 24 - When 1, enables Bank56 of the SRAM"]
    #[inline(always)]
    pub fn bnk56_en(&mut self) -> BNK56_EN_W<24> {
        BNK56_EN_W::new(self)
    }
    #[doc = "Bit 25 - When 1, enables Bank57 of the SRAM"]
    #[inline(always)]
    pub fn bnk57_en(&mut self) -> BNK57_EN_W<25> {
        BNK57_EN_W::new(self)
    }
    #[doc = "Bit 26 - When 1, enables Bank58 of the SRAM"]
    #[inline(always)]
    pub fn bnk58_en(&mut self) -> BNK58_EN_W<26> {
        BNK58_EN_W::new(self)
    }
    #[doc = "Bit 27 - When 1, enables Bank59 of the SRAM"]
    #[inline(always)]
    pub fn bnk59_en(&mut self) -> BNK59_EN_W<27> {
        BNK59_EN_W::new(self)
    }
    #[doc = "Bit 28 - When 1, enables Bank60 of the SRAM"]
    #[inline(always)]
    pub fn bnk60_en(&mut self) -> BNK60_EN_W<28> {
        BNK60_EN_W::new(self)
    }
    #[doc = "Bit 29 - When 1, enables Bank61 of the SRAM"]
    #[inline(always)]
    pub fn bnk61_en(&mut self) -> BNK61_EN_W<29> {
        BNK61_EN_W::new(self)
    }
    #[doc = "Bit 30 - When 1, enables Bank62 of the SRAM"]
    #[inline(always)]
    pub fn bnk62_en(&mut self) -> BNK62_EN_W<30> {
        BNK62_EN_W::new(self)
    }
    #[doc = "Bit 31 - When 1, enables Bank63 of the SRAM"]
    #[inline(always)]
    pub fn bnk63_en(&mut self) -> BNK63_EN_W<31> {
        BNK63_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Bank Enable Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_banken_ctl1](index.html) module"]
pub struct SYS_SRAM_BANKEN_CTL1_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BANKEN_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_banken_ctl1::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BANKEN_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_banken_ctl1::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_BANKEN_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_BANKEN_CTL1 to value 0xffff_ffff"]
impl crate::Resettable for SYS_SRAM_BANKEN_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
