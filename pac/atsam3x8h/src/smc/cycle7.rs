#[doc = "Register `CYCLE7` reader"]
pub struct R(crate::R<CYCLE7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CYCLE7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CYCLE7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CYCLE7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CYCLE7` writer"]
pub struct W(crate::W<CYCLE7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CYCLE7_SPEC>;
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
impl From<crate::W<CYCLE7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CYCLE7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NWE_CYCLE` reader - Total Write Cycle Length"]
pub type NWE_CYCLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NWE_CYCLE` writer - Total Write Cycle Length"]
pub type NWE_CYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CYCLE7_SPEC, u16, u16, 9, O>;
#[doc = "Field `NRD_CYCLE` reader - Total Read Cycle Length"]
pub type NRD_CYCLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NRD_CYCLE` writer - Total Read Cycle Length"]
pub type NRD_CYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CYCLE7_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Total Write Cycle Length"]
    #[inline(always)]
    pub fn nwe_cycle(&self) -> NWE_CYCLE_R {
        NWE_CYCLE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Total Read Cycle Length"]
    #[inline(always)]
    pub fn nrd_cycle(&self) -> NRD_CYCLE_R {
        NRD_CYCLE_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Total Write Cycle Length"]
    #[inline(always)]
    pub fn nwe_cycle(&mut self) -> NWE_CYCLE_W<0> {
        NWE_CYCLE_W::new(self)
    }
    #[doc = "Bits 16:24 - Total Read Cycle Length"]
    #[inline(always)]
    pub fn nrd_cycle(&mut self) -> NRD_CYCLE_W<16> {
        NRD_CYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Cycle Register (CS_number = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle7](index.html) module"]
pub struct CYCLE7_SPEC;
impl crate::RegisterSpec for CYCLE7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cycle7::R](R) reader structure"]
impl crate::Readable for CYCLE7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cycle7::W](W) writer structure"]
impl crate::Writable for CYCLE7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CYCLE7 to value 0x0003_0003"]
impl crate::Resettable for CYCLE7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0003
    }
}
