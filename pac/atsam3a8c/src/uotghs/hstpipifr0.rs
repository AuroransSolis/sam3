#[doc = "Register `HSTPIPIFR0` writer"]
pub type W = crate::W<Hstpipifr0Spec>;
#[doc = "Field `RXINIS` writer - Received IN Data Interrupt Set"]
pub type RxinisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTIS` writer - Transmitted OUT Data Interrupt Set"]
pub type TxoutisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTPIS` writer - Transmitted SETUP Interrupt Set"]
pub type TxstpisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRIS` writer - Pipe Error Interrupt Set"]
pub type PerrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDIS` writer - NAKed Interrupt Set"]
pub type NakedisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OverfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALLDIS` writer - Received STALLed Interrupt Set"]
pub type RxstalldisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIS` writer - Short Packet Interrupt Set"]
pub type ShortpacketisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Set"]
pub type NbusybksW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxinis(&mut self) -> RxinisW<Hstpipifr0Spec> {
        RxinisW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txoutis(&mut self) -> TxoutisW<Hstpipifr0Spec> {
        TxoutisW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txstpis(&mut self) -> TxstpisW<Hstpipifr0Spec> {
        TxstpisW::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn perris(&mut self) -> PerrisW<Hstpipifr0Spec> {
        PerrisW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakedis(&mut self) -> NakedisW<Hstpipifr0Spec> {
        NakedisW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn overfis(&mut self) -> OverfisW<Hstpipifr0Spec> {
        OverfisW::new(self, 5)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldis(&mut self) -> RxstalldisW<Hstpipifr0Spec> {
        RxstalldisW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketis(&mut self) -> ShortpacketisW<Hstpipifr0Spec> {
        ShortpacketisW::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybks(&mut self) -> NbusybksW<Hstpipifr0Spec> {
        NbusybksW::new(self, 12)
    }
}
#[doc = "Host Pipe Set Register (n = 0) 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hstpipifr0Spec;
impl crate::RegisterSpec for Hstpipifr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstpipifr0::W`](W) writer structure"]
impl crate::Writable for Hstpipifr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
