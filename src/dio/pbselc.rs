#[doc = "Register `PBSELC` reader"]
pub struct R(crate::R<PBSELC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBSELC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBSELC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBSELC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBSELC` writer"]
pub struct W(crate::W<PBSELC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBSELC_SPEC>;
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
impl From<crate::W<PBSELC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBSELC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3SELC` reader - Port 3 Complement Select"]
pub type P3SELC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P3SELC` writer - Port 3 Complement Select"]
pub type P3SELC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PBSELC_SPEC, u8, u8, 8, O>;
#[doc = "Field `P4SELC` reader - Port 4 Complement Select"]
pub type P4SELC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P4SELC` writer - Port 4 Complement Select"]
pub type P4SELC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PBSELC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Complement Select"]
    #[inline(always)]
    pub fn p3selc(&self) -> P3SELC_R {
        P3SELC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Complement Select"]
    #[inline(always)]
    pub fn p4selc(&self) -> P4SELC_R {
        P4SELC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Complement Select"]
    #[inline(always)]
    pub fn p3selc(&mut self) -> P3SELC_W<0> {
        P3SELC_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 4 Complement Select"]
    #[inline(always)]
    pub fn p4selc(&mut self) -> P4SELC_W<8> {
        P4SELC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port B Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbselc](index.html) module"]
pub struct PBSELC_SPEC;
impl crate::RegisterSpec for PBSELC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pbselc::R](R) reader structure"]
impl crate::Readable for PBSELC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbselc::W](W) writer structure"]
impl crate::Writable for PBSELC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBSELC to value 0"]
impl crate::Resettable for PBSELC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
