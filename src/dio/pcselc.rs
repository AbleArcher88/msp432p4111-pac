#[doc = "Register `PCSELC` reader"]
pub struct R(crate::R<PCSELC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSELC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSELC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSELC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCSELC` writer"]
pub struct W(crate::W<PCSELC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCSELC_SPEC>;
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
impl From<crate::W<PCSELC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCSELC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5SELC` reader - Port 5 Complement Select"]
pub type P5SELC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P5SELC` writer - Port 5 Complement Select"]
pub type P5SELC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PCSELC_SPEC, u8, u8, 8, O>;
#[doc = "Field `P6SELC` reader - Port 6 Complement Select"]
pub type P6SELC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P6SELC` writer - Port 6 Complement Select"]
pub type P6SELC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PCSELC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Complement Select"]
    #[inline(always)]
    pub fn p5selc(&self) -> P5SELC_R {
        P5SELC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Complement Select"]
    #[inline(always)]
    pub fn p6selc(&self) -> P6SELC_R {
        P6SELC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Complement Select"]
    #[inline(always)]
    pub fn p5selc(&mut self) -> P5SELC_W<0> {
        P5SELC_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 6 Complement Select"]
    #[inline(always)]
    pub fn p6selc(&mut self) -> P6SELC_W<8> {
        P6SELC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcselc](index.html) module"]
pub struct PCSELC_SPEC;
impl crate::RegisterSpec for PCSELC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pcselc::R](R) reader structure"]
impl crate::Readable for PCSELC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcselc::W](W) writer structure"]
impl crate::Writable for PCSELC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCSELC to value 0"]
impl crate::Resettable for PCSELC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
