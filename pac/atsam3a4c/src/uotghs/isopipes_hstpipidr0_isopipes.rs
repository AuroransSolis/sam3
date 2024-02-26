#[doc = "Register `HSTPIPIDR0_ISOPIPES` writer"]
pub type W = crate::W<IsopipesHstpipidr0IsopipesSpec>;
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
#[doc = "Field `CRCERREC` writer - CRC Error Interrupt Disable"]
pub type CrcerrecW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn rxinec(&mut self) -> RxinecW<IsopipesHstpipidr0IsopipesSpec> {
        RxinecW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txoutec(&mut self) -> TxoutecW<IsopipesHstpipidr0IsopipesSpec> {
        TxoutecW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn underfiec(&mut self) -> UnderfiecW<IsopipesHstpipidr0IsopipesSpec> {
        UnderfiecW::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn perrec(&mut self) -> PerrecW<IsopipesHstpipidr0IsopipesSpec> {
        PerrecW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nakedec(&mut self) -> NakedecW<IsopipesHstpipidr0IsopipesSpec> {
        NakedecW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn overfiec(&mut self) -> OverfiecW<IsopipesHstpipidr0IsopipesSpec> {
        OverfiecW::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn crcerrec(&mut self) -> CrcerrecW<IsopipesHstpipidr0IsopipesSpec> {
        CrcerrecW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketiec(&mut self) -> ShortpacketiecW<IsopipesHstpipidr0IsopipesSpec> {
        ShortpacketiecW::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkec(&mut self) -> NbusybkecW<IsopipesHstpipidr0IsopipesSpec> {
        NbusybkecW::new(self, 12)
    }
    #[doc = "Bit 14 - FIFO Control Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoconc(&mut self) -> FifoconcW<IsopipesHstpipidr0IsopipesSpec> {
        FifoconcW::new(self, 14)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pdishdmac(&mut self) -> PdishdmacW<IsopipesHstpipidr0IsopipesSpec> {
        PdishdmacW::new(self, 16)
    }
    #[doc = "Bit 17 - Pipe Freeze Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pfreezec(&mut self) -> PfreezecW<IsopipesHstpipidr0IsopipesSpec> {
        PfreezecW::new(self, 17)
    }
}
#[doc = "Host Pipe Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isopipes_hstpipidr0_isopipes::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsopipesHstpipidr0IsopipesSpec;
impl crate::RegisterSpec for IsopipesHstpipidr0IsopipesSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isopipes_hstpipidr0_isopipes::W`](W) writer structure"]
impl crate::Writable for IsopipesHstpipidr0IsopipesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
