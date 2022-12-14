#[doc = "Register `AESAKEY` writer"]
pub struct W(crate::W<AESAKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESAKEY_SPEC>;
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
impl From<crate::W<AESAKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESAKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AESKEY0x` writer - AES key byte n when AESAKEY is written as half-word"]
pub type AESKEY0X_W<'a, const O: u8> = crate::FieldWriter<'a, u16, AESAKEY_SPEC, u8, u8, 8, O>;
#[doc = "Field `AESKEY1x` writer - AES key byte n+1 when AESAKEY is written as half-word"]
pub type AESKEY1X_W<'a, const O: u8> = crate::FieldWriter<'a, u16, AESAKEY_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - AES key byte n when AESAKEY is written as half-word"]
    #[inline(always)]
    pub fn aeskey0x(&mut self) -> AESKEY0X_W<0> {
        AESKEY0X_W::new(self)
    }
    #[doc = "Bits 8:15 - AES key byte n+1 when AESAKEY is written as half-word"]
    #[inline(always)]
    pub fn aeskey1x(&mut self) -> AESKEY1X_W<8> {
        AESKEY1X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Accelerator Key Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesakey](index.html) module"]
pub struct AESAKEY_SPEC;
impl crate::RegisterSpec for AESAKEY_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [aesakey::W](W) writer structure"]
impl crate::Writable for AESAKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESAKEY to value 0"]
impl crate::Resettable for AESAKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
