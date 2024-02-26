#[doc = "Register `PTCR` writer"]
pub type W = crate::W<PtcrSpec>;
#[doc = "Field `RXTEN` writer - Receiver Transfer Enable"]
pub type RxtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTDIS` writer - Receiver Transfer Disable"]
pub type RxtdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTEN` writer - Transmitter Transfer Enable"]
pub type TxtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTDIS` writer - Transmitter Transfer Disable"]
pub type TxtdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Receiver Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxten(&mut self) -> RxtenW<PtcrSpec> {
        RxtenW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxtdis(&mut self) -> RxtdisW<PtcrSpec> {
        RxtdisW::new(self, 1)
    }
    #[doc = "Bit 8 - Transmitter Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txten(&mut self) -> TxtenW<PtcrSpec> {
        TxtenW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmitter Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txtdis(&mut self) -> TxtdisW<PtcrSpec> {
        TxtdisW::new(self, 9)
    }
}
#[doc = "Transfer Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtcrSpec;
impl crate::RegisterSpec for PtcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ptcr::W`](W) writer structure"]
impl crate::Writable for PtcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTCR to value 0"]
impl crate::Resettable for PtcrSpec {
    const RESET_VALUE: u32 = 0;
}
