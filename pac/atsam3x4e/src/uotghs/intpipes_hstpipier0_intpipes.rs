#[doc = "Register `HSTPIPIER0_INTPIPES` writer"]
pub type W = crate::W<IntpipesHstpipier0IntpipesSpec>;
#[doc = "Field `RXINES` writer - Received IN Data Interrupt Enable"]
pub type RxinesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTES` writer - Transmitted OUT Data Interrupt Enable"]
pub type TxoutesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIES` writer - Underflow Interrupt Enable"]
pub type UnderfiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRES` writer - Pipe Error Interrupt Enable"]
pub type PerresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDES` writer - NAKed Interrupt Enable"]
pub type NakedesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIES` writer - Overflow Interrupt Enable"]
pub type OverfiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALLDES` writer - Received STALLed Interrupt Enable"]
pub type RxstalldesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIES` writer - Short Packet Interrupt Enable"]
pub type ShortpacketiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Enable"]
pub type NbusybkesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDISHDMAS` writer - Pipe Interrupts Disable HDMA Request Enable"]
pub type PdishdmasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFREEZES` writer - Pipe Freeze Enable"]
pub type PfreezesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTDTS` writer - Reset Data Toggle Enable"]
pub type RstdtsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxines(&mut self) -> RxinesW<IntpipesHstpipier0IntpipesSpec> {
        RxinesW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txoutes(&mut self) -> TxoutesW<IntpipesHstpipier0IntpipesSpec> {
        TxoutesW::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn underfies(&mut self) -> UnderfiesW<IntpipesHstpipier0IntpipesSpec> {
        UnderfiesW::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn perres(&mut self) -> PerresW<IntpipesHstpipier0IntpipesSpec> {
        PerresW::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakedes(&mut self) -> NakedesW<IntpipesHstpipier0IntpipesSpec> {
        NakedesW::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overfies(&mut self) -> OverfiesW<IntpipesHstpipier0IntpipesSpec> {
        OverfiesW::new(self, 5)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldes(&mut self) -> RxstalldesW<IntpipesHstpipier0IntpipesSpec> {
        RxstalldesW::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketies(&mut self) -> ShortpacketiesW<IntpipesHstpipier0IntpipesSpec> {
        ShortpacketiesW::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkes(&mut self) -> NbusybkesW<IntpipesHstpipier0IntpipesSpec> {
        NbusybkesW::new(self, 12)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdishdmas(&mut self) -> PdishdmasW<IntpipesHstpipier0IntpipesSpec> {
        PdishdmasW::new(self, 16)
    }
    #[doc = "Bit 17 - Pipe Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfreezes(&mut self) -> PfreezesW<IntpipesHstpipier0IntpipesSpec> {
        PfreezesW::new(self, 17)
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstdts(&mut self) -> RstdtsW<IntpipesHstpipier0IntpipesSpec> {
        RstdtsW::new(self, 18)
    }
}
#[doc = "Host Pipe Enable Register (n = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpipes_hstpipier0_intpipes::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpipesHstpipier0IntpipesSpec;
impl crate::RegisterSpec for IntpipesHstpipier0IntpipesSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intpipes_hstpipier0_intpipes::W`](W) writer structure"]
impl crate::Writable for IntpipesHstpipier0IntpipesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
