#[doc = "Register `FLCTL_BANK0_MAIN_WEPROT5` reader"]
pub struct R(crate::R<FLCTL_BANK0_MAIN_WEPROT5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BANK0_MAIN_WEPROT5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BANK0_MAIN_WEPROT5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BANK0_MAIN_WEPROT5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BANK0_MAIN_WEPROT5` writer"]
pub struct W(crate::W<FLCTL_BANK0_MAIN_WEPROT5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BANK0_MAIN_WEPROT5_SPEC>;
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
impl From<crate::W<FLCTL_BANK0_MAIN_WEPROT5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BANK0_MAIN_WEPROT5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT160` reader - Protects Sector 160 from program or erase"]
pub type PROT160_R = crate::BitReader<bool>;
#[doc = "Field `PROT160` writer - Protects Sector 160 from program or erase"]
pub type PROT160_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT161` reader - Protects Sector 161 from program or erase"]
pub type PROT161_R = crate::BitReader<bool>;
#[doc = "Field `PROT161` writer - Protects Sector 161 from program or erase"]
pub type PROT161_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT162` reader - Protects Sector 162 from program or erase"]
pub type PROT162_R = crate::BitReader<bool>;
#[doc = "Field `PROT162` writer - Protects Sector 162 from program or erase"]
pub type PROT162_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT163` reader - Protects Sector 163 from program or erase"]
pub type PROT163_R = crate::BitReader<bool>;
#[doc = "Field `PROT163` writer - Protects Sector 163 from program or erase"]
pub type PROT163_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT164` reader - Protects Sector 164 from program or erase"]
pub type PROT164_R = crate::BitReader<bool>;
#[doc = "Field `PROT164` writer - Protects Sector 164 from program or erase"]
pub type PROT164_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT165` reader - Protects Sector 165 from program or erase"]
pub type PROT165_R = crate::BitReader<bool>;
#[doc = "Field `PROT165` writer - Protects Sector 165 from program or erase"]
pub type PROT165_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT166` reader - Protects Sector 166 from program or erase"]
pub type PROT166_R = crate::BitReader<bool>;
#[doc = "Field `PROT166` writer - Protects Sector 166 from program or erase"]
pub type PROT166_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT167` reader - Protects Sector 167 from program or erase"]
pub type PROT167_R = crate::BitReader<bool>;
#[doc = "Field `PROT167` writer - Protects Sector 167 from program or erase"]
pub type PROT167_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT168` reader - Protects Sector 168 from program or erase"]
pub type PROT168_R = crate::BitReader<bool>;
#[doc = "Field `PROT168` writer - Protects Sector 168 from program or erase"]
pub type PROT168_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT169` reader - Protects Sector 169 from program or erase"]
pub type PROT169_R = crate::BitReader<bool>;
#[doc = "Field `PROT169` writer - Protects Sector 169 from program or erase"]
pub type PROT169_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT170` reader - Protects Sector 170 from program or erase"]
pub type PROT170_R = crate::BitReader<bool>;
#[doc = "Field `PROT170` writer - Protects Sector 170 from program or erase"]
pub type PROT170_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT171` reader - Protects Sector 171 from program or erase"]
pub type PROT171_R = crate::BitReader<bool>;
#[doc = "Field `PROT171` writer - Protects Sector 171 from program or erase"]
pub type PROT171_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT172` reader - Protects Sector 172 from program or erase"]
pub type PROT172_R = crate::BitReader<bool>;
#[doc = "Field `PROT172` writer - Protects Sector 172 from program or erase"]
pub type PROT172_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT173` reader - Protects Sector 173 from program or erase"]
pub type PROT173_R = crate::BitReader<bool>;
#[doc = "Field `PROT173` writer - Protects Sector 173 from program or erase"]
pub type PROT173_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT174` reader - Protects Sector 174 from program or erase"]
pub type PROT174_R = crate::BitReader<bool>;
#[doc = "Field `PROT174` writer - Protects Sector 174 from program or erase"]
pub type PROT174_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT175` reader - Protects Sector 175 from program or erase"]
pub type PROT175_R = crate::BitReader<bool>;
#[doc = "Field `PROT175` writer - Protects Sector 175 from program or erase"]
pub type PROT175_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT176` reader - Protects Sector 176 from program or erase"]
pub type PROT176_R = crate::BitReader<bool>;
#[doc = "Field `PROT176` writer - Protects Sector 176 from program or erase"]
pub type PROT176_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT177` reader - Protects Sector 177 from program or erase"]
pub type PROT177_R = crate::BitReader<bool>;
#[doc = "Field `PROT177` writer - Protects Sector 177 from program or erase"]
pub type PROT177_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT178` reader - Protects Sector 178 from program or erase"]
pub type PROT178_R = crate::BitReader<bool>;
#[doc = "Field `PROT178` writer - Protects Sector 178 from program or erase"]
pub type PROT178_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT179` reader - Protects Sector 179 from program or erase"]
pub type PROT179_R = crate::BitReader<bool>;
#[doc = "Field `PROT179` writer - Protects Sector 179 from program or erase"]
pub type PROT179_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT180` reader - Protects Sector 180 from program or erase"]
pub type PROT180_R = crate::BitReader<bool>;
#[doc = "Field `PROT180` writer - Protects Sector 180 from program or erase"]
pub type PROT180_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT181` reader - Protects Sector 181 from program or erase"]
pub type PROT181_R = crate::BitReader<bool>;
#[doc = "Field `PROT181` writer - Protects Sector 181 from program or erase"]
pub type PROT181_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT182` reader - Protects Sector 182 from program or erase"]
pub type PROT182_R = crate::BitReader<bool>;
#[doc = "Field `PROT182` writer - Protects Sector 182 from program or erase"]
pub type PROT182_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT183` reader - Protects Sector 183 from program or erase"]
pub type PROT183_R = crate::BitReader<bool>;
#[doc = "Field `PROT183` writer - Protects Sector 183 from program or erase"]
pub type PROT183_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT184` reader - Protects Sector 184 from program or erase"]
pub type PROT184_R = crate::BitReader<bool>;
#[doc = "Field `PROT184` writer - Protects Sector 184 from program or erase"]
pub type PROT184_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT185` reader - Protects Sector 185 from program or erase"]
pub type PROT185_R = crate::BitReader<bool>;
#[doc = "Field `PROT185` writer - Protects Sector 185 from program or erase"]
pub type PROT185_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT186` reader - Protects Sector 186 from program or erase"]
pub type PROT186_R = crate::BitReader<bool>;
#[doc = "Field `PROT186` writer - Protects Sector 186 from program or erase"]
pub type PROT186_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT187` reader - Protects Sector 187 from program or erase"]
pub type PROT187_R = crate::BitReader<bool>;
#[doc = "Field `PROT187` writer - Protects Sector 187 from program or erase"]
pub type PROT187_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT188` reader - Protects Sector 188 from program or erase"]
pub type PROT188_R = crate::BitReader<bool>;
#[doc = "Field `PROT188` writer - Protects Sector 188 from program or erase"]
pub type PROT188_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT189` reader - Protects Sector 189 from program or erase"]
pub type PROT189_R = crate::BitReader<bool>;
#[doc = "Field `PROT189` writer - Protects Sector 189 from program or erase"]
pub type PROT189_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT190` reader - Protects Sector 190 from program or erase"]
pub type PROT190_R = crate::BitReader<bool>;
#[doc = "Field `PROT190` writer - Protects Sector 190 from program or erase"]
pub type PROT190_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
#[doc = "Field `PROT191` reader - Protects Sector 191 from program or erase"]
pub type PROT191_R = crate::BitReader<bool>;
#[doc = "Field `PROT191` writer - Protects Sector 191 from program or erase"]
pub type PROT191_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT5_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Protects Sector 160 from program or erase"]
    #[inline(always)]
    pub fn prot160(&self) -> PROT160_R {
        PROT160_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protects Sector 161 from program or erase"]
    #[inline(always)]
    pub fn prot161(&self) -> PROT161_R {
        PROT161_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protects Sector 162 from program or erase"]
    #[inline(always)]
    pub fn prot162(&self) -> PROT162_R {
        PROT162_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protects Sector 163 from program or erase"]
    #[inline(always)]
    pub fn prot163(&self) -> PROT163_R {
        PROT163_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protects Sector 164 from program or erase"]
    #[inline(always)]
    pub fn prot164(&self) -> PROT164_R {
        PROT164_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Protects Sector 165 from program or erase"]
    #[inline(always)]
    pub fn prot165(&self) -> PROT165_R {
        PROT165_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protects Sector 166 from program or erase"]
    #[inline(always)]
    pub fn prot166(&self) -> PROT166_R {
        PROT166_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Protects Sector 167 from program or erase"]
    #[inline(always)]
    pub fn prot167(&self) -> PROT167_R {
        PROT167_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Protects Sector 168 from program or erase"]
    #[inline(always)]
    pub fn prot168(&self) -> PROT168_R {
        PROT168_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protects Sector 169 from program or erase"]
    #[inline(always)]
    pub fn prot169(&self) -> PROT169_R {
        PROT169_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protects Sector 170 from program or erase"]
    #[inline(always)]
    pub fn prot170(&self) -> PROT170_R {
        PROT170_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protects Sector 171 from program or erase"]
    #[inline(always)]
    pub fn prot171(&self) -> PROT171_R {
        PROT171_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protects Sector 172 from program or erase"]
    #[inline(always)]
    pub fn prot172(&self) -> PROT172_R {
        PROT172_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Protects Sector 173 from program or erase"]
    #[inline(always)]
    pub fn prot173(&self) -> PROT173_R {
        PROT173_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protects Sector 174 from program or erase"]
    #[inline(always)]
    pub fn prot174(&self) -> PROT174_R {
        PROT174_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Protects Sector 175 from program or erase"]
    #[inline(always)]
    pub fn prot175(&self) -> PROT175_R {
        PROT175_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Protects Sector 176 from program or erase"]
    #[inline(always)]
    pub fn prot176(&self) -> PROT176_R {
        PROT176_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protects Sector 177 from program or erase"]
    #[inline(always)]
    pub fn prot177(&self) -> PROT177_R {
        PROT177_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protects Sector 178 from program or erase"]
    #[inline(always)]
    pub fn prot178(&self) -> PROT178_R {
        PROT178_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protects Sector 179 from program or erase"]
    #[inline(always)]
    pub fn prot179(&self) -> PROT179_R {
        PROT179_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protects Sector 180 from program or erase"]
    #[inline(always)]
    pub fn prot180(&self) -> PROT180_R {
        PROT180_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protects Sector 181 from program or erase"]
    #[inline(always)]
    pub fn prot181(&self) -> PROT181_R {
        PROT181_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protects Sector 182 from program or erase"]
    #[inline(always)]
    pub fn prot182(&self) -> PROT182_R {
        PROT182_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Protects Sector 183 from program or erase"]
    #[inline(always)]
    pub fn prot183(&self) -> PROT183_R {
        PROT183_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Protects Sector 184 from program or erase"]
    #[inline(always)]
    pub fn prot184(&self) -> PROT184_R {
        PROT184_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protects Sector 185 from program or erase"]
    #[inline(always)]
    pub fn prot185(&self) -> PROT185_R {
        PROT185_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protects Sector 186 from program or erase"]
    #[inline(always)]
    pub fn prot186(&self) -> PROT186_R {
        PROT186_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protects Sector 187 from program or erase"]
    #[inline(always)]
    pub fn prot187(&self) -> PROT187_R {
        PROT187_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protects Sector 188 from program or erase"]
    #[inline(always)]
    pub fn prot188(&self) -> PROT188_R {
        PROT188_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Protects Sector 189 from program or erase"]
    #[inline(always)]
    pub fn prot189(&self) -> PROT189_R {
        PROT189_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Protects Sector 190 from program or erase"]
    #[inline(always)]
    pub fn prot190(&self) -> PROT190_R {
        PROT190_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Protects Sector 191 from program or erase"]
    #[inline(always)]
    pub fn prot191(&self) -> PROT191_R {
        PROT191_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protects Sector 160 from program or erase"]
    #[inline(always)]
    pub fn prot160(&mut self) -> PROT160_W<0> {
        PROT160_W::new(self)
    }
    #[doc = "Bit 1 - Protects Sector 161 from program or erase"]
    #[inline(always)]
    pub fn prot161(&mut self) -> PROT161_W<1> {
        PROT161_W::new(self)
    }
    #[doc = "Bit 2 - Protects Sector 162 from program or erase"]
    #[inline(always)]
    pub fn prot162(&mut self) -> PROT162_W<2> {
        PROT162_W::new(self)
    }
    #[doc = "Bit 3 - Protects Sector 163 from program or erase"]
    #[inline(always)]
    pub fn prot163(&mut self) -> PROT163_W<3> {
        PROT163_W::new(self)
    }
    #[doc = "Bit 4 - Protects Sector 164 from program or erase"]
    #[inline(always)]
    pub fn prot164(&mut self) -> PROT164_W<4> {
        PROT164_W::new(self)
    }
    #[doc = "Bit 5 - Protects Sector 165 from program or erase"]
    #[inline(always)]
    pub fn prot165(&mut self) -> PROT165_W<5> {
        PROT165_W::new(self)
    }
    #[doc = "Bit 6 - Protects Sector 166 from program or erase"]
    #[inline(always)]
    pub fn prot166(&mut self) -> PROT166_W<6> {
        PROT166_W::new(self)
    }
    #[doc = "Bit 7 - Protects Sector 167 from program or erase"]
    #[inline(always)]
    pub fn prot167(&mut self) -> PROT167_W<7> {
        PROT167_W::new(self)
    }
    #[doc = "Bit 8 - Protects Sector 168 from program or erase"]
    #[inline(always)]
    pub fn prot168(&mut self) -> PROT168_W<8> {
        PROT168_W::new(self)
    }
    #[doc = "Bit 9 - Protects Sector 169 from program or erase"]
    #[inline(always)]
    pub fn prot169(&mut self) -> PROT169_W<9> {
        PROT169_W::new(self)
    }
    #[doc = "Bit 10 - Protects Sector 170 from program or erase"]
    #[inline(always)]
    pub fn prot170(&mut self) -> PROT170_W<10> {
        PROT170_W::new(self)
    }
    #[doc = "Bit 11 - Protects Sector 171 from program or erase"]
    #[inline(always)]
    pub fn prot171(&mut self) -> PROT171_W<11> {
        PROT171_W::new(self)
    }
    #[doc = "Bit 12 - Protects Sector 172 from program or erase"]
    #[inline(always)]
    pub fn prot172(&mut self) -> PROT172_W<12> {
        PROT172_W::new(self)
    }
    #[doc = "Bit 13 - Protects Sector 173 from program or erase"]
    #[inline(always)]
    pub fn prot173(&mut self) -> PROT173_W<13> {
        PROT173_W::new(self)
    }
    #[doc = "Bit 14 - Protects Sector 174 from program or erase"]
    #[inline(always)]
    pub fn prot174(&mut self) -> PROT174_W<14> {
        PROT174_W::new(self)
    }
    #[doc = "Bit 15 - Protects Sector 175 from program or erase"]
    #[inline(always)]
    pub fn prot175(&mut self) -> PROT175_W<15> {
        PROT175_W::new(self)
    }
    #[doc = "Bit 16 - Protects Sector 176 from program or erase"]
    #[inline(always)]
    pub fn prot176(&mut self) -> PROT176_W<16> {
        PROT176_W::new(self)
    }
    #[doc = "Bit 17 - Protects Sector 177 from program or erase"]
    #[inline(always)]
    pub fn prot177(&mut self) -> PROT177_W<17> {
        PROT177_W::new(self)
    }
    #[doc = "Bit 18 - Protects Sector 178 from program or erase"]
    #[inline(always)]
    pub fn prot178(&mut self) -> PROT178_W<18> {
        PROT178_W::new(self)
    }
    #[doc = "Bit 19 - Protects Sector 179 from program or erase"]
    #[inline(always)]
    pub fn prot179(&mut self) -> PROT179_W<19> {
        PROT179_W::new(self)
    }
    #[doc = "Bit 20 - Protects Sector 180 from program or erase"]
    #[inline(always)]
    pub fn prot180(&mut self) -> PROT180_W<20> {
        PROT180_W::new(self)
    }
    #[doc = "Bit 21 - Protects Sector 181 from program or erase"]
    #[inline(always)]
    pub fn prot181(&mut self) -> PROT181_W<21> {
        PROT181_W::new(self)
    }
    #[doc = "Bit 22 - Protects Sector 182 from program or erase"]
    #[inline(always)]
    pub fn prot182(&mut self) -> PROT182_W<22> {
        PROT182_W::new(self)
    }
    #[doc = "Bit 23 - Protects Sector 183 from program or erase"]
    #[inline(always)]
    pub fn prot183(&mut self) -> PROT183_W<23> {
        PROT183_W::new(self)
    }
    #[doc = "Bit 24 - Protects Sector 184 from program or erase"]
    #[inline(always)]
    pub fn prot184(&mut self) -> PROT184_W<24> {
        PROT184_W::new(self)
    }
    #[doc = "Bit 25 - Protects Sector 185 from program or erase"]
    #[inline(always)]
    pub fn prot185(&mut self) -> PROT185_W<25> {
        PROT185_W::new(self)
    }
    #[doc = "Bit 26 - Protects Sector 186 from program or erase"]
    #[inline(always)]
    pub fn prot186(&mut self) -> PROT186_W<26> {
        PROT186_W::new(self)
    }
    #[doc = "Bit 27 - Protects Sector 187 from program or erase"]
    #[inline(always)]
    pub fn prot187(&mut self) -> PROT187_W<27> {
        PROT187_W::new(self)
    }
    #[doc = "Bit 28 - Protects Sector 188 from program or erase"]
    #[inline(always)]
    pub fn prot188(&mut self) -> PROT188_W<28> {
        PROT188_W::new(self)
    }
    #[doc = "Bit 29 - Protects Sector 189 from program or erase"]
    #[inline(always)]
    pub fn prot189(&mut self) -> PROT189_W<29> {
        PROT189_W::new(self)
    }
    #[doc = "Bit 30 - Protects Sector 190 from program or erase"]
    #[inline(always)]
    pub fn prot190(&mut self) -> PROT190_W<30> {
        PROT190_W::new(self)
    }
    #[doc = "Bit 31 - Protects Sector 191 from program or erase"]
    #[inline(always)]
    pub fn prot191(&mut self) -> PROT191_W<31> {
        PROT191_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Memory Bank0 Write/Erase Protection Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank0_main_weprot5](index.html) module"]
pub struct FLCTL_BANK0_MAIN_WEPROT5_SPEC;
impl crate::RegisterSpec for FLCTL_BANK0_MAIN_WEPROT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bank0_main_weprot5::R](R) reader structure"]
impl crate::Readable for FLCTL_BANK0_MAIN_WEPROT5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bank0_main_weprot5::W](W) writer structure"]
impl crate::Writable for FLCTL_BANK0_MAIN_WEPROT5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BANK0_MAIN_WEPROT5 to value 0xffff_ffff"]
impl crate::Resettable for FLCTL_BANK0_MAIN_WEPROT5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
