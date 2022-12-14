#[doc = "Register `DMA_ALTBASE` reader"]
pub struct R(crate::R<DMA_ALTBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ALTBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_ALTBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_ALTBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - Base address of the alternate data structure"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address of the alternate data structure"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "Channel Alternate Control Data Base Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_altbase](index.html) module"]
pub struct DMA_ALTBASE_SPEC;
impl crate::RegisterSpec for DMA_ALTBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_altbase::R](R) reader structure"]
impl crate::Readable for DMA_ALTBASE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_ALTBASE to value 0"]
impl crate::Resettable for DMA_ALTBASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
