#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Field `TXRDY` reader - Transmit Ready Interrupt Flag"]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `EOC` reader - End of Conversion Interrupt Flag"]
pub type EOC_R = crate::BitReader;
#[doc = "Field `ENDTX` reader - End of DMA Interrupt Flag"]
pub type ENDTX_R = crate::BitReader;
#[doc = "Field `TXBUFE` reader - Transmit Buffer Empty"]
pub type TXBUFE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit Ready Interrupt Flag"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Flag"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of DMA Interrupt Flag"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Buffer Empty"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: u32 = 0;
}
