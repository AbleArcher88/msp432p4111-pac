#[doc = "Register `SYS_INFOFLASH_SIZE` reader"]
pub struct R(crate::R<SYS_INFOFLASH_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_INFOFLASH_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_INFOFLASH_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_INFOFLASH_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIZE` reader - Flash information memory size"]
pub type SIZE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Flash information memory size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(self.bits)
    }
}
#[doc = "Flash Information Memory Size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_infoflash_size](index.html) module"]
pub struct SYS_INFOFLASH_SIZE_SPEC;
impl crate::RegisterSpec for SYS_INFOFLASH_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_infoflash_size::R](R) reader structure"]
impl crate::Readable for SYS_INFOFLASH_SIZE_SPEC {
    type Reader = R;
}
