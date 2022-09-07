#[doc = "Register `PMC_FOCR` writer"]
pub struct W(crate::W<PMC_FOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_FOCR_SPEC>;
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
impl From<crate::W<PMC_FOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_FOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FOCLR` writer - Fault Output Clear"]
pub type FOCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_FOCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Fault Output Clear"]
    #[inline(always)]
    pub fn foclr(&mut self) -> FOCLR_W<0> {
        FOCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Output Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_focr](index.html) module"]
pub struct PMC_FOCR_SPEC;
impl crate::RegisterSpec for PMC_FOCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_focr::W](W) writer structure"]
impl crate::Writable for PMC_FOCR_SPEC {
    type Writer = W;
}
