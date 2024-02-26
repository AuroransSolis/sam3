#[doc = "Register `DEVEPTIDR0_ISOENPT` writer"]
pub type W = crate::W<IsoenptDeveptidr0IsoenptSpec>;
#[doc = "Field `TXINEC` writer - Transmitted IN Interrupt Clear"]
pub type TxinecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTEC` writer - Received OUT Data Interrupt Clear"]
pub type RxoutecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFEC` writer - Underflow Interrupt Clear"]
pub type UnderfecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOINERREC` writer - High Bandwidth Isochronous IN Error Interrupt Clear"]
pub type HbisoinerrecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOFLUSHEC` writer - High Bandwidth Isochronous IN Flush Interrupt Clear"]
pub type HbisoflushecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFEC` writer - Overflow Interrupt Clear"]
pub type OverfecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERREC` writer - CRC Error Interrupt Clear"]
pub type CrcerrecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETEC` writer - Shortpacket Interrupt Clear"]
pub type ShortpacketecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDATEC` writer - MData Interrupt Clear"]
pub type MdatecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAXEC` writer - DataX Interrupt Clear"]
pub type DataxecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORTRANSEC` writer - Transaction Error Interrupt Clear"]
pub type ErrortransecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Interrupt Clear"]
pub type NbusybkecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Clear"]
pub type FifoconcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISHDMAC` writer - Endpoint Interrupts Disable HDMA Request Clear"]
pub type EpdishdmacW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinec(&mut self) -> TxinecW<IsoenptDeveptidr0IsoenptSpec> {
        TxinecW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutec(&mut self) -> RxoutecW<IsoenptDeveptidr0IsoenptSpec> {
        RxoutecW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn underfec(&mut self) -> UnderfecW<IsoenptDeveptidr0IsoenptSpec> {
        UnderfecW::new(self, 2)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoinerrec(&mut self) -> HbisoinerrecW<IsoenptDeveptidr0IsoenptSpec> {
        HbisoinerrecW::new(self, 3)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoflushec(&mut self) -> HbisoflushecW<IsoenptDeveptidr0IsoenptSpec> {
        HbisoflushecW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfec(&mut self) -> OverfecW<IsoenptDeveptidr0IsoenptSpec> {
        OverfecW::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn crcerrec(&mut self) -> CrcerrecW<IsoenptDeveptidr0IsoenptSpec> {
        CrcerrecW::new(self, 6)
    }
    #[doc = "Bit 7 - Shortpacket Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketec(&mut self) -> ShortpacketecW<IsoenptDeveptidr0IsoenptSpec> {
        ShortpacketecW::new(self, 7)
    }
    #[doc = "Bit 8 - MData Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mdatec(&mut self) -> MdatecW<IsoenptDeveptidr0IsoenptSpec> {
        MdatecW::new(self, 8)
    }
    #[doc = "Bit 9 - DataX Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dataxec(&mut self) -> DataxecW<IsoenptDeveptidr0IsoenptSpec> {
        DataxecW::new(self, 9)
    }
    #[doc = "Bit 10 - Transaction Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn errortransec(&mut self) -> ErrortransecW<IsoenptDeveptidr0IsoenptSpec> {
        ErrortransecW::new(self, 10)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkec(&mut self) -> NbusybkecW<IsoenptDeveptidr0IsoenptSpec> {
        NbusybkecW::new(self, 12)
    }
    #[doc = "Bit 14 - FIFO Control Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoconc(&mut self) -> FifoconcW<IsoenptDeveptidr0IsoenptSpec> {
        FifoconcW::new(self, 14)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn epdishdmac(&mut self) -> EpdishdmacW<IsoenptDeveptidr0IsoenptSpec> {
        EpdishdmacW::new(self, 16)
    }
}
#[doc = "Device Endpoint Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoenpt_deveptidr0_isoenpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoenptDeveptidr0IsoenptSpec;
impl crate::RegisterSpec for IsoenptDeveptidr0IsoenptSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoenpt_deveptidr0_isoenpt::W`](W) writer structure"]
impl crate::Writable for IsoenptDeveptidr0IsoenptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
