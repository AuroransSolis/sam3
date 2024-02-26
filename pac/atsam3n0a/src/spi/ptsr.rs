#[doc = "Register `PTSR` reader"]
pub type R = crate::R<PtsrSpec>;
#[doc = "Field `RXTEN` reader - Receiver Transfer Enable"]
pub type RxtenR = crate::BitReader;
#[doc = "Field `TXTEN` reader - Transmitter Transfer Enable"]
pub type TxtenR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receiver Transfer Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RxtenR {
        RxtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Transmitter Transfer Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TxtenR {
        TxtenR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Transfer Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtsrSpec;
impl crate::RegisterSpec for PtsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptsr::R`](R) reader structure"]
impl crate::Readable for PtsrSpec {}
#[doc = "`reset()` method sets PTSR to value 0"]
impl crate::Resettable for PtsrSpec {
    const RESET_VALUE: u32 = 0;
}
