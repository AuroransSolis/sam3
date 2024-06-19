#[doc = "Register `DEVEPTIFR0_ISOENPT` writer"]
pub type W = crate::W<IsoenptDeveptifr0IsoenptSpec>;
#[doc = "Field `TXINIS` writer - Transmitted IN Data Interrupt Set"]
pub type TxinisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTIS` writer - Received OUT Data Interrupt Set"]
pub type RxoutisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIS` writer - Underflow Interrupt Set"]
pub type UnderfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOINERRIS` writer - High bandwidth isochronous IN Underflow Error Interrupt Set"]
pub type HbisoinerrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOFLUSHIS` writer - High Bandwidth Isochronous IN Flush Interrupt Set"]
pub type HbisoflushisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OverfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRIS` writer - CRC Error Interrupt Set"]
pub type CrcerrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETS` writer - Short Packet Interrupt Set"]
pub type ShortpacketsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Interrupt Set"]
pub type NbusybksW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txinis(&mut self) -> TxinisW<IsoenptDeveptifr0IsoenptSpec> {
        TxinisW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutis(&mut self) -> RxoutisW<IsoenptDeveptifr0IsoenptSpec> {
        RxoutisW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn underfis(&mut self) -> UnderfisW<IsoenptDeveptifr0IsoenptSpec> {
        UnderfisW::new(self, 2)
    }
    #[doc = "Bit 3 - High bandwidth isochronous IN Underflow Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoinerris(&mut self) -> HbisoinerrisW<IsoenptDeveptifr0IsoenptSpec> {
        HbisoinerrisW::new(self, 3)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoflushis(&mut self) -> HbisoflushisW<IsoenptDeveptifr0IsoenptSpec> {
        HbisoflushisW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn overfis(&mut self) -> OverfisW<IsoenptDeveptifr0IsoenptSpec> {
        OverfisW::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn crcerris(&mut self) -> CrcerrisW<IsoenptDeveptifr0IsoenptSpec> {
        CrcerrisW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn shortpackets(&mut self) -> ShortpacketsW<IsoenptDeveptifr0IsoenptSpec> {
        ShortpacketsW::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybks(&mut self) -> NbusybksW<IsoenptDeveptifr0IsoenptSpec> {
        NbusybksW::new(self, 12)
    }
}
#[doc = "Device Endpoint Set Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoenpt_deveptifr0_isoenpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoenptDeveptifr0IsoenptSpec;
impl crate::RegisterSpec for IsoenptDeveptifr0IsoenptSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoenpt_deveptifr0_isoenpt::W`](W) writer structure"]
impl crate::Writable for IsoenptDeveptifr0IsoenptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
