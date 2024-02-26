#[doc = "Register `DEVEPTIER7` writer"]
pub type W = crate::W<Deveptier7Spec>;
#[doc = "Field `TXINES` writer - Transmitted IN Data Interrupt Enable"]
pub type TxinesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTES` writer - Received OUT Data Interrupt Enable"]
pub type RxoutesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTPES` writer - Received SETUP Interrupt Enable"]
pub type RxstpesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKOUTES` writer - NAKed OUT Interrupt Enable"]
pub type NakoutesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINES` writer - NAKed IN Interrupt Enable"]
pub type NakinesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFES` writer - Overflow Interrupt Enable"]
pub type OverfesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLEDES` writer - STALLed Interrupt Enable"]
pub type StalledesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETES` writer - Short Packet Interrupt Enable"]
pub type ShortpacketesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Interrupt Enable"]
pub type NbusybkesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KILLBKS` writer - Kill IN Bank"]
pub type KillbksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONS` writer - FIFO Control"]
pub type FifoconsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISHDMAS` writer - Endpoint Interrupts Disable HDMA Request Enable"]
pub type EpdishdmasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETDISS` writer - NYET Token Disable Enable"]
pub type NyetdissW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTDTS` writer - Reset Data Toggle Enable"]
pub type RstdtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLRQS` writer - STALL Request Enable"]
pub type StallrqsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txines(&mut self) -> TxinesW<Deveptier7Spec> {
        TxinesW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutes(&mut self) -> RxoutesW<Deveptier7Spec> {
        RxoutesW::new(self, 1)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpes(&mut self) -> RxstpesW<Deveptier7Spec> {
        RxstpesW::new(self, 2)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutes(&mut self) -> NakoutesW<Deveptier7Spec> {
        NakoutesW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakines(&mut self) -> NakinesW<Deveptier7Spec> {
        NakinesW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overfes(&mut self) -> OverfesW<Deveptier7Spec> {
        OverfesW::new(self, 5)
    }
    #[doc = "Bit 6 - STALLed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stalledes(&mut self) -> StalledesW<Deveptier7Spec> {
        StalledesW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketes(&mut self) -> ShortpacketesW<Deveptier7Spec> {
        ShortpacketesW::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkes(&mut self) -> NbusybkesW<Deveptier7Spec> {
        NbusybkesW::new(self, 12)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    #[must_use]
    pub fn killbks(&mut self) -> KillbksW<Deveptier7Spec> {
        KillbksW::new(self, 13)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    #[must_use]
    pub fn fifocons(&mut self) -> FifoconsW<Deveptier7Spec> {
        FifoconsW::new(self, 14)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epdishdmas(&mut self) -> EpdishdmasW<Deveptier7Spec> {
        EpdishdmasW::new(self, 16)
    }
    #[doc = "Bit 17 - NYET Token Disable Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nyetdiss(&mut self) -> NyetdissW<Deveptier7Spec> {
        NyetdissW::new(self, 17)
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstdts(&mut self) -> RstdtsW<Deveptier7Spec> {
        RstdtsW::new(self, 18)
    }
    #[doc = "Bit 19 - STALL Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stallrqs(&mut self) -> StallrqsW<Deveptier7Spec> {
        StallrqsW::new(self, 19)
    }
}
#[doc = "Device Endpoint Enable Register (n = 0) 7\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Deveptier7Spec;
impl crate::RegisterSpec for Deveptier7Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptier7::W`](W) writer structure"]
impl crate::Writable for Deveptier7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
