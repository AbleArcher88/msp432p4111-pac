#[doc = "Register `FLCTL_BANK1_MAIN_WEPROT1` reader"]
pub struct R(crate::R<FLCTL_BANK1_MAIN_WEPROT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BANK1_MAIN_WEPROT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BANK1_MAIN_WEPROT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BANK1_MAIN_WEPROT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BANK1_MAIN_WEPROT1` writer"]
pub struct W(crate::W<FLCTL_BANK1_MAIN_WEPROT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BANK1_MAIN_WEPROT1_SPEC>;
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
impl From<crate::W<FLCTL_BANK1_MAIN_WEPROT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BANK1_MAIN_WEPROT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT32` reader - Protects Sector 32 from program or erase"]
pub type PROT32_R = crate::BitReader<bool>;
#[doc = "Field `PROT32` writer - Protects Sector 32 from program or erase"]
pub type PROT32_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT33` reader - Protects Sector 33 from program or erase"]
pub type PROT33_R = crate::BitReader<bool>;
#[doc = "Field `PROT33` writer - Protects Sector 33 from program or erase"]
pub type PROT33_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT34` reader - Protects Sector 34 from program or erase"]
pub type PROT34_R = crate::BitReader<bool>;
#[doc = "Field `PROT34` writer - Protects Sector 34 from program or erase"]
pub type PROT34_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT35` reader - Protects Sector 35 from program or erase"]
pub type PROT35_R = crate::BitReader<bool>;
#[doc = "Field `PROT35` writer - Protects Sector 35 from program or erase"]
pub type PROT35_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT36` reader - Protects Sector 36 from program or erase"]
pub type PROT36_R = crate::BitReader<bool>;
#[doc = "Field `PROT36` writer - Protects Sector 36 from program or erase"]
pub type PROT36_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT37` reader - Protects Sector 37 from program or erase"]
pub type PROT37_R = crate::BitReader<bool>;
#[doc = "Field `PROT37` writer - Protects Sector 37 from program or erase"]
pub type PROT37_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT38` reader - Protects Sector 38 from program or erase"]
pub type PROT38_R = crate::BitReader<bool>;
#[doc = "Field `PROT38` writer - Protects Sector 38 from program or erase"]
pub type PROT38_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT39` reader - Protects Sector 39 from program or erase"]
pub type PROT39_R = crate::BitReader<bool>;
#[doc = "Field `PROT39` writer - Protects Sector 39 from program or erase"]
pub type PROT39_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT40` reader - Protects Sector 40 from program or erase"]
pub type PROT40_R = crate::BitReader<bool>;
#[doc = "Field `PROT40` writer - Protects Sector 40 from program or erase"]
pub type PROT40_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT41` reader - Protects Sector 41 from program or erase"]
pub type PROT41_R = crate::BitReader<bool>;
#[doc = "Field `PROT41` writer - Protects Sector 41 from program or erase"]
pub type PROT41_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT42` reader - Protects Sector 42 from program or erase"]
pub type PROT42_R = crate::BitReader<bool>;
#[doc = "Field `PROT42` writer - Protects Sector 42 from program or erase"]
pub type PROT42_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT43` reader - Protects Sector 43 from program or erase"]
pub type PROT43_R = crate::BitReader<bool>;
#[doc = "Field `PROT43` writer - Protects Sector 43 from program or erase"]
pub type PROT43_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT44` reader - Protects Sector 44 from program or erase"]
pub type PROT44_R = crate::BitReader<bool>;
#[doc = "Field `PROT44` writer - Protects Sector 44 from program or erase"]
pub type PROT44_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT45` reader - Protects Sector 45 from program or erase"]
pub type PROT45_R = crate::BitReader<bool>;
#[doc = "Field `PROT45` writer - Protects Sector 45 from program or erase"]
pub type PROT45_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT46` reader - Protects Sector 46 from program or erase"]
pub type PROT46_R = crate::BitReader<bool>;
#[doc = "Field `PROT46` writer - Protects Sector 46 from program or erase"]
pub type PROT46_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT47` reader - Protects Sector 47 from program or erase"]
pub type PROT47_R = crate::BitReader<bool>;
#[doc = "Field `PROT47` writer - Protects Sector 47 from program or erase"]
pub type PROT47_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT48` reader - Protects Sector 48 from program or erase"]
pub type PROT48_R = crate::BitReader<bool>;
#[doc = "Field `PROT48` writer - Protects Sector 48 from program or erase"]
pub type PROT48_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT49` reader - Protects Sector 49 from program or erase"]
pub type PROT49_R = crate::BitReader<bool>;
#[doc = "Field `PROT49` writer - Protects Sector 49 from program or erase"]
pub type PROT49_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT50` reader - Protects Sector 50 from program or erase"]
pub type PROT50_R = crate::BitReader<bool>;
#[doc = "Field `PROT50` writer - Protects Sector 50 from program or erase"]
pub type PROT50_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT51` reader - Protects Sector 51 from program or erase"]
pub type PROT51_R = crate::BitReader<bool>;
#[doc = "Field `PROT51` writer - Protects Sector 51 from program or erase"]
pub type PROT51_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT52` reader - Protects Sector 52 from program or erase"]
pub type PROT52_R = crate::BitReader<bool>;
#[doc = "Field `PROT52` writer - Protects Sector 52 from program or erase"]
pub type PROT52_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT53` reader - Protects Sector 53 from program or erase"]
pub type PROT53_R = crate::BitReader<bool>;
#[doc = "Field `PROT53` writer - Protects Sector 53 from program or erase"]
pub type PROT53_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT54` reader - Protects Sector 54 from program or erase"]
pub type PROT54_R = crate::BitReader<bool>;
#[doc = "Field `PROT54` writer - Protects Sector 54 from program or erase"]
pub type PROT54_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT55` reader - Protects Sector 55 from program or erase"]
pub type PROT55_R = crate::BitReader<bool>;
#[doc = "Field `PROT55` writer - Protects Sector 55 from program or erase"]
pub type PROT55_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT56` reader - Protects Sector 56 from program or erase"]
pub type PROT56_R = crate::BitReader<bool>;
#[doc = "Field `PROT56` writer - Protects Sector 56 from program or erase"]
pub type PROT56_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT57` reader - Protects Sector 57 from program or erase"]
pub type PROT57_R = crate::BitReader<bool>;
#[doc = "Field `PROT57` writer - Protects Sector 57 from program or erase"]
pub type PROT57_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT58` reader - Protects Sector 58 from program or erase"]
pub type PROT58_R = crate::BitReader<bool>;
#[doc = "Field `PROT58` writer - Protects Sector 58 from program or erase"]
pub type PROT58_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT59` reader - Protects Sector 59 from program or erase"]
pub type PROT59_R = crate::BitReader<bool>;
#[doc = "Field `PROT59` writer - Protects Sector 59 from program or erase"]
pub type PROT59_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT60` reader - Protects Sector 60 from program or erase"]
pub type PROT60_R = crate::BitReader<bool>;
#[doc = "Field `PROT60` writer - Protects Sector 60 from program or erase"]
pub type PROT60_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT61` reader - Protects Sector 61 from program or erase"]
pub type PROT61_R = crate::BitReader<bool>;
#[doc = "Field `PROT61` writer - Protects Sector 61 from program or erase"]
pub type PROT61_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT62` reader - Protects Sector 62 from program or erase"]
pub type PROT62_R = crate::BitReader<bool>;
#[doc = "Field `PROT62` writer - Protects Sector 62 from program or erase"]
pub type PROT62_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
#[doc = "Field `PROT63` reader - Protects Sector 63 from program or erase"]
pub type PROT63_R = crate::BitReader<bool>;
#[doc = "Field `PROT63` writer - Protects Sector 63 from program or erase"]
pub type PROT63_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Protects Sector 32 from program or erase"]
    #[inline(always)]
    pub fn prot32(&self) -> PROT32_R {
        PROT32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protects Sector 33 from program or erase"]
    #[inline(always)]
    pub fn prot33(&self) -> PROT33_R {
        PROT33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protects Sector 34 from program or erase"]
    #[inline(always)]
    pub fn prot34(&self) -> PROT34_R {
        PROT34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protects Sector 35 from program or erase"]
    #[inline(always)]
    pub fn prot35(&self) -> PROT35_R {
        PROT35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protects Sector 36 from program or erase"]
    #[inline(always)]
    pub fn prot36(&self) -> PROT36_R {
        PROT36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Protects Sector 37 from program or erase"]
    #[inline(always)]
    pub fn prot37(&self) -> PROT37_R {
        PROT37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protects Sector 38 from program or erase"]
    #[inline(always)]
    pub fn prot38(&self) -> PROT38_R {
        PROT38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Protects Sector 39 from program or erase"]
    #[inline(always)]
    pub fn prot39(&self) -> PROT39_R {
        PROT39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Protects Sector 40 from program or erase"]
    #[inline(always)]
    pub fn prot40(&self) -> PROT40_R {
        PROT40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protects Sector 41 from program or erase"]
    #[inline(always)]
    pub fn prot41(&self) -> PROT41_R {
        PROT41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protects Sector 42 from program or erase"]
    #[inline(always)]
    pub fn prot42(&self) -> PROT42_R {
        PROT42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protects Sector 43 from program or erase"]
    #[inline(always)]
    pub fn prot43(&self) -> PROT43_R {
        PROT43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protects Sector 44 from program or erase"]
    #[inline(always)]
    pub fn prot44(&self) -> PROT44_R {
        PROT44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Protects Sector 45 from program or erase"]
    #[inline(always)]
    pub fn prot45(&self) -> PROT45_R {
        PROT45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protects Sector 46 from program or erase"]
    #[inline(always)]
    pub fn prot46(&self) -> PROT46_R {
        PROT46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Protects Sector 47 from program or erase"]
    #[inline(always)]
    pub fn prot47(&self) -> PROT47_R {
        PROT47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Protects Sector 48 from program or erase"]
    #[inline(always)]
    pub fn prot48(&self) -> PROT48_R {
        PROT48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protects Sector 49 from program or erase"]
    #[inline(always)]
    pub fn prot49(&self) -> PROT49_R {
        PROT49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protects Sector 50 from program or erase"]
    #[inline(always)]
    pub fn prot50(&self) -> PROT50_R {
        PROT50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protects Sector 51 from program or erase"]
    #[inline(always)]
    pub fn prot51(&self) -> PROT51_R {
        PROT51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protects Sector 52 from program or erase"]
    #[inline(always)]
    pub fn prot52(&self) -> PROT52_R {
        PROT52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protects Sector 53 from program or erase"]
    #[inline(always)]
    pub fn prot53(&self) -> PROT53_R {
        PROT53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protects Sector 54 from program or erase"]
    #[inline(always)]
    pub fn prot54(&self) -> PROT54_R {
        PROT54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Protects Sector 55 from program or erase"]
    #[inline(always)]
    pub fn prot55(&self) -> PROT55_R {
        PROT55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Protects Sector 56 from program or erase"]
    #[inline(always)]
    pub fn prot56(&self) -> PROT56_R {
        PROT56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protects Sector 57 from program or erase"]
    #[inline(always)]
    pub fn prot57(&self) -> PROT57_R {
        PROT57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protects Sector 58 from program or erase"]
    #[inline(always)]
    pub fn prot58(&self) -> PROT58_R {
        PROT58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protects Sector 59 from program or erase"]
    #[inline(always)]
    pub fn prot59(&self) -> PROT59_R {
        PROT59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protects Sector 60 from program or erase"]
    #[inline(always)]
    pub fn prot60(&self) -> PROT60_R {
        PROT60_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Protects Sector 61 from program or erase"]
    #[inline(always)]
    pub fn prot61(&self) -> PROT61_R {
        PROT61_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Protects Sector 62 from program or erase"]
    #[inline(always)]
    pub fn prot62(&self) -> PROT62_R {
        PROT62_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Protects Sector 63 from program or erase"]
    #[inline(always)]
    pub fn prot63(&self) -> PROT63_R {
        PROT63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protects Sector 32 from program or erase"]
    #[inline(always)]
    pub fn prot32(&mut self) -> PROT32_W<0> {
        PROT32_W::new(self)
    }
    #[doc = "Bit 1 - Protects Sector 33 from program or erase"]
    #[inline(always)]
    pub fn prot33(&mut self) -> PROT33_W<1> {
        PROT33_W::new(self)
    }
    #[doc = "Bit 2 - Protects Sector 34 from program or erase"]
    #[inline(always)]
    pub fn prot34(&mut self) -> PROT34_W<2> {
        PROT34_W::new(self)
    }
    #[doc = "Bit 3 - Protects Sector 35 from program or erase"]
    #[inline(always)]
    pub fn prot35(&mut self) -> PROT35_W<3> {
        PROT35_W::new(self)
    }
    #[doc = "Bit 4 - Protects Sector 36 from program or erase"]
    #[inline(always)]
    pub fn prot36(&mut self) -> PROT36_W<4> {
        PROT36_W::new(self)
    }
    #[doc = "Bit 5 - Protects Sector 37 from program or erase"]
    #[inline(always)]
    pub fn prot37(&mut self) -> PROT37_W<5> {
        PROT37_W::new(self)
    }
    #[doc = "Bit 6 - Protects Sector 38 from program or erase"]
    #[inline(always)]
    pub fn prot38(&mut self) -> PROT38_W<6> {
        PROT38_W::new(self)
    }
    #[doc = "Bit 7 - Protects Sector 39 from program or erase"]
    #[inline(always)]
    pub fn prot39(&mut self) -> PROT39_W<7> {
        PROT39_W::new(self)
    }
    #[doc = "Bit 8 - Protects Sector 40 from program or erase"]
    #[inline(always)]
    pub fn prot40(&mut self) -> PROT40_W<8> {
        PROT40_W::new(self)
    }
    #[doc = "Bit 9 - Protects Sector 41 from program or erase"]
    #[inline(always)]
    pub fn prot41(&mut self) -> PROT41_W<9> {
        PROT41_W::new(self)
    }
    #[doc = "Bit 10 - Protects Sector 42 from program or erase"]
    #[inline(always)]
    pub fn prot42(&mut self) -> PROT42_W<10> {
        PROT42_W::new(self)
    }
    #[doc = "Bit 11 - Protects Sector 43 from program or erase"]
    #[inline(always)]
    pub fn prot43(&mut self) -> PROT43_W<11> {
        PROT43_W::new(self)
    }
    #[doc = "Bit 12 - Protects Sector 44 from program or erase"]
    #[inline(always)]
    pub fn prot44(&mut self) -> PROT44_W<12> {
        PROT44_W::new(self)
    }
    #[doc = "Bit 13 - Protects Sector 45 from program or erase"]
    #[inline(always)]
    pub fn prot45(&mut self) -> PROT45_W<13> {
        PROT45_W::new(self)
    }
    #[doc = "Bit 14 - Protects Sector 46 from program or erase"]
    #[inline(always)]
    pub fn prot46(&mut self) -> PROT46_W<14> {
        PROT46_W::new(self)
    }
    #[doc = "Bit 15 - Protects Sector 47 from program or erase"]
    #[inline(always)]
    pub fn prot47(&mut self) -> PROT47_W<15> {
        PROT47_W::new(self)
    }
    #[doc = "Bit 16 - Protects Sector 48 from program or erase"]
    #[inline(always)]
    pub fn prot48(&mut self) -> PROT48_W<16> {
        PROT48_W::new(self)
    }
    #[doc = "Bit 17 - Protects Sector 49 from program or erase"]
    #[inline(always)]
    pub fn prot49(&mut self) -> PROT49_W<17> {
        PROT49_W::new(self)
    }
    #[doc = "Bit 18 - Protects Sector 50 from program or erase"]
    #[inline(always)]
    pub fn prot50(&mut self) -> PROT50_W<18> {
        PROT50_W::new(self)
    }
    #[doc = "Bit 19 - Protects Sector 51 from program or erase"]
    #[inline(always)]
    pub fn prot51(&mut self) -> PROT51_W<19> {
        PROT51_W::new(self)
    }
    #[doc = "Bit 20 - Protects Sector 52 from program or erase"]
    #[inline(always)]
    pub fn prot52(&mut self) -> PROT52_W<20> {
        PROT52_W::new(self)
    }
    #[doc = "Bit 21 - Protects Sector 53 from program or erase"]
    #[inline(always)]
    pub fn prot53(&mut self) -> PROT53_W<21> {
        PROT53_W::new(self)
    }
    #[doc = "Bit 22 - Protects Sector 54 from program or erase"]
    #[inline(always)]
    pub fn prot54(&mut self) -> PROT54_W<22> {
        PROT54_W::new(self)
    }
    #[doc = "Bit 23 - Protects Sector 55 from program or erase"]
    #[inline(always)]
    pub fn prot55(&mut self) -> PROT55_W<23> {
        PROT55_W::new(self)
    }
    #[doc = "Bit 24 - Protects Sector 56 from program or erase"]
    #[inline(always)]
    pub fn prot56(&mut self) -> PROT56_W<24> {
        PROT56_W::new(self)
    }
    #[doc = "Bit 25 - Protects Sector 57 from program or erase"]
    #[inline(always)]
    pub fn prot57(&mut self) -> PROT57_W<25> {
        PROT57_W::new(self)
    }
    #[doc = "Bit 26 - Protects Sector 58 from program or erase"]
    #[inline(always)]
    pub fn prot58(&mut self) -> PROT58_W<26> {
        PROT58_W::new(self)
    }
    #[doc = "Bit 27 - Protects Sector 59 from program or erase"]
    #[inline(always)]
    pub fn prot59(&mut self) -> PROT59_W<27> {
        PROT59_W::new(self)
    }
    #[doc = "Bit 28 - Protects Sector 60 from program or erase"]
    #[inline(always)]
    pub fn prot60(&mut self) -> PROT60_W<28> {
        PROT60_W::new(self)
    }
    #[doc = "Bit 29 - Protects Sector 61 from program or erase"]
    #[inline(always)]
    pub fn prot61(&mut self) -> PROT61_W<29> {
        PROT61_W::new(self)
    }
    #[doc = "Bit 30 - Protects Sector 62 from program or erase"]
    #[inline(always)]
    pub fn prot62(&mut self) -> PROT62_W<30> {
        PROT62_W::new(self)
    }
    #[doc = "Bit 31 - Protects Sector 63 from program or erase"]
    #[inline(always)]
    pub fn prot63(&mut self) -> PROT63_W<31> {
        PROT63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Memory Bank1 Write/Erase Protection Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank1_main_weprot1](index.html) module"]
pub struct FLCTL_BANK1_MAIN_WEPROT1_SPEC;
impl crate::RegisterSpec for FLCTL_BANK1_MAIN_WEPROT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bank1_main_weprot1::R](R) reader structure"]
impl crate::Readable for FLCTL_BANK1_MAIN_WEPROT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bank1_main_weprot1::W](W) writer structure"]
impl crate::Writable for FLCTL_BANK1_MAIN_WEPROT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BANK1_MAIN_WEPROT1 to value 0xffff_ffff"]
impl crate::Resettable for FLCTL_BANK1_MAIN_WEPROT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
