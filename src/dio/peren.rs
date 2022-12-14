#[doc = "Register `PEREN` reader"]
pub struct R(crate::R<PEREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEREN` writer"]
pub struct W(crate::W<PEREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEREN_SPEC>;
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
impl From<crate::W<PEREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9REN` reader - Port 9 Resistor Enable"]
pub type P9REN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P9REN` writer - Port 9 Resistor Enable"]
pub type P9REN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PEREN_SPEC, u8, u8, 8, O>;
#[doc = "Field `P10REN` reader - Port 10 Resistor Enable"]
pub type P10REN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P10REN` writer - Port 10 Resistor Enable"]
pub type P10REN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PEREN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Resistor Enable"]
    #[inline(always)]
    pub fn p9ren(&self) -> P9REN_R {
        P9REN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Resistor Enable"]
    #[inline(always)]
    pub fn p10ren(&self) -> P10REN_R {
        P10REN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Resistor Enable"]
    #[inline(always)]
    pub fn p9ren(&mut self) -> P9REN_W<0> {
        P9REN_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 10 Resistor Enable"]
    #[inline(always)]
    pub fn p10ren(&mut self) -> P10REN_W<8> {
        P10REN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port E Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peren](index.html) module"]
pub struct PEREN_SPEC;
impl crate::RegisterSpec for PEREN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [peren::R](R) reader structure"]
impl crate::Readable for PEREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peren::W](W) writer structure"]
impl crate::Writable for PEREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEREN to value 0"]
impl crate::Resettable for PEREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
