#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Field `TXRDY` reader - Transmission Ready Interrupt Mask"]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `ENDTX` reader - End of PDC Interrupt Mask"]
pub type ENDTX_R = crate::BitReader;
#[doc = "Field `TXBUFE` reader - Buffer Empty Interrupt Mask"]
pub type TXBUFE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of PDC Interrupt Mask"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
