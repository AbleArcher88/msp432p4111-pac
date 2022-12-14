#[doc = "Register `LCDVCTL` reader"]
pub struct R(crate::R<LCDVCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDVCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDVCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDVCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDVCTL` writer"]
pub struct W(crate::W<LCDVCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDVCTL_SPEC>;
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
impl From<crate::W<LCDVCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDVCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD2B` reader - Bias select."]
pub type LCD2B_R = crate::BitReader<LCD2B_A>;
#[doc = "Bias select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCD2B_A {
    #[doc = "0: 1/3 bias"]
    LCD2B_0 = 0,
    #[doc = "1: For 5-mux to 8-mux modes:1/2 bias. For 5-mux to 8-mux modes - 1/4 bias."]
    LCD2B_1 = 1,
}
impl From<LCD2B_A> for bool {
    #[inline(always)]
    fn from(variant: LCD2B_A) -> Self {
        variant as u8 != 0
    }
}
impl LCD2B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD2B_A {
        match self.bits {
            false => LCD2B_A::LCD2B_0,
            true => LCD2B_A::LCD2B_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCD2B_0`"]
    #[inline(always)]
    pub fn is_lcd2b_0(&self) -> bool {
        *self == LCD2B_A::LCD2B_0
    }
    #[doc = "Checks if the value of the field is `LCD2B_1`"]
    #[inline(always)]
    pub fn is_lcd2b_1(&self) -> bool {
        *self == LCD2B_A::LCD2B_1
    }
}
#[doc = "Field `LCD2B` writer - Bias select."]
pub type LCD2B_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDVCTL_SPEC, LCD2B_A, O>;
impl<'a, const O: u8> LCD2B_W<'a, O> {
    #[doc = "1/3 bias"]
    #[inline(always)]
    pub fn lcd2b_0(self) -> &'a mut W {
        self.variant(LCD2B_A::LCD2B_0)
    }
    #[doc = "For 5-mux to 8-mux modes:1/2 bias. For 5-mux to 8-mux modes - 1/4 bias."]
    #[inline(always)]
    pub fn lcd2b_1(self) -> &'a mut W {
        self.variant(LCD2B_A::LCD2B_1)
    }
}
#[doc = "Field `LCDEXTBIAS` reader - V2 to V4 voltage select"]
pub type LCDEXTBIAS_R = crate::BitReader<LCDEXTBIAS_A>;
#[doc = "V2 to V4 voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDEXTBIAS_A {
    #[doc = "0: V2 to V4 are generated internally"]
    LCDEXTBIAS_0 = 0,
    #[doc = "1: V2 to V4 are sourced externally and the internal bias generator is switched off"]
    LCDEXTBIAS_1 = 1,
}
impl From<LCDEXTBIAS_A> for bool {
    #[inline(always)]
    fn from(variant: LCDEXTBIAS_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDEXTBIAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDEXTBIAS_A {
        match self.bits {
            false => LCDEXTBIAS_A::LCDEXTBIAS_0,
            true => LCDEXTBIAS_A::LCDEXTBIAS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDEXTBIAS_0`"]
    #[inline(always)]
    pub fn is_lcdextbias_0(&self) -> bool {
        *self == LCDEXTBIAS_A::LCDEXTBIAS_0
    }
    #[doc = "Checks if the value of the field is `LCDEXTBIAS_1`"]
    #[inline(always)]
    pub fn is_lcdextbias_1(&self) -> bool {
        *self == LCDEXTBIAS_A::LCDEXTBIAS_1
    }
}
#[doc = "Field `LCDEXTBIAS` writer - V2 to V4 voltage select"]
pub type LCDEXTBIAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDVCTL_SPEC, LCDEXTBIAS_A, O>;
impl<'a, const O: u8> LCDEXTBIAS_W<'a, O> {
    #[doc = "V2 to V4 are generated internally"]
    #[inline(always)]
    pub fn lcdextbias_0(self) -> &'a mut W {
        self.variant(LCDEXTBIAS_A::LCDEXTBIAS_0)
    }
    #[doc = "V2 to V4 are sourced externally and the internal bias generator is switched off"]
    #[inline(always)]
    pub fn lcdextbias_1(self) -> &'a mut W {
        self.variant(LCDEXTBIAS_A::LCDEXTBIAS_1)
    }
}
#[doc = "Field `R03EXT` reader - V5 voltage select"]
pub type R03EXT_R = crate::BitReader<R03EXT_A>;
#[doc = "V5 voltage select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum R03EXT_A {
    #[doc = "0: V5 is VSS"]
    R03EXT_0 = 0,
    #[doc = "1: V5 is sourced from the R03 pin"]
    R03EXT_1 = 1,
}
impl From<R03EXT_A> for bool {
    #[inline(always)]
    fn from(variant: R03EXT_A) -> Self {
        variant as u8 != 0
    }
}
impl R03EXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> R03EXT_A {
        match self.bits {
            false => R03EXT_A::R03EXT_0,
            true => R03EXT_A::R03EXT_1,
        }
    }
    #[doc = "Checks if the value of the field is `R03EXT_0`"]
    #[inline(always)]
    pub fn is_r03ext_0(&self) -> bool {
        *self == R03EXT_A::R03EXT_0
    }
    #[doc = "Checks if the value of the field is `R03EXT_1`"]
    #[inline(always)]
    pub fn is_r03ext_1(&self) -> bool {
        *self == R03EXT_A::R03EXT_1
    }
}
#[doc = "Field `R03EXT` writer - V5 voltage select"]
pub type R03EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDVCTL_SPEC, R03EXT_A, O>;
impl<'a, const O: u8> R03EXT_W<'a, O> {
    #[doc = "V5 is VSS"]
    #[inline(always)]
    pub fn r03ext_0(self) -> &'a mut W {
        self.variant(R03EXT_A::R03EXT_0)
    }
    #[doc = "V5 is sourced from the R03 pin"]
    #[inline(always)]
    pub fn r03ext_1(self) -> &'a mut W {
        self.variant(R03EXT_A::R03EXT_1)
    }
}
#[doc = "Field `LCDREXT` reader - V2 to V4 voltage on external Rx3 pins"]
pub type LCDREXT_R = crate::BitReader<LCDREXT_A>;
#[doc = "V2 to V4 voltage on external Rx3 pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDREXT_A {
    #[doc = "0: Internally generated V2 to V4 are not switched to pins (LCDEXTBIAS = 0)"]
    LCDREXT_0 = 0,
    #[doc = "1: Internally generated V2 to V4 are switched to pins (LCDEXTBIAS = 0)"]
    LCDREXT_1 = 1,
}
impl From<LCDREXT_A> for bool {
    #[inline(always)]
    fn from(variant: LCDREXT_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDREXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDREXT_A {
        match self.bits {
            false => LCDREXT_A::LCDREXT_0,
            true => LCDREXT_A::LCDREXT_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDREXT_0`"]
    #[inline(always)]
    pub fn is_lcdrext_0(&self) -> bool {
        *self == LCDREXT_A::LCDREXT_0
    }
    #[doc = "Checks if the value of the field is `LCDREXT_1`"]
    #[inline(always)]
    pub fn is_lcdrext_1(&self) -> bool {
        *self == LCDREXT_A::LCDREXT_1
    }
}
#[doc = "Field `LCDREXT` writer - V2 to V4 voltage on external Rx3 pins"]
pub type LCDREXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDVCTL_SPEC, LCDREXT_A, O>;
impl<'a, const O: u8> LCDREXT_W<'a, O> {
    #[doc = "Internally generated V2 to V4 are not switched to pins (LCDEXTBIAS = 0)"]
    #[inline(always)]
    pub fn lcdrext_0(self) -> &'a mut W {
        self.variant(LCDREXT_A::LCDREXT_0)
    }
    #[doc = "Internally generated V2 to V4 are switched to pins (LCDEXTBIAS = 0)"]
    #[inline(always)]
    pub fn lcdrext_1(self) -> &'a mut W {
        self.variant(LCDREXT_A::LCDREXT_1)
    }
}
impl R {
    #[doc = "Bit 0 - Bias select."]
    #[inline(always)]
    pub fn lcd2b(&self) -> LCD2B_R {
        LCD2B_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - V2 to V4 voltage select"]
    #[inline(always)]
    pub fn lcdextbias(&self) -> LCDEXTBIAS_R {
        LCDEXTBIAS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - V5 voltage select"]
    #[inline(always)]
    pub fn r03ext(&self) -> R03EXT_R {
        R03EXT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - V2 to V4 voltage on external Rx3 pins"]
    #[inline(always)]
    pub fn lcdrext(&self) -> LCDREXT_R {
        LCDREXT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bias select."]
    #[inline(always)]
    pub fn lcd2b(&mut self) -> LCD2B_W<0> {
        LCD2B_W::new(self)
    }
    #[doc = "Bit 5 - V2 to V4 voltage select"]
    #[inline(always)]
    pub fn lcdextbias(&mut self) -> LCDEXTBIAS_W<5> {
        LCDEXTBIAS_W::new(self)
    }
    #[doc = "Bit 6 - V5 voltage select"]
    #[inline(always)]
    pub fn r03ext(&mut self) -> R03EXT_W<6> {
        R03EXT_W::new(self)
    }
    #[doc = "Bit 7 - V2 to V4 voltage on external Rx3 pins"]
    #[inline(always)]
    pub fn lcdrext(&mut self) -> LCDREXT_W<7> {
        LCDREXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_F voltage control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdvctl](index.html) module"]
pub struct LCDVCTL_SPEC;
impl crate::RegisterSpec for LCDVCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdvctl::R](R) reader structure"]
impl crate::Readable for LCDVCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdvctl::W](W) writer structure"]
impl crate::Writable for LCDVCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDVCTL to value 0"]
impl crate::Resettable for LCDVCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
