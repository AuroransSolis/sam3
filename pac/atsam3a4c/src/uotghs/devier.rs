#[doc = "Register `DEVIER` writer"]
pub type W = crate::W<DevierSpec>;
#[doc = "Field `SUSPES` writer - Suspend Interrupt Enable"]
pub type SuspesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSOFES` writer - Micro Start of Frame Interrupt Enable"]
pub type MsofesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFES` writer - Start of Frame Interrupt Enable"]
pub type SofesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSTES` writer - End of Reset Interrupt Enable"]
pub type EorstesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPES` writer - Wake-Up Interrupt Enable"]
pub type WakeupesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSMES` writer - End of Resume Interrupt Enable"]
pub type EorsmesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPRSMES` writer - Upstream Resume Interrupt Enable"]
pub type UprsmesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_0` writer - Endpoint 0 Interrupt Enable"]
pub type Pep0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_1` writer - Endpoint 1 Interrupt Enable"]
pub type Pep1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_2` writer - Endpoint 2 Interrupt Enable"]
pub type Pep2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_3` writer - Endpoint 3 Interrupt Enable"]
pub type Pep3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_4` writer - Endpoint 4 Interrupt Enable"]
pub type Pep4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_5` writer - Endpoint 5 Interrupt Enable"]
pub type Pep5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_6` writer - Endpoint 6 Interrupt Enable"]
pub type Pep6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_7` writer - Endpoint 7 Interrupt Enable"]
pub type Pep7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_8` writer - Endpoint 8 Interrupt Enable"]
pub type Pep8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_9` writer - Endpoint 9 Interrupt Enable"]
pub type Pep9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Enable"]
pub type Dma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Enable"]
pub type Dma2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Enable"]
pub type Dma3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Enable"]
pub type Dma4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Enable"]
pub type Dma5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Enable"]
pub type Dma6W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn suspes(&mut self) -> SuspesW<DevierSpec> {
        SuspesW::new(self, 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn msofes(&mut self) -> MsofesW<DevierSpec> {
        MsofesW::new(self, 1)
    }
    #[doc = "Bit 2 - Start of Frame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofes(&mut self) -> SofesW<DevierSpec> {
        SofesW::new(self, 2)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eorstes(&mut self) -> EorstesW<DevierSpec> {
        EorstesW::new(self, 3)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupes(&mut self) -> WakeupesW<DevierSpec> {
        WakeupesW::new(self, 4)
    }
    #[doc = "Bit 5 - End of Resume Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eorsmes(&mut self) -> EorsmesW<DevierSpec> {
        EorsmesW::new(self, 5)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uprsmes(&mut self) -> UprsmesW<DevierSpec> {
        UprsmesW::new(self, 6)
    }
    #[doc = "Bit 12 - Endpoint 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_0(&mut self) -> Pep0W<DevierSpec> {
        Pep0W::new(self, 12)
    }
    #[doc = "Bit 13 - Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_1(&mut self) -> Pep1W<DevierSpec> {
        Pep1W::new(self, 13)
    }
    #[doc = "Bit 14 - Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_2(&mut self) -> Pep2W<DevierSpec> {
        Pep2W::new(self, 14)
    }
    #[doc = "Bit 15 - Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_3(&mut self) -> Pep3W<DevierSpec> {
        Pep3W::new(self, 15)
    }
    #[doc = "Bit 16 - Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_4(&mut self) -> Pep4W<DevierSpec> {
        Pep4W::new(self, 16)
    }
    #[doc = "Bit 17 - Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_5(&mut self) -> Pep5W<DevierSpec> {
        Pep5W::new(self, 17)
    }
    #[doc = "Bit 18 - Endpoint 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_6(&mut self) -> Pep6W<DevierSpec> {
        Pep6W::new(self, 18)
    }
    #[doc = "Bit 19 - Endpoint 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_7(&mut self) -> Pep7W<DevierSpec> {
        Pep7W::new(self, 19)
    }
    #[doc = "Bit 20 - Endpoint 8 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_8(&mut self) -> Pep8W<DevierSpec> {
        Pep8W::new(self, 20)
    }
    #[doc = "Bit 21 - Endpoint 9 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_9(&mut self) -> Pep9W<DevierSpec> {
        Pep9W::new(self, 21)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> Dma1W<DevierSpec> {
        Dma1W::new(self, 25)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> Dma2W<DevierSpec> {
        Dma2W::new(self, 26)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> Dma3W<DevierSpec> {
        Dma3W::new(self, 27)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> Dma4W<DevierSpec> {
        Dma4W::new(self, 28)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> Dma5W<DevierSpec> {
        Dma5W::new(self, 29)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> Dma6W<DevierSpec> {
        Dma6W::new(self, 30)
    }
}
#[doc = "Device Global Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevierSpec;
impl crate::RegisterSpec for DevierSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devier::W`](W) writer structure"]
impl crate::Writable for DevierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
