#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `NFCEN` writer - NAND Flash Controller Enable"]
pub type NfcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFCDIS` writer - NAND Flash Controller Disable"]
pub type NfcdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - NAND Flash Controller Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfcen(&mut self) -> NfcenW<CtrlSpec> {
        NfcenW::new(self, 0)
    }
    #[doc = "Bit 1 - NAND Flash Controller Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nfcdis(&mut self) -> NfcdisW<CtrlSpec> {
        NfcdisW::new(self, 1)
    }
}
#[doc = "SMC NFC Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
