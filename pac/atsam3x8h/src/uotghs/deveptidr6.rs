#[doc = "Register `DEVEPTIDR6` writer"]
pub type W = crate::W<Deveptidr6Spec>;
#[doc = "Field `TXINEC` writer - Transmitted IN Interrupt Clear"]
pub type TxinecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTEC` writer - Received OUT Data Interrupt Clear"]
pub type RxoutecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTPEC` writer - Received SETUP Interrupt Clear"]
pub type RxstpecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKOUTEC` writer - NAKed OUT Interrupt Clear"]
pub type NakoutecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINEC` writer - NAKed IN Interrupt Clear"]
pub type NakinecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFEC` writer - Overflow Interrupt Clear"]
pub type OverfecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLEDEC` writer - STALLed Interrupt Clear"]
pub type StalledecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETEC` writer - Shortpacket Interrupt Clear"]
pub type ShortpacketecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Interrupt Clear"]
pub type NbusybkecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Clear"]
pub type FifoconcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISHDMAC` writer - Endpoint Interrupts Disable HDMA Request Clear"]
pub type EpdishdmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETDISC` writer - NYET Token Disable Clear"]
pub type NyetdiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLRQC` writer - STALL Request Clear"]
pub type StallrqcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinec(&mut self) -> TxinecW<Deveptidr6Spec> {
        TxinecW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutec(&mut self) -> RxoutecW<Deveptidr6Spec> {
        RxoutecW::new(self, 1)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpec(&mut self) -> RxstpecW<Deveptidr6Spec> {
        RxstpecW::new(self, 2)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutec(&mut self) -> NakoutecW<Deveptidr6Spec> {
        NakoutecW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakinec(&mut self) -> NakinecW<Deveptidr6Spec> {
        NakinecW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfec(&mut self) -> OverfecW<Deveptidr6Spec> {
        OverfecW::new(self, 5)
    }
    #[doc = "Bit 6 - STALLed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stalledec(&mut self) -> StalledecW<Deveptidr6Spec> {
        StalledecW::new(self, 6)
    }
    #[doc = "Bit 7 - Shortpacket Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketec(&mut self) -> ShortpacketecW<Deveptidr6Spec> {
        ShortpacketecW::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkec(&mut self) -> NbusybkecW<Deveptidr6Spec> {
        NbusybkecW::new(self, 12)
    }
    #[doc = "Bit 14 - FIFO Control Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoconc(&mut self) -> FifoconcW<Deveptidr6Spec> {
        FifoconcW::new(self, 14)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn epdishdmac(&mut self) -> EpdishdmacW<Deveptidr6Spec> {
        EpdishdmacW::new(self, 16)
    }
    #[doc = "Bit 17 - NYET Token Disable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nyetdisc(&mut self) -> NyetdiscW<Deveptidr6Spec> {
        NyetdiscW::new(self, 17)
    }
    #[doc = "Bit 19 - STALL Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stallrqc(&mut self) -> StallrqcW<Deveptidr6Spec> {
        StallrqcW::new(self, 19)
    }
}
#[doc = "Device Endpoint Disable Register (n = 0) 6\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr6::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Deveptidr6Spec;
impl crate::RegisterSpec for Deveptidr6Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptidr6::W`](W) writer structure"]
impl crate::Writable for Deveptidr6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
