#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `TXRDY` reader - Transmission Ready Interrupt Flag"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `ENDTX` reader - End of PDC Interrupt Flag"]
pub type EndtxR = crate::BitReader;
#[doc = "Field `TXBUFE` reader - Buffer Empty Interrupt Flag"]
pub type TxbufeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission Ready Interrupt Flag"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of PDC Interrupt Flag"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer Empty Interrupt Flag"]
    #[inline(always)]
    pub fn txbufe(&self) -> TxbufeR {
        TxbufeR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
