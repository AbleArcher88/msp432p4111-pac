#[doc = "Register `PCREN` reader"]
pub struct R(crate::R<PCREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCREN` writer"]
pub struct W(crate::W<PCREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCREN_SPEC>;
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
impl From<crate::W<PCREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5REN` reader - Port 5 Resistor Enable"]
pub type P5REN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P5REN` writer - Port 5 Resistor Enable"]
pub type P5REN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PCREN_SPEC, u8, u8, 8, O>;
#[doc = "Field `P6REN` reader - Port 6 Resistor Enable"]
pub type P6REN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P6REN` writer - Port 6 Resistor Enable"]
pub type P6REN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PCREN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Resistor Enable"]
    #[inline(always)]
    pub fn p5ren(&self) -> P5REN_R {
        P5REN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Resistor Enable"]
    #[inline(always)]
    pub fn p6ren(&self) -> P6REN_R {
        P6REN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Resistor Enable"]
    #[inline(always)]
    pub fn p5ren(&mut self) -> P5REN_W<0> {
        P5REN_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 6 Resistor Enable"]
    #[inline(always)]
    pub fn p6ren(&mut self) -> P6REN_W<8> {
        P6REN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcren](index.html) module"]
pub struct PCREN_SPEC;
impl crate::RegisterSpec for PCREN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pcren::R](R) reader structure"]
impl crate::Readable for PCREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcren::W](W) writer structure"]
impl crate::Writable for PCREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCREN to value 0"]
impl crate::Resettable for PCREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
