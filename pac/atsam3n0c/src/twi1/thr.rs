#[doc = "Register `THR` writer"]
pub type W = crate::W<ThrSpec>;
#[doc = "Field `TXDATA` writer - Master or Slave Transmit Holding Data"]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Master or Slave Transmit Holding Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TxdataW<ThrSpec> {
        TxdataW::new(self, 0)
    }
}
#[doc = "Transmit Holding Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThrSpec;
impl crate::RegisterSpec for ThrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`thr::W`](W) writer structure"]
impl crate::Writable for ThrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets THR to value 0"]
impl crate::Resettable for ThrSpec {
    const RESET_VALUE: u32 = 0;
}
