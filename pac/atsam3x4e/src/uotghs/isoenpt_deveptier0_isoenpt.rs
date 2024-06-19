#[doc = "Register `DEVEPTIER0_ISOENPT` writer"]
pub type W = crate::W<IsoenptDeveptier0IsoenptSpec>;
#[doc = "Field `TXINES` writer - Transmitted IN Data Interrupt Enable"]
pub type TxinesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTES` writer - Received OUT Data Interrupt Enable"]
pub type RxoutesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFES` writer - Underflow Interrupt Enable"]
pub type UnderfesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOINERRES` writer - High Bandwidth Isochronous IN Error Interrupt Enable"]
pub type HbisoinerresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOFLUSHES` writer - High Bandwidth Isochronous IN Flush Interrupt Enable"]
pub type HbisoflushesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFES` writer - Overflow Interrupt Enable"]
pub type OverfesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRES` writer - CRC Error Interrupt Enable"]
pub type CrcerresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETES` writer - Short Packet Interrupt Enable"]
pub type ShortpacketesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDATAES` writer - MData Interrupt Enable"]
pub type MdataesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAXES` writer - DataX Interrupt Enable"]
pub type DataxesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORTRANSES` writer - Transaction Error Interrupt Enable"]
pub type ErrortransesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Interrupt Enable"]
pub type NbusybkesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KILLBKS` writer - Kill IN Bank"]
pub type KillbksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONS` writer - FIFO Control"]
pub type FifoconsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISHDMAS` writer - Endpoint Interrupts Disable HDMA Request Enable"]
pub type EpdishdmasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTDTS` writer - Reset Data Toggle Enable"]
pub type RstdtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLRQS` writer - STALL Request Enable"]
pub type StallrqsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txines(&mut self) -> TxinesW<IsoenptDeveptier0IsoenptSpec> {
        TxinesW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutes(&mut self) -> RxoutesW<IsoenptDeveptier0IsoenptSpec> {
        RxoutesW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn underfes(&mut self) -> UnderfesW<IsoenptDeveptier0IsoenptSpec> {
        UnderfesW::new(self, 2)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoinerres(&mut self) -> HbisoinerresW<IsoenptDeveptier0IsoenptSpec> {
        HbisoinerresW::new(self, 3)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoflushes(&mut self) -> HbisoflushesW<IsoenptDeveptier0IsoenptSpec> {
        HbisoflushesW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overfes(&mut self) -> OverfesW<IsoenptDeveptier0IsoenptSpec> {
        OverfesW::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcerres(&mut self) -> CrcerresW<IsoenptDeveptier0IsoenptSpec> {
        CrcerresW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketes(&mut self) -> ShortpacketesW<IsoenptDeveptier0IsoenptSpec> {
        ShortpacketesW::new(self, 7)
    }
    #[doc = "Bit 8 - MData Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdataes(&mut self) -> MdataesW<IsoenptDeveptier0IsoenptSpec> {
        MdataesW::new(self, 8)
    }
    #[doc = "Bit 9 - DataX Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dataxes(&mut self) -> DataxesW<IsoenptDeveptier0IsoenptSpec> {
        DataxesW::new(self, 9)
    }
    #[doc = "Bit 10 - Transaction Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn errortranses(&mut self) -> ErrortransesW<IsoenptDeveptier0IsoenptSpec> {
        ErrortransesW::new(self, 10)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkes(&mut self) -> NbusybkesW<IsoenptDeveptier0IsoenptSpec> {
        NbusybkesW::new(self, 12)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    #[must_use]
    pub fn killbks(&mut self) -> KillbksW<IsoenptDeveptier0IsoenptSpec> {
        KillbksW::new(self, 13)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    #[must_use]
    pub fn fifocons(&mut self) -> FifoconsW<IsoenptDeveptier0IsoenptSpec> {
        FifoconsW::new(self, 14)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epdishdmas(&mut self) -> EpdishdmasW<IsoenptDeveptier0IsoenptSpec> {
        EpdishdmasW::new(self, 16)
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstdts(&mut self) -> RstdtsW<IsoenptDeveptier0IsoenptSpec> {
        RstdtsW::new(self, 18)
    }
    #[doc = "Bit 19 - STALL Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stallrqs(&mut self) -> StallrqsW<IsoenptDeveptier0IsoenptSpec> {
        StallrqsW::new(self, 19)
    }
}
#[doc = "Device Endpoint Enable Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoenpt_deveptier0_isoenpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoenptDeveptier0IsoenptSpec;
impl crate::RegisterSpec for IsoenptDeveptier0IsoenptSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoenpt_deveptier0_isoenpt::W`](W) writer structure"]
impl crate::Writable for IsoenptDeveptier0IsoenptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
