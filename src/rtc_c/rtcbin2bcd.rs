#[doc = "Register `RTCBIN2BCD` reader"]
pub struct R(crate::R<RTCBIN2BCD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCBIN2BCD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCBIN2BCD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCBIN2BCD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCBIN2BCD` writer"]
pub struct W(crate::W<RTCBIN2BCD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCBIN2BCD_SPEC>;
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
impl From<crate::W<RTCBIN2BCD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCBIN2BCD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIN2BCD` reader - bin to bcd conversion"]
pub type BIN2BCD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BIN2BCD` writer - bin to bcd conversion"]
pub type BIN2BCD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCBIN2BCD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - bin to bcd conversion"]
    #[inline(always)]
    pub fn bin2bcd(&self) -> BIN2BCD_R {
        BIN2BCD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - bin to bcd conversion"]
    #[inline(always)]
    pub fn bin2bcd(&mut self) -> BIN2BCD_W<0> {
        BIN2BCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Binary-to-BCD Conversion Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcbin2bcd](index.html) module"]
pub struct RTCBIN2BCD_SPEC;
impl crate::RegisterSpec for RTCBIN2BCD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcbin2bcd::R](R) reader structure"]
impl crate::Readable for RTCBIN2BCD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcbin2bcd::W](W) writer structure"]
impl crate::Writable for RTCBIN2BCD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCBIN2BCD to value 0"]
impl crate::Resettable for RTCBIN2BCD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
