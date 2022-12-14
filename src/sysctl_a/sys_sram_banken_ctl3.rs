#[doc = "Register `SYS_SRAM_BANKEN_CTL3` reader"]
pub struct R(crate::R<SYS_SRAM_BANKEN_CTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BANKEN_CTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_BANKEN_CTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_BANKEN_CTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_BANKEN_CTL3` writer"]
pub struct W(crate::W<SYS_SRAM_BANKEN_CTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_BANKEN_CTL3_SPEC>;
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
impl From<crate::W<SYS_SRAM_BANKEN_CTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SRAM_BANKEN_CTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNK96_EN` reader - When 1, enables Bank96 of the SRAM"]
pub type BNK96_EN_R = crate::BitReader<BNK96_EN_A>;
#[doc = "When 1, enables Bank96 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK96_EN_A {
    #[doc = "0: Disables Bank96 of the SRAM"]
    BNK96_EN_0 = 0,
    #[doc = "1: Enables Bank96 of the SRAM"]
    BNK96_EN_1 = 1,
}
impl From<BNK96_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK96_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK96_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK96_EN_A {
        match self.bits {
            false => BNK96_EN_A::BNK96_EN_0,
            true => BNK96_EN_A::BNK96_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK96_EN_0`"]
    #[inline(always)]
    pub fn is_bnk96_en_0(&self) -> bool {
        *self == BNK96_EN_A::BNK96_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK96_EN_1`"]
    #[inline(always)]
    pub fn is_bnk96_en_1(&self) -> bool {
        *self == BNK96_EN_A::BNK96_EN_1
    }
}
#[doc = "Field `BNK96_EN` writer - When 1, enables Bank96 of the SRAM"]
pub type BNK96_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK96_EN_A, O>;
impl<'a, const O: u8> BNK96_EN_W<'a, O> {
    #[doc = "Disables Bank96 of the SRAM"]
    #[inline(always)]
    pub fn bnk96_en_0(self) -> &'a mut W {
        self.variant(BNK96_EN_A::BNK96_EN_0)
    }
    #[doc = "Enables Bank96 of the SRAM"]
    #[inline(always)]
    pub fn bnk96_en_1(self) -> &'a mut W {
        self.variant(BNK96_EN_A::BNK96_EN_1)
    }
}
#[doc = "Field `BNK97_EN` reader - When 1, enables Bank97 of the SRAM"]
pub type BNK97_EN_R = crate::BitReader<BNK97_EN_A>;
#[doc = "When 1, enables Bank97 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK97_EN_A {
    #[doc = "0: Disables Bank97 of the SRAM"]
    BNK97_EN_0 = 0,
    #[doc = "1: Enables Bank97 of the SRAM"]
    BNK97_EN_1 = 1,
}
impl From<BNK97_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK97_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK97_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK97_EN_A {
        match self.bits {
            false => BNK97_EN_A::BNK97_EN_0,
            true => BNK97_EN_A::BNK97_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK97_EN_0`"]
    #[inline(always)]
    pub fn is_bnk97_en_0(&self) -> bool {
        *self == BNK97_EN_A::BNK97_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK97_EN_1`"]
    #[inline(always)]
    pub fn is_bnk97_en_1(&self) -> bool {
        *self == BNK97_EN_A::BNK97_EN_1
    }
}
#[doc = "Field `BNK97_EN` writer - When 1, enables Bank97 of the SRAM"]
pub type BNK97_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK97_EN_A, O>;
impl<'a, const O: u8> BNK97_EN_W<'a, O> {
    #[doc = "Disables Bank97 of the SRAM"]
    #[inline(always)]
    pub fn bnk97_en_0(self) -> &'a mut W {
        self.variant(BNK97_EN_A::BNK97_EN_0)
    }
    #[doc = "Enables Bank97 of the SRAM"]
    #[inline(always)]
    pub fn bnk97_en_1(self) -> &'a mut W {
        self.variant(BNK97_EN_A::BNK97_EN_1)
    }
}
#[doc = "Field `BNK98_EN` reader - When 1, enables Bank98 of the SRAM"]
pub type BNK98_EN_R = crate::BitReader<BNK98_EN_A>;
#[doc = "When 1, enables Bank98 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK98_EN_A {
    #[doc = "0: Disables Bank98 of the SRAM"]
    BNK98_EN_0 = 0,
    #[doc = "1: Enables Bank98 of the SRAM"]
    BNK98_EN_1 = 1,
}
impl From<BNK98_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK98_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK98_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK98_EN_A {
        match self.bits {
            false => BNK98_EN_A::BNK98_EN_0,
            true => BNK98_EN_A::BNK98_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK98_EN_0`"]
    #[inline(always)]
    pub fn is_bnk98_en_0(&self) -> bool {
        *self == BNK98_EN_A::BNK98_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK98_EN_1`"]
    #[inline(always)]
    pub fn is_bnk98_en_1(&self) -> bool {
        *self == BNK98_EN_A::BNK98_EN_1
    }
}
#[doc = "Field `BNK98_EN` writer - When 1, enables Bank98 of the SRAM"]
pub type BNK98_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK98_EN_A, O>;
impl<'a, const O: u8> BNK98_EN_W<'a, O> {
    #[doc = "Disables Bank98 of the SRAM"]
    #[inline(always)]
    pub fn bnk98_en_0(self) -> &'a mut W {
        self.variant(BNK98_EN_A::BNK98_EN_0)
    }
    #[doc = "Enables Bank98 of the SRAM"]
    #[inline(always)]
    pub fn bnk98_en_1(self) -> &'a mut W {
        self.variant(BNK98_EN_A::BNK98_EN_1)
    }
}
#[doc = "Field `BNK99_EN` reader - When 1, enables Bank99 of the SRAM"]
pub type BNK99_EN_R = crate::BitReader<BNK99_EN_A>;
#[doc = "When 1, enables Bank99 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK99_EN_A {
    #[doc = "0: Disables Bank99 of the SRAM"]
    BNK99_EN_0 = 0,
    #[doc = "1: Enables Bank99 of the SRAM"]
    BNK99_EN_1 = 1,
}
impl From<BNK99_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK99_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK99_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK99_EN_A {
        match self.bits {
            false => BNK99_EN_A::BNK99_EN_0,
            true => BNK99_EN_A::BNK99_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK99_EN_0`"]
    #[inline(always)]
    pub fn is_bnk99_en_0(&self) -> bool {
        *self == BNK99_EN_A::BNK99_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK99_EN_1`"]
    #[inline(always)]
    pub fn is_bnk99_en_1(&self) -> bool {
        *self == BNK99_EN_A::BNK99_EN_1
    }
}
#[doc = "Field `BNK99_EN` writer - When 1, enables Bank99 of the SRAM"]
pub type BNK99_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK99_EN_A, O>;
impl<'a, const O: u8> BNK99_EN_W<'a, O> {
    #[doc = "Disables Bank99 of the SRAM"]
    #[inline(always)]
    pub fn bnk99_en_0(self) -> &'a mut W {
        self.variant(BNK99_EN_A::BNK99_EN_0)
    }
    #[doc = "Enables Bank99 of the SRAM"]
    #[inline(always)]
    pub fn bnk99_en_1(self) -> &'a mut W {
        self.variant(BNK99_EN_A::BNK99_EN_1)
    }
}
#[doc = "Field `BNK100_EN` reader - When 1, enables Bank100 of the SRAM"]
pub type BNK100_EN_R = crate::BitReader<BNK100_EN_A>;
#[doc = "When 1, enables Bank100 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK100_EN_A {
    #[doc = "0: Disables Bank100 of the SRAM"]
    BNK100_EN_0 = 0,
    #[doc = "1: Enables Bank100 of the SRAM"]
    BNK100_EN_1 = 1,
}
impl From<BNK100_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK100_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK100_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK100_EN_A {
        match self.bits {
            false => BNK100_EN_A::BNK100_EN_0,
            true => BNK100_EN_A::BNK100_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK100_EN_0`"]
    #[inline(always)]
    pub fn is_bnk100_en_0(&self) -> bool {
        *self == BNK100_EN_A::BNK100_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK100_EN_1`"]
    #[inline(always)]
    pub fn is_bnk100_en_1(&self) -> bool {
        *self == BNK100_EN_A::BNK100_EN_1
    }
}
#[doc = "Field `BNK100_EN` writer - When 1, enables Bank100 of the SRAM"]
pub type BNK100_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK100_EN_A, O>;
impl<'a, const O: u8> BNK100_EN_W<'a, O> {
    #[doc = "Disables Bank100 of the SRAM"]
    #[inline(always)]
    pub fn bnk100_en_0(self) -> &'a mut W {
        self.variant(BNK100_EN_A::BNK100_EN_0)
    }
    #[doc = "Enables Bank100 of the SRAM"]
    #[inline(always)]
    pub fn bnk100_en_1(self) -> &'a mut W {
        self.variant(BNK100_EN_A::BNK100_EN_1)
    }
}
#[doc = "Field `BNK101_EN` reader - When 1, enables Bank101 of the SRAM"]
pub type BNK101_EN_R = crate::BitReader<BNK101_EN_A>;
#[doc = "When 1, enables Bank101 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK101_EN_A {
    #[doc = "0: Disables Bank101 of the SRAM"]
    BNK101_EN_0 = 0,
    #[doc = "1: Enables Bank101 of the SRAM"]
    BNK101_EN_1 = 1,
}
impl From<BNK101_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK101_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK101_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK101_EN_A {
        match self.bits {
            false => BNK101_EN_A::BNK101_EN_0,
            true => BNK101_EN_A::BNK101_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK101_EN_0`"]
    #[inline(always)]
    pub fn is_bnk101_en_0(&self) -> bool {
        *self == BNK101_EN_A::BNK101_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK101_EN_1`"]
    #[inline(always)]
    pub fn is_bnk101_en_1(&self) -> bool {
        *self == BNK101_EN_A::BNK101_EN_1
    }
}
#[doc = "Field `BNK101_EN` writer - When 1, enables Bank101 of the SRAM"]
pub type BNK101_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK101_EN_A, O>;
impl<'a, const O: u8> BNK101_EN_W<'a, O> {
    #[doc = "Disables Bank101 of the SRAM"]
    #[inline(always)]
    pub fn bnk101_en_0(self) -> &'a mut W {
        self.variant(BNK101_EN_A::BNK101_EN_0)
    }
    #[doc = "Enables Bank101 of the SRAM"]
    #[inline(always)]
    pub fn bnk101_en_1(self) -> &'a mut W {
        self.variant(BNK101_EN_A::BNK101_EN_1)
    }
}
#[doc = "Field `BNK102_EN` reader - When 1, enables Bank102 of the SRAM"]
pub type BNK102_EN_R = crate::BitReader<BNK102_EN_A>;
#[doc = "When 1, enables Bank102 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK102_EN_A {
    #[doc = "0: Disables Bank102 of the SRAM"]
    BNK102_EN_0 = 0,
    #[doc = "1: Enables Bank102 of the SRAM"]
    BNK102_EN_1 = 1,
}
impl From<BNK102_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK102_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK102_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK102_EN_A {
        match self.bits {
            false => BNK102_EN_A::BNK102_EN_0,
            true => BNK102_EN_A::BNK102_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK102_EN_0`"]
    #[inline(always)]
    pub fn is_bnk102_en_0(&self) -> bool {
        *self == BNK102_EN_A::BNK102_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK102_EN_1`"]
    #[inline(always)]
    pub fn is_bnk102_en_1(&self) -> bool {
        *self == BNK102_EN_A::BNK102_EN_1
    }
}
#[doc = "Field `BNK102_EN` writer - When 1, enables Bank102 of the SRAM"]
pub type BNK102_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK102_EN_A, O>;
impl<'a, const O: u8> BNK102_EN_W<'a, O> {
    #[doc = "Disables Bank102 of the SRAM"]
    #[inline(always)]
    pub fn bnk102_en_0(self) -> &'a mut W {
        self.variant(BNK102_EN_A::BNK102_EN_0)
    }
    #[doc = "Enables Bank102 of the SRAM"]
    #[inline(always)]
    pub fn bnk102_en_1(self) -> &'a mut W {
        self.variant(BNK102_EN_A::BNK102_EN_1)
    }
}
#[doc = "Field `BNK103_EN` reader - When 1, enables Bank103 of the SRAM"]
pub type BNK103_EN_R = crate::BitReader<BNK103_EN_A>;
#[doc = "When 1, enables Bank103 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK103_EN_A {
    #[doc = "0: Disables Bank103 of the SRAM"]
    BNK103_EN_0 = 0,
    #[doc = "1: Enables Bank103 of the SRAM"]
    BNK103_EN_1 = 1,
}
impl From<BNK103_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK103_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK103_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK103_EN_A {
        match self.bits {
            false => BNK103_EN_A::BNK103_EN_0,
            true => BNK103_EN_A::BNK103_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK103_EN_0`"]
    #[inline(always)]
    pub fn is_bnk103_en_0(&self) -> bool {
        *self == BNK103_EN_A::BNK103_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK103_EN_1`"]
    #[inline(always)]
    pub fn is_bnk103_en_1(&self) -> bool {
        *self == BNK103_EN_A::BNK103_EN_1
    }
}
#[doc = "Field `BNK103_EN` writer - When 1, enables Bank103 of the SRAM"]
pub type BNK103_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK103_EN_A, O>;
impl<'a, const O: u8> BNK103_EN_W<'a, O> {
    #[doc = "Disables Bank103 of the SRAM"]
    #[inline(always)]
    pub fn bnk103_en_0(self) -> &'a mut W {
        self.variant(BNK103_EN_A::BNK103_EN_0)
    }
    #[doc = "Enables Bank103 of the SRAM"]
    #[inline(always)]
    pub fn bnk103_en_1(self) -> &'a mut W {
        self.variant(BNK103_EN_A::BNK103_EN_1)
    }
}
#[doc = "Field `BNK104_EN` reader - When 1, enables Bank104 of the SRAM"]
pub type BNK104_EN_R = crate::BitReader<BNK104_EN_A>;
#[doc = "When 1, enables Bank104 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK104_EN_A {
    #[doc = "0: Disables Bank104 of the SRAM"]
    BNK104_EN_0 = 0,
    #[doc = "1: Enables Bank104 of the SRAM"]
    BNK104_EN_1 = 1,
}
impl From<BNK104_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK104_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK104_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK104_EN_A {
        match self.bits {
            false => BNK104_EN_A::BNK104_EN_0,
            true => BNK104_EN_A::BNK104_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK104_EN_0`"]
    #[inline(always)]
    pub fn is_bnk104_en_0(&self) -> bool {
        *self == BNK104_EN_A::BNK104_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK104_EN_1`"]
    #[inline(always)]
    pub fn is_bnk104_en_1(&self) -> bool {
        *self == BNK104_EN_A::BNK104_EN_1
    }
}
#[doc = "Field `BNK104_EN` writer - When 1, enables Bank104 of the SRAM"]
pub type BNK104_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK104_EN_A, O>;
impl<'a, const O: u8> BNK104_EN_W<'a, O> {
    #[doc = "Disables Bank104 of the SRAM"]
    #[inline(always)]
    pub fn bnk104_en_0(self) -> &'a mut W {
        self.variant(BNK104_EN_A::BNK104_EN_0)
    }
    #[doc = "Enables Bank104 of the SRAM"]
    #[inline(always)]
    pub fn bnk104_en_1(self) -> &'a mut W {
        self.variant(BNK104_EN_A::BNK104_EN_1)
    }
}
#[doc = "Field `BNK105_EN` reader - When 1, enables Bank105 of the SRAM"]
pub type BNK105_EN_R = crate::BitReader<BNK105_EN_A>;
#[doc = "When 1, enables Bank105 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK105_EN_A {
    #[doc = "0: Disables Bank105 of the SRAM"]
    BNK105_EN_0 = 0,
    #[doc = "1: Enables Bank105 of the SRAM"]
    BNK105_EN_1 = 1,
}
impl From<BNK105_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK105_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK105_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK105_EN_A {
        match self.bits {
            false => BNK105_EN_A::BNK105_EN_0,
            true => BNK105_EN_A::BNK105_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK105_EN_0`"]
    #[inline(always)]
    pub fn is_bnk105_en_0(&self) -> bool {
        *self == BNK105_EN_A::BNK105_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK105_EN_1`"]
    #[inline(always)]
    pub fn is_bnk105_en_1(&self) -> bool {
        *self == BNK105_EN_A::BNK105_EN_1
    }
}
#[doc = "Field `BNK105_EN` writer - When 1, enables Bank105 of the SRAM"]
pub type BNK105_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK105_EN_A, O>;
impl<'a, const O: u8> BNK105_EN_W<'a, O> {
    #[doc = "Disables Bank105 of the SRAM"]
    #[inline(always)]
    pub fn bnk105_en_0(self) -> &'a mut W {
        self.variant(BNK105_EN_A::BNK105_EN_0)
    }
    #[doc = "Enables Bank105 of the SRAM"]
    #[inline(always)]
    pub fn bnk105_en_1(self) -> &'a mut W {
        self.variant(BNK105_EN_A::BNK105_EN_1)
    }
}
#[doc = "Field `BNK106_EN` reader - When 1, enables Bank106 of the SRAM"]
pub type BNK106_EN_R = crate::BitReader<BNK106_EN_A>;
#[doc = "When 1, enables Bank106 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK106_EN_A {
    #[doc = "0: Disables Bank106 of the SRAM"]
    BNK106_EN_0 = 0,
    #[doc = "1: Enables Bank106 of the SRAM"]
    BNK106_EN_1 = 1,
}
impl From<BNK106_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK106_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK106_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK106_EN_A {
        match self.bits {
            false => BNK106_EN_A::BNK106_EN_0,
            true => BNK106_EN_A::BNK106_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK106_EN_0`"]
    #[inline(always)]
    pub fn is_bnk106_en_0(&self) -> bool {
        *self == BNK106_EN_A::BNK106_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK106_EN_1`"]
    #[inline(always)]
    pub fn is_bnk106_en_1(&self) -> bool {
        *self == BNK106_EN_A::BNK106_EN_1
    }
}
#[doc = "Field `BNK106_EN` writer - When 1, enables Bank106 of the SRAM"]
pub type BNK106_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK106_EN_A, O>;
impl<'a, const O: u8> BNK106_EN_W<'a, O> {
    #[doc = "Disables Bank106 of the SRAM"]
    #[inline(always)]
    pub fn bnk106_en_0(self) -> &'a mut W {
        self.variant(BNK106_EN_A::BNK106_EN_0)
    }
    #[doc = "Enables Bank106 of the SRAM"]
    #[inline(always)]
    pub fn bnk106_en_1(self) -> &'a mut W {
        self.variant(BNK106_EN_A::BNK106_EN_1)
    }
}
#[doc = "Field `BNK107_EN` reader - When 1, enables Bank107 of the SRAM"]
pub type BNK107_EN_R = crate::BitReader<BNK107_EN_A>;
#[doc = "When 1, enables Bank107 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK107_EN_A {
    #[doc = "0: Disables Bank107 of the SRAM"]
    BNK107_EN_0 = 0,
    #[doc = "1: Enables Bank107 of the SRAM"]
    BNK107_EN_1 = 1,
}
impl From<BNK107_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK107_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK107_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK107_EN_A {
        match self.bits {
            false => BNK107_EN_A::BNK107_EN_0,
            true => BNK107_EN_A::BNK107_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK107_EN_0`"]
    #[inline(always)]
    pub fn is_bnk107_en_0(&self) -> bool {
        *self == BNK107_EN_A::BNK107_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK107_EN_1`"]
    #[inline(always)]
    pub fn is_bnk107_en_1(&self) -> bool {
        *self == BNK107_EN_A::BNK107_EN_1
    }
}
#[doc = "Field `BNK107_EN` writer - When 1, enables Bank107 of the SRAM"]
pub type BNK107_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK107_EN_A, O>;
impl<'a, const O: u8> BNK107_EN_W<'a, O> {
    #[doc = "Disables Bank107 of the SRAM"]
    #[inline(always)]
    pub fn bnk107_en_0(self) -> &'a mut W {
        self.variant(BNK107_EN_A::BNK107_EN_0)
    }
    #[doc = "Enables Bank107 of the SRAM"]
    #[inline(always)]
    pub fn bnk107_en_1(self) -> &'a mut W {
        self.variant(BNK107_EN_A::BNK107_EN_1)
    }
}
#[doc = "Field `BNK108_EN` reader - When 1, enables Bank108 of the SRAM"]
pub type BNK108_EN_R = crate::BitReader<BNK108_EN_A>;
#[doc = "When 1, enables Bank108 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK108_EN_A {
    #[doc = "0: Disables Bank108 of the SRAM"]
    BNK108_EN_0 = 0,
    #[doc = "1: Enables Bank108 of the SRAM"]
    BNK108_EN_1 = 1,
}
impl From<BNK108_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK108_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK108_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK108_EN_A {
        match self.bits {
            false => BNK108_EN_A::BNK108_EN_0,
            true => BNK108_EN_A::BNK108_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK108_EN_0`"]
    #[inline(always)]
    pub fn is_bnk108_en_0(&self) -> bool {
        *self == BNK108_EN_A::BNK108_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK108_EN_1`"]
    #[inline(always)]
    pub fn is_bnk108_en_1(&self) -> bool {
        *self == BNK108_EN_A::BNK108_EN_1
    }
}
#[doc = "Field `BNK108_EN` writer - When 1, enables Bank108 of the SRAM"]
pub type BNK108_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK108_EN_A, O>;
impl<'a, const O: u8> BNK108_EN_W<'a, O> {
    #[doc = "Disables Bank108 of the SRAM"]
    #[inline(always)]
    pub fn bnk108_en_0(self) -> &'a mut W {
        self.variant(BNK108_EN_A::BNK108_EN_0)
    }
    #[doc = "Enables Bank108 of the SRAM"]
    #[inline(always)]
    pub fn bnk108_en_1(self) -> &'a mut W {
        self.variant(BNK108_EN_A::BNK108_EN_1)
    }
}
#[doc = "Field `BNK109_EN` reader - When 1, enables Bank109 of the SRAM"]
pub type BNK109_EN_R = crate::BitReader<BNK109_EN_A>;
#[doc = "When 1, enables Bank109 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK109_EN_A {
    #[doc = "0: Disables Bank109 of the SRAM"]
    BNK109_EN_0 = 0,
    #[doc = "1: Enables Bank109 of the SRAM"]
    BNK109_EN_1 = 1,
}
impl From<BNK109_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK109_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK109_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK109_EN_A {
        match self.bits {
            false => BNK109_EN_A::BNK109_EN_0,
            true => BNK109_EN_A::BNK109_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK109_EN_0`"]
    #[inline(always)]
    pub fn is_bnk109_en_0(&self) -> bool {
        *self == BNK109_EN_A::BNK109_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK109_EN_1`"]
    #[inline(always)]
    pub fn is_bnk109_en_1(&self) -> bool {
        *self == BNK109_EN_A::BNK109_EN_1
    }
}
#[doc = "Field `BNK109_EN` writer - When 1, enables Bank109 of the SRAM"]
pub type BNK109_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK109_EN_A, O>;
impl<'a, const O: u8> BNK109_EN_W<'a, O> {
    #[doc = "Disables Bank109 of the SRAM"]
    #[inline(always)]
    pub fn bnk109_en_0(self) -> &'a mut W {
        self.variant(BNK109_EN_A::BNK109_EN_0)
    }
    #[doc = "Enables Bank109 of the SRAM"]
    #[inline(always)]
    pub fn bnk109_en_1(self) -> &'a mut W {
        self.variant(BNK109_EN_A::BNK109_EN_1)
    }
}
#[doc = "Field `BNK110_EN` reader - When 1, enables Bank110 of the SRAM"]
pub type BNK110_EN_R = crate::BitReader<BNK110_EN_A>;
#[doc = "When 1, enables Bank110 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK110_EN_A {
    #[doc = "0: Disables Bank110 of the SRAM"]
    BNK110_EN_0 = 0,
    #[doc = "1: Enables Bank110 of the SRAM"]
    BNK110_EN_1 = 1,
}
impl From<BNK110_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK110_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK110_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK110_EN_A {
        match self.bits {
            false => BNK110_EN_A::BNK110_EN_0,
            true => BNK110_EN_A::BNK110_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK110_EN_0`"]
    #[inline(always)]
    pub fn is_bnk110_en_0(&self) -> bool {
        *self == BNK110_EN_A::BNK110_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK110_EN_1`"]
    #[inline(always)]
    pub fn is_bnk110_en_1(&self) -> bool {
        *self == BNK110_EN_A::BNK110_EN_1
    }
}
#[doc = "Field `BNK110_EN` writer - When 1, enables Bank110 of the SRAM"]
pub type BNK110_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK110_EN_A, O>;
impl<'a, const O: u8> BNK110_EN_W<'a, O> {
    #[doc = "Disables Bank110 of the SRAM"]
    #[inline(always)]
    pub fn bnk110_en_0(self) -> &'a mut W {
        self.variant(BNK110_EN_A::BNK110_EN_0)
    }
    #[doc = "Enables Bank110 of the SRAM"]
    #[inline(always)]
    pub fn bnk110_en_1(self) -> &'a mut W {
        self.variant(BNK110_EN_A::BNK110_EN_1)
    }
}
#[doc = "Field `BNK111_EN` reader - When 1, enables Bank111 of the SRAM"]
pub type BNK111_EN_R = crate::BitReader<BNK111_EN_A>;
#[doc = "When 1, enables Bank111 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK111_EN_A {
    #[doc = "0: Disables Bank111 of the SRAM"]
    BNK111_EN_0 = 0,
    #[doc = "1: Enables Bank111 of the SRAM"]
    BNK111_EN_1 = 1,
}
impl From<BNK111_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK111_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK111_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK111_EN_A {
        match self.bits {
            false => BNK111_EN_A::BNK111_EN_0,
            true => BNK111_EN_A::BNK111_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK111_EN_0`"]
    #[inline(always)]
    pub fn is_bnk111_en_0(&self) -> bool {
        *self == BNK111_EN_A::BNK111_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK111_EN_1`"]
    #[inline(always)]
    pub fn is_bnk111_en_1(&self) -> bool {
        *self == BNK111_EN_A::BNK111_EN_1
    }
}
#[doc = "Field `BNK111_EN` writer - When 1, enables Bank111 of the SRAM"]
pub type BNK111_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK111_EN_A, O>;
impl<'a, const O: u8> BNK111_EN_W<'a, O> {
    #[doc = "Disables Bank111 of the SRAM"]
    #[inline(always)]
    pub fn bnk111_en_0(self) -> &'a mut W {
        self.variant(BNK111_EN_A::BNK111_EN_0)
    }
    #[doc = "Enables Bank111 of the SRAM"]
    #[inline(always)]
    pub fn bnk111_en_1(self) -> &'a mut W {
        self.variant(BNK111_EN_A::BNK111_EN_1)
    }
}
#[doc = "Field `BNK112_EN` reader - When 1, enables Bank112 of the SRAM"]
pub type BNK112_EN_R = crate::BitReader<BNK112_EN_A>;
#[doc = "When 1, enables Bank112 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK112_EN_A {
    #[doc = "0: Disables Bank112 of the SRAM"]
    BNK112_EN_0 = 0,
    #[doc = "1: Enables Bank112 of the SRAM"]
    BNK112_EN_1 = 1,
}
impl From<BNK112_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK112_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK112_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK112_EN_A {
        match self.bits {
            false => BNK112_EN_A::BNK112_EN_0,
            true => BNK112_EN_A::BNK112_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK112_EN_0`"]
    #[inline(always)]
    pub fn is_bnk112_en_0(&self) -> bool {
        *self == BNK112_EN_A::BNK112_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK112_EN_1`"]
    #[inline(always)]
    pub fn is_bnk112_en_1(&self) -> bool {
        *self == BNK112_EN_A::BNK112_EN_1
    }
}
#[doc = "Field `BNK112_EN` writer - When 1, enables Bank112 of the SRAM"]
pub type BNK112_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK112_EN_A, O>;
impl<'a, const O: u8> BNK112_EN_W<'a, O> {
    #[doc = "Disables Bank112 of the SRAM"]
    #[inline(always)]
    pub fn bnk112_en_0(self) -> &'a mut W {
        self.variant(BNK112_EN_A::BNK112_EN_0)
    }
    #[doc = "Enables Bank112 of the SRAM"]
    #[inline(always)]
    pub fn bnk112_en_1(self) -> &'a mut W {
        self.variant(BNK112_EN_A::BNK112_EN_1)
    }
}
#[doc = "Field `BNK113_EN` reader - When 1, enables Bank113 of the SRAM"]
pub type BNK113_EN_R = crate::BitReader<BNK113_EN_A>;
#[doc = "When 1, enables Bank113 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK113_EN_A {
    #[doc = "0: Disables Bank113 of the SRAM"]
    BNK113_EN_0 = 0,
    #[doc = "1: Enables Bank113 of the SRAM"]
    BNK113_EN_1 = 1,
}
impl From<BNK113_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK113_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK113_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK113_EN_A {
        match self.bits {
            false => BNK113_EN_A::BNK113_EN_0,
            true => BNK113_EN_A::BNK113_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK113_EN_0`"]
    #[inline(always)]
    pub fn is_bnk113_en_0(&self) -> bool {
        *self == BNK113_EN_A::BNK113_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK113_EN_1`"]
    #[inline(always)]
    pub fn is_bnk113_en_1(&self) -> bool {
        *self == BNK113_EN_A::BNK113_EN_1
    }
}
#[doc = "Field `BNK113_EN` writer - When 1, enables Bank113 of the SRAM"]
pub type BNK113_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK113_EN_A, O>;
impl<'a, const O: u8> BNK113_EN_W<'a, O> {
    #[doc = "Disables Bank113 of the SRAM"]
    #[inline(always)]
    pub fn bnk113_en_0(self) -> &'a mut W {
        self.variant(BNK113_EN_A::BNK113_EN_0)
    }
    #[doc = "Enables Bank113 of the SRAM"]
    #[inline(always)]
    pub fn bnk113_en_1(self) -> &'a mut W {
        self.variant(BNK113_EN_A::BNK113_EN_1)
    }
}
#[doc = "Field `BNK114_EN` reader - When 1, enables Bank114 of the SRAM"]
pub type BNK114_EN_R = crate::BitReader<BNK114_EN_A>;
#[doc = "When 1, enables Bank114 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK114_EN_A {
    #[doc = "0: Disables Bank114 of the SRAM"]
    BNK114_EN_0 = 0,
    #[doc = "1: Enables Bank114 of the SRAM"]
    BNK114_EN_1 = 1,
}
impl From<BNK114_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK114_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK114_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK114_EN_A {
        match self.bits {
            false => BNK114_EN_A::BNK114_EN_0,
            true => BNK114_EN_A::BNK114_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK114_EN_0`"]
    #[inline(always)]
    pub fn is_bnk114_en_0(&self) -> bool {
        *self == BNK114_EN_A::BNK114_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK114_EN_1`"]
    #[inline(always)]
    pub fn is_bnk114_en_1(&self) -> bool {
        *self == BNK114_EN_A::BNK114_EN_1
    }
}
#[doc = "Field `BNK114_EN` writer - When 1, enables Bank114 of the SRAM"]
pub type BNK114_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK114_EN_A, O>;
impl<'a, const O: u8> BNK114_EN_W<'a, O> {
    #[doc = "Disables Bank114 of the SRAM"]
    #[inline(always)]
    pub fn bnk114_en_0(self) -> &'a mut W {
        self.variant(BNK114_EN_A::BNK114_EN_0)
    }
    #[doc = "Enables Bank114 of the SRAM"]
    #[inline(always)]
    pub fn bnk114_en_1(self) -> &'a mut W {
        self.variant(BNK114_EN_A::BNK114_EN_1)
    }
}
#[doc = "Field `BNK115_EN` reader - When 1, enables Bank115 of the SRAM"]
pub type BNK115_EN_R = crate::BitReader<BNK115_EN_A>;
#[doc = "When 1, enables Bank115 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK115_EN_A {
    #[doc = "0: Disables Bank115 of the SRAM"]
    BNK115_EN_0 = 0,
    #[doc = "1: Enables Bank115 of the SRAM"]
    BNK115_EN_1 = 1,
}
impl From<BNK115_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK115_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK115_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK115_EN_A {
        match self.bits {
            false => BNK115_EN_A::BNK115_EN_0,
            true => BNK115_EN_A::BNK115_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK115_EN_0`"]
    #[inline(always)]
    pub fn is_bnk115_en_0(&self) -> bool {
        *self == BNK115_EN_A::BNK115_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK115_EN_1`"]
    #[inline(always)]
    pub fn is_bnk115_en_1(&self) -> bool {
        *self == BNK115_EN_A::BNK115_EN_1
    }
}
#[doc = "Field `BNK115_EN` writer - When 1, enables Bank115 of the SRAM"]
pub type BNK115_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK115_EN_A, O>;
impl<'a, const O: u8> BNK115_EN_W<'a, O> {
    #[doc = "Disables Bank115 of the SRAM"]
    #[inline(always)]
    pub fn bnk115_en_0(self) -> &'a mut W {
        self.variant(BNK115_EN_A::BNK115_EN_0)
    }
    #[doc = "Enables Bank115 of the SRAM"]
    #[inline(always)]
    pub fn bnk115_en_1(self) -> &'a mut W {
        self.variant(BNK115_EN_A::BNK115_EN_1)
    }
}
#[doc = "Field `BNK116_EN` reader - When 1, enables Bank116 of the SRAM"]
pub type BNK116_EN_R = crate::BitReader<BNK116_EN_A>;
#[doc = "When 1, enables Bank116 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK116_EN_A {
    #[doc = "0: Disables Bank116 of the SRAM"]
    BNK116_EN_0 = 0,
    #[doc = "1: Enables Bank116 of the SRAM"]
    BNK116_EN_1 = 1,
}
impl From<BNK116_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK116_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK116_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK116_EN_A {
        match self.bits {
            false => BNK116_EN_A::BNK116_EN_0,
            true => BNK116_EN_A::BNK116_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK116_EN_0`"]
    #[inline(always)]
    pub fn is_bnk116_en_0(&self) -> bool {
        *self == BNK116_EN_A::BNK116_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK116_EN_1`"]
    #[inline(always)]
    pub fn is_bnk116_en_1(&self) -> bool {
        *self == BNK116_EN_A::BNK116_EN_1
    }
}
#[doc = "Field `BNK116_EN` writer - When 1, enables Bank116 of the SRAM"]
pub type BNK116_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK116_EN_A, O>;
impl<'a, const O: u8> BNK116_EN_W<'a, O> {
    #[doc = "Disables Bank116 of the SRAM"]
    #[inline(always)]
    pub fn bnk116_en_0(self) -> &'a mut W {
        self.variant(BNK116_EN_A::BNK116_EN_0)
    }
    #[doc = "Enables Bank116 of the SRAM"]
    #[inline(always)]
    pub fn bnk116_en_1(self) -> &'a mut W {
        self.variant(BNK116_EN_A::BNK116_EN_1)
    }
}
#[doc = "Field `BNK117_EN` reader - When 1, enables Bank117 of the SRAM"]
pub type BNK117_EN_R = crate::BitReader<BNK117_EN_A>;
#[doc = "When 1, enables Bank117 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK117_EN_A {
    #[doc = "0: Disables Bank117 of the SRAM"]
    BNK117_EN_0 = 0,
    #[doc = "1: Enables Bank117 of the SRAM"]
    BNK117_EN_1 = 1,
}
impl From<BNK117_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK117_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK117_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK117_EN_A {
        match self.bits {
            false => BNK117_EN_A::BNK117_EN_0,
            true => BNK117_EN_A::BNK117_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK117_EN_0`"]
    #[inline(always)]
    pub fn is_bnk117_en_0(&self) -> bool {
        *self == BNK117_EN_A::BNK117_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK117_EN_1`"]
    #[inline(always)]
    pub fn is_bnk117_en_1(&self) -> bool {
        *self == BNK117_EN_A::BNK117_EN_1
    }
}
#[doc = "Field `BNK117_EN` writer - When 1, enables Bank117 of the SRAM"]
pub type BNK117_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK117_EN_A, O>;
impl<'a, const O: u8> BNK117_EN_W<'a, O> {
    #[doc = "Disables Bank117 of the SRAM"]
    #[inline(always)]
    pub fn bnk117_en_0(self) -> &'a mut W {
        self.variant(BNK117_EN_A::BNK117_EN_0)
    }
    #[doc = "Enables Bank117 of the SRAM"]
    #[inline(always)]
    pub fn bnk117_en_1(self) -> &'a mut W {
        self.variant(BNK117_EN_A::BNK117_EN_1)
    }
}
#[doc = "Field `BNK118_EN` reader - When 1, enables Bank118 of the SRAM"]
pub type BNK118_EN_R = crate::BitReader<BNK118_EN_A>;
#[doc = "When 1, enables Bank118 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK118_EN_A {
    #[doc = "0: Disables Bank118 of the SRAM"]
    BNK118_EN_0 = 0,
    #[doc = "1: Enables Bank118 of the SRAM"]
    BNK118_EN_1 = 1,
}
impl From<BNK118_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK118_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK118_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK118_EN_A {
        match self.bits {
            false => BNK118_EN_A::BNK118_EN_0,
            true => BNK118_EN_A::BNK118_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK118_EN_0`"]
    #[inline(always)]
    pub fn is_bnk118_en_0(&self) -> bool {
        *self == BNK118_EN_A::BNK118_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK118_EN_1`"]
    #[inline(always)]
    pub fn is_bnk118_en_1(&self) -> bool {
        *self == BNK118_EN_A::BNK118_EN_1
    }
}
#[doc = "Field `BNK118_EN` writer - When 1, enables Bank118 of the SRAM"]
pub type BNK118_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK118_EN_A, O>;
impl<'a, const O: u8> BNK118_EN_W<'a, O> {
    #[doc = "Disables Bank118 of the SRAM"]
    #[inline(always)]
    pub fn bnk118_en_0(self) -> &'a mut W {
        self.variant(BNK118_EN_A::BNK118_EN_0)
    }
    #[doc = "Enables Bank118 of the SRAM"]
    #[inline(always)]
    pub fn bnk118_en_1(self) -> &'a mut W {
        self.variant(BNK118_EN_A::BNK118_EN_1)
    }
}
#[doc = "Field `BNK119_EN` reader - When 1, enables Bank119 of the SRAM"]
pub type BNK119_EN_R = crate::BitReader<BNK119_EN_A>;
#[doc = "When 1, enables Bank119 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK119_EN_A {
    #[doc = "0: Disables Bank119 of the SRAM"]
    BNK119_EN_0 = 0,
    #[doc = "1: Enables Bank119 of the SRAM"]
    BNK119_EN_1 = 1,
}
impl From<BNK119_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK119_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK119_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK119_EN_A {
        match self.bits {
            false => BNK119_EN_A::BNK119_EN_0,
            true => BNK119_EN_A::BNK119_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK119_EN_0`"]
    #[inline(always)]
    pub fn is_bnk119_en_0(&self) -> bool {
        *self == BNK119_EN_A::BNK119_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK119_EN_1`"]
    #[inline(always)]
    pub fn is_bnk119_en_1(&self) -> bool {
        *self == BNK119_EN_A::BNK119_EN_1
    }
}
#[doc = "Field `BNK119_EN` writer - When 1, enables Bank119 of the SRAM"]
pub type BNK119_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK119_EN_A, O>;
impl<'a, const O: u8> BNK119_EN_W<'a, O> {
    #[doc = "Disables Bank119 of the SRAM"]
    #[inline(always)]
    pub fn bnk119_en_0(self) -> &'a mut W {
        self.variant(BNK119_EN_A::BNK119_EN_0)
    }
    #[doc = "Enables Bank119 of the SRAM"]
    #[inline(always)]
    pub fn bnk119_en_1(self) -> &'a mut W {
        self.variant(BNK119_EN_A::BNK119_EN_1)
    }
}
#[doc = "Field `BNK120_EN` reader - When 1, enables Bank120 of the SRAM"]
pub type BNK120_EN_R = crate::BitReader<BNK120_EN_A>;
#[doc = "When 1, enables Bank120 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK120_EN_A {
    #[doc = "0: Disables Bank120 of the SRAM"]
    BNK120_EN_0 = 0,
    #[doc = "1: Enables Bank120 of the SRAM"]
    BNK120_EN_1 = 1,
}
impl From<BNK120_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK120_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK120_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK120_EN_A {
        match self.bits {
            false => BNK120_EN_A::BNK120_EN_0,
            true => BNK120_EN_A::BNK120_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK120_EN_0`"]
    #[inline(always)]
    pub fn is_bnk120_en_0(&self) -> bool {
        *self == BNK120_EN_A::BNK120_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK120_EN_1`"]
    #[inline(always)]
    pub fn is_bnk120_en_1(&self) -> bool {
        *self == BNK120_EN_A::BNK120_EN_1
    }
}
#[doc = "Field `BNK120_EN` writer - When 1, enables Bank120 of the SRAM"]
pub type BNK120_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK120_EN_A, O>;
impl<'a, const O: u8> BNK120_EN_W<'a, O> {
    #[doc = "Disables Bank120 of the SRAM"]
    #[inline(always)]
    pub fn bnk120_en_0(self) -> &'a mut W {
        self.variant(BNK120_EN_A::BNK120_EN_0)
    }
    #[doc = "Enables Bank120 of the SRAM"]
    #[inline(always)]
    pub fn bnk120_en_1(self) -> &'a mut W {
        self.variant(BNK120_EN_A::BNK120_EN_1)
    }
}
#[doc = "Field `BNK121_EN` reader - When 1, enables Bank121 of the SRAM"]
pub type BNK121_EN_R = crate::BitReader<BNK121_EN_A>;
#[doc = "When 1, enables Bank121 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK121_EN_A {
    #[doc = "0: Disables Bank121 of the SRAM"]
    BNK121_EN_0 = 0,
    #[doc = "1: Enables Bank121 of the SRAM"]
    BNK121_EN_1 = 1,
}
impl From<BNK121_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK121_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK121_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK121_EN_A {
        match self.bits {
            false => BNK121_EN_A::BNK121_EN_0,
            true => BNK121_EN_A::BNK121_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK121_EN_0`"]
    #[inline(always)]
    pub fn is_bnk121_en_0(&self) -> bool {
        *self == BNK121_EN_A::BNK121_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK121_EN_1`"]
    #[inline(always)]
    pub fn is_bnk121_en_1(&self) -> bool {
        *self == BNK121_EN_A::BNK121_EN_1
    }
}
#[doc = "Field `BNK121_EN` writer - When 1, enables Bank121 of the SRAM"]
pub type BNK121_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK121_EN_A, O>;
impl<'a, const O: u8> BNK121_EN_W<'a, O> {
    #[doc = "Disables Bank121 of the SRAM"]
    #[inline(always)]
    pub fn bnk121_en_0(self) -> &'a mut W {
        self.variant(BNK121_EN_A::BNK121_EN_0)
    }
    #[doc = "Enables Bank121 of the SRAM"]
    #[inline(always)]
    pub fn bnk121_en_1(self) -> &'a mut W {
        self.variant(BNK121_EN_A::BNK121_EN_1)
    }
}
#[doc = "Field `BNK122_EN` reader - When 1, enables Bank122 of the SRAM"]
pub type BNK122_EN_R = crate::BitReader<BNK122_EN_A>;
#[doc = "When 1, enables Bank122 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK122_EN_A {
    #[doc = "0: Disables Bank122 of the SRAM"]
    BNK122_EN_0 = 0,
    #[doc = "1: Enables Bank122 of the SRAM"]
    BNK122_EN_1 = 1,
}
impl From<BNK122_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK122_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK122_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK122_EN_A {
        match self.bits {
            false => BNK122_EN_A::BNK122_EN_0,
            true => BNK122_EN_A::BNK122_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK122_EN_0`"]
    #[inline(always)]
    pub fn is_bnk122_en_0(&self) -> bool {
        *self == BNK122_EN_A::BNK122_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK122_EN_1`"]
    #[inline(always)]
    pub fn is_bnk122_en_1(&self) -> bool {
        *self == BNK122_EN_A::BNK122_EN_1
    }
}
#[doc = "Field `BNK122_EN` writer - When 1, enables Bank122 of the SRAM"]
pub type BNK122_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK122_EN_A, O>;
impl<'a, const O: u8> BNK122_EN_W<'a, O> {
    #[doc = "Disables Bank122 of the SRAM"]
    #[inline(always)]
    pub fn bnk122_en_0(self) -> &'a mut W {
        self.variant(BNK122_EN_A::BNK122_EN_0)
    }
    #[doc = "Enables Bank122 of the SRAM"]
    #[inline(always)]
    pub fn bnk122_en_1(self) -> &'a mut W {
        self.variant(BNK122_EN_A::BNK122_EN_1)
    }
}
#[doc = "Field `BNK123_EN` reader - When 1, enables Bank123 of the SRAM"]
pub type BNK123_EN_R = crate::BitReader<BNK123_EN_A>;
#[doc = "When 1, enables Bank123 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK123_EN_A {
    #[doc = "0: Disables Bank123 of the SRAM"]
    BNK123_EN_0 = 0,
    #[doc = "1: Enables Bank123 of the SRAM"]
    BNK123_EN_1 = 1,
}
impl From<BNK123_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK123_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK123_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK123_EN_A {
        match self.bits {
            false => BNK123_EN_A::BNK123_EN_0,
            true => BNK123_EN_A::BNK123_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK123_EN_0`"]
    #[inline(always)]
    pub fn is_bnk123_en_0(&self) -> bool {
        *self == BNK123_EN_A::BNK123_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK123_EN_1`"]
    #[inline(always)]
    pub fn is_bnk123_en_1(&self) -> bool {
        *self == BNK123_EN_A::BNK123_EN_1
    }
}
#[doc = "Field `BNK123_EN` writer - When 1, enables Bank123 of the SRAM"]
pub type BNK123_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK123_EN_A, O>;
impl<'a, const O: u8> BNK123_EN_W<'a, O> {
    #[doc = "Disables Bank123 of the SRAM"]
    #[inline(always)]
    pub fn bnk123_en_0(self) -> &'a mut W {
        self.variant(BNK123_EN_A::BNK123_EN_0)
    }
    #[doc = "Enables Bank123 of the SRAM"]
    #[inline(always)]
    pub fn bnk123_en_1(self) -> &'a mut W {
        self.variant(BNK123_EN_A::BNK123_EN_1)
    }
}
#[doc = "Field `BNK124_EN` reader - When 1, enables Bank124 of the SRAM"]
pub type BNK124_EN_R = crate::BitReader<BNK124_EN_A>;
#[doc = "When 1, enables Bank124 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK124_EN_A {
    #[doc = "0: Disables Bank124 of the SRAM"]
    BNK124_EN_0 = 0,
    #[doc = "1: Enables Bank124 of the SRAM"]
    BNK124_EN_1 = 1,
}
impl From<BNK124_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK124_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK124_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK124_EN_A {
        match self.bits {
            false => BNK124_EN_A::BNK124_EN_0,
            true => BNK124_EN_A::BNK124_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK124_EN_0`"]
    #[inline(always)]
    pub fn is_bnk124_en_0(&self) -> bool {
        *self == BNK124_EN_A::BNK124_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK124_EN_1`"]
    #[inline(always)]
    pub fn is_bnk124_en_1(&self) -> bool {
        *self == BNK124_EN_A::BNK124_EN_1
    }
}
#[doc = "Field `BNK124_EN` writer - When 1, enables Bank124 of the SRAM"]
pub type BNK124_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK124_EN_A, O>;
impl<'a, const O: u8> BNK124_EN_W<'a, O> {
    #[doc = "Disables Bank124 of the SRAM"]
    #[inline(always)]
    pub fn bnk124_en_0(self) -> &'a mut W {
        self.variant(BNK124_EN_A::BNK124_EN_0)
    }
    #[doc = "Enables Bank124 of the SRAM"]
    #[inline(always)]
    pub fn bnk124_en_1(self) -> &'a mut W {
        self.variant(BNK124_EN_A::BNK124_EN_1)
    }
}
#[doc = "Field `BNK125_EN` reader - When 1, enables Bank125 of the SRAM"]
pub type BNK125_EN_R = crate::BitReader<BNK125_EN_A>;
#[doc = "When 1, enables Bank125 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK125_EN_A {
    #[doc = "0: Disables Bank125 of the SRAM"]
    BNK125_EN_0 = 0,
    #[doc = "1: Enables Bank125 of the SRAM"]
    BNK125_EN_1 = 1,
}
impl From<BNK125_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK125_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK125_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK125_EN_A {
        match self.bits {
            false => BNK125_EN_A::BNK125_EN_0,
            true => BNK125_EN_A::BNK125_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK125_EN_0`"]
    #[inline(always)]
    pub fn is_bnk125_en_0(&self) -> bool {
        *self == BNK125_EN_A::BNK125_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK125_EN_1`"]
    #[inline(always)]
    pub fn is_bnk125_en_1(&self) -> bool {
        *self == BNK125_EN_A::BNK125_EN_1
    }
}
#[doc = "Field `BNK125_EN` writer - When 1, enables Bank125 of the SRAM"]
pub type BNK125_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK125_EN_A, O>;
impl<'a, const O: u8> BNK125_EN_W<'a, O> {
    #[doc = "Disables Bank125 of the SRAM"]
    #[inline(always)]
    pub fn bnk125_en_0(self) -> &'a mut W {
        self.variant(BNK125_EN_A::BNK125_EN_0)
    }
    #[doc = "Enables Bank125 of the SRAM"]
    #[inline(always)]
    pub fn bnk125_en_1(self) -> &'a mut W {
        self.variant(BNK125_EN_A::BNK125_EN_1)
    }
}
#[doc = "Field `BNK126_EN` reader - When 1, enables Bank126 of the SRAM"]
pub type BNK126_EN_R = crate::BitReader<BNK126_EN_A>;
#[doc = "When 1, enables Bank126 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK126_EN_A {
    #[doc = "0: Disables Bank126 of the SRAM"]
    BNK126_EN_0 = 0,
    #[doc = "1: Enables Bank126 of the SRAM"]
    BNK126_EN_1 = 1,
}
impl From<BNK126_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK126_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK126_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK126_EN_A {
        match self.bits {
            false => BNK126_EN_A::BNK126_EN_0,
            true => BNK126_EN_A::BNK126_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK126_EN_0`"]
    #[inline(always)]
    pub fn is_bnk126_en_0(&self) -> bool {
        *self == BNK126_EN_A::BNK126_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK126_EN_1`"]
    #[inline(always)]
    pub fn is_bnk126_en_1(&self) -> bool {
        *self == BNK126_EN_A::BNK126_EN_1
    }
}
#[doc = "Field `BNK126_EN` writer - When 1, enables Bank126 of the SRAM"]
pub type BNK126_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK126_EN_A, O>;
impl<'a, const O: u8> BNK126_EN_W<'a, O> {
    #[doc = "Disables Bank126 of the SRAM"]
    #[inline(always)]
    pub fn bnk126_en_0(self) -> &'a mut W {
        self.variant(BNK126_EN_A::BNK126_EN_0)
    }
    #[doc = "Enables Bank126 of the SRAM"]
    #[inline(always)]
    pub fn bnk126_en_1(self) -> &'a mut W {
        self.variant(BNK126_EN_A::BNK126_EN_1)
    }
}
#[doc = "Field `BNK127_EN` reader - When 1, enables Bank127 of the SRAM"]
pub type BNK127_EN_R = crate::BitReader<BNK127_EN_A>;
#[doc = "When 1, enables Bank127 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK127_EN_A {
    #[doc = "0: Disables Bank127 of the SRAM"]
    BNK127_EN_0 = 0,
    #[doc = "1: Enables Bank127 of the SRAM"]
    BNK127_EN_1 = 1,
}
impl From<BNK127_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK127_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK127_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK127_EN_A {
        match self.bits {
            false => BNK127_EN_A::BNK127_EN_0,
            true => BNK127_EN_A::BNK127_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK127_EN_0`"]
    #[inline(always)]
    pub fn is_bnk127_en_0(&self) -> bool {
        *self == BNK127_EN_A::BNK127_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK127_EN_1`"]
    #[inline(always)]
    pub fn is_bnk127_en_1(&self) -> bool {
        *self == BNK127_EN_A::BNK127_EN_1
    }
}
#[doc = "Field `BNK127_EN` writer - When 1, enables Bank127 of the SRAM"]
pub type BNK127_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL3_SPEC, BNK127_EN_A, O>;
impl<'a, const O: u8> BNK127_EN_W<'a, O> {
    #[doc = "Disables Bank127 of the SRAM"]
    #[inline(always)]
    pub fn bnk127_en_0(self) -> &'a mut W {
        self.variant(BNK127_EN_A::BNK127_EN_0)
    }
    #[doc = "Enables Bank127 of the SRAM"]
    #[inline(always)]
    pub fn bnk127_en_1(self) -> &'a mut W {
        self.variant(BNK127_EN_A::BNK127_EN_1)
    }
}
impl R {
    #[doc = "Bit 0 - When 1, enables Bank96 of the SRAM"]
    #[inline(always)]
    pub fn bnk96_en(&self) -> BNK96_EN_R {
        BNK96_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, enables Bank97 of the SRAM"]
    #[inline(always)]
    pub fn bnk97_en(&self) -> BNK97_EN_R {
        BNK97_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, enables Bank98 of the SRAM"]
    #[inline(always)]
    pub fn bnk98_en(&self) -> BNK98_EN_R {
        BNK98_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, enables Bank99 of the SRAM"]
    #[inline(always)]
    pub fn bnk99_en(&self) -> BNK99_EN_R {
        BNK99_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, enables Bank100 of the SRAM"]
    #[inline(always)]
    pub fn bnk100_en(&self) -> BNK100_EN_R {
        BNK100_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, enables Bank101 of the SRAM"]
    #[inline(always)]
    pub fn bnk101_en(&self) -> BNK101_EN_R {
        BNK101_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, enables Bank102 of the SRAM"]
    #[inline(always)]
    pub fn bnk102_en(&self) -> BNK102_EN_R {
        BNK102_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, enables Bank103 of the SRAM"]
    #[inline(always)]
    pub fn bnk103_en(&self) -> BNK103_EN_R {
        BNK103_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, enables Bank104 of the SRAM"]
    #[inline(always)]
    pub fn bnk104_en(&self) -> BNK104_EN_R {
        BNK104_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, enables Bank105 of the SRAM"]
    #[inline(always)]
    pub fn bnk105_en(&self) -> BNK105_EN_R {
        BNK105_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When 1, enables Bank106 of the SRAM"]
    #[inline(always)]
    pub fn bnk106_en(&self) -> BNK106_EN_R {
        BNK106_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When 1, enables Bank107 of the SRAM"]
    #[inline(always)]
    pub fn bnk107_en(&self) -> BNK107_EN_R {
        BNK107_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When 1, enables Bank108 of the SRAM"]
    #[inline(always)]
    pub fn bnk108_en(&self) -> BNK108_EN_R {
        BNK108_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When 1, enables Bank109 of the SRAM"]
    #[inline(always)]
    pub fn bnk109_en(&self) -> BNK109_EN_R {
        BNK109_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When 1, enables Bank110 of the SRAM"]
    #[inline(always)]
    pub fn bnk110_en(&self) -> BNK110_EN_R {
        BNK110_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When 1, enables Bank111 of the SRAM"]
    #[inline(always)]
    pub fn bnk111_en(&self) -> BNK111_EN_R {
        BNK111_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When 1, enables Bank112 of the SRAM"]
    #[inline(always)]
    pub fn bnk112_en(&self) -> BNK112_EN_R {
        BNK112_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When 1, enables Bank113 of the SRAM"]
    #[inline(always)]
    pub fn bnk113_en(&self) -> BNK113_EN_R {
        BNK113_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When 1, enables Bank114 of the SRAM"]
    #[inline(always)]
    pub fn bnk114_en(&self) -> BNK114_EN_R {
        BNK114_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When 1, enables Bank115 of the SRAM"]
    #[inline(always)]
    pub fn bnk115_en(&self) -> BNK115_EN_R {
        BNK115_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When 1, enables Bank116 of the SRAM"]
    #[inline(always)]
    pub fn bnk116_en(&self) -> BNK116_EN_R {
        BNK116_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When 1, enables Bank117 of the SRAM"]
    #[inline(always)]
    pub fn bnk117_en(&self) -> BNK117_EN_R {
        BNK117_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When 1, enables Bank118 of the SRAM"]
    #[inline(always)]
    pub fn bnk118_en(&self) -> BNK118_EN_R {
        BNK118_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When 1, enables Bank119 of the SRAM"]
    #[inline(always)]
    pub fn bnk119_en(&self) -> BNK119_EN_R {
        BNK119_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When 1, enables Bank120 of the SRAM"]
    #[inline(always)]
    pub fn bnk120_en(&self) -> BNK120_EN_R {
        BNK120_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When 1, enables Bank121 of the SRAM"]
    #[inline(always)]
    pub fn bnk121_en(&self) -> BNK121_EN_R {
        BNK121_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When 1, enables Bank122 of the SRAM"]
    #[inline(always)]
    pub fn bnk122_en(&self) -> BNK122_EN_R {
        BNK122_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When 1, enables Bank123 of the SRAM"]
    #[inline(always)]
    pub fn bnk123_en(&self) -> BNK123_EN_R {
        BNK123_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When 1, enables Bank124 of the SRAM"]
    #[inline(always)]
    pub fn bnk124_en(&self) -> BNK124_EN_R {
        BNK124_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When 1, enables Bank125 of the SRAM"]
    #[inline(always)]
    pub fn bnk125_en(&self) -> BNK125_EN_R {
        BNK125_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When 1, enables Bank126 of the SRAM"]
    #[inline(always)]
    pub fn bnk126_en(&self) -> BNK126_EN_R {
        BNK126_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When 1, enables Bank127 of the SRAM"]
    #[inline(always)]
    pub fn bnk127_en(&self) -> BNK127_EN_R {
        BNK127_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enables Bank96 of the SRAM"]
    #[inline(always)]
    pub fn bnk96_en(&mut self) -> BNK96_EN_W<0> {
        BNK96_EN_W::new(self)
    }
    #[doc = "Bit 1 - When 1, enables Bank97 of the SRAM"]
    #[inline(always)]
    pub fn bnk97_en(&mut self) -> BNK97_EN_W<1> {
        BNK97_EN_W::new(self)
    }
    #[doc = "Bit 2 - When 1, enables Bank98 of the SRAM"]
    #[inline(always)]
    pub fn bnk98_en(&mut self) -> BNK98_EN_W<2> {
        BNK98_EN_W::new(self)
    }
    #[doc = "Bit 3 - When 1, enables Bank99 of the SRAM"]
    #[inline(always)]
    pub fn bnk99_en(&mut self) -> BNK99_EN_W<3> {
        BNK99_EN_W::new(self)
    }
    #[doc = "Bit 4 - When 1, enables Bank100 of the SRAM"]
    #[inline(always)]
    pub fn bnk100_en(&mut self) -> BNK100_EN_W<4> {
        BNK100_EN_W::new(self)
    }
    #[doc = "Bit 5 - When 1, enables Bank101 of the SRAM"]
    #[inline(always)]
    pub fn bnk101_en(&mut self) -> BNK101_EN_W<5> {
        BNK101_EN_W::new(self)
    }
    #[doc = "Bit 6 - When 1, enables Bank102 of the SRAM"]
    #[inline(always)]
    pub fn bnk102_en(&mut self) -> BNK102_EN_W<6> {
        BNK102_EN_W::new(self)
    }
    #[doc = "Bit 7 - When 1, enables Bank103 of the SRAM"]
    #[inline(always)]
    pub fn bnk103_en(&mut self) -> BNK103_EN_W<7> {
        BNK103_EN_W::new(self)
    }
    #[doc = "Bit 8 - When 1, enables Bank104 of the SRAM"]
    #[inline(always)]
    pub fn bnk104_en(&mut self) -> BNK104_EN_W<8> {
        BNK104_EN_W::new(self)
    }
    #[doc = "Bit 9 - When 1, enables Bank105 of the SRAM"]
    #[inline(always)]
    pub fn bnk105_en(&mut self) -> BNK105_EN_W<9> {
        BNK105_EN_W::new(self)
    }
    #[doc = "Bit 10 - When 1, enables Bank106 of the SRAM"]
    #[inline(always)]
    pub fn bnk106_en(&mut self) -> BNK106_EN_W<10> {
        BNK106_EN_W::new(self)
    }
    #[doc = "Bit 11 - When 1, enables Bank107 of the SRAM"]
    #[inline(always)]
    pub fn bnk107_en(&mut self) -> BNK107_EN_W<11> {
        BNK107_EN_W::new(self)
    }
    #[doc = "Bit 12 - When 1, enables Bank108 of the SRAM"]
    #[inline(always)]
    pub fn bnk108_en(&mut self) -> BNK108_EN_W<12> {
        BNK108_EN_W::new(self)
    }
    #[doc = "Bit 13 - When 1, enables Bank109 of the SRAM"]
    #[inline(always)]
    pub fn bnk109_en(&mut self) -> BNK109_EN_W<13> {
        BNK109_EN_W::new(self)
    }
    #[doc = "Bit 14 - When 1, enables Bank110 of the SRAM"]
    #[inline(always)]
    pub fn bnk110_en(&mut self) -> BNK110_EN_W<14> {
        BNK110_EN_W::new(self)
    }
    #[doc = "Bit 15 - When 1, enables Bank111 of the SRAM"]
    #[inline(always)]
    pub fn bnk111_en(&mut self) -> BNK111_EN_W<15> {
        BNK111_EN_W::new(self)
    }
    #[doc = "Bit 16 - When 1, enables Bank112 of the SRAM"]
    #[inline(always)]
    pub fn bnk112_en(&mut self) -> BNK112_EN_W<16> {
        BNK112_EN_W::new(self)
    }
    #[doc = "Bit 17 - When 1, enables Bank113 of the SRAM"]
    #[inline(always)]
    pub fn bnk113_en(&mut self) -> BNK113_EN_W<17> {
        BNK113_EN_W::new(self)
    }
    #[doc = "Bit 18 - When 1, enables Bank114 of the SRAM"]
    #[inline(always)]
    pub fn bnk114_en(&mut self) -> BNK114_EN_W<18> {
        BNK114_EN_W::new(self)
    }
    #[doc = "Bit 19 - When 1, enables Bank115 of the SRAM"]
    #[inline(always)]
    pub fn bnk115_en(&mut self) -> BNK115_EN_W<19> {
        BNK115_EN_W::new(self)
    }
    #[doc = "Bit 20 - When 1, enables Bank116 of the SRAM"]
    #[inline(always)]
    pub fn bnk116_en(&mut self) -> BNK116_EN_W<20> {
        BNK116_EN_W::new(self)
    }
    #[doc = "Bit 21 - When 1, enables Bank117 of the SRAM"]
    #[inline(always)]
    pub fn bnk117_en(&mut self) -> BNK117_EN_W<21> {
        BNK117_EN_W::new(self)
    }
    #[doc = "Bit 22 - When 1, enables Bank118 of the SRAM"]
    #[inline(always)]
    pub fn bnk118_en(&mut self) -> BNK118_EN_W<22> {
        BNK118_EN_W::new(self)
    }
    #[doc = "Bit 23 - When 1, enables Bank119 of the SRAM"]
    #[inline(always)]
    pub fn bnk119_en(&mut self) -> BNK119_EN_W<23> {
        BNK119_EN_W::new(self)
    }
    #[doc = "Bit 24 - When 1, enables Bank120 of the SRAM"]
    #[inline(always)]
    pub fn bnk120_en(&mut self) -> BNK120_EN_W<24> {
        BNK120_EN_W::new(self)
    }
    #[doc = "Bit 25 - When 1, enables Bank121 of the SRAM"]
    #[inline(always)]
    pub fn bnk121_en(&mut self) -> BNK121_EN_W<25> {
        BNK121_EN_W::new(self)
    }
    #[doc = "Bit 26 - When 1, enables Bank122 of the SRAM"]
    #[inline(always)]
    pub fn bnk122_en(&mut self) -> BNK122_EN_W<26> {
        BNK122_EN_W::new(self)
    }
    #[doc = "Bit 27 - When 1, enables Bank123 of the SRAM"]
    #[inline(always)]
    pub fn bnk123_en(&mut self) -> BNK123_EN_W<27> {
        BNK123_EN_W::new(self)
    }
    #[doc = "Bit 28 - When 1, enables Bank124 of the SRAM"]
    #[inline(always)]
    pub fn bnk124_en(&mut self) -> BNK124_EN_W<28> {
        BNK124_EN_W::new(self)
    }
    #[doc = "Bit 29 - When 1, enables Bank125 of the SRAM"]
    #[inline(always)]
    pub fn bnk125_en(&mut self) -> BNK125_EN_W<29> {
        BNK125_EN_W::new(self)
    }
    #[doc = "Bit 30 - When 1, enables Bank126 of the SRAM"]
    #[inline(always)]
    pub fn bnk126_en(&mut self) -> BNK126_EN_W<30> {
        BNK126_EN_W::new(self)
    }
    #[doc = "Bit 31 - When 1, enables Bank127 of the SRAM"]
    #[inline(always)]
    pub fn bnk127_en(&mut self) -> BNK127_EN_W<31> {
        BNK127_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Bank Enable Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_banken_ctl3](index.html) module"]
pub struct SYS_SRAM_BANKEN_CTL3_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BANKEN_CTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_banken_ctl3::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BANKEN_CTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_banken_ctl3::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_BANKEN_CTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_BANKEN_CTL3 to value 0xffff_ffff"]
impl crate::Resettable for SYS_SRAM_BANKEN_CTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
