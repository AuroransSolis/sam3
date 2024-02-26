#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `TXRDY` reader - Transmit Ready Interrupt Mask"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `EOC` reader - End of Conversion Interrupt Mask"]
pub type EocR = crate::BitReader;
#[doc = "Field `ENDTX` reader - End of Transmit Buffer Interrupt Mask"]
pub type EndtxR = crate::BitReader;
#[doc = "Field `TXBUFE` reader - Transmit Buffer Empty Interrupt Mask"]
pub type TxbufeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Mask"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Transmit Buffer Interrupt Mask"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Buffer Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txbufe(&self) -> TxbufeR {
        TxbufeR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
