#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `RSTRX` writer - Reset Receiver"]
pub type RstrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTTX` writer - Reset Transmitter"]
pub type RsttxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTSTA` writer - Reset Status Bits"]
pub type RststaW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rstrx(&mut self) -> RstrxW<CrSpec> {
        RstrxW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn rsttx(&mut self) -> RsttxW<CrSpec> {
        RsttxW::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RxenW<CrSpec> {
        RxenW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RxdisW<CrSpec> {
        RxdisW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TxenW<CrSpec> {
        TxenW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TxdisW<CrSpec> {
        TxdisW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rststa(&mut self) -> RststaW<CrSpec> {
        RststaW::new(self, 8)
    }
}
#[doc = "Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
