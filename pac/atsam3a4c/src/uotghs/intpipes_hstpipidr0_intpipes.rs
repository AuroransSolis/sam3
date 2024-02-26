#[doc = "Register `HSTPIPIDR0_INTPIPES` writer"]
pub type W = crate::W<IntpipesHstpipidr0IntpipesSpec>;
#[doc = "Field `RXINEC` writer - Received IN Data Interrupt Disable"]
pub type RxinecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTEC` writer - Transmitted OUT Data Interrupt Disable"]
pub type TxoutecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIEC` writer - Underflow Interrupt Disable"]
pub type UnderfiecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERREC` writer - Pipe Error Interrupt Disable"]
pub type PerrecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDEC` writer - NAKed Interrupt Disable"]
pub type NakedecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIEC` writer - Overflow Interrupt Disable"]
pub type OverfiecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALLDEC` writer - Received STALLed Interrupt Disable"]
pub type RxstalldecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIEC` writer - Short Packet Interrupt Disable"]
pub type ShortpacketiecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Disable"]
pub type NbusybkecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Disable"]
pub type FifoconcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDISHDMAC` writer - Pipe Interrupts Disable HDMA Request Disable"]
pub type PdishdmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFREEZEC` writer - Pipe Freeze Disable"]
pub type PfreezecW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxinec(&mut self) -> RxinecW<IntpipesHstpipidr0IntpipesSpec> {
        RxinecW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txoutec(&mut self) -> TxoutecW<IntpipesHstpipidr0IntpipesSpec> {
        TxoutecW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn underfiec(&mut self) -> UnderfiecW<IntpipesHstpipidr0IntpipesSpec> {
        UnderfiecW::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn perrec(&mut self) -> PerrecW<IntpipesHstpipidr0IntpipesSpec> {
        PerrecW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nakedec(&mut self) -> NakedecW<IntpipesHstpipidr0IntpipesSpec> {
        NakedecW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn overfiec(&mut self) -> OverfiecW<IntpipesHstpipidr0IntpipesSpec> {
        OverfiecW::new(self, 5)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldec(&mut self) -> RxstalldecW<IntpipesHstpipidr0IntpipesSpec> {
        RxstalldecW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketiec(&mut self) -> ShortpacketiecW<IntpipesHstpipidr0IntpipesSpec> {
        ShortpacketiecW::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkec(&mut self) -> NbusybkecW<IntpipesHstpipidr0IntpipesSpec> {
        NbusybkecW::new(self, 12)
    }
    #[doc = "Bit 14 - FIFO Control Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoconc(&mut self) -> FifoconcW<IntpipesHstpipidr0IntpipesSpec> {
        FifoconcW::new(self, 14)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pdishdmac(&mut self) -> PdishdmacW<IntpipesHstpipidr0IntpipesSpec> {
        PdishdmacW::new(self, 16)
    }
    #[doc = "Bit 17 - Pipe Freeze Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pfreezec(&mut self) -> PfreezecW<IntpipesHstpipidr0IntpipesSpec> {
        PfreezecW::new(self, 17)
    }
}
#[doc = "Host Pipe Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpipes_hstpipidr0_intpipes::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpipesHstpipidr0IntpipesSpec;
impl crate::RegisterSpec for IntpipesHstpipidr0IntpipesSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intpipes_hstpipidr0_intpipes::W`](W) writer structure"]
impl crate::Writable for IntpipesHstpipidr0IntpipesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
