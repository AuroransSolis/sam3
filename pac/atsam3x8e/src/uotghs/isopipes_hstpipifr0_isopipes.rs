#[doc = "Register `HSTPIPIFR0_ISOPIPES` writer"]
pub type W = crate::W<IsopipesHstpipifr0IsopipesSpec>;
#[doc = "Field `RXINIS` writer - Received IN Data Interrupt Set"]
pub type RxinisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTIS` writer - Transmitted OUT Data Interrupt Set"]
pub type TxoutisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIS` writer - Underflow Interrupt Set"]
pub type UnderfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRIS` writer - Pipe Error Interrupt Set"]
pub type PerrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDIS` writer - NAKed Interrupt Set"]
pub type NakedisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OverfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRIS` writer - CRC Error Interrupt Set"]
pub type CrcerrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIS` writer - Short Packet Interrupt Set"]
pub type ShortpacketisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Set"]
pub type NbusybksW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxinis(&mut self) -> RxinisW<IsopipesHstpipifr0IsopipesSpec> {
        RxinisW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txoutis(&mut self) -> TxoutisW<IsopipesHstpipifr0IsopipesSpec> {
        TxoutisW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn underfis(&mut self) -> UnderfisW<IsopipesHstpipifr0IsopipesSpec> {
        UnderfisW::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn perris(&mut self) -> PerrisW<IsopipesHstpipifr0IsopipesSpec> {
        PerrisW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakedis(&mut self) -> NakedisW<IsopipesHstpipifr0IsopipesSpec> {
        NakedisW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn overfis(&mut self) -> OverfisW<IsopipesHstpipifr0IsopipesSpec> {
        OverfisW::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn crcerris(&mut self) -> CrcerrisW<IsopipesHstpipifr0IsopipesSpec> {
        CrcerrisW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketis(&mut self) -> ShortpacketisW<IsopipesHstpipifr0IsopipesSpec> {
        ShortpacketisW::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybks(&mut self) -> NbusybksW<IsopipesHstpipifr0IsopipesSpec> {
        NbusybksW::new(self, 12)
    }
}
#[doc = "Host Pipe Set Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isopipes_hstpipifr0_isopipes::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsopipesHstpipifr0IsopipesSpec;
impl crate::RegisterSpec for IsopipesHstpipifr0IsopipesSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isopipes_hstpipifr0_isopipes::W`](W) writer structure"]
impl crate::Writable for IsopipesHstpipifr0IsopipesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
