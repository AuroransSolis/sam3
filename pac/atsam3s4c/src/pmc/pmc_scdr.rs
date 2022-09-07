#[doc = "Register `PMC_SCDR` writer"]
pub struct W(crate::W<PMC_SCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_SCDR_SPEC>;
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
impl From<crate::W<PMC_SCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_SCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDP` writer - USB Device Port Clock Disable"]
pub type UDP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SCDR_SPEC, bool, O>;
#[doc = "Field `PCK0` writer - Programmable Clock 0 Output Disable"]
pub type PCK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SCDR_SPEC, bool, O>;
#[doc = "Field `PCK1` writer - Programmable Clock 1 Output Disable"]
pub type PCK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SCDR_SPEC, bool, O>;
#[doc = "Field `PCK2` writer - Programmable Clock 2 Output Disable"]
pub type PCK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SCDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 7 - USB Device Port Clock Disable"]
    #[inline(always)]
    pub fn udp(&mut self) -> UDP_W<7> {
        UDP_W::new(self)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Disable"]
    #[inline(always)]
    pub fn pck0(&mut self) -> PCK0_W<8> {
        PCK0_W::new(self)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Disable"]
    #[inline(always)]
    pub fn pck1(&mut self) -> PCK1_W<9> {
        PCK1_W::new(self)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Disable"]
    #[inline(always)]
    pub fn pck2(&mut self) -> PCK2_W<10> {
        PCK2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_scdr](index.html) module"]
pub struct PMC_SCDR_SPEC;
impl crate::RegisterSpec for PMC_SCDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_scdr::W](W) writer structure"]
impl crate::Writable for PMC_SCDR_SPEC {
    type Writer = W;
}
