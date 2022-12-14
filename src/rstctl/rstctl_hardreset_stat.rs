#[doc = "Register `RSTCTL_HARDRESET_STAT` reader"]
pub struct R(crate::R<RSTCTL_HARDRESET_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTL_HARDRESET_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTL_HARDRESET_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTL_HARDRESET_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRC0` reader - Indicates that SRC0 was the source of the Hard Reset"]
pub type SRC0_R = crate::BitReader<bool>;
#[doc = "Field `SRC1` reader - Indicates that SRC1 was the source of the Hard Reset"]
pub type SRC1_R = crate::BitReader<bool>;
#[doc = "Field `SRC2` reader - Indicates that SRC2 was the source of the Hard Reset"]
pub type SRC2_R = crate::BitReader<bool>;
#[doc = "Field `SRC3` reader - Indicates that SRC3 was the source of the Hard Reset"]
pub type SRC3_R = crate::BitReader<bool>;
#[doc = "Field `SRC4` reader - Indicates that SRC4 was the source of the Hard Reset"]
pub type SRC4_R = crate::BitReader<bool>;
#[doc = "Field `SRC5` reader - Indicates that SRC5 was the source of the Hard Reset"]
pub type SRC5_R = crate::BitReader<bool>;
#[doc = "Field `SRC6` reader - Indicates that SRC6 was the source of the Hard Reset"]
pub type SRC6_R = crate::BitReader<bool>;
#[doc = "Field `SRC7` reader - Indicates that SRC7 was the source of the Hard Reset"]
pub type SRC7_R = crate::BitReader<bool>;
#[doc = "Field `SRC8` reader - Indicates that SRC8 was the source of the Hard Reset"]
pub type SRC8_R = crate::BitReader<bool>;
#[doc = "Field `SRC9` reader - Indicates that SRC9 was the source of the Hard Reset"]
pub type SRC9_R = crate::BitReader<bool>;
#[doc = "Field `SRC10` reader - Indicates that SRC10 was the source of the Hard Reset"]
pub type SRC10_R = crate::BitReader<bool>;
#[doc = "Field `SRC11` reader - Indicates that SRC11 was the source of the Hard Reset"]
pub type SRC11_R = crate::BitReader<bool>;
#[doc = "Field `SRC12` reader - Indicates that SRC12 was the source of the Hard Reset"]
pub type SRC12_R = crate::BitReader<bool>;
#[doc = "Field `SRC13` reader - Indicates that SRC13 was the source of the Hard Reset"]
pub type SRC13_R = crate::BitReader<bool>;
#[doc = "Field `SRC14` reader - Indicates that SRC14 was the source of the Hard Reset"]
pub type SRC14_R = crate::BitReader<bool>;
#[doc = "Field `SRC15` reader - Indicates that SRC15 was the source of the Hard Reset"]
pub type SRC15_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates that SRC0 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src0(&self) -> SRC0_R {
        SRC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that SRC1 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src1(&self) -> SRC1_R {
        SRC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that SRC2 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src2(&self) -> SRC2_R {
        SRC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that SRC3 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src3(&self) -> SRC3_R {
        SRC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that SRC4 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src4(&self) -> SRC4_R {
        SRC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates that SRC5 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src5(&self) -> SRC5_R {
        SRC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that SRC6 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src6(&self) -> SRC6_R {
        SRC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates that SRC7 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src7(&self) -> SRC7_R {
        SRC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates that SRC8 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src8(&self) -> SRC8_R {
        SRC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates that SRC9 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src9(&self) -> SRC9_R {
        SRC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates that SRC10 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src10(&self) -> SRC10_R {
        SRC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates that SRC11 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src11(&self) -> SRC11_R {
        SRC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates that SRC12 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src12(&self) -> SRC12_R {
        SRC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates that SRC13 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src13(&self) -> SRC13_R {
        SRC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates that SRC14 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src14(&self) -> SRC14_R {
        SRC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicates that SRC15 was the source of the Hard Reset"]
    #[inline(always)]
    pub fn src15(&self) -> SRC15_R {
        SRC15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Hard Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_hardreset_stat](index.html) module"]
pub struct RSTCTL_HARDRESET_STAT_SPEC;
impl crate::RegisterSpec for RSTCTL_HARDRESET_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctl_hardreset_stat::R](R) reader structure"]
impl crate::Readable for RSTCTL_HARDRESET_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTCTL_HARDRESET_STAT to value 0"]
impl crate::Resettable for RSTCTL_HARDRESET_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
