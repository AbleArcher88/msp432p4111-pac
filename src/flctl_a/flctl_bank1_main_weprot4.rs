#[doc = "Register `FLCTL_BANK1_MAIN_WEPROT4` reader"]
pub struct R(crate::R<FLCTL_BANK1_MAIN_WEPROT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BANK1_MAIN_WEPROT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BANK1_MAIN_WEPROT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BANK1_MAIN_WEPROT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BANK1_MAIN_WEPROT4` writer"]
pub struct W(crate::W<FLCTL_BANK1_MAIN_WEPROT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BANK1_MAIN_WEPROT4_SPEC>;
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
impl From<crate::W<FLCTL_BANK1_MAIN_WEPROT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BANK1_MAIN_WEPROT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT128` reader - Protects Sector 128 from program or erase"]
pub type PROT128_R = crate::BitReader<bool>;
#[doc = "Field `PROT128` writer - Protects Sector 128 from program or erase"]
pub type PROT128_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT129` reader - Protects Sector 129 from program or erase"]
pub type PROT129_R = crate::BitReader<bool>;
#[doc = "Field `PROT129` writer - Protects Sector 129 from program or erase"]
pub type PROT129_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT130` reader - Protects Sector 130 from program or erase"]
pub type PROT130_R = crate::BitReader<bool>;
#[doc = "Field `PROT130` writer - Protects Sector 130 from program or erase"]
pub type PROT130_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT131` reader - Protects Sector 131 from program or erase"]
pub type PROT131_R = crate::BitReader<bool>;
#[doc = "Field `PROT131` writer - Protects Sector 131 from program or erase"]
pub type PROT131_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT132` reader - Protects Sector 132 from program or erase"]
pub type PROT132_R = crate::BitReader<bool>;
#[doc = "Field `PROT132` writer - Protects Sector 132 from program or erase"]
pub type PROT132_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT133` reader - Protects Sector 133 from program or erase"]
pub type PROT133_R = crate::BitReader<bool>;
#[doc = "Field `PROT133` writer - Protects Sector 133 from program or erase"]
pub type PROT133_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT134` reader - Protects Sector 134 from program or erase"]
pub type PROT134_R = crate::BitReader<bool>;
#[doc = "Field `PROT134` writer - Protects Sector 134 from program or erase"]
pub type PROT134_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT135` reader - Protects Sector 135 from program or erase"]
pub type PROT135_R = crate::BitReader<bool>;
#[doc = "Field `PROT135` writer - Protects Sector 135 from program or erase"]
pub type PROT135_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT136` reader - Protects Sector 136 from program or erase"]
pub type PROT136_R = crate::BitReader<bool>;
#[doc = "Field `PROT136` writer - Protects Sector 136 from program or erase"]
pub type PROT136_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT137` reader - Protects Sector 137 from program or erase"]
pub type PROT137_R = crate::BitReader<bool>;
#[doc = "Field `PROT137` writer - Protects Sector 137 from program or erase"]
pub type PROT137_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT138` reader - Protects Sector 138 from program or erase"]
pub type PROT138_R = crate::BitReader<bool>;
#[doc = "Field `PROT138` writer - Protects Sector 138 from program or erase"]
pub type PROT138_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT139` reader - Protects Sector 139 from program or erase"]
pub type PROT139_R = crate::BitReader<bool>;
#[doc = "Field `PROT139` writer - Protects Sector 139 from program or erase"]
pub type PROT139_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT140` reader - Protects Sector 140 from program or erase"]
pub type PROT140_R = crate::BitReader<bool>;
#[doc = "Field `PROT140` writer - Protects Sector 140 from program or erase"]
pub type PROT140_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT141` reader - Protects Sector 141 from program or erase"]
pub type PROT141_R = crate::BitReader<bool>;
#[doc = "Field `PROT141` writer - Protects Sector 141 from program or erase"]
pub type PROT141_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT142` reader - Protects Sector 142 from program or erase"]
pub type PROT142_R = crate::BitReader<bool>;
#[doc = "Field `PROT142` writer - Protects Sector 142 from program or erase"]
pub type PROT142_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT143` reader - Protects Sector 143 from program or erase"]
pub type PROT143_R = crate::BitReader<bool>;
#[doc = "Field `PROT143` writer - Protects Sector 143 from program or erase"]
pub type PROT143_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT144` reader - Protects Sector 144 from program or erase"]
pub type PROT144_R = crate::BitReader<bool>;
#[doc = "Field `PROT144` writer - Protects Sector 144 from program or erase"]
pub type PROT144_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT145` reader - Protects Sector 145 from program or erase"]
pub type PROT145_R = crate::BitReader<bool>;
#[doc = "Field `PROT145` writer - Protects Sector 145 from program or erase"]
pub type PROT145_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT146` reader - Protects Sector 146 from program or erase"]
pub type PROT146_R = crate::BitReader<bool>;
#[doc = "Field `PROT146` writer - Protects Sector 146 from program or erase"]
pub type PROT146_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT147` reader - Protects Sector 147 from program or erase"]
pub type PROT147_R = crate::BitReader<bool>;
#[doc = "Field `PROT147` writer - Protects Sector 147 from program or erase"]
pub type PROT147_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT148` reader - Protects Sector 148 from program or erase"]
pub type PROT148_R = crate::BitReader<bool>;
#[doc = "Field `PROT148` writer - Protects Sector 148 from program or erase"]
pub type PROT148_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT149` reader - Protects Sector 149 from program or erase"]
pub type PROT149_R = crate::BitReader<bool>;
#[doc = "Field `PROT149` writer - Protects Sector 149 from program or erase"]
pub type PROT149_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT150` reader - Protects Sector 150 from program or erase"]
pub type PROT150_R = crate::BitReader<bool>;
#[doc = "Field `PROT150` writer - Protects Sector 150 from program or erase"]
pub type PROT150_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT151` reader - Protects Sector 151 from program or erase"]
pub type PROT151_R = crate::BitReader<bool>;
#[doc = "Field `PROT151` writer - Protects Sector 151 from program or erase"]
pub type PROT151_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT152` reader - Protects Sector 152 from program or erase"]
pub type PROT152_R = crate::BitReader<bool>;
#[doc = "Field `PROT152` writer - Protects Sector 152 from program or erase"]
pub type PROT152_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT153` reader - Protects Sector 153 from program or erase"]
pub type PROT153_R = crate::BitReader<bool>;
#[doc = "Field `PROT153` writer - Protects Sector 153 from program or erase"]
pub type PROT153_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT154` reader - Protects Sector 154 from program or erase"]
pub type PROT154_R = crate::BitReader<bool>;
#[doc = "Field `PROT154` writer - Protects Sector 154 from program or erase"]
pub type PROT154_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT155` reader - Protects Sector 155 from program or erase"]
pub type PROT155_R = crate::BitReader<bool>;
#[doc = "Field `PROT155` writer - Protects Sector 155 from program or erase"]
pub type PROT155_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT156` reader - Protects Sector 156 from program or erase"]
pub type PROT156_R = crate::BitReader<bool>;
#[doc = "Field `PROT156` writer - Protects Sector 156 from program or erase"]
pub type PROT156_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT157` reader - Protects Sector 157 from program or erase"]
pub type PROT157_R = crate::BitReader<bool>;
#[doc = "Field `PROT157` writer - Protects Sector 157 from program or erase"]
pub type PROT157_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT158` reader - Protects Sector 158 from program or erase"]
pub type PROT158_R = crate::BitReader<bool>;
#[doc = "Field `PROT158` writer - Protects Sector 158 from program or erase"]
pub type PROT158_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
#[doc = "Field `PROT159` reader - Protects Sector 159 from program or erase"]
pub type PROT159_R = crate::BitReader<bool>;
#[doc = "Field `PROT159` writer - Protects Sector 159 from program or erase"]
pub type PROT159_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Protects Sector 128 from program or erase"]
    #[inline(always)]
    pub fn prot128(&self) -> PROT128_R {
        PROT128_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protects Sector 129 from program or erase"]
    #[inline(always)]
    pub fn prot129(&self) -> PROT129_R {
        PROT129_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protects Sector 130 from program or erase"]
    #[inline(always)]
    pub fn prot130(&self) -> PROT130_R {
        PROT130_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protects Sector 131 from program or erase"]
    #[inline(always)]
    pub fn prot131(&self) -> PROT131_R {
        PROT131_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protects Sector 132 from program or erase"]
    #[inline(always)]
    pub fn prot132(&self) -> PROT132_R {
        PROT132_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Protects Sector 133 from program or erase"]
    #[inline(always)]
    pub fn prot133(&self) -> PROT133_R {
        PROT133_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protects Sector 134 from program or erase"]
    #[inline(always)]
    pub fn prot134(&self) -> PROT134_R {
        PROT134_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Protects Sector 135 from program or erase"]
    #[inline(always)]
    pub fn prot135(&self) -> PROT135_R {
        PROT135_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Protects Sector 136 from program or erase"]
    #[inline(always)]
    pub fn prot136(&self) -> PROT136_R {
        PROT136_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protects Sector 137 from program or erase"]
    #[inline(always)]
    pub fn prot137(&self) -> PROT137_R {
        PROT137_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protects Sector 138 from program or erase"]
    #[inline(always)]
    pub fn prot138(&self) -> PROT138_R {
        PROT138_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protects Sector 139 from program or erase"]
    #[inline(always)]
    pub fn prot139(&self) -> PROT139_R {
        PROT139_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protects Sector 140 from program or erase"]
    #[inline(always)]
    pub fn prot140(&self) -> PROT140_R {
        PROT140_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Protects Sector 141 from program or erase"]
    #[inline(always)]
    pub fn prot141(&self) -> PROT141_R {
        PROT141_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protects Sector 142 from program or erase"]
    #[inline(always)]
    pub fn prot142(&self) -> PROT142_R {
        PROT142_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Protects Sector 143 from program or erase"]
    #[inline(always)]
    pub fn prot143(&self) -> PROT143_R {
        PROT143_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Protects Sector 144 from program or erase"]
    #[inline(always)]
    pub fn prot144(&self) -> PROT144_R {
        PROT144_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protects Sector 145 from program or erase"]
    #[inline(always)]
    pub fn prot145(&self) -> PROT145_R {
        PROT145_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protects Sector 146 from program or erase"]
    #[inline(always)]
    pub fn prot146(&self) -> PROT146_R {
        PROT146_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protects Sector 147 from program or erase"]
    #[inline(always)]
    pub fn prot147(&self) -> PROT147_R {
        PROT147_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protects Sector 148 from program or erase"]
    #[inline(always)]
    pub fn prot148(&self) -> PROT148_R {
        PROT148_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protects Sector 149 from program or erase"]
    #[inline(always)]
    pub fn prot149(&self) -> PROT149_R {
        PROT149_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protects Sector 150 from program or erase"]
    #[inline(always)]
    pub fn prot150(&self) -> PROT150_R {
        PROT150_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Protects Sector 151 from program or erase"]
    #[inline(always)]
    pub fn prot151(&self) -> PROT151_R {
        PROT151_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Protects Sector 152 from program or erase"]
    #[inline(always)]
    pub fn prot152(&self) -> PROT152_R {
        PROT152_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protects Sector 153 from program or erase"]
    #[inline(always)]
    pub fn prot153(&self) -> PROT153_R {
        PROT153_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protects Sector 154 from program or erase"]
    #[inline(always)]
    pub fn prot154(&self) -> PROT154_R {
        PROT154_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protects Sector 155 from program or erase"]
    #[inline(always)]
    pub fn prot155(&self) -> PROT155_R {
        PROT155_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protects Sector 156 from program or erase"]
    #[inline(always)]
    pub fn prot156(&self) -> PROT156_R {
        PROT156_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Protects Sector 157 from program or erase"]
    #[inline(always)]
    pub fn prot157(&self) -> PROT157_R {
        PROT157_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Protects Sector 158 from program or erase"]
    #[inline(always)]
    pub fn prot158(&self) -> PROT158_R {
        PROT158_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Protects Sector 159 from program or erase"]
    #[inline(always)]
    pub fn prot159(&self) -> PROT159_R {
        PROT159_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protects Sector 128 from program or erase"]
    #[inline(always)]
    pub fn prot128(&mut self) -> PROT128_W<0> {
        PROT128_W::new(self)
    }
    #[doc = "Bit 1 - Protects Sector 129 from program or erase"]
    #[inline(always)]
    pub fn prot129(&mut self) -> PROT129_W<1> {
        PROT129_W::new(self)
    }
    #[doc = "Bit 2 - Protects Sector 130 from program or erase"]
    #[inline(always)]
    pub fn prot130(&mut self) -> PROT130_W<2> {
        PROT130_W::new(self)
    }
    #[doc = "Bit 3 - Protects Sector 131 from program or erase"]
    #[inline(always)]
    pub fn prot131(&mut self) -> PROT131_W<3> {
        PROT131_W::new(self)
    }
    #[doc = "Bit 4 - Protects Sector 132 from program or erase"]
    #[inline(always)]
    pub fn prot132(&mut self) -> PROT132_W<4> {
        PROT132_W::new(self)
    }
    #[doc = "Bit 5 - Protects Sector 133 from program or erase"]
    #[inline(always)]
    pub fn prot133(&mut self) -> PROT133_W<5> {
        PROT133_W::new(self)
    }
    #[doc = "Bit 6 - Protects Sector 134 from program or erase"]
    #[inline(always)]
    pub fn prot134(&mut self) -> PROT134_W<6> {
        PROT134_W::new(self)
    }
    #[doc = "Bit 7 - Protects Sector 135 from program or erase"]
    #[inline(always)]
    pub fn prot135(&mut self) -> PROT135_W<7> {
        PROT135_W::new(self)
    }
    #[doc = "Bit 8 - Protects Sector 136 from program or erase"]
    #[inline(always)]
    pub fn prot136(&mut self) -> PROT136_W<8> {
        PROT136_W::new(self)
    }
    #[doc = "Bit 9 - Protects Sector 137 from program or erase"]
    #[inline(always)]
    pub fn prot137(&mut self) -> PROT137_W<9> {
        PROT137_W::new(self)
    }
    #[doc = "Bit 10 - Protects Sector 138 from program or erase"]
    #[inline(always)]
    pub fn prot138(&mut self) -> PROT138_W<10> {
        PROT138_W::new(self)
    }
    #[doc = "Bit 11 - Protects Sector 139 from program or erase"]
    #[inline(always)]
    pub fn prot139(&mut self) -> PROT139_W<11> {
        PROT139_W::new(self)
    }
    #[doc = "Bit 12 - Protects Sector 140 from program or erase"]
    #[inline(always)]
    pub fn prot140(&mut self) -> PROT140_W<12> {
        PROT140_W::new(self)
    }
    #[doc = "Bit 13 - Protects Sector 141 from program or erase"]
    #[inline(always)]
    pub fn prot141(&mut self) -> PROT141_W<13> {
        PROT141_W::new(self)
    }
    #[doc = "Bit 14 - Protects Sector 142 from program or erase"]
    #[inline(always)]
    pub fn prot142(&mut self) -> PROT142_W<14> {
        PROT142_W::new(self)
    }
    #[doc = "Bit 15 - Protects Sector 143 from program or erase"]
    #[inline(always)]
    pub fn prot143(&mut self) -> PROT143_W<15> {
        PROT143_W::new(self)
    }
    #[doc = "Bit 16 - Protects Sector 144 from program or erase"]
    #[inline(always)]
    pub fn prot144(&mut self) -> PROT144_W<16> {
        PROT144_W::new(self)
    }
    #[doc = "Bit 17 - Protects Sector 145 from program or erase"]
    #[inline(always)]
    pub fn prot145(&mut self) -> PROT145_W<17> {
        PROT145_W::new(self)
    }
    #[doc = "Bit 18 - Protects Sector 146 from program or erase"]
    #[inline(always)]
    pub fn prot146(&mut self) -> PROT146_W<18> {
        PROT146_W::new(self)
    }
    #[doc = "Bit 19 - Protects Sector 147 from program or erase"]
    #[inline(always)]
    pub fn prot147(&mut self) -> PROT147_W<19> {
        PROT147_W::new(self)
    }
    #[doc = "Bit 20 - Protects Sector 148 from program or erase"]
    #[inline(always)]
    pub fn prot148(&mut self) -> PROT148_W<20> {
        PROT148_W::new(self)
    }
    #[doc = "Bit 21 - Protects Sector 149 from program or erase"]
    #[inline(always)]
    pub fn prot149(&mut self) -> PROT149_W<21> {
        PROT149_W::new(self)
    }
    #[doc = "Bit 22 - Protects Sector 150 from program or erase"]
    #[inline(always)]
    pub fn prot150(&mut self) -> PROT150_W<22> {
        PROT150_W::new(self)
    }
    #[doc = "Bit 23 - Protects Sector 151 from program or erase"]
    #[inline(always)]
    pub fn prot151(&mut self) -> PROT151_W<23> {
        PROT151_W::new(self)
    }
    #[doc = "Bit 24 - Protects Sector 152 from program or erase"]
    #[inline(always)]
    pub fn prot152(&mut self) -> PROT152_W<24> {
        PROT152_W::new(self)
    }
    #[doc = "Bit 25 - Protects Sector 153 from program or erase"]
    #[inline(always)]
    pub fn prot153(&mut self) -> PROT153_W<25> {
        PROT153_W::new(self)
    }
    #[doc = "Bit 26 - Protects Sector 154 from program or erase"]
    #[inline(always)]
    pub fn prot154(&mut self) -> PROT154_W<26> {
        PROT154_W::new(self)
    }
    #[doc = "Bit 27 - Protects Sector 155 from program or erase"]
    #[inline(always)]
    pub fn prot155(&mut self) -> PROT155_W<27> {
        PROT155_W::new(self)
    }
    #[doc = "Bit 28 - Protects Sector 156 from program or erase"]
    #[inline(always)]
    pub fn prot156(&mut self) -> PROT156_W<28> {
        PROT156_W::new(self)
    }
    #[doc = "Bit 29 - Protects Sector 157 from program or erase"]
    #[inline(always)]
    pub fn prot157(&mut self) -> PROT157_W<29> {
        PROT157_W::new(self)
    }
    #[doc = "Bit 30 - Protects Sector 158 from program or erase"]
    #[inline(always)]
    pub fn prot158(&mut self) -> PROT158_W<30> {
        PROT158_W::new(self)
    }
    #[doc = "Bit 31 - Protects Sector 159 from program or erase"]
    #[inline(always)]
    pub fn prot159(&mut self) -> PROT159_W<31> {
        PROT159_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Memory Bank1 Write/Erase Protection Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank1_main_weprot4](index.html) module"]
pub struct FLCTL_BANK1_MAIN_WEPROT4_SPEC;
impl crate::RegisterSpec for FLCTL_BANK1_MAIN_WEPROT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bank1_main_weprot4::R](R) reader structure"]
impl crate::Readable for FLCTL_BANK1_MAIN_WEPROT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bank1_main_weprot4::W](W) writer structure"]
impl crate::Writable for FLCTL_BANK1_MAIN_WEPROT4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BANK1_MAIN_WEPROT4 to value 0xffff_ffff"]
impl crate::Resettable for FLCTL_BANK1_MAIN_WEPROT4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
