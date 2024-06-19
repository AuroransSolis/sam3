#[doc = "Register `HSTPIPICR4` writer"]
pub type W = crate::W<Hstpipicr4Spec>;
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
    pub fn rxinic(&mut self) -> RxinicW<Hstpipicr4Spec> {
        RxinicW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txoutic(&mut self) -> TxouticW<Hstpipicr4Spec> {
        TxouticW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txstpic(&mut self) -> TxstpicW<Hstpipicr4Spec> {
        TxstpicW::new(self, 2)
    }
    #[doc = "Bit 4 - NAKed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakedic(&mut self) -> NakedicW<Hstpipicr4Spec> {
        NakedicW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfic(&mut self) -> OverficW<Hstpipicr4Spec> {
        OverficW::new(self, 5)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldic(&mut self) -> RxstalldicW<Hstpipicr4Spec> {
        RxstalldicW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketic(&mut self) -> ShortpacketicW<Hstpipicr4Spec> {
        ShortpacketicW::new(self, 7)
    }
}
#[doc = "Host Pipe Clear Register (n = 0) 4\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipicr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hstpipicr4Spec;
impl crate::RegisterSpec for Hstpipicr4Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstpipicr4::W`](W) writer structure"]
impl crate::Writable for Hstpipicr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
