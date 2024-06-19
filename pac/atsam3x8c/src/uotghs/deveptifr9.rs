#[doc = "Register `DEVEPTIFR9` writer"]
pub type W = crate::W<Deveptifr9Spec>;
#[doc = "Field `TXINIS` writer - Transmitted IN Data Interrupt Set"]
pub type TxinisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTIS` writer - Received OUT Data Interrupt Set"]
pub type RxoutisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTPIS` writer - Received SETUP Interrupt Set"]
pub type RxstpisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKOUTIS` writer - NAKed OUT Interrupt Set"]
pub type NakoutisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINIS` writer - NAKed IN Interrupt Set"]
pub type NakinisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OverfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLEDIS` writer - STALLed Interrupt Set"]
pub type StalledisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETS` writer - Short Packet Interrupt Set"]
pub type ShortpacketsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Interrupt Set"]
pub type NbusybksW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txinis(&mut self) -> TxinisW<Deveptifr9Spec> {
        TxinisW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutis(&mut self) -> RxoutisW<Deveptifr9Spec> {
        RxoutisW::new(self, 1)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpis(&mut self) -> RxstpisW<Deveptifr9Spec> {
        RxstpisW::new(self, 2)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutis(&mut self) -> NakoutisW<Deveptifr9Spec> {
        NakoutisW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakinis(&mut self) -> NakinisW<Deveptifr9Spec> {
        NakinisW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn overfis(&mut self) -> OverfisW<Deveptifr9Spec> {
        OverfisW::new(self, 5)
    }
    #[doc = "Bit 6 - STALLed Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn stalledis(&mut self) -> StalledisW<Deveptifr9Spec> {
        StalledisW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn shortpackets(&mut self) -> ShortpacketsW<Deveptifr9Spec> {
        ShortpacketsW::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybks(&mut self) -> NbusybksW<Deveptifr9Spec> {
        NbusybksW::new(self, 12)
    }
}
#[doc = "Device Endpoint Set Register (n = 0) 9\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deveptifr9::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Deveptifr9Spec;
impl crate::RegisterSpec for Deveptifr9Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptifr9::W`](W) writer structure"]
impl crate::Writable for Deveptifr9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
