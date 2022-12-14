#[doc = "Register `PASELC` reader"]
pub struct R(crate::R<PASELC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PASELC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PASELC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PASELC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PASELC` writer"]
pub struct W(crate::W<PASELC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PASELC_SPEC>;
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
impl From<crate::W<PASELC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PASELC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1SELC` reader - Port 1 Complement Select"]
pub type P1SELC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P1SELC` writer - Port 1 Complement Select"]
pub type P1SELC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PASELC_SPEC, u8, u8, 8, O>;
#[doc = "Field `P2SELC` reader - Port 2 Complement Select"]
pub type P2SELC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P2SELC` writer - Port 2 Complement Select"]
pub type P2SELC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PASELC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Complement Select"]
    #[inline(always)]
    pub fn p1selc(&self) -> P1SELC_R {
        P1SELC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Complement Select"]
    #[inline(always)]
    pub fn p2selc(&self) -> P2SELC_R {
        P2SELC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Complement Select"]
    #[inline(always)]
    pub fn p1selc(&mut self) -> P1SELC_W<0> {
        P1SELC_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 2 Complement Select"]
    #[inline(always)]
    pub fn p2selc(&mut self) -> P2SELC_W<8> {
        P2SELC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port A Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paselc](index.html) module"]
pub struct PASELC_SPEC;
impl crate::RegisterSpec for PASELC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [paselc::R](R) reader structure"]
impl crate::Readable for PASELC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paselc::W](W) writer structure"]
impl crate::Writable for PASELC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PASELC to value 0"]
impl crate::Resettable for PASELC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
