#[doc = "Register `RSTCTL_CSRESET_CLR` reader"]
pub struct R(crate::R<RSTCTL_CSRESET_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTL_CSRESET_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTL_CSRESET_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTL_CSRESET_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCTL_CSRESET_CLR` writer"]
pub struct W(crate::W<RSTCTL_CSRESET_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCTL_CSRESET_CLR_SPEC>;
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
impl From<crate::W<RSTCTL_CSRESET_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCTL_CSRESET_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR` writer - Write 1 clears the DCOR_SHT Flag in RSTCTL_CSRESET_STAT as well as DCOR_SHTIFG flag in CSIFG register of clock system"]
pub type CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_CSRESET_CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Write 1 clears the DCOR_SHT Flag in RSTCTL_CSRESET_STAT as well as DCOR_SHTIFG flag in CSIFG register of clock system"]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W<0> {
        CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS Reset Status Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_csreset_clr](index.html) module"]
pub struct RSTCTL_CSRESET_CLR_SPEC;
impl crate::RegisterSpec for RSTCTL_CSRESET_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctl_csreset_clr::R](R) reader structure"]
impl crate::Readable for RSTCTL_CSRESET_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstctl_csreset_clr::W](W) writer structure"]
impl crate::Writable for RSTCTL_CSRESET_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTCTL_CSRESET_CLR to value 0"]
impl crate::Resettable for RSTCTL_CSRESET_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
