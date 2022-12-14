#[doc = "Register `SYS_SRAM_BLKRET_CTL3` reader"]
pub struct R(crate::R<SYS_SRAM_BLKRET_CTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BLKRET_CTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_BLKRET_CTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_BLKRET_CTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_BLKRET_CTL3` writer"]
pub struct W(crate::W<SYS_SRAM_BLKRET_CTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_BLKRET_CTL3_SPEC>;
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
impl From<crate::W<SYS_SRAM_BLKRET_CTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SRAM_BLKRET_CTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLK96_EN` reader - When 1, Block96 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK96_EN_R = crate::BitReader<BLK96_EN_A>;
#[doc = "When 1, Block96 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK96_EN_A {
    #[doc = "0: Block96 of the SRAM is not retained in LPM3 or LPM4"]
    BLK96_EN_0 = 0,
    #[doc = "1: Block96 of the SRAM is retained in LPM3 and LPM4"]
    BLK96_EN_1 = 1,
}
impl From<BLK96_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK96_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK96_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK96_EN_A {
        match self.bits {
            false => BLK96_EN_A::BLK96_EN_0,
            true => BLK96_EN_A::BLK96_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK96_EN_0`"]
    #[inline(always)]
    pub fn is_blk96_en_0(&self) -> bool {
        *self == BLK96_EN_A::BLK96_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK96_EN_1`"]
    #[inline(always)]
    pub fn is_blk96_en_1(&self) -> bool {
        *self == BLK96_EN_A::BLK96_EN_1
    }
}
#[doc = "Field `BLK96_EN` writer - When 1, Block96 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK96_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK96_EN_A, O>;
impl<'a, const O: u8> BLK96_EN_W<'a, O> {
    #[doc = "Block96 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk96_en_0(self) -> &'a mut W {
        self.variant(BLK96_EN_A::BLK96_EN_0)
    }
    #[doc = "Block96 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk96_en_1(self) -> &'a mut W {
        self.variant(BLK96_EN_A::BLK96_EN_1)
    }
}
#[doc = "Field `BLK97_EN` reader - When 1, Block97 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK97_EN_R = crate::BitReader<BLK97_EN_A>;
#[doc = "When 1, Block97 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK97_EN_A {
    #[doc = "0: Block97 of the SRAM is not retained in LPM3 or LPM4"]
    BLK97_EN_0 = 0,
    #[doc = "1: Block97 of the SRAM is retained in LPM3 and LPM4"]
    BLK97_EN_1 = 1,
}
impl From<BLK97_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK97_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK97_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK97_EN_A {
        match self.bits {
            false => BLK97_EN_A::BLK97_EN_0,
            true => BLK97_EN_A::BLK97_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK97_EN_0`"]
    #[inline(always)]
    pub fn is_blk97_en_0(&self) -> bool {
        *self == BLK97_EN_A::BLK97_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK97_EN_1`"]
    #[inline(always)]
    pub fn is_blk97_en_1(&self) -> bool {
        *self == BLK97_EN_A::BLK97_EN_1
    }
}
#[doc = "Field `BLK97_EN` writer - When 1, Block97 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK97_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK97_EN_A, O>;
impl<'a, const O: u8> BLK97_EN_W<'a, O> {
    #[doc = "Block97 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk97_en_0(self) -> &'a mut W {
        self.variant(BLK97_EN_A::BLK97_EN_0)
    }
    #[doc = "Block97 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk97_en_1(self) -> &'a mut W {
        self.variant(BLK97_EN_A::BLK97_EN_1)
    }
}
#[doc = "Field `BLK98_EN` reader - When 1, Block98 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK98_EN_R = crate::BitReader<BLK98_EN_A>;
#[doc = "When 1, Block98 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK98_EN_A {
    #[doc = "0: Block98 of the SRAM is not retained in LPM3 or LPM4"]
    BLK98_EN_0 = 0,
    #[doc = "1: Block98 of the SRAM is retained in LPM3 and LPM4"]
    BLK98_EN_1 = 1,
}
impl From<BLK98_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK98_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK98_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK98_EN_A {
        match self.bits {
            false => BLK98_EN_A::BLK98_EN_0,
            true => BLK98_EN_A::BLK98_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK98_EN_0`"]
    #[inline(always)]
    pub fn is_blk98_en_0(&self) -> bool {
        *self == BLK98_EN_A::BLK98_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK98_EN_1`"]
    #[inline(always)]
    pub fn is_blk98_en_1(&self) -> bool {
        *self == BLK98_EN_A::BLK98_EN_1
    }
}
#[doc = "Field `BLK98_EN` writer - When 1, Block98 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK98_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK98_EN_A, O>;
impl<'a, const O: u8> BLK98_EN_W<'a, O> {
    #[doc = "Block98 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk98_en_0(self) -> &'a mut W {
        self.variant(BLK98_EN_A::BLK98_EN_0)
    }
    #[doc = "Block98 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk98_en_1(self) -> &'a mut W {
        self.variant(BLK98_EN_A::BLK98_EN_1)
    }
}
#[doc = "Field `BLK99_EN` reader - When 1, Block99 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK99_EN_R = crate::BitReader<BLK99_EN_A>;
#[doc = "When 1, Block99 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK99_EN_A {
    #[doc = "0: Block99 of the SRAM is not retained in LPM3 or LPM4"]
    BLK99_EN_0 = 0,
    #[doc = "1: Block99 of the SRAM is retained in LPM3 and LPM4"]
    BLK99_EN_1 = 1,
}
impl From<BLK99_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK99_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK99_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK99_EN_A {
        match self.bits {
            false => BLK99_EN_A::BLK99_EN_0,
            true => BLK99_EN_A::BLK99_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK99_EN_0`"]
    #[inline(always)]
    pub fn is_blk99_en_0(&self) -> bool {
        *self == BLK99_EN_A::BLK99_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK99_EN_1`"]
    #[inline(always)]
    pub fn is_blk99_en_1(&self) -> bool {
        *self == BLK99_EN_A::BLK99_EN_1
    }
}
#[doc = "Field `BLK99_EN` writer - When 1, Block99 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK99_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK99_EN_A, O>;
impl<'a, const O: u8> BLK99_EN_W<'a, O> {
    #[doc = "Block99 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk99_en_0(self) -> &'a mut W {
        self.variant(BLK99_EN_A::BLK99_EN_0)
    }
    #[doc = "Block99 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk99_en_1(self) -> &'a mut W {
        self.variant(BLK99_EN_A::BLK99_EN_1)
    }
}
#[doc = "Field `BLK100_EN` reader - When 1, Block100 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK100_EN_R = crate::BitReader<BLK100_EN_A>;
#[doc = "When 1, Block100 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK100_EN_A {
    #[doc = "0: Block100 of the SRAM is not retained in LPM3 or LPM4"]
    BLK100_EN_0 = 0,
    #[doc = "1: Block100 of the SRAM is retained in LPM3 and LPM4"]
    BLK100_EN_1 = 1,
}
impl From<BLK100_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK100_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK100_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK100_EN_A {
        match self.bits {
            false => BLK100_EN_A::BLK100_EN_0,
            true => BLK100_EN_A::BLK100_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK100_EN_0`"]
    #[inline(always)]
    pub fn is_blk100_en_0(&self) -> bool {
        *self == BLK100_EN_A::BLK100_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK100_EN_1`"]
    #[inline(always)]
    pub fn is_blk100_en_1(&self) -> bool {
        *self == BLK100_EN_A::BLK100_EN_1
    }
}
#[doc = "Field `BLK100_EN` writer - When 1, Block100 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK100_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK100_EN_A, O>;
impl<'a, const O: u8> BLK100_EN_W<'a, O> {
    #[doc = "Block100 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk100_en_0(self) -> &'a mut W {
        self.variant(BLK100_EN_A::BLK100_EN_0)
    }
    #[doc = "Block100 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk100_en_1(self) -> &'a mut W {
        self.variant(BLK100_EN_A::BLK100_EN_1)
    }
}
#[doc = "Field `BLK101_EN` reader - When 1, Block101 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK101_EN_R = crate::BitReader<BLK101_EN_A>;
#[doc = "When 1, Block101 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK101_EN_A {
    #[doc = "0: Block101 of the SRAM is not retained in LPM3 or LPM4"]
    BLK101_EN_0 = 0,
    #[doc = "1: Block101 of the SRAM is retained in LPM3 and LPM4"]
    BLK101_EN_1 = 1,
}
impl From<BLK101_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK101_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK101_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK101_EN_A {
        match self.bits {
            false => BLK101_EN_A::BLK101_EN_0,
            true => BLK101_EN_A::BLK101_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK101_EN_0`"]
    #[inline(always)]
    pub fn is_blk101_en_0(&self) -> bool {
        *self == BLK101_EN_A::BLK101_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK101_EN_1`"]
    #[inline(always)]
    pub fn is_blk101_en_1(&self) -> bool {
        *self == BLK101_EN_A::BLK101_EN_1
    }
}
#[doc = "Field `BLK101_EN` writer - When 1, Block101 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK101_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK101_EN_A, O>;
impl<'a, const O: u8> BLK101_EN_W<'a, O> {
    #[doc = "Block101 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk101_en_0(self) -> &'a mut W {
        self.variant(BLK101_EN_A::BLK101_EN_0)
    }
    #[doc = "Block101 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk101_en_1(self) -> &'a mut W {
        self.variant(BLK101_EN_A::BLK101_EN_1)
    }
}
#[doc = "Field `BLK102_EN` reader - When 1, Block102 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK102_EN_R = crate::BitReader<BLK102_EN_A>;
#[doc = "When 1, Block102 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK102_EN_A {
    #[doc = "0: Block102 of the SRAM is not retained in LPM3 or LPM4"]
    BLK102_EN_0 = 0,
    #[doc = "1: Block102 of the SRAM is retained in LPM3 and LPM4"]
    BLK102_EN_1 = 1,
}
impl From<BLK102_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK102_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK102_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK102_EN_A {
        match self.bits {
            false => BLK102_EN_A::BLK102_EN_0,
            true => BLK102_EN_A::BLK102_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK102_EN_0`"]
    #[inline(always)]
    pub fn is_blk102_en_0(&self) -> bool {
        *self == BLK102_EN_A::BLK102_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK102_EN_1`"]
    #[inline(always)]
    pub fn is_blk102_en_1(&self) -> bool {
        *self == BLK102_EN_A::BLK102_EN_1
    }
}
#[doc = "Field `BLK102_EN` writer - When 1, Block102 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK102_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK102_EN_A, O>;
impl<'a, const O: u8> BLK102_EN_W<'a, O> {
    #[doc = "Block102 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk102_en_0(self) -> &'a mut W {
        self.variant(BLK102_EN_A::BLK102_EN_0)
    }
    #[doc = "Block102 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk102_en_1(self) -> &'a mut W {
        self.variant(BLK102_EN_A::BLK102_EN_1)
    }
}
#[doc = "Field `BLK103_EN` reader - When 1, Block103 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK103_EN_R = crate::BitReader<BLK103_EN_A>;
#[doc = "When 1, Block103 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK103_EN_A {
    #[doc = "0: Block103 of the SRAM is not retained in LPM3 or LPM4"]
    BLK103_EN_0 = 0,
    #[doc = "1: Block103 of the SRAM is retained in LPM3 and LPM4"]
    BLK103_EN_1 = 1,
}
impl From<BLK103_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK103_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK103_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK103_EN_A {
        match self.bits {
            false => BLK103_EN_A::BLK103_EN_0,
            true => BLK103_EN_A::BLK103_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK103_EN_0`"]
    #[inline(always)]
    pub fn is_blk103_en_0(&self) -> bool {
        *self == BLK103_EN_A::BLK103_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK103_EN_1`"]
    #[inline(always)]
    pub fn is_blk103_en_1(&self) -> bool {
        *self == BLK103_EN_A::BLK103_EN_1
    }
}
#[doc = "Field `BLK103_EN` writer - When 1, Block103 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK103_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK103_EN_A, O>;
impl<'a, const O: u8> BLK103_EN_W<'a, O> {
    #[doc = "Block103 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk103_en_0(self) -> &'a mut W {
        self.variant(BLK103_EN_A::BLK103_EN_0)
    }
    #[doc = "Block103 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk103_en_1(self) -> &'a mut W {
        self.variant(BLK103_EN_A::BLK103_EN_1)
    }
}
#[doc = "Field `BLK104_EN` reader - When 1, Block104 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK104_EN_R = crate::BitReader<BLK104_EN_A>;
#[doc = "When 1, Block104 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK104_EN_A {
    #[doc = "0: Block104 of the SRAM is not retained in LPM3 or LPM4"]
    BLK104_EN_0 = 0,
    #[doc = "1: Block104 of the SRAM is retained in LPM3 and LPM4"]
    BLK104_EN_1 = 1,
}
impl From<BLK104_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK104_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK104_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK104_EN_A {
        match self.bits {
            false => BLK104_EN_A::BLK104_EN_0,
            true => BLK104_EN_A::BLK104_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK104_EN_0`"]
    #[inline(always)]
    pub fn is_blk104_en_0(&self) -> bool {
        *self == BLK104_EN_A::BLK104_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK104_EN_1`"]
    #[inline(always)]
    pub fn is_blk104_en_1(&self) -> bool {
        *self == BLK104_EN_A::BLK104_EN_1
    }
}
#[doc = "Field `BLK104_EN` writer - When 1, Block104 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK104_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK104_EN_A, O>;
impl<'a, const O: u8> BLK104_EN_W<'a, O> {
    #[doc = "Block104 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk104_en_0(self) -> &'a mut W {
        self.variant(BLK104_EN_A::BLK104_EN_0)
    }
    #[doc = "Block104 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk104_en_1(self) -> &'a mut W {
        self.variant(BLK104_EN_A::BLK104_EN_1)
    }
}
#[doc = "Field `BLK105_EN` reader - When 1, Block105 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK105_EN_R = crate::BitReader<BLK105_EN_A>;
#[doc = "When 1, Block105 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK105_EN_A {
    #[doc = "0: Block105 of the SRAM is not retained in LPM3 or LPM4"]
    BLK105_EN_0 = 0,
    #[doc = "1: Block105 of the SRAM is retained in LPM3 and LPM4"]
    BLK105_EN_1 = 1,
}
impl From<BLK105_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK105_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK105_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK105_EN_A {
        match self.bits {
            false => BLK105_EN_A::BLK105_EN_0,
            true => BLK105_EN_A::BLK105_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK105_EN_0`"]
    #[inline(always)]
    pub fn is_blk105_en_0(&self) -> bool {
        *self == BLK105_EN_A::BLK105_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK105_EN_1`"]
    #[inline(always)]
    pub fn is_blk105_en_1(&self) -> bool {
        *self == BLK105_EN_A::BLK105_EN_1
    }
}
#[doc = "Field `BLK105_EN` writer - When 1, Block105 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK105_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK105_EN_A, O>;
impl<'a, const O: u8> BLK105_EN_W<'a, O> {
    #[doc = "Block105 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk105_en_0(self) -> &'a mut W {
        self.variant(BLK105_EN_A::BLK105_EN_0)
    }
    #[doc = "Block105 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk105_en_1(self) -> &'a mut W {
        self.variant(BLK105_EN_A::BLK105_EN_1)
    }
}
#[doc = "Field `BLK106_EN` reader - When 1, Block106 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK106_EN_R = crate::BitReader<BLK106_EN_A>;
#[doc = "When 1, Block106 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK106_EN_A {
    #[doc = "0: Block106 of the SRAM is not retained in LPM3 or LPM4"]
    BLK106_EN_0 = 0,
    #[doc = "1: Block106 of the SRAM is retained in LPM3 and LPM4"]
    BLK106_EN_1 = 1,
}
impl From<BLK106_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK106_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK106_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK106_EN_A {
        match self.bits {
            false => BLK106_EN_A::BLK106_EN_0,
            true => BLK106_EN_A::BLK106_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK106_EN_0`"]
    #[inline(always)]
    pub fn is_blk106_en_0(&self) -> bool {
        *self == BLK106_EN_A::BLK106_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK106_EN_1`"]
    #[inline(always)]
    pub fn is_blk106_en_1(&self) -> bool {
        *self == BLK106_EN_A::BLK106_EN_1
    }
}
#[doc = "Field `BLK106_EN` writer - When 1, Block106 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK106_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK106_EN_A, O>;
impl<'a, const O: u8> BLK106_EN_W<'a, O> {
    #[doc = "Block106 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk106_en_0(self) -> &'a mut W {
        self.variant(BLK106_EN_A::BLK106_EN_0)
    }
    #[doc = "Block106 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk106_en_1(self) -> &'a mut W {
        self.variant(BLK106_EN_A::BLK106_EN_1)
    }
}
#[doc = "Field `BLK107_EN` reader - When 1, Block107 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK107_EN_R = crate::BitReader<BLK107_EN_A>;
#[doc = "When 1, Block107 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK107_EN_A {
    #[doc = "0: Block107 of the SRAM is not retained in LPM3 or LPM4"]
    BLK107_EN_0 = 0,
    #[doc = "1: Block107 of the SRAM is retained in LPM3 and LPM4"]
    BLK107_EN_1 = 1,
}
impl From<BLK107_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK107_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK107_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK107_EN_A {
        match self.bits {
            false => BLK107_EN_A::BLK107_EN_0,
            true => BLK107_EN_A::BLK107_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK107_EN_0`"]
    #[inline(always)]
    pub fn is_blk107_en_0(&self) -> bool {
        *self == BLK107_EN_A::BLK107_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK107_EN_1`"]
    #[inline(always)]
    pub fn is_blk107_en_1(&self) -> bool {
        *self == BLK107_EN_A::BLK107_EN_1
    }
}
#[doc = "Field `BLK107_EN` writer - When 1, Block107 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK107_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK107_EN_A, O>;
impl<'a, const O: u8> BLK107_EN_W<'a, O> {
    #[doc = "Block107 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk107_en_0(self) -> &'a mut W {
        self.variant(BLK107_EN_A::BLK107_EN_0)
    }
    #[doc = "Block107 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk107_en_1(self) -> &'a mut W {
        self.variant(BLK107_EN_A::BLK107_EN_1)
    }
}
#[doc = "Field `BLK108_EN` reader - When 1, Block108 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK108_EN_R = crate::BitReader<BLK108_EN_A>;
#[doc = "When 1, Block108 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK108_EN_A {
    #[doc = "0: Block108 of the SRAM is not retained in LPM3 or LPM4"]
    BLK108_EN_0 = 0,
    #[doc = "1: Block108 of the SRAM is retained in LPM3 and LPM4"]
    BLK108_EN_1 = 1,
}
impl From<BLK108_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK108_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK108_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK108_EN_A {
        match self.bits {
            false => BLK108_EN_A::BLK108_EN_0,
            true => BLK108_EN_A::BLK108_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK108_EN_0`"]
    #[inline(always)]
    pub fn is_blk108_en_0(&self) -> bool {
        *self == BLK108_EN_A::BLK108_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK108_EN_1`"]
    #[inline(always)]
    pub fn is_blk108_en_1(&self) -> bool {
        *self == BLK108_EN_A::BLK108_EN_1
    }
}
#[doc = "Field `BLK108_EN` writer - When 1, Block108 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK108_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK108_EN_A, O>;
impl<'a, const O: u8> BLK108_EN_W<'a, O> {
    #[doc = "Block108 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk108_en_0(self) -> &'a mut W {
        self.variant(BLK108_EN_A::BLK108_EN_0)
    }
    #[doc = "Block108 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk108_en_1(self) -> &'a mut W {
        self.variant(BLK108_EN_A::BLK108_EN_1)
    }
}
#[doc = "Field `BLK109_EN` reader - When 1, Block109 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK109_EN_R = crate::BitReader<BLK109_EN_A>;
#[doc = "When 1, Block109 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK109_EN_A {
    #[doc = "0: Block109 of the SRAM is not retained in LPM3 or LPM4"]
    BLK109_EN_0 = 0,
    #[doc = "1: Block109 of the SRAM is retained in LPM3 and LPM4"]
    BLK109_EN_1 = 1,
}
impl From<BLK109_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK109_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK109_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK109_EN_A {
        match self.bits {
            false => BLK109_EN_A::BLK109_EN_0,
            true => BLK109_EN_A::BLK109_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK109_EN_0`"]
    #[inline(always)]
    pub fn is_blk109_en_0(&self) -> bool {
        *self == BLK109_EN_A::BLK109_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK109_EN_1`"]
    #[inline(always)]
    pub fn is_blk109_en_1(&self) -> bool {
        *self == BLK109_EN_A::BLK109_EN_1
    }
}
#[doc = "Field `BLK109_EN` writer - When 1, Block109 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK109_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK109_EN_A, O>;
impl<'a, const O: u8> BLK109_EN_W<'a, O> {
    #[doc = "Block109 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk109_en_0(self) -> &'a mut W {
        self.variant(BLK109_EN_A::BLK109_EN_0)
    }
    #[doc = "Block109 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk109_en_1(self) -> &'a mut W {
        self.variant(BLK109_EN_A::BLK109_EN_1)
    }
}
#[doc = "Field `BLK110_EN` reader - When 1, Block110 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK110_EN_R = crate::BitReader<BLK110_EN_A>;
#[doc = "When 1, Block110 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK110_EN_A {
    #[doc = "0: Block110 of the SRAM is not retained in LPM3 or LPM4"]
    BLK110_EN_0 = 0,
    #[doc = "1: Block110 of the SRAM is retained in LPM3 and LPM4"]
    BLK110_EN_1 = 1,
}
impl From<BLK110_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK110_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK110_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK110_EN_A {
        match self.bits {
            false => BLK110_EN_A::BLK110_EN_0,
            true => BLK110_EN_A::BLK110_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK110_EN_0`"]
    #[inline(always)]
    pub fn is_blk110_en_0(&self) -> bool {
        *self == BLK110_EN_A::BLK110_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK110_EN_1`"]
    #[inline(always)]
    pub fn is_blk110_en_1(&self) -> bool {
        *self == BLK110_EN_A::BLK110_EN_1
    }
}
#[doc = "Field `BLK110_EN` writer - When 1, Block110 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK110_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK110_EN_A, O>;
impl<'a, const O: u8> BLK110_EN_W<'a, O> {
    #[doc = "Block110 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk110_en_0(self) -> &'a mut W {
        self.variant(BLK110_EN_A::BLK110_EN_0)
    }
    #[doc = "Block110 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk110_en_1(self) -> &'a mut W {
        self.variant(BLK110_EN_A::BLK110_EN_1)
    }
}
#[doc = "Field `BLK111_EN` reader - When 1, Block111 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK111_EN_R = crate::BitReader<BLK111_EN_A>;
#[doc = "When 1, Block111 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK111_EN_A {
    #[doc = "0: Block111 of the SRAM is not retained in LPM3 or LPM4"]
    BLK111_EN_0 = 0,
    #[doc = "1: Block111 of the SRAM is retained in LPM3 and LPM4"]
    BLK111_EN_1 = 1,
}
impl From<BLK111_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK111_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK111_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK111_EN_A {
        match self.bits {
            false => BLK111_EN_A::BLK111_EN_0,
            true => BLK111_EN_A::BLK111_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK111_EN_0`"]
    #[inline(always)]
    pub fn is_blk111_en_0(&self) -> bool {
        *self == BLK111_EN_A::BLK111_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK111_EN_1`"]
    #[inline(always)]
    pub fn is_blk111_en_1(&self) -> bool {
        *self == BLK111_EN_A::BLK111_EN_1
    }
}
#[doc = "Field `BLK111_EN` writer - When 1, Block111 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK111_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK111_EN_A, O>;
impl<'a, const O: u8> BLK111_EN_W<'a, O> {
    #[doc = "Block111 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk111_en_0(self) -> &'a mut W {
        self.variant(BLK111_EN_A::BLK111_EN_0)
    }
    #[doc = "Block111 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk111_en_1(self) -> &'a mut W {
        self.variant(BLK111_EN_A::BLK111_EN_1)
    }
}
#[doc = "Field `BLK112_EN` reader - When 1, Block112 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK112_EN_R = crate::BitReader<BLK112_EN_A>;
#[doc = "When 1, Block112 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK112_EN_A {
    #[doc = "0: Block112 of the SRAM is not retained in LPM3 or LPM4"]
    BLK112_EN_0 = 0,
    #[doc = "1: Block112 of the SRAM is retained in LPM3 and LPM4"]
    BLK112_EN_1 = 1,
}
impl From<BLK112_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK112_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK112_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK112_EN_A {
        match self.bits {
            false => BLK112_EN_A::BLK112_EN_0,
            true => BLK112_EN_A::BLK112_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK112_EN_0`"]
    #[inline(always)]
    pub fn is_blk112_en_0(&self) -> bool {
        *self == BLK112_EN_A::BLK112_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK112_EN_1`"]
    #[inline(always)]
    pub fn is_blk112_en_1(&self) -> bool {
        *self == BLK112_EN_A::BLK112_EN_1
    }
}
#[doc = "Field `BLK112_EN` writer - When 1, Block112 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK112_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK112_EN_A, O>;
impl<'a, const O: u8> BLK112_EN_W<'a, O> {
    #[doc = "Block112 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk112_en_0(self) -> &'a mut W {
        self.variant(BLK112_EN_A::BLK112_EN_0)
    }
    #[doc = "Block112 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk112_en_1(self) -> &'a mut W {
        self.variant(BLK112_EN_A::BLK112_EN_1)
    }
}
#[doc = "Field `BLK113_EN` reader - When 1, Block113 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK113_EN_R = crate::BitReader<BLK113_EN_A>;
#[doc = "When 1, Block113 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK113_EN_A {
    #[doc = "0: Block113 of the SRAM is not retained in LPM3 or LPM4"]
    BLK113_EN_0 = 0,
    #[doc = "1: Block113 of the SRAM is retained in LPM3 and LPM4"]
    BLK113_EN_1 = 1,
}
impl From<BLK113_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK113_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK113_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK113_EN_A {
        match self.bits {
            false => BLK113_EN_A::BLK113_EN_0,
            true => BLK113_EN_A::BLK113_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK113_EN_0`"]
    #[inline(always)]
    pub fn is_blk113_en_0(&self) -> bool {
        *self == BLK113_EN_A::BLK113_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK113_EN_1`"]
    #[inline(always)]
    pub fn is_blk113_en_1(&self) -> bool {
        *self == BLK113_EN_A::BLK113_EN_1
    }
}
#[doc = "Field `BLK113_EN` writer - When 1, Block113 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK113_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK113_EN_A, O>;
impl<'a, const O: u8> BLK113_EN_W<'a, O> {
    #[doc = "Block113 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk113_en_0(self) -> &'a mut W {
        self.variant(BLK113_EN_A::BLK113_EN_0)
    }
    #[doc = "Block113 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk113_en_1(self) -> &'a mut W {
        self.variant(BLK113_EN_A::BLK113_EN_1)
    }
}
#[doc = "Field `BLK114_EN` reader - When 1, Block114 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK114_EN_R = crate::BitReader<BLK114_EN_A>;
#[doc = "When 1, Block114 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK114_EN_A {
    #[doc = "0: Block114 of the SRAM is not retained in LPM3 or LPM4"]
    BLK114_EN_0 = 0,
    #[doc = "1: Block114 of the SRAM is retained in LPM3 and LPM4"]
    BLK114_EN_1 = 1,
}
impl From<BLK114_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK114_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK114_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK114_EN_A {
        match self.bits {
            false => BLK114_EN_A::BLK114_EN_0,
            true => BLK114_EN_A::BLK114_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK114_EN_0`"]
    #[inline(always)]
    pub fn is_blk114_en_0(&self) -> bool {
        *self == BLK114_EN_A::BLK114_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK114_EN_1`"]
    #[inline(always)]
    pub fn is_blk114_en_1(&self) -> bool {
        *self == BLK114_EN_A::BLK114_EN_1
    }
}
#[doc = "Field `BLK114_EN` writer - When 1, Block114 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK114_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK114_EN_A, O>;
impl<'a, const O: u8> BLK114_EN_W<'a, O> {
    #[doc = "Block114 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk114_en_0(self) -> &'a mut W {
        self.variant(BLK114_EN_A::BLK114_EN_0)
    }
    #[doc = "Block114 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk114_en_1(self) -> &'a mut W {
        self.variant(BLK114_EN_A::BLK114_EN_1)
    }
}
#[doc = "Field `BLK115_EN` reader - When 1, Block115 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK115_EN_R = crate::BitReader<BLK115_EN_A>;
#[doc = "When 1, Block115 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK115_EN_A {
    #[doc = "0: Block115 of the SRAM is not retained in LPM3 or LPM4"]
    BLK115_EN_0 = 0,
    #[doc = "1: Block115 of the SRAM is retained in LPM3 and LPM4"]
    BLK115_EN_1 = 1,
}
impl From<BLK115_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK115_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK115_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK115_EN_A {
        match self.bits {
            false => BLK115_EN_A::BLK115_EN_0,
            true => BLK115_EN_A::BLK115_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK115_EN_0`"]
    #[inline(always)]
    pub fn is_blk115_en_0(&self) -> bool {
        *self == BLK115_EN_A::BLK115_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK115_EN_1`"]
    #[inline(always)]
    pub fn is_blk115_en_1(&self) -> bool {
        *self == BLK115_EN_A::BLK115_EN_1
    }
}
#[doc = "Field `BLK115_EN` writer - When 1, Block115 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK115_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK115_EN_A, O>;
impl<'a, const O: u8> BLK115_EN_W<'a, O> {
    #[doc = "Block115 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk115_en_0(self) -> &'a mut W {
        self.variant(BLK115_EN_A::BLK115_EN_0)
    }
    #[doc = "Block115 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk115_en_1(self) -> &'a mut W {
        self.variant(BLK115_EN_A::BLK115_EN_1)
    }
}
#[doc = "Field `BLK116_EN` reader - When 1, Block116 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK116_EN_R = crate::BitReader<BLK116_EN_A>;
#[doc = "When 1, Block116 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK116_EN_A {
    #[doc = "0: Block116 of the SRAM is not retained in LPM3 or LPM4"]
    BLK116_EN_0 = 0,
    #[doc = "1: Block116 of the SRAM is retained in LPM3 and LPM4"]
    BLK116_EN_1 = 1,
}
impl From<BLK116_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK116_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK116_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK116_EN_A {
        match self.bits {
            false => BLK116_EN_A::BLK116_EN_0,
            true => BLK116_EN_A::BLK116_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK116_EN_0`"]
    #[inline(always)]
    pub fn is_blk116_en_0(&self) -> bool {
        *self == BLK116_EN_A::BLK116_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK116_EN_1`"]
    #[inline(always)]
    pub fn is_blk116_en_1(&self) -> bool {
        *self == BLK116_EN_A::BLK116_EN_1
    }
}
#[doc = "Field `BLK116_EN` writer - When 1, Block116 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK116_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK116_EN_A, O>;
impl<'a, const O: u8> BLK116_EN_W<'a, O> {
    #[doc = "Block116 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk116_en_0(self) -> &'a mut W {
        self.variant(BLK116_EN_A::BLK116_EN_0)
    }
    #[doc = "Block116 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk116_en_1(self) -> &'a mut W {
        self.variant(BLK116_EN_A::BLK116_EN_1)
    }
}
#[doc = "Field `BLK117_EN` reader - When 1, Block117 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK117_EN_R = crate::BitReader<BLK117_EN_A>;
#[doc = "When 1, Block117 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK117_EN_A {
    #[doc = "0: Block117 of the SRAM is not retained in LPM3 or LPM4"]
    BLK117_EN_0 = 0,
    #[doc = "1: Block117 of the SRAM is retained in LPM3 and LPM4"]
    BLK117_EN_1 = 1,
}
impl From<BLK117_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK117_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK117_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK117_EN_A {
        match self.bits {
            false => BLK117_EN_A::BLK117_EN_0,
            true => BLK117_EN_A::BLK117_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK117_EN_0`"]
    #[inline(always)]
    pub fn is_blk117_en_0(&self) -> bool {
        *self == BLK117_EN_A::BLK117_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK117_EN_1`"]
    #[inline(always)]
    pub fn is_blk117_en_1(&self) -> bool {
        *self == BLK117_EN_A::BLK117_EN_1
    }
}
#[doc = "Field `BLK117_EN` writer - When 1, Block117 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK117_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK117_EN_A, O>;
impl<'a, const O: u8> BLK117_EN_W<'a, O> {
    #[doc = "Block117 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk117_en_0(self) -> &'a mut W {
        self.variant(BLK117_EN_A::BLK117_EN_0)
    }
    #[doc = "Block117 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk117_en_1(self) -> &'a mut W {
        self.variant(BLK117_EN_A::BLK117_EN_1)
    }
}
#[doc = "Field `BLK118_EN` reader - When 1, Block118 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK118_EN_R = crate::BitReader<BLK118_EN_A>;
#[doc = "When 1, Block118 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK118_EN_A {
    #[doc = "0: Block118 of the SRAM is not retained in LPM3 or LPM4"]
    BLK118_EN_0 = 0,
    #[doc = "1: Block118 of the SRAM is retained in LPM3 and LPM4"]
    BLK118_EN_1 = 1,
}
impl From<BLK118_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK118_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK118_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK118_EN_A {
        match self.bits {
            false => BLK118_EN_A::BLK118_EN_0,
            true => BLK118_EN_A::BLK118_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK118_EN_0`"]
    #[inline(always)]
    pub fn is_blk118_en_0(&self) -> bool {
        *self == BLK118_EN_A::BLK118_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK118_EN_1`"]
    #[inline(always)]
    pub fn is_blk118_en_1(&self) -> bool {
        *self == BLK118_EN_A::BLK118_EN_1
    }
}
#[doc = "Field `BLK118_EN` writer - When 1, Block118 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK118_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK118_EN_A, O>;
impl<'a, const O: u8> BLK118_EN_W<'a, O> {
    #[doc = "Block118 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk118_en_0(self) -> &'a mut W {
        self.variant(BLK118_EN_A::BLK118_EN_0)
    }
    #[doc = "Block118 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk118_en_1(self) -> &'a mut W {
        self.variant(BLK118_EN_A::BLK118_EN_1)
    }
}
#[doc = "Field `BLK119_EN` reader - When 1, Block119 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK119_EN_R = crate::BitReader<BLK119_EN_A>;
#[doc = "When 1, Block119 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK119_EN_A {
    #[doc = "0: Block119 of the SRAM is not retained in LPM3 or LPM4"]
    BLK119_EN_0 = 0,
    #[doc = "1: Block119 of the SRAM is retained in LPM3 and LPM4"]
    BLK119_EN_1 = 1,
}
impl From<BLK119_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK119_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK119_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK119_EN_A {
        match self.bits {
            false => BLK119_EN_A::BLK119_EN_0,
            true => BLK119_EN_A::BLK119_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK119_EN_0`"]
    #[inline(always)]
    pub fn is_blk119_en_0(&self) -> bool {
        *self == BLK119_EN_A::BLK119_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK119_EN_1`"]
    #[inline(always)]
    pub fn is_blk119_en_1(&self) -> bool {
        *self == BLK119_EN_A::BLK119_EN_1
    }
}
#[doc = "Field `BLK119_EN` writer - When 1, Block119 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK119_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK119_EN_A, O>;
impl<'a, const O: u8> BLK119_EN_W<'a, O> {
    #[doc = "Block119 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk119_en_0(self) -> &'a mut W {
        self.variant(BLK119_EN_A::BLK119_EN_0)
    }
    #[doc = "Block119 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk119_en_1(self) -> &'a mut W {
        self.variant(BLK119_EN_A::BLK119_EN_1)
    }
}
#[doc = "Field `BLK120_EN` reader - When 1, Block120 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK120_EN_R = crate::BitReader<BLK120_EN_A>;
#[doc = "When 1, Block120 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK120_EN_A {
    #[doc = "0: Block120 of the SRAM is not retained in LPM3 or LPM4"]
    BLK120_EN_0 = 0,
    #[doc = "1: Block120 of the SRAM is retained in LPM3 and LPM4"]
    BLK120_EN_1 = 1,
}
impl From<BLK120_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK120_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK120_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK120_EN_A {
        match self.bits {
            false => BLK120_EN_A::BLK120_EN_0,
            true => BLK120_EN_A::BLK120_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK120_EN_0`"]
    #[inline(always)]
    pub fn is_blk120_en_0(&self) -> bool {
        *self == BLK120_EN_A::BLK120_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK120_EN_1`"]
    #[inline(always)]
    pub fn is_blk120_en_1(&self) -> bool {
        *self == BLK120_EN_A::BLK120_EN_1
    }
}
#[doc = "Field `BLK120_EN` writer - When 1, Block120 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK120_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK120_EN_A, O>;
impl<'a, const O: u8> BLK120_EN_W<'a, O> {
    #[doc = "Block120 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk120_en_0(self) -> &'a mut W {
        self.variant(BLK120_EN_A::BLK120_EN_0)
    }
    #[doc = "Block120 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk120_en_1(self) -> &'a mut W {
        self.variant(BLK120_EN_A::BLK120_EN_1)
    }
}
#[doc = "Field `BLK121_EN` reader - When 1, Block121 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK121_EN_R = crate::BitReader<BLK121_EN_A>;
#[doc = "When 1, Block121 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK121_EN_A {
    #[doc = "0: Block121 of the SRAM is not retained in LPM3 or LPM4"]
    BLK121_EN_0 = 0,
    #[doc = "1: Block121 of the SRAM is retained in LPM3 and LPM4"]
    BLK121_EN_1 = 1,
}
impl From<BLK121_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK121_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK121_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK121_EN_A {
        match self.bits {
            false => BLK121_EN_A::BLK121_EN_0,
            true => BLK121_EN_A::BLK121_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK121_EN_0`"]
    #[inline(always)]
    pub fn is_blk121_en_0(&self) -> bool {
        *self == BLK121_EN_A::BLK121_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK121_EN_1`"]
    #[inline(always)]
    pub fn is_blk121_en_1(&self) -> bool {
        *self == BLK121_EN_A::BLK121_EN_1
    }
}
#[doc = "Field `BLK121_EN` writer - When 1, Block121 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK121_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK121_EN_A, O>;
impl<'a, const O: u8> BLK121_EN_W<'a, O> {
    #[doc = "Block121 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk121_en_0(self) -> &'a mut W {
        self.variant(BLK121_EN_A::BLK121_EN_0)
    }
    #[doc = "Block121 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk121_en_1(self) -> &'a mut W {
        self.variant(BLK121_EN_A::BLK121_EN_1)
    }
}
#[doc = "Field `BLK122_EN` reader - When 1, Block122 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK122_EN_R = crate::BitReader<BLK122_EN_A>;
#[doc = "When 1, Block122 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK122_EN_A {
    #[doc = "0: Block122 of the SRAM is not retained in LPM3 or LPM4"]
    BLK122_EN_0 = 0,
    #[doc = "1: Block122 of the SRAM is retained in LPM3 and LPM4"]
    BLK122_EN_1 = 1,
}
impl From<BLK122_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK122_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK122_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK122_EN_A {
        match self.bits {
            false => BLK122_EN_A::BLK122_EN_0,
            true => BLK122_EN_A::BLK122_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK122_EN_0`"]
    #[inline(always)]
    pub fn is_blk122_en_0(&self) -> bool {
        *self == BLK122_EN_A::BLK122_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK122_EN_1`"]
    #[inline(always)]
    pub fn is_blk122_en_1(&self) -> bool {
        *self == BLK122_EN_A::BLK122_EN_1
    }
}
#[doc = "Field `BLK122_EN` writer - When 1, Block122 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK122_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK122_EN_A, O>;
impl<'a, const O: u8> BLK122_EN_W<'a, O> {
    #[doc = "Block122 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk122_en_0(self) -> &'a mut W {
        self.variant(BLK122_EN_A::BLK122_EN_0)
    }
    #[doc = "Block122 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk122_en_1(self) -> &'a mut W {
        self.variant(BLK122_EN_A::BLK122_EN_1)
    }
}
#[doc = "Field `BLK123_EN` reader - When 1, Block123 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK123_EN_R = crate::BitReader<BLK123_EN_A>;
#[doc = "When 1, Block123 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK123_EN_A {
    #[doc = "0: Block123 of the SRAM is not retained in LPM3 or LPM4"]
    BLK123_EN_0 = 0,
    #[doc = "1: Block123 of the SRAM is retained in LPM3 and LPM4"]
    BLK123_EN_1 = 1,
}
impl From<BLK123_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK123_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK123_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK123_EN_A {
        match self.bits {
            false => BLK123_EN_A::BLK123_EN_0,
            true => BLK123_EN_A::BLK123_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK123_EN_0`"]
    #[inline(always)]
    pub fn is_blk123_en_0(&self) -> bool {
        *self == BLK123_EN_A::BLK123_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK123_EN_1`"]
    #[inline(always)]
    pub fn is_blk123_en_1(&self) -> bool {
        *self == BLK123_EN_A::BLK123_EN_1
    }
}
#[doc = "Field `BLK123_EN` writer - When 1, Block123 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK123_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK123_EN_A, O>;
impl<'a, const O: u8> BLK123_EN_W<'a, O> {
    #[doc = "Block123 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk123_en_0(self) -> &'a mut W {
        self.variant(BLK123_EN_A::BLK123_EN_0)
    }
    #[doc = "Block123 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk123_en_1(self) -> &'a mut W {
        self.variant(BLK123_EN_A::BLK123_EN_1)
    }
}
#[doc = "Field `BLK124_EN` reader - When 1, Block124 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK124_EN_R = crate::BitReader<BLK124_EN_A>;
#[doc = "When 1, Block124 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK124_EN_A {
    #[doc = "0: Block124 of the SRAM is not retained in LPM3 or LPM4"]
    BLK124_EN_0 = 0,
    #[doc = "1: Block124 of the SRAM is retained in LPM3 and LPM4"]
    BLK124_EN_1 = 1,
}
impl From<BLK124_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK124_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK124_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK124_EN_A {
        match self.bits {
            false => BLK124_EN_A::BLK124_EN_0,
            true => BLK124_EN_A::BLK124_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK124_EN_0`"]
    #[inline(always)]
    pub fn is_blk124_en_0(&self) -> bool {
        *self == BLK124_EN_A::BLK124_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK124_EN_1`"]
    #[inline(always)]
    pub fn is_blk124_en_1(&self) -> bool {
        *self == BLK124_EN_A::BLK124_EN_1
    }
}
#[doc = "Field `BLK124_EN` writer - When 1, Block124 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK124_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK124_EN_A, O>;
impl<'a, const O: u8> BLK124_EN_W<'a, O> {
    #[doc = "Block124 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk124_en_0(self) -> &'a mut W {
        self.variant(BLK124_EN_A::BLK124_EN_0)
    }
    #[doc = "Block124 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk124_en_1(self) -> &'a mut W {
        self.variant(BLK124_EN_A::BLK124_EN_1)
    }
}
#[doc = "Field `BLK125_EN` reader - When 1, Block125 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK125_EN_R = crate::BitReader<BLK125_EN_A>;
#[doc = "When 1, Block125 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK125_EN_A {
    #[doc = "0: Block125 of the SRAM is not retained in LPM3 or LPM4"]
    BLK125_EN_0 = 0,
    #[doc = "1: Block125 of the SRAM is retained in LPM3 and LPM4"]
    BLK125_EN_1 = 1,
}
impl From<BLK125_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK125_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK125_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK125_EN_A {
        match self.bits {
            false => BLK125_EN_A::BLK125_EN_0,
            true => BLK125_EN_A::BLK125_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK125_EN_0`"]
    #[inline(always)]
    pub fn is_blk125_en_0(&self) -> bool {
        *self == BLK125_EN_A::BLK125_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK125_EN_1`"]
    #[inline(always)]
    pub fn is_blk125_en_1(&self) -> bool {
        *self == BLK125_EN_A::BLK125_EN_1
    }
}
#[doc = "Field `BLK125_EN` writer - When 1, Block125 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK125_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK125_EN_A, O>;
impl<'a, const O: u8> BLK125_EN_W<'a, O> {
    #[doc = "Block125 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk125_en_0(self) -> &'a mut W {
        self.variant(BLK125_EN_A::BLK125_EN_0)
    }
    #[doc = "Block125 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk125_en_1(self) -> &'a mut W {
        self.variant(BLK125_EN_A::BLK125_EN_1)
    }
}
#[doc = "Field `BLK126_EN` reader - When 1, Block126 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK126_EN_R = crate::BitReader<BLK126_EN_A>;
#[doc = "When 1, Block126 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK126_EN_A {
    #[doc = "0: Block126 of the SRAM is not retained in LPM3 or LPM4"]
    BLK126_EN_0 = 0,
    #[doc = "1: Block126 of the SRAM is retained in LPM3 and LPM4"]
    BLK126_EN_1 = 1,
}
impl From<BLK126_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK126_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK126_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK126_EN_A {
        match self.bits {
            false => BLK126_EN_A::BLK126_EN_0,
            true => BLK126_EN_A::BLK126_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK126_EN_0`"]
    #[inline(always)]
    pub fn is_blk126_en_0(&self) -> bool {
        *self == BLK126_EN_A::BLK126_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK126_EN_1`"]
    #[inline(always)]
    pub fn is_blk126_en_1(&self) -> bool {
        *self == BLK126_EN_A::BLK126_EN_1
    }
}
#[doc = "Field `BLK126_EN` writer - When 1, Block126 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK126_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK126_EN_A, O>;
impl<'a, const O: u8> BLK126_EN_W<'a, O> {
    #[doc = "Block126 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk126_en_0(self) -> &'a mut W {
        self.variant(BLK126_EN_A::BLK126_EN_0)
    }
    #[doc = "Block126 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk126_en_1(self) -> &'a mut W {
        self.variant(BLK126_EN_A::BLK126_EN_1)
    }
}
#[doc = "Field `BLK127_EN` reader - When 1, Block127 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK127_EN_R = crate::BitReader<BLK127_EN_A>;
#[doc = "When 1, Block127 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK127_EN_A {
    #[doc = "0: Block127 of the SRAM is not retained in LPM3 or LPM4"]
    BLK127_EN_0 = 0,
    #[doc = "1: Block127 of the SRAM is retained in LPM3 and LPM4"]
    BLK127_EN_1 = 1,
}
impl From<BLK127_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK127_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK127_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK127_EN_A {
        match self.bits {
            false => BLK127_EN_A::BLK127_EN_0,
            true => BLK127_EN_A::BLK127_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK127_EN_0`"]
    #[inline(always)]
    pub fn is_blk127_en_0(&self) -> bool {
        *self == BLK127_EN_A::BLK127_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK127_EN_1`"]
    #[inline(always)]
    pub fn is_blk127_en_1(&self) -> bool {
        *self == BLK127_EN_A::BLK127_EN_1
    }
}
#[doc = "Field `BLK127_EN` writer - When 1, Block127 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK127_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL3_SPEC, BLK127_EN_A, O>;
impl<'a, const O: u8> BLK127_EN_W<'a, O> {
    #[doc = "Block127 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk127_en_0(self) -> &'a mut W {
        self.variant(BLK127_EN_A::BLK127_EN_0)
    }
    #[doc = "Block127 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk127_en_1(self) -> &'a mut W {
        self.variant(BLK127_EN_A::BLK127_EN_1)
    }
}
impl R {
    #[doc = "Bit 0 - When 1, Block96 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk96_en(&self) -> BLK96_EN_R {
        BLK96_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, Block97 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk97_en(&self) -> BLK97_EN_R {
        BLK97_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, Block98 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk98_en(&self) -> BLK98_EN_R {
        BLK98_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, Block99 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk99_en(&self) -> BLK99_EN_R {
        BLK99_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, Block100 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk100_en(&self) -> BLK100_EN_R {
        BLK100_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, Block101 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk101_en(&self) -> BLK101_EN_R {
        BLK101_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, Block102 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk102_en(&self) -> BLK102_EN_R {
        BLK102_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, Block103 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk103_en(&self) -> BLK103_EN_R {
        BLK103_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, Block104 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk104_en(&self) -> BLK104_EN_R {
        BLK104_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, Block105 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk105_en(&self) -> BLK105_EN_R {
        BLK105_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When 1, Block106 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk106_en(&self) -> BLK106_EN_R {
        BLK106_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When 1, Block107 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk107_en(&self) -> BLK107_EN_R {
        BLK107_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When 1, Block108 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk108_en(&self) -> BLK108_EN_R {
        BLK108_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When 1, Block109 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk109_en(&self) -> BLK109_EN_R {
        BLK109_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When 1, Block110 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk110_en(&self) -> BLK110_EN_R {
        BLK110_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When 1, Block111 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk111_en(&self) -> BLK111_EN_R {
        BLK111_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When 1, Block112 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk112_en(&self) -> BLK112_EN_R {
        BLK112_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When 1, Block113 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk113_en(&self) -> BLK113_EN_R {
        BLK113_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When 1, Block114 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk114_en(&self) -> BLK114_EN_R {
        BLK114_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When 1, Block115 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk115_en(&self) -> BLK115_EN_R {
        BLK115_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When 1, Block116 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk116_en(&self) -> BLK116_EN_R {
        BLK116_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When 1, Block117 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk117_en(&self) -> BLK117_EN_R {
        BLK117_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When 1, Block118 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk118_en(&self) -> BLK118_EN_R {
        BLK118_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When 1, Block119 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk119_en(&self) -> BLK119_EN_R {
        BLK119_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When 1, Block120 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk120_en(&self) -> BLK120_EN_R {
        BLK120_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When 1, Block121 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk121_en(&self) -> BLK121_EN_R {
        BLK121_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When 1, Block122 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk122_en(&self) -> BLK122_EN_R {
        BLK122_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When 1, Block123 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk123_en(&self) -> BLK123_EN_R {
        BLK123_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When 1, Block124 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk124_en(&self) -> BLK124_EN_R {
        BLK124_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When 1, Block125 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk125_en(&self) -> BLK125_EN_R {
        BLK125_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When 1, Block126 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk126_en(&self) -> BLK126_EN_R {
        BLK126_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When 1, Block127 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk127_en(&self) -> BLK127_EN_R {
        BLK127_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, Block96 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk96_en(&mut self) -> BLK96_EN_W<0> {
        BLK96_EN_W::new(self)
    }
    #[doc = "Bit 1 - When 1, Block97 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk97_en(&mut self) -> BLK97_EN_W<1> {
        BLK97_EN_W::new(self)
    }
    #[doc = "Bit 2 - When 1, Block98 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk98_en(&mut self) -> BLK98_EN_W<2> {
        BLK98_EN_W::new(self)
    }
    #[doc = "Bit 3 - When 1, Block99 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk99_en(&mut self) -> BLK99_EN_W<3> {
        BLK99_EN_W::new(self)
    }
    #[doc = "Bit 4 - When 1, Block100 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk100_en(&mut self) -> BLK100_EN_W<4> {
        BLK100_EN_W::new(self)
    }
    #[doc = "Bit 5 - When 1, Block101 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk101_en(&mut self) -> BLK101_EN_W<5> {
        BLK101_EN_W::new(self)
    }
    #[doc = "Bit 6 - When 1, Block102 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk102_en(&mut self) -> BLK102_EN_W<6> {
        BLK102_EN_W::new(self)
    }
    #[doc = "Bit 7 - When 1, Block103 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk103_en(&mut self) -> BLK103_EN_W<7> {
        BLK103_EN_W::new(self)
    }
    #[doc = "Bit 8 - When 1, Block104 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk104_en(&mut self) -> BLK104_EN_W<8> {
        BLK104_EN_W::new(self)
    }
    #[doc = "Bit 9 - When 1, Block105 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk105_en(&mut self) -> BLK105_EN_W<9> {
        BLK105_EN_W::new(self)
    }
    #[doc = "Bit 10 - When 1, Block106 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk106_en(&mut self) -> BLK106_EN_W<10> {
        BLK106_EN_W::new(self)
    }
    #[doc = "Bit 11 - When 1, Block107 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk107_en(&mut self) -> BLK107_EN_W<11> {
        BLK107_EN_W::new(self)
    }
    #[doc = "Bit 12 - When 1, Block108 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk108_en(&mut self) -> BLK108_EN_W<12> {
        BLK108_EN_W::new(self)
    }
    #[doc = "Bit 13 - When 1, Block109 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk109_en(&mut self) -> BLK109_EN_W<13> {
        BLK109_EN_W::new(self)
    }
    #[doc = "Bit 14 - When 1, Block110 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk110_en(&mut self) -> BLK110_EN_W<14> {
        BLK110_EN_W::new(self)
    }
    #[doc = "Bit 15 - When 1, Block111 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk111_en(&mut self) -> BLK111_EN_W<15> {
        BLK111_EN_W::new(self)
    }
    #[doc = "Bit 16 - When 1, Block112 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk112_en(&mut self) -> BLK112_EN_W<16> {
        BLK112_EN_W::new(self)
    }
    #[doc = "Bit 17 - When 1, Block113 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk113_en(&mut self) -> BLK113_EN_W<17> {
        BLK113_EN_W::new(self)
    }
    #[doc = "Bit 18 - When 1, Block114 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk114_en(&mut self) -> BLK114_EN_W<18> {
        BLK114_EN_W::new(self)
    }
    #[doc = "Bit 19 - When 1, Block115 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk115_en(&mut self) -> BLK115_EN_W<19> {
        BLK115_EN_W::new(self)
    }
    #[doc = "Bit 20 - When 1, Block116 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk116_en(&mut self) -> BLK116_EN_W<20> {
        BLK116_EN_W::new(self)
    }
    #[doc = "Bit 21 - When 1, Block117 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk117_en(&mut self) -> BLK117_EN_W<21> {
        BLK117_EN_W::new(self)
    }
    #[doc = "Bit 22 - When 1, Block118 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk118_en(&mut self) -> BLK118_EN_W<22> {
        BLK118_EN_W::new(self)
    }
    #[doc = "Bit 23 - When 1, Block119 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk119_en(&mut self) -> BLK119_EN_W<23> {
        BLK119_EN_W::new(self)
    }
    #[doc = "Bit 24 - When 1, Block120 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk120_en(&mut self) -> BLK120_EN_W<24> {
        BLK120_EN_W::new(self)
    }
    #[doc = "Bit 25 - When 1, Block121 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk121_en(&mut self) -> BLK121_EN_W<25> {
        BLK121_EN_W::new(self)
    }
    #[doc = "Bit 26 - When 1, Block122 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk122_en(&mut self) -> BLK122_EN_W<26> {
        BLK122_EN_W::new(self)
    }
    #[doc = "Bit 27 - When 1, Block123 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk123_en(&mut self) -> BLK123_EN_W<27> {
        BLK123_EN_W::new(self)
    }
    #[doc = "Bit 28 - When 1, Block124 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk124_en(&mut self) -> BLK124_EN_W<28> {
        BLK124_EN_W::new(self)
    }
    #[doc = "Bit 29 - When 1, Block125 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk125_en(&mut self) -> BLK125_EN_W<29> {
        BLK125_EN_W::new(self)
    }
    #[doc = "Bit 30 - When 1, Block126 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk126_en(&mut self) -> BLK126_EN_W<30> {
        BLK126_EN_W::new(self)
    }
    #[doc = "Bit 31 - When 1, Block127 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk127_en(&mut self) -> BLK127_EN_W<31> {
        BLK127_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Block Retention Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_blkret_ctl3](index.html) module"]
pub struct SYS_SRAM_BLKRET_CTL3_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BLKRET_CTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_blkret_ctl3::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BLKRET_CTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_blkret_ctl3::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_BLKRET_CTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_BLKRET_CTL3 to value 0xffff_ffff"]
impl crate::Resettable for SYS_SRAM_BLKRET_CTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
