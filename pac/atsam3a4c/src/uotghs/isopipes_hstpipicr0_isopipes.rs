#[doc = "Register `HSTPIPICR0_ISOPIPES` writer"]
pub type W = crate::W<IsopipesHstpipicr0IsopipesSpec>;
#[doc = "Field `RXINIC` writer - Received IN Data Interrupt Clear"]
pub type RxinicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTIC` writer - Transmitted OUT Data Interrupt Clear"]
pub type TxouticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIC` writer - Underflow Interrupt Clear"]
pub type UnderficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDIC` writer - NAKed Interrupt Clear"]
pub type NakedicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OverficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRIC` writer - CRC Error Interrupt Clear"]
pub type CrcerricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIC` writer - Short Packet Interrupt Clear"]
pub type ShortpacketicW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxinic(&mut self) -> RxinicW<IsopipesHstpipicr0IsopipesSpec> {
        RxinicW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txoutic(&mut self) -> TxouticW<IsopipesHstpipicr0IsopipesSpec> {
        TxouticW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn underfic(&mut self) -> UnderficW<IsopipesHstpipicr0IsopipesSpec> {
        UnderficW::new(self, 2)
    }
    #[doc = "Bit 4 - NAKed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakedic(&mut self) -> NakedicW<IsopipesHstpipicr0IsopipesSpec> {
        NakedicW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfic(&mut self) -> OverficW<IsopipesHstpipicr0IsopipesSpec> {
        OverficW::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn crcerric(&mut self) -> CrcerricW<IsopipesHstpipicr0IsopipesSpec> {
        CrcerricW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketic(&mut self) -> ShortpacketicW<IsopipesHstpipicr0IsopipesSpec> {
        ShortpacketicW::new(self, 7)
    }
}
#[doc = "Host Pipe Clear Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isopipes_hstpipicr0_isopipes::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsopipesHstpipicr0IsopipesSpec;
impl crate::RegisterSpec for IsopipesHstpipicr0IsopipesSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isopipes_hstpipicr0_isopipes::W`](W) writer structure"]
impl crate::Writable for IsopipesHstpipicr0IsopipesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
