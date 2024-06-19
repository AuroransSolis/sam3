#[doc = "Register `TXVC` reader"]
pub type R = crate::R<TxvcSpec>;
#[doc = "Register `TXVC` writer"]
pub type W = crate::W<TxvcSpec>;
#[doc = "Field `TXVDIS` reader - Transceiver Disable"]
pub type TxvdisR = crate::BitReader;
#[doc = "Field `TXVDIS` writer - Transceiver Disable"]
pub type TxvdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUON` reader - Pull-up On"]
pub type PuonR = crate::BitReader;
#[doc = "Field `PUON` writer - Pull-up On"]
pub type PuonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Transceiver Disable"]
    #[inline(always)]
    pub fn txvdis(&self) -> TxvdisR {
        TxvdisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pull-up On"]
    #[inline(always)]
    pub fn puon(&self) -> PuonR {
        PuonR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Transceiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txvdis(&mut self) -> TxvdisW<TxvcSpec> {
        TxvdisW::new(self, 8)
    }
    #[doc = "Bit 9 - Pull-up On"]
    #[inline(always)]
    #[must_use]
    pub fn puon(&mut self) -> PuonW<TxvcSpec> {
        PuonW::new(self, 9)
    }
}
#[doc = "Transceiver Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txvc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txvc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxvcSpec;
impl crate::RegisterSpec for TxvcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txvc::R`](R) reader structure"]
impl crate::Readable for TxvcSpec {}
#[doc = "`write(|w| ..)` method takes [`txvc::W`](W) writer structure"]
impl crate::Writable for TxvcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXVC to value 0x0100"]
impl crate::Resettable for TxvcSpec {
    const RESET_VALUE: u32 = 0x0100;
}
