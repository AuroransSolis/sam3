#[doc = "Register `EMR` reader"]
pub type R = crate::R<EMR_SPEC>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EMR_SPEC>;
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub type CMPMODE_R = crate::FieldReader<CMPMODE_A>;
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPMODE_A {
    #[doc = "0: Generates an event when the converted data is lower than the low threshold of the window."]
    Low = 0,
    #[doc = "1: Generates an event when the converted data is higher than the high threshold of the window."]
    High = 1,
    #[doc = "2: Generates an event when the converted data is in the comparison window."]
    In = 2,
    #[doc = "3: Generates an event when the converted data is out of the comparison window."]
    Out = 3,
}
impl From<CMPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMPMODE_A {
    type Ux = u8;
}
impl CMPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMODE_A {
        match self.bits {
            0 => CMPMODE_A::Low,
            1 => CMPMODE_A::High,
            2 => CMPMODE_A::In,
            3 => CMPMODE_A::Out,
            _ => unreachable!(),
        }
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CMPMODE_A::Low
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CMPMODE_A::High
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == CMPMODE_A::In
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == CMPMODE_A::Out
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub type CMPMODE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CMPMODE_A>;
impl<'a, REG, const O: u8> CMPMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMODE_A::Low)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMODE_A::High)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMODE_A::In)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMODE_A::Out)
    }
}
#[doc = "Field `CMPSEL` reader - Comparison Selected Channel"]
pub type CMPSEL_R = crate::FieldReader;
#[doc = "Field `CMPSEL` writer - Comparison Selected Channel"]
pub type CMPSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CMPALL` reader - Compare All Channels"]
pub type CMPALL_R = crate::BitReader;
#[doc = "Field `CMPALL` writer - Compare All Channels"]
pub type CMPALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPFILTER` reader - Compare Event Filtering"]
pub type CMPFILTER_R = crate::FieldReader;
#[doc = "Field `CMPFILTER` writer - Compare Event Filtering"]
pub type CMPFILTER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TAG` reader - TAG of ADC_LDCR register"]
pub type TAG_R = crate::BitReader;
#[doc = "Field `TAG` writer - TAG of ADC_LDCR register"]
pub type TAG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CMPSEL_R {
        CMPSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&self) -> CMPALL_R {
        CMPALL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&self) -> CMPFILTER_R {
        CMPFILTER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 24 - TAG of ADC_LDCR register"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmode(&mut self) -> CMPMODE_W<EMR_SPEC, 0> {
        CMPMODE_W::new(self)
    }
    #[doc = "Bits 4:7 - Comparison Selected Channel"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsel(&mut self) -> CMPSEL_W<EMR_SPEC, 4> {
        CMPSEL_W::new(self)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    #[must_use]
    pub fn cmpall(&mut self) -> CMPALL_W<EMR_SPEC, 9> {
        CMPALL_W::new(self)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn cmpfilter(&mut self) -> CMPFILTER_W<EMR_SPEC, 12> {
        CMPFILTER_W::new(self)
    }
    #[doc = "Bit 24 - TAG of ADC_LDCR register"]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TAG_W<EMR_SPEC, 24> {
        TAG_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Extended Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMR_SPEC;
impl crate::RegisterSpec for EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
