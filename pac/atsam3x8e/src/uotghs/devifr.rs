#[doc = "Register `DEVIFR` writer"]
pub type W = crate::W<DevifrSpec>;
#[doc = "Field `SUSPS` writer - Suspend Interrupt Set"]
pub type SuspsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSOFS` writer - Micro Start of Frame Interrupt Set"]
pub type MsofsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFS` writer - Start of Frame Interrupt Set"]
pub type SofsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSTS` writer - End of Reset Interrupt Set"]
pub type EorstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPS` writer - Wake-Up Interrupt Set"]
pub type WakeupsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSMS` writer - End of Resume Interrupt Set"]
pub type EorsmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPRSMS` writer - Upstream Resume Interrupt Set"]
pub type UprsmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Set"]
pub type Dma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Set"]
pub type Dma2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Set"]
pub type Dma3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Set"]
pub type Dma4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Set"]
pub type Dma5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Set"]
pub type Dma6W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn susps(&mut self) -> SuspsW<DevifrSpec> {
        SuspsW::new(self, 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn msofs(&mut self) -> MsofsW<DevifrSpec> {
        MsofsW::new(self, 1)
    }
    #[doc = "Bit 2 - Start of Frame Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn sofs(&mut self) -> SofsW<DevifrSpec> {
        SofsW::new(self, 2)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn eorsts(&mut self) -> EorstsW<DevifrSpec> {
        EorstsW::new(self, 3)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn wakeups(&mut self) -> WakeupsW<DevifrSpec> {
        WakeupsW::new(self, 4)
    }
    #[doc = "Bit 5 - End of Resume Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn eorsms(&mut self) -> EorsmsW<DevifrSpec> {
        EorsmsW::new(self, 5)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn uprsms(&mut self) -> UprsmsW<DevifrSpec> {
        UprsmsW::new(self, 6)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> Dma1W<DevifrSpec> {
        Dma1W::new(self, 25)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> Dma2W<DevifrSpec> {
        Dma2W::new(self, 26)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> Dma3W<DevifrSpec> {
        Dma3W::new(self, 27)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> Dma4W<DevifrSpec> {
        Dma4W::new(self, 28)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> Dma5W<DevifrSpec> {
        Dma5W::new(self, 29)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> Dma6W<DevifrSpec> {
        Dma6W::new(self, 30)
    }
}
#[doc = "Device Global Interrupt Set Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devifr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevifrSpec;
impl crate::RegisterSpec for DevifrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devifr::W`](W) writer structure"]
impl crate::Writable for DevifrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
