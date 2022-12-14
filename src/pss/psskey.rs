#[doc = "Register `PSSKEY` reader"]
pub struct R(crate::R<PSSKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSSKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSSKEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSSKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSSKEY` writer"]
pub struct W(crate::W<PSSKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSSKEY_SPEC>;
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
impl From<crate::W<PSSKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSSKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSSKEY` reader - PSS control key"]
pub type PSSKEY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PSSKEY` writer - PSS control key"]
pub type PSSKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSSKEY_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - PSS control key"]
    #[inline(always)]
    pub fn psskey(&self) -> PSSKEY_R {
        PSSKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PSS control key"]
    #[inline(always)]
    pub fn psskey(&mut self) -> PSSKEY_W<0> {
        PSSKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psskey](index.html) module"]
pub struct PSSKEY_SPEC;
impl crate::RegisterSpec for PSSKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psskey::R](R) reader structure"]
impl crate::Readable for PSSKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psskey::W](W) writer structure"]
impl crate::Writable for PSSKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSSKEY to value 0xa596"]
impl crate::Resettable for PSSKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa596
    }
}
