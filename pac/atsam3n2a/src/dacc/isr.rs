#[doc = "Register `ISR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<ISR_SPEC>);
#[doc = "Field `TXRDY` reader - Transmission Ready Interrupt Flag"]
pub type TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `ENDTX` reader - End of PDC Interrupt Flag"]
pub type ENDTX_R = crate::BitReader<bool>;
#[doc = "Field `TXBUFE` reader - Buffer Empty Interrupt Flag"]
pub type TXBUFE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Transmission Ready Interrupt Flag"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of PDC Interrupt Flag"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer Empty Interrupt Flag"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
