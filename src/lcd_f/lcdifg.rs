#[doc = "Register `LCDIFG` reader"]
pub struct R(crate::R<LCDIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LCDBLKOFFIFG` reader - LCD Blink, segments off interrupt flag"]
pub type LCDBLKOFFIFG_R = crate::BitReader<LCDBLKOFFIFG_A>;
#[doc = "LCD Blink, segments off interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDBLKOFFIFG_A {
    #[doc = "0: No interrupt pending"]
    LCDBLKOFFIFG_0 = 0,
    #[doc = "1: Interrupt due to LCD Blink, segments on"]
    LCDBLKOFFIFG_1 = 1,
}
impl From<LCDBLKOFFIFG_A> for bool {
    #[inline(always)]
    fn from(variant: LCDBLKOFFIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDBLKOFFIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDBLKOFFIFG_A {
        match self.bits {
            false => LCDBLKOFFIFG_A::LCDBLKOFFIFG_0,
            true => LCDBLKOFFIFG_A::LCDBLKOFFIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDBLKOFFIFG_0`"]
    #[inline(always)]
    pub fn is_lcdblkoffifg_0(&self) -> bool {
        *self == LCDBLKOFFIFG_A::LCDBLKOFFIFG_0
    }
    #[doc = "Checks if the value of the field is `LCDBLKOFFIFG_1`"]
    #[inline(always)]
    pub fn is_lcdblkoffifg_1(&self) -> bool {
        *self == LCDBLKOFFIFG_A::LCDBLKOFFIFG_1
    }
}
#[doc = "Field `LCDBLKONIFG` reader - LCD Blink, segments on interrupt flag"]
pub type LCDBLKONIFG_R = crate::BitReader<LCDBLKONIFG_A>;
#[doc = "LCD Blink, segments on interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDBLKONIFG_A {
    #[doc = "0: No interrupt pending"]
    LCDBLKONIFG_0 = 0,
    #[doc = "1: Interrupt due to LCD Blink, segments on"]
    LCDBLKONIFG_1 = 1,
}
impl From<LCDBLKONIFG_A> for bool {
    #[inline(always)]
    fn from(variant: LCDBLKONIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDBLKONIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDBLKONIFG_A {
        match self.bits {
            false => LCDBLKONIFG_A::LCDBLKONIFG_0,
            true => LCDBLKONIFG_A::LCDBLKONIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDBLKONIFG_0`"]
    #[inline(always)]
    pub fn is_lcdblkonifg_0(&self) -> bool {
        *self == LCDBLKONIFG_A::LCDBLKONIFG_0
    }
    #[doc = "Checks if the value of the field is `LCDBLKONIFG_1`"]
    #[inline(always)]
    pub fn is_lcdblkonifg_1(&self) -> bool {
        *self == LCDBLKONIFG_A::LCDBLKONIFG_1
    }
}
#[doc = "Field `LCDFRMIFG` reader - LCD Frame interrupt flag"]
pub type LCDFRMIFG_R = crate::BitReader<LCDFRMIFG_A>;
#[doc = "LCD Frame interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDFRMIFG_A {
    #[doc = "0: No interrupt pending"]
    LCDFRMIFG_0 = 0,
    #[doc = "1: Interrupt due to LCD frame"]
    LCDFRMIFG_1 = 1,
}
impl From<LCDFRMIFG_A> for bool {
    #[inline(always)]
    fn from(variant: LCDFRMIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDFRMIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDFRMIFG_A {
        match self.bits {
            false => LCDFRMIFG_A::LCDFRMIFG_0,
            true => LCDFRMIFG_A::LCDFRMIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDFRMIFG_0`"]
    #[inline(always)]
    pub fn is_lcdfrmifg_0(&self) -> bool {
        *self == LCDFRMIFG_A::LCDFRMIFG_0
    }
    #[doc = "Checks if the value of the field is `LCDFRMIFG_1`"]
    #[inline(always)]
    pub fn is_lcdfrmifg_1(&self) -> bool {
        *self == LCDFRMIFG_A::LCDFRMIFG_1
    }
}
#[doc = "Field `LCDANMSTPIFG` reader - LCD Animation step interrupt flag"]
pub type LCDANMSTPIFG_R = crate::BitReader<LCDANMSTPIFG_A>;
#[doc = "LCD Animation step interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDANMSTPIFG_A {
    #[doc = "0: No interrupt pending"]
    LCDANMSTPIFG_0 = 0,
    #[doc = "1: Interrupt due to LCD Animation step on"]
    LCDANMSTPIFG_1 = 1,
}
impl From<LCDANMSTPIFG_A> for bool {
    #[inline(always)]
    fn from(variant: LCDANMSTPIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDANMSTPIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDANMSTPIFG_A {
        match self.bits {
            false => LCDANMSTPIFG_A::LCDANMSTPIFG_0,
            true => LCDANMSTPIFG_A::LCDANMSTPIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDANMSTPIFG_0`"]
    #[inline(always)]
    pub fn is_lcdanmstpifg_0(&self) -> bool {
        *self == LCDANMSTPIFG_A::LCDANMSTPIFG_0
    }
    #[doc = "Checks if the value of the field is `LCDANMSTPIFG_1`"]
    #[inline(always)]
    pub fn is_lcdanmstpifg_1(&self) -> bool {
        *self == LCDANMSTPIFG_A::LCDANMSTPIFG_1
    }
}
#[doc = "Field `LCDANMLOOPIFG` reader - LCD Animation loop interrupt flag"]
pub type LCDANMLOOPIFG_R = crate::BitReader<LCDANMLOOPIFG_A>;
#[doc = "LCD Animation loop interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDANMLOOPIFG_A {
    #[doc = "0: No interrupt pending"]
    LCDANMLOOPIFG_0 = 0,
    #[doc = "1: Interrupt due to LCD Animation loop on"]
    LCDANMLOOPIFG_1 = 1,
}
impl From<LCDANMLOOPIFG_A> for bool {
    #[inline(always)]
    fn from(variant: LCDANMLOOPIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDANMLOOPIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDANMLOOPIFG_A {
        match self.bits {
            false => LCDANMLOOPIFG_A::LCDANMLOOPIFG_0,
            true => LCDANMLOOPIFG_A::LCDANMLOOPIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDANMLOOPIFG_0`"]
    #[inline(always)]
    pub fn is_lcdanmloopifg_0(&self) -> bool {
        *self == LCDANMLOOPIFG_A::LCDANMLOOPIFG_0
    }
    #[doc = "Checks if the value of the field is `LCDANMLOOPIFG_1`"]
    #[inline(always)]
    pub fn is_lcdanmloopifg_1(&self) -> bool {
        *self == LCDANMLOOPIFG_A::LCDANMLOOPIFG_1
    }
}
impl R {
    #[doc = "Bit 1 - LCD Blink, segments off interrupt flag"]
    #[inline(always)]
    pub fn lcdblkoffifg(&self) -> LCDBLKOFFIFG_R {
        LCDBLKOFFIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD Blink, segments on interrupt flag"]
    #[inline(always)]
    pub fn lcdblkonifg(&self) -> LCDBLKONIFG_R {
        LCDBLKONIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD Frame interrupt flag"]
    #[inline(always)]
    pub fn lcdfrmifg(&self) -> LCDFRMIFG_R {
        LCDFRMIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD Animation step interrupt flag"]
    #[inline(always)]
    pub fn lcdanmstpifg(&self) -> LCDANMSTPIFG_R {
        LCDANMSTPIFG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD Animation loop interrupt flag"]
    #[inline(always)]
    pub fn lcdanmloopifg(&self) -> LCDANMLOOPIFG_R {
        LCDANMLOOPIFG_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "LCD_F interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdifg](index.html) module"]
pub struct LCDIFG_SPEC;
impl crate::RegisterSpec for LCDIFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdifg::R](R) reader structure"]
impl crate::Readable for LCDIFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LCDIFG to value 0"]
impl crate::Resettable for LCDIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
