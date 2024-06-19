#[doc = "Register `THR` writer"]
pub type W = crate::W<ThrSpec>;
#[doc = "Field `TXCHR` writer - Character to be Transmitted"]
pub type TxchrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TXSYNH` writer - Sync Field to be Transmitted"]
pub type TxsynhW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:8 - Character to be Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn txchr(&mut self) -> TxchrW<ThrSpec> {
        TxchrW::new(self, 0)
    }
    #[doc = "Bit 15 - Sync Field to be Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn txsynh(&mut self) -> TxsynhW<ThrSpec> {
        TxsynhW::new(self, 15)
    }
}
#[doc = "Transmit Holding Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
