#[doc = "Register `PMC_PCDR1` writer"]
pub struct W(crate::W<PMC_PCDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_PCDR1_SPEC>;
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
impl From<crate::W<PMC_PCDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_PCDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID32` writer - Peripheral Clock 32 Disable"]
pub type PID32_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_PCDR1_SPEC, bool, O>;
#[doc = "Field `PID33` writer - Peripheral Clock 33 Disable"]
pub type PID33_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_PCDR1_SPEC, bool, O>;
#[doc = "Field `PID34` writer - Peripheral Clock 34 Disable"]
pub type PID34_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_PCDR1_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Peripheral Clock 32 Disable"]
    #[inline(always)]
    pub fn pid32(&mut self) -> PID32_W<0> {
        PID32_W::new(self)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Disable"]
    #[inline(always)]
    pub fn pid33(&mut self) -> PID33_W<1> {
        PID33_W::new(self)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Disable"]
    #[inline(always)]
    pub fn pid34(&mut self) -> PID34_W<2> {
        PID34_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Disable Register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcdr1](index.html) module"]
pub struct PMC_PCDR1_SPEC;
impl crate::RegisterSpec for PMC_PCDR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_pcdr1::W](W) writer structure"]
impl crate::Writable for PMC_PCDR1_SPEC {
    type Writer = W;
}
