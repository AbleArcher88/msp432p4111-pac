#[doc = "Register `FLCTL_BANK0_MAIN_WEPROT2` reader"]
pub struct R(crate::R<FLCTL_BANK0_MAIN_WEPROT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BANK0_MAIN_WEPROT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BANK0_MAIN_WEPROT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BANK0_MAIN_WEPROT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BANK0_MAIN_WEPROT2` writer"]
pub struct W(crate::W<FLCTL_BANK0_MAIN_WEPROT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BANK0_MAIN_WEPROT2_SPEC>;
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
impl From<crate::W<FLCTL_BANK0_MAIN_WEPROT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BANK0_MAIN_WEPROT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT64` reader - Protects Sector 64 from program or erase"]
pub type PROT64_R = crate::BitReader<bool>;
#[doc = "Field `PROT64` writer - Protects Sector 64 from program or erase"]
pub type PROT64_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT65` reader - Protects Sector 65 from program or erase"]
pub type PROT65_R = crate::BitReader<bool>;
#[doc = "Field `PROT65` writer - Protects Sector 65 from program or erase"]
pub type PROT65_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT66` reader - Protects Sector 66 from program or erase"]
pub type PROT66_R = crate::BitReader<bool>;
#[doc = "Field `PROT66` writer - Protects Sector 66 from program or erase"]
pub type PROT66_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT67` reader - Protects Sector 67 from program or erase"]
pub type PROT67_R = crate::BitReader<bool>;
#[doc = "Field `PROT67` writer - Protects Sector 67 from program or erase"]
pub type PROT67_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT68` reader - Protects Sector 68 from program or erase"]
pub type PROT68_R = crate::BitReader<bool>;
#[doc = "Field `PROT68` writer - Protects Sector 68 from program or erase"]
pub type PROT68_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT69` reader - Protects Sector 69 from program or erase"]
pub type PROT69_R = crate::BitReader<bool>;
#[doc = "Field `PROT69` writer - Protects Sector 69 from program or erase"]
pub type PROT69_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT70` reader - Protects Sector 70 from program or erase"]
pub type PROT70_R = crate::BitReader<bool>;
#[doc = "Field `PROT70` writer - Protects Sector 70 from program or erase"]
pub type PROT70_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT71` reader - Protects Sector 71 from program or erase"]
pub type PROT71_R = crate::BitReader<bool>;
#[doc = "Field `PROT71` writer - Protects Sector 71 from program or erase"]
pub type PROT71_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT72` reader - Protects Sector 72 from program or erase"]
pub type PROT72_R = crate::BitReader<bool>;
#[doc = "Field `PROT72` writer - Protects Sector 72 from program or erase"]
pub type PROT72_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT73` reader - Protects Sector 73 from program or erase"]
pub type PROT73_R = crate::BitReader<bool>;
#[doc = "Field `PROT73` writer - Protects Sector 73 from program or erase"]
pub type PROT73_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT74` reader - Protects Sector 74 from program or erase"]
pub type PROT74_R = crate::BitReader<bool>;
#[doc = "Field `PROT74` writer - Protects Sector 74 from program or erase"]
pub type PROT74_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT75` reader - Protects Sector 75 from program or erase"]
pub type PROT75_R = crate::BitReader<bool>;
#[doc = "Field `PROT75` writer - Protects Sector 75 from program or erase"]
pub type PROT75_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT76` reader - Protects Sector 76 from program or erase"]
pub type PROT76_R = crate::BitReader<bool>;
#[doc = "Field `PROT76` writer - Protects Sector 76 from program or erase"]
pub type PROT76_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT77` reader - Protects Sector 77 from program or erase"]
pub type PROT77_R = crate::BitReader<bool>;
#[doc = "Field `PROT77` writer - Protects Sector 77 from program or erase"]
pub type PROT77_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT78` reader - Protects Sector 78 from program or erase"]
pub type PROT78_R = crate::BitReader<bool>;
#[doc = "Field `PROT78` writer - Protects Sector 78 from program or erase"]
pub type PROT78_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT79` reader - Protects Sector 79 from program or erase"]
pub type PROT79_R = crate::BitReader<bool>;
#[doc = "Field `PROT79` writer - Protects Sector 79 from program or erase"]
pub type PROT79_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT80` reader - Protects Sector 80 from program or erase"]
pub type PROT80_R = crate::BitReader<bool>;
#[doc = "Field `PROT80` writer - Protects Sector 80 from program or erase"]
pub type PROT80_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT81` reader - Protects Sector 81 from program or erase"]
pub type PROT81_R = crate::BitReader<bool>;
#[doc = "Field `PROT81` writer - Protects Sector 81 from program or erase"]
pub type PROT81_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT82` reader - Protects Sector 82 from program or erase"]
pub type PROT82_R = crate::BitReader<bool>;
#[doc = "Field `PROT82` writer - Protects Sector 82 from program or erase"]
pub type PROT82_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT83` reader - Protects Sector 83 from program or erase"]
pub type PROT83_R = crate::BitReader<bool>;
#[doc = "Field `PROT83` writer - Protects Sector 83 from program or erase"]
pub type PROT83_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT84` reader - Protects Sector 84 from program or erase"]
pub type PROT84_R = crate::BitReader<bool>;
#[doc = "Field `PROT84` writer - Protects Sector 84 from program or erase"]
pub type PROT84_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT85` reader - Protects Sector 85 from program or erase"]
pub type PROT85_R = crate::BitReader<bool>;
#[doc = "Field `PROT85` writer - Protects Sector 85 from program or erase"]
pub type PROT85_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT86` reader - Protects Sector 86 from program or erase"]
pub type PROT86_R = crate::BitReader<bool>;
#[doc = "Field `PROT86` writer - Protects Sector 86 from program or erase"]
pub type PROT86_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT87` reader - Protects Sector 87 from program or erase"]
pub type PROT87_R = crate::BitReader<bool>;
#[doc = "Field `PROT87` writer - Protects Sector 87 from program or erase"]
pub type PROT87_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT88` reader - Protects Sector 88 from program or erase"]
pub type PROT88_R = crate::BitReader<bool>;
#[doc = "Field `PROT88` writer - Protects Sector 88 from program or erase"]
pub type PROT88_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT89` reader - Protects Sector 89 from program or erase"]
pub type PROT89_R = crate::BitReader<bool>;
#[doc = "Field `PROT89` writer - Protects Sector 89 from program or erase"]
pub type PROT89_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT90` reader - Protects Sector 90 from program or erase"]
pub type PROT90_R = crate::BitReader<bool>;
#[doc = "Field `PROT90` writer - Protects Sector 90 from program or erase"]
pub type PROT90_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT91` reader - Protects Sector 91 from program or erase"]
pub type PROT91_R = crate::BitReader<bool>;
#[doc = "Field `PROT91` writer - Protects Sector 91 from program or erase"]
pub type PROT91_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT92` reader - Protects Sector 92 from program or erase"]
pub type PROT92_R = crate::BitReader<bool>;
#[doc = "Field `PROT92` writer - Protects Sector 92 from program or erase"]
pub type PROT92_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT93` reader - Protects Sector 93 from program or erase"]
pub type PROT93_R = crate::BitReader<bool>;
#[doc = "Field `PROT93` writer - Protects Sector 93 from program or erase"]
pub type PROT93_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT94` reader - Protects Sector 94 from program or erase"]
pub type PROT94_R = crate::BitReader<bool>;
#[doc = "Field `PROT94` writer - Protects Sector 94 from program or erase"]
pub type PROT94_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
#[doc = "Field `PROT95` reader - Protects Sector 95 from program or erase"]
pub type PROT95_R = crate::BitReader<bool>;
#[doc = "Field `PROT95` writer - Protects Sector 95 from program or erase"]
pub type PROT95_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Protects Sector 64 from program or erase"]
    #[inline(always)]
    pub fn prot64(&self) -> PROT64_R {
        PROT64_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protects Sector 65 from program or erase"]
    #[inline(always)]
    pub fn prot65(&self) -> PROT65_R {
        PROT65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protects Sector 66 from program or erase"]
    #[inline(always)]
    pub fn prot66(&self) -> PROT66_R {
        PROT66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protects Sector 67 from program or erase"]
    #[inline(always)]
    pub fn prot67(&self) -> PROT67_R {
        PROT67_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protects Sector 68 from program or erase"]
    #[inline(always)]
    pub fn prot68(&self) -> PROT68_R {
        PROT68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Protects Sector 69 from program or erase"]
    #[inline(always)]
    pub fn prot69(&self) -> PROT69_R {
        PROT69_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protects Sector 70 from program or erase"]
    #[inline(always)]
    pub fn prot70(&self) -> PROT70_R {
        PROT70_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Protects Sector 71 from program or erase"]
    #[inline(always)]
    pub fn prot71(&self) -> PROT71_R {
        PROT71_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Protects Sector 72 from program or erase"]
    #[inline(always)]
    pub fn prot72(&self) -> PROT72_R {
        PROT72_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protects Sector 73 from program or erase"]
    #[inline(always)]
    pub fn prot73(&self) -> PROT73_R {
        PROT73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protects Sector 74 from program or erase"]
    #[inline(always)]
    pub fn prot74(&self) -> PROT74_R {
        PROT74_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protects Sector 75 from program or erase"]
    #[inline(always)]
    pub fn prot75(&self) -> PROT75_R {
        PROT75_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protects Sector 76 from program or erase"]
    #[inline(always)]
    pub fn prot76(&self) -> PROT76_R {
        PROT76_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Protects Sector 77 from program or erase"]
    #[inline(always)]
    pub fn prot77(&self) -> PROT77_R {
        PROT77_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protects Sector 78 from program or erase"]
    #[inline(always)]
    pub fn prot78(&self) -> PROT78_R {
        PROT78_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Protects Sector 79 from program or erase"]
    #[inline(always)]
    pub fn prot79(&self) -> PROT79_R {
        PROT79_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Protects Sector 80 from program or erase"]
    #[inline(always)]
    pub fn prot80(&self) -> PROT80_R {
        PROT80_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protects Sector 81 from program or erase"]
    #[inline(always)]
    pub fn prot81(&self) -> PROT81_R {
        PROT81_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protects Sector 82 from program or erase"]
    #[inline(always)]
    pub fn prot82(&self) -> PROT82_R {
        PROT82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protects Sector 83 from program or erase"]
    #[inline(always)]
    pub fn prot83(&self) -> PROT83_R {
        PROT83_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protects Sector 84 from program or erase"]
    #[inline(always)]
    pub fn prot84(&self) -> PROT84_R {
        PROT84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protects Sector 85 from program or erase"]
    #[inline(always)]
    pub fn prot85(&self) -> PROT85_R {
        PROT85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protects Sector 86 from program or erase"]
    #[inline(always)]
    pub fn prot86(&self) -> PROT86_R {
        PROT86_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Protects Sector 87 from program or erase"]
    #[inline(always)]
    pub fn prot87(&self) -> PROT87_R {
        PROT87_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Protects Sector 88 from program or erase"]
    #[inline(always)]
    pub fn prot88(&self) -> PROT88_R {
        PROT88_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protects Sector 89 from program or erase"]
    #[inline(always)]
    pub fn prot89(&self) -> PROT89_R {
        PROT89_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protects Sector 90 from program or erase"]
    #[inline(always)]
    pub fn prot90(&self) -> PROT90_R {
        PROT90_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protects Sector 91 from program or erase"]
    #[inline(always)]
    pub fn prot91(&self) -> PROT91_R {
        PROT91_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protects Sector 92 from program or erase"]
    #[inline(always)]
    pub fn prot92(&self) -> PROT92_R {
        PROT92_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Protects Sector 93 from program or erase"]
    #[inline(always)]
    pub fn prot93(&self) -> PROT93_R {
        PROT93_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Protects Sector 94 from program or erase"]
    #[inline(always)]
    pub fn prot94(&self) -> PROT94_R {
        PROT94_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Protects Sector 95 from program or erase"]
    #[inline(always)]
    pub fn prot95(&self) -> PROT95_R {
        PROT95_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protects Sector 64 from program or erase"]
    #[inline(always)]
    pub fn prot64(&mut self) -> PROT64_W<0> {
        PROT64_W::new(self)
    }
    #[doc = "Bit 1 - Protects Sector 65 from program or erase"]
    #[inline(always)]
    pub fn prot65(&mut self) -> PROT65_W<1> {
        PROT65_W::new(self)
    }
    #[doc = "Bit 2 - Protects Sector 66 from program or erase"]
    #[inline(always)]
    pub fn prot66(&mut self) -> PROT66_W<2> {
        PROT66_W::new(self)
    }
    #[doc = "Bit 3 - Protects Sector 67 from program or erase"]
    #[inline(always)]
    pub fn prot67(&mut self) -> PROT67_W<3> {
        PROT67_W::new(self)
    }
    #[doc = "Bit 4 - Protects Sector 68 from program or erase"]
    #[inline(always)]
    pub fn prot68(&mut self) -> PROT68_W<4> {
        PROT68_W::new(self)
    }
    #[doc = "Bit 5 - Protects Sector 69 from program or erase"]
    #[inline(always)]
    pub fn prot69(&mut self) -> PROT69_W<5> {
        PROT69_W::new(self)
    }
    #[doc = "Bit 6 - Protects Sector 70 from program or erase"]
    #[inline(always)]
    pub fn prot70(&mut self) -> PROT70_W<6> {
        PROT70_W::new(self)
    }
    #[doc = "Bit 7 - Protects Sector 71 from program or erase"]
    #[inline(always)]
    pub fn prot71(&mut self) -> PROT71_W<7> {
        PROT71_W::new(self)
    }
    #[doc = "Bit 8 - Protects Sector 72 from program or erase"]
    #[inline(always)]
    pub fn prot72(&mut self) -> PROT72_W<8> {
        PROT72_W::new(self)
    }
    #[doc = "Bit 9 - Protects Sector 73 from program or erase"]
    #[inline(always)]
    pub fn prot73(&mut self) -> PROT73_W<9> {
        PROT73_W::new(self)
    }
    #[doc = "Bit 10 - Protects Sector 74 from program or erase"]
    #[inline(always)]
    pub fn prot74(&mut self) -> PROT74_W<10> {
        PROT74_W::new(self)
    }
    #[doc = "Bit 11 - Protects Sector 75 from program or erase"]
    #[inline(always)]
    pub fn prot75(&mut self) -> PROT75_W<11> {
        PROT75_W::new(self)
    }
    #[doc = "Bit 12 - Protects Sector 76 from program or erase"]
    #[inline(always)]
    pub fn prot76(&mut self) -> PROT76_W<12> {
        PROT76_W::new(self)
    }
    #[doc = "Bit 13 - Protects Sector 77 from program or erase"]
    #[inline(always)]
    pub fn prot77(&mut self) -> PROT77_W<13> {
        PROT77_W::new(self)
    }
    #[doc = "Bit 14 - Protects Sector 78 from program or erase"]
    #[inline(always)]
    pub fn prot78(&mut self) -> PROT78_W<14> {
        PROT78_W::new(self)
    }
    #[doc = "Bit 15 - Protects Sector 79 from program or erase"]
    #[inline(always)]
    pub fn prot79(&mut self) -> PROT79_W<15> {
        PROT79_W::new(self)
    }
    #[doc = "Bit 16 - Protects Sector 80 from program or erase"]
    #[inline(always)]
    pub fn prot80(&mut self) -> PROT80_W<16> {
        PROT80_W::new(self)
    }
    #[doc = "Bit 17 - Protects Sector 81 from program or erase"]
    #[inline(always)]
    pub fn prot81(&mut self) -> PROT81_W<17> {
        PROT81_W::new(self)
    }
    #[doc = "Bit 18 - Protects Sector 82 from program or erase"]
    #[inline(always)]
    pub fn prot82(&mut self) -> PROT82_W<18> {
        PROT82_W::new(self)
    }
    #[doc = "Bit 19 - Protects Sector 83 from program or erase"]
    #[inline(always)]
    pub fn prot83(&mut self) -> PROT83_W<19> {
        PROT83_W::new(self)
    }
    #[doc = "Bit 20 - Protects Sector 84 from program or erase"]
    #[inline(always)]
    pub fn prot84(&mut self) -> PROT84_W<20> {
        PROT84_W::new(self)
    }
    #[doc = "Bit 21 - Protects Sector 85 from program or erase"]
    #[inline(always)]
    pub fn prot85(&mut self) -> PROT85_W<21> {
        PROT85_W::new(self)
    }
    #[doc = "Bit 22 - Protects Sector 86 from program or erase"]
    #[inline(always)]
    pub fn prot86(&mut self) -> PROT86_W<22> {
        PROT86_W::new(self)
    }
    #[doc = "Bit 23 - Protects Sector 87 from program or erase"]
    #[inline(always)]
    pub fn prot87(&mut self) -> PROT87_W<23> {
        PROT87_W::new(self)
    }
    #[doc = "Bit 24 - Protects Sector 88 from program or erase"]
    #[inline(always)]
    pub fn prot88(&mut self) -> PROT88_W<24> {
        PROT88_W::new(self)
    }
    #[doc = "Bit 25 - Protects Sector 89 from program or erase"]
    #[inline(always)]
    pub fn prot89(&mut self) -> PROT89_W<25> {
        PROT89_W::new(self)
    }
    #[doc = "Bit 26 - Protects Sector 90 from program or erase"]
    #[inline(always)]
    pub fn prot90(&mut self) -> PROT90_W<26> {
        PROT90_W::new(self)
    }
    #[doc = "Bit 27 - Protects Sector 91 from program or erase"]
    #[inline(always)]
    pub fn prot91(&mut self) -> PROT91_W<27> {
        PROT91_W::new(self)
    }
    #[doc = "Bit 28 - Protects Sector 92 from program or erase"]
    #[inline(always)]
    pub fn prot92(&mut self) -> PROT92_W<28> {
        PROT92_W::new(self)
    }
    #[doc = "Bit 29 - Protects Sector 93 from program or erase"]
    #[inline(always)]
    pub fn prot93(&mut self) -> PROT93_W<29> {
        PROT93_W::new(self)
    }
    #[doc = "Bit 30 - Protects Sector 94 from program or erase"]
    #[inline(always)]
    pub fn prot94(&mut self) -> PROT94_W<30> {
        PROT94_W::new(self)
    }
    #[doc = "Bit 31 - Protects Sector 95 from program or erase"]
    #[inline(always)]
    pub fn prot95(&mut self) -> PROT95_W<31> {
        PROT95_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Memory Bank0 Write/Erase Protection Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank0_main_weprot2](index.html) module"]
pub struct FLCTL_BANK0_MAIN_WEPROT2_SPEC;
impl crate::RegisterSpec for FLCTL_BANK0_MAIN_WEPROT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bank0_main_weprot2::R](R) reader structure"]
impl crate::Readable for FLCTL_BANK0_MAIN_WEPROT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bank0_main_weprot2::W](W) writer structure"]
impl crate::Writable for FLCTL_BANK0_MAIN_WEPROT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BANK0_MAIN_WEPROT2 to value 0xffff_ffff"]
impl crate::Resettable for FLCTL_BANK0_MAIN_WEPROT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
