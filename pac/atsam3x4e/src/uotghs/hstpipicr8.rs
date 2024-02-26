#[doc = "Register `HSTPIPICR8` writer"]
pub type W = crate::W<Hstpipicr8Spec>;
#[doc = "Field `RXINIC` writer - Received IN Data Interrupt Clear"]
pub type RxinicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTIC` writer - Transmitted OUT Data Interrupt Clear"]
pub type TxouticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTPIC` writer - Transmitted SETUP Interrupt Clear"]
pub type TxstpicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDIC` writer - NAKed Interrupt Clear"]
pub type NakedicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OverficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALLDIC` writer - Received STALLed Interrupt Clear"]
pub type RxstalldicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIC` writer - Short Packet Interrupt Clear"]
pub type ShortpacketicW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxinic(&mut self) -> RxinicW<Hstpipicr8Spec> {
        RxinicW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txoutic(&mut self) -> TxouticW<Hstpipicr8Spec> {
        TxouticW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txstpic(&mut self) -> TxstpicW<Hstpipicr8Spec> {
        TxstpicW::new(self, 2)
    }
    #[doc = "Bit 4 - NAKed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakedic(&mut self) -> NakedicW<Hstpipicr8Spec> {
        NakedicW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfic(&mut self) -> OverficW<Hstpipicr8Spec> {
        OverficW::new(self, 5)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldic(&mut self) -> RxstalldicW<Hstpipicr8Spec> {
        RxstalldicW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketic(&mut self) -> ShortpacketicW<Hstpipicr8Spec> {
        ShortpacketicW::new(self, 7)
    }
}
#[doc = "Host Pipe Clear Register (n = 0) 8\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr8::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hstpipicr8Spec;
impl crate::RegisterSpec for Hstpipicr8Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstpipicr8::W`](W) writer structure"]
impl crate::Writable for Hstpipicr8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
