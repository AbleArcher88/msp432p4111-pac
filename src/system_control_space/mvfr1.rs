#[doc = "Register `MVFR1` reader"]
pub struct R(crate::R<MVFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MVFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MVFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MVFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FTZ_MODE` reader - Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation. The value of this field is: 0b0001 - hardware supports full denormalized number arithmetic."]
pub type FTZ_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D_NAN_MODE` reader - Indicates whether the FP hardware implementation supports only the Default NaN mode. The value of this field is: 0b0001 - hardware supports propagation of NaN values."]
pub type D_NAN_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FP_HPFP` reader - Indicates whether the FP supports half-precision floating-point conversion operations. The value of this field is: 0b0001 - supported."]
pub type FP_HPFP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FP_FUSED_MAC` reader - Indicates whether the FP supports fused multiply accumulate operations. The value of this field is: 0b0001 - supported."]
pub type FP_FUSED_MAC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation. The value of this field is: 0b0001 - hardware supports full denormalized number arithmetic."]
    #[inline(always)]
    pub fn ftz_mode(&self) -> FTZ_MODE_R {
        FTZ_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates whether the FP hardware implementation supports only the Default NaN mode. The value of this field is: 0b0001 - hardware supports propagation of NaN values."]
    #[inline(always)]
    pub fn d_nan_mode(&self) -> D_NAN_MODE_R {
        D_NAN_MODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates whether the FP supports half-precision floating-point conversion operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn fp_hpfp(&self) -> FP_HPFP_R {
        FP_HPFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Indicates whether the FP supports fused multiply accumulate operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn fp_fused_mac(&self) -> FP_FUSED_MAC_R {
        FP_FUSED_MAC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Media and FP Feature Register 1 (MVFR1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mvfr1](index.html) module"]
pub struct MVFR1_SPEC;
impl crate::RegisterSpec for MVFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mvfr1::R](R) reader structure"]
impl crate::Readable for MVFR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MVFR1 to value 0x1100_0011"]
impl crate::Resettable for MVFR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1100_0011
    }
}
