#[doc = "Register `DEVEPTICR1` writer"]
pub type W = crate::W<Devepticr1Spec>;
#[doc = "Field `TXINIC` writer - Transmitted IN Data Interrupt Clear"]
pub type TxinicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTIC` writer - Received OUT Data Interrupt Clear"]
pub type RxouticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTPIC` writer - Received SETUP Interrupt Clear"]
pub type RxstpicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKOUTIC` writer - NAKed OUT Interrupt Clear"]
pub type NakouticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINIC` writer - NAKed IN Interrupt Clear"]
pub type NakinicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OverficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLEDIC` writer - STALLed Interrupt Clear"]
pub type StalledicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETC` writer - Short Packet Interrupt Clear"]
pub type ShortpacketcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinic(&mut self) -> TxinicW<Devepticr1Spec> {
        TxinicW::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutic(&mut self) -> RxouticW<Devepticr1Spec> {
        RxouticW::new(self, 1)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpic(&mut self) -> RxstpicW<Devepticr1Spec> {
        RxstpicW::new(self, 2)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutic(&mut self) -> NakouticW<Devepticr1Spec> {
        NakouticW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakinic(&mut self) -> NakinicW<Devepticr1Spec> {
        NakinicW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfic(&mut self) -> OverficW<Devepticr1Spec> {
        OverficW::new(self, 5)
    }
    #[doc = "Bit 6 - STALLed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stalledic(&mut self) -> StalledicW<Devepticr1Spec> {
        StalledicW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketc(&mut self) -> ShortpacketcW<Devepticr1Spec> {
        ShortpacketcW::new(self, 7)
    }
}
#[doc = "Device Endpoint Clear Register (n = 0) 1\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devepticr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devepticr1Spec;
impl crate::RegisterSpec for Devepticr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devepticr1::W`](W) writer structure"]
impl crate::Writable for Devepticr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
