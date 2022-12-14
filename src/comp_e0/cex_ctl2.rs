#[doc = "Register `CExCTL2` reader"]
pub struct R(crate::R<CEX_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEX_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEX_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEX_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CExCTL2` writer"]
pub struct W(crate::W<CEX_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEX_CTL2_SPEC>;
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
impl From<crate::W<CEX_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEX_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEREF0` reader - Reference resistor tap 0"]
pub type CEREF0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CEREF0` writer - Reference resistor tap 0"]
pub type CEREF0_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CEX_CTL2_SPEC, u8, u8, 5, O>;
#[doc = "Field `CERSEL` reader - Reference select"]
pub type CERSEL_R = crate::BitReader<CERSEL_A>;
#[doc = "Reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CERSEL_A {
    #[doc = "0: When CEEX = 0, VREF is applied to the V+ terminal; When CEEX = 1, VREF is applied to the V- terminal"]
    CERSEL_0 = 0,
    #[doc = "1: When CEEX = 0, VREF is applied to the V- terminal; When CEEX = 1, VREF is applied to the V+ terminal"]
    CERSEL_1 = 1,
}
impl From<CERSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CERSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CERSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERSEL_A {
        match self.bits {
            false => CERSEL_A::CERSEL_0,
            true => CERSEL_A::CERSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CERSEL_0`"]
    #[inline(always)]
    pub fn is_cersel_0(&self) -> bool {
        *self == CERSEL_A::CERSEL_0
    }
    #[doc = "Checks if the value of the field is `CERSEL_1`"]
    #[inline(always)]
    pub fn is_cersel_1(&self) -> bool {
        *self == CERSEL_A::CERSEL_1
    }
}
#[doc = "Field `CERSEL` writer - Reference select"]
pub type CERSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEX_CTL2_SPEC, CERSEL_A, O>;
impl<'a, const O: u8> CERSEL_W<'a, O> {
    #[doc = "When CEEX = 0, VREF is applied to the V+ terminal; When CEEX = 1, VREF is applied to the V- terminal"]
    #[inline(always)]
    pub fn cersel_0(self) -> &'a mut W {
        self.variant(CERSEL_A::CERSEL_0)
    }
    #[doc = "When CEEX = 0, VREF is applied to the V- terminal; When CEEX = 1, VREF is applied to the V+ terminal"]
    #[inline(always)]
    pub fn cersel_1(self) -> &'a mut W {
        self.variant(CERSEL_A::CERSEL_1)
    }
}
#[doc = "Field `CERS` reader - Reference source"]
pub type CERS_R = crate::FieldReader<u8, CERS_A>;
#[doc = "Reference source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CERS_A {
    #[doc = "0: No current is drawn by the reference circuitry"]
    CERS_0 = 0,
    #[doc = "1: VCC applied to the resistor ladder"]
    CERS_1 = 1,
    #[doc = "2: Shared reference voltage applied to the resistor ladder"]
    CERS_2 = 2,
    #[doc = "3: Shared reference voltage supplied to V(CREF). Resistor ladder is off"]
    CERS_3 = 3,
}
impl From<CERS_A> for u8 {
    #[inline(always)]
    fn from(variant: CERS_A) -> Self {
        variant as _
    }
}
impl CERS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERS_A {
        match self.bits {
            0 => CERS_A::CERS_0,
            1 => CERS_A::CERS_1,
            2 => CERS_A::CERS_2,
            3 => CERS_A::CERS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CERS_0`"]
    #[inline(always)]
    pub fn is_cers_0(&self) -> bool {
        *self == CERS_A::CERS_0
    }
    #[doc = "Checks if the value of the field is `CERS_1`"]
    #[inline(always)]
    pub fn is_cers_1(&self) -> bool {
        *self == CERS_A::CERS_1
    }
    #[doc = "Checks if the value of the field is `CERS_2`"]
    #[inline(always)]
    pub fn is_cers_2(&self) -> bool {
        *self == CERS_A::CERS_2
    }
    #[doc = "Checks if the value of the field is `CERS_3`"]
    #[inline(always)]
    pub fn is_cers_3(&self) -> bool {
        *self == CERS_A::CERS_3
    }
}
#[doc = "Field `CERS` writer - Reference source"]
pub type CERS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CEX_CTL2_SPEC, u8, CERS_A, 2, O>;
impl<'a, const O: u8> CERS_W<'a, O> {
    #[doc = "No current is drawn by the reference circuitry"]
    #[inline(always)]
    pub fn cers_0(self) -> &'a mut W {
        self.variant(CERS_A::CERS_0)
    }
    #[doc = "VCC applied to the resistor ladder"]
    #[inline(always)]
    pub fn cers_1(self) -> &'a mut W {
        self.variant(CERS_A::CERS_1)
    }
    #[doc = "Shared reference voltage applied to the resistor ladder"]
    #[inline(always)]
    pub fn cers_2(self) -> &'a mut W {
        self.variant(CERS_A::CERS_2)
    }
    #[doc = "Shared reference voltage supplied to V(CREF). Resistor ladder is off"]
    #[inline(always)]
    pub fn cers_3(self) -> &'a mut W {
        self.variant(CERS_A::CERS_3)
    }
}
#[doc = "Field `CEREF1` reader - Reference resistor tap 1"]
pub type CEREF1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CEREF1` writer - Reference resistor tap 1"]
pub type CEREF1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CEX_CTL2_SPEC, u8, u8, 5, O>;
#[doc = "Field `CEREFL` reader - Reference voltage level"]
pub type CEREFL_R = crate::FieldReader<u8, CEREFL_A>;
#[doc = "Reference voltage level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CEREFL_A {
    #[doc = "0: Reference amplifier is disabled. No reference voltage is requested"]
    CEREFL_0 = 0,
    #[doc = "1: 1.2 V is selected as shared reference voltage input"]
    CEREFL_1 = 1,
    #[doc = "2: 2.0 V is selected as shared reference voltage input"]
    CEREFL_2 = 2,
    #[doc = "3: 2.5 V is selected as shared reference voltage input"]
    CEREFL_3 = 3,
}
impl From<CEREFL_A> for u8 {
    #[inline(always)]
    fn from(variant: CEREFL_A) -> Self {
        variant as _
    }
}
impl CEREFL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEREFL_A {
        match self.bits {
            0 => CEREFL_A::CEREFL_0,
            1 => CEREFL_A::CEREFL_1,
            2 => CEREFL_A::CEREFL_2,
            3 => CEREFL_A::CEREFL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CEREFL_0`"]
    #[inline(always)]
    pub fn is_cerefl_0(&self) -> bool {
        *self == CEREFL_A::CEREFL_0
    }
    #[doc = "Checks if the value of the field is `CEREFL_1`"]
    #[inline(always)]
    pub fn is_cerefl_1(&self) -> bool {
        *self == CEREFL_A::CEREFL_1
    }
    #[doc = "Checks if the value of the field is `CEREFL_2`"]
    #[inline(always)]
    pub fn is_cerefl_2(&self) -> bool {
        *self == CEREFL_A::CEREFL_2
    }
    #[doc = "Checks if the value of the field is `CEREFL_3`"]
    #[inline(always)]
    pub fn is_cerefl_3(&self) -> bool {
        *self == CEREFL_A::CEREFL_3
    }
}
#[doc = "Field `CEREFL` writer - Reference voltage level"]
pub type CEREFL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CEX_CTL2_SPEC, u8, CEREFL_A, 2, O>;
impl<'a, const O: u8> CEREFL_W<'a, O> {
    #[doc = "Reference amplifier is disabled. No reference voltage is requested"]
    #[inline(always)]
    pub fn cerefl_0(self) -> &'a mut W {
        self.variant(CEREFL_A::CEREFL_0)
    }
    #[doc = "1.2 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn cerefl_1(self) -> &'a mut W {
        self.variant(CEREFL_A::CEREFL_1)
    }
    #[doc = "2.0 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn cerefl_2(self) -> &'a mut W {
        self.variant(CEREFL_A::CEREFL_2)
    }
    #[doc = "2.5 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn cerefl_3(self) -> &'a mut W {
        self.variant(CEREFL_A::CEREFL_3)
    }
}
#[doc = "Field `CEREFACC` reader - Reference accuracy"]
pub type CEREFACC_R = crate::BitReader<CEREFACC_A>;
#[doc = "Reference accuracy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEREFACC_A {
    #[doc = "0: Static mode"]
    CEREFACC_0 = 0,
    #[doc = "1: Clocked (low power, low accuracy) mode"]
    CEREFACC_1 = 1,
}
impl From<CEREFACC_A> for bool {
    #[inline(always)]
    fn from(variant: CEREFACC_A) -> Self {
        variant as u8 != 0
    }
}
impl CEREFACC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEREFACC_A {
        match self.bits {
            false => CEREFACC_A::CEREFACC_0,
            true => CEREFACC_A::CEREFACC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEREFACC_0`"]
    #[inline(always)]
    pub fn is_cerefacc_0(&self) -> bool {
        *self == CEREFACC_A::CEREFACC_0
    }
    #[doc = "Checks if the value of the field is `CEREFACC_1`"]
    #[inline(always)]
    pub fn is_cerefacc_1(&self) -> bool {
        *self == CEREFACC_A::CEREFACC_1
    }
}
#[doc = "Field `CEREFACC` writer - Reference accuracy"]
pub type CEREFACC_W<'a, const O: u8> = crate::BitWriter<'a, u16, CEX_CTL2_SPEC, CEREFACC_A, O>;
impl<'a, const O: u8> CEREFACC_W<'a, O> {
    #[doc = "Static mode"]
    #[inline(always)]
    pub fn cerefacc_0(self) -> &'a mut W {
        self.variant(CEREFACC_A::CEREFACC_0)
    }
    #[doc = "Clocked (low power, low accuracy) mode"]
    #[inline(always)]
    pub fn cerefacc_1(self) -> &'a mut W {
        self.variant(CEREFACC_A::CEREFACC_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Reference resistor tap 0"]
    #[inline(always)]
    pub fn ceref0(&self) -> CEREF0_R {
        CEREF0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Reference select"]
    #[inline(always)]
    pub fn cersel(&self) -> CERSEL_R {
        CERSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reference source"]
    #[inline(always)]
    pub fn cers(&self) -> CERS_R {
        CERS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Reference resistor tap 1"]
    #[inline(always)]
    pub fn ceref1(&self) -> CEREF1_R {
        CEREF1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Reference voltage level"]
    #[inline(always)]
    pub fn cerefl(&self) -> CEREFL_R {
        CEREFL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Reference accuracy"]
    #[inline(always)]
    pub fn cerefacc(&self) -> CEREFACC_R {
        CEREFACC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reference resistor tap 0"]
    #[inline(always)]
    pub fn ceref0(&mut self) -> CEREF0_W<0> {
        CEREF0_W::new(self)
    }
    #[doc = "Bit 5 - Reference select"]
    #[inline(always)]
    pub fn cersel(&mut self) -> CERSEL_W<5> {
        CERSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - Reference source"]
    #[inline(always)]
    pub fn cers(&mut self) -> CERS_W<6> {
        CERS_W::new(self)
    }
    #[doc = "Bits 8:12 - Reference resistor tap 1"]
    #[inline(always)]
    pub fn ceref1(&mut self) -> CEREF1_W<8> {
        CEREF1_W::new(self)
    }
    #[doc = "Bits 13:14 - Reference voltage level"]
    #[inline(always)]
    pub fn cerefl(&mut self) -> CEREFL_W<13> {
        CEREFL_W::new(self)
    }
    #[doc = "Bit 15 - Reference accuracy"]
    #[inline(always)]
    pub fn cerefacc(&mut self) -> CEREFACC_W<15> {
        CEREFACC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cex_ctl2](index.html) module"]
pub struct CEX_CTL2_SPEC;
impl crate::RegisterSpec for CEX_CTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cex_ctl2::R](R) reader structure"]
impl crate::Readable for CEX_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cex_ctl2::W](W) writer structure"]
impl crate::Writable for CEX_CTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CExCTL2 to value 0"]
impl crate::Resettable for CEX_CTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
