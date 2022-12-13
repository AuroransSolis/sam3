#[doc = "Register `PMC_SCSR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<PMC_SCSR_SPEC>);
#[doc = "Field `UDP` reader - USB Device Port Clock Status"]
pub type UDP_R = crate::BitReader<bool>;
#[doc = "Field `PCK0` reader - Programmable Clock 0 Output Status"]
pub type PCK0_R = crate::BitReader<bool>;
#[doc = "Field `PCK1` reader - Programmable Clock 1 Output Status"]
pub type PCK1_R = crate::BitReader<bool>;
#[doc = "Field `PCK2` reader - Programmable Clock 2 Output Status"]
pub type PCK2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 7 - USB Device Port Clock Status"]
    #[inline(always)]
    pub fn udp(&self) -> UDP_R {
        UDP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Status"]
    #[inline(always)]
    pub fn pck0(&self) -> PCK0_R {
        PCK0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Status"]
    #[inline(always)]
    pub fn pck1(&self) -> PCK1_R {
        PCK1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Status"]
    #[inline(always)]
    pub fn pck2(&self) -> PCK2_R {
        PCK2_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "System Clock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_scsr](index.html) module"]
pub struct PMC_SCSR_SPEC;
impl crate::RegisterSpec for PMC_SCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_scsr::R](R) reader structure"]
impl crate::Readable for PMC_SCSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PMC_SCSR to value 0x01"]
impl crate::Resettable for PMC_SCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
