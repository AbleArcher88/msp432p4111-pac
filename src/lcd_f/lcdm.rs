#[doc = "Register `LCDM[%s]` reader"]
pub struct R(crate::R<LCDM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDM[%s]` writer"]
pub struct W(crate::W<LCDM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDM_SPEC>;
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
impl From<crate::W<LCDM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDMEMORY` reader - LCD Memory Contains LCD Segment on/off data for Line Lx if LCDSSx = 0 Contains COM assignment for Line Lx if LCDSSx = 1"]
pub type LCDMEMORY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCDMEMORY` writer - LCD Memory Contains LCD Segment on/off data for Line Lx if LCDSSx = 0 Contains COM assignment for Line Lx if LCDSSx = 1"]
pub type LCDMEMORY_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LCDM_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - LCD Memory Contains LCD Segment on/off data for Line Lx if LCDSSx = 0 Contains COM assignment for Line Lx if LCDSSx = 1"]
    #[inline(always)]
    pub fn lcdmemory(&self) -> LCDMEMORY_R {
        LCDMEMORY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - LCD Memory Contains LCD Segment on/off data for Line Lx if LCDSSx = 0 Contains COM assignment for Line Lx if LCDSSx = 1"]
    #[inline(always)]
    pub fn lcdmemory(&mut self) -> LCDMEMORY_W<0> {
        LCDMEMORY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD memory registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm](index.html) module"]
pub struct LCDM_SPEC;
impl crate::RegisterSpec for LCDM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lcdm::R](R) reader structure"]
impl crate::Readable for LCDM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdm::W](W) writer structure"]
impl crate::Writable for LCDM_SPEC {
    type Writer = W;
}
