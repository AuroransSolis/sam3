#[doc = "Register `DEVIMR` reader"]
pub type R = crate::R<DevimrSpec>;
#[doc = "Field `SUSPE` reader - Suspend Interrupt Mask"]
pub type SuspeR = crate::BitReader;
#[doc = "Field `MSOFE` reader - Micro Start of Frame Interrupt Mask"]
pub type MsofeR = crate::BitReader;
#[doc = "Field `SOFE` reader - Start of Frame Interrupt Mask"]
pub type SofeR = crate::BitReader;
#[doc = "Field `EORSTE` reader - End of Reset Interrupt Mask"]
pub type EorsteR = crate::BitReader;
#[doc = "Field `WAKEUPE` reader - Wake-Up Interrupt Mask"]
pub type WakeupeR = crate::BitReader;
#[doc = "Field `EORSME` reader - End of Resume Interrupt Mask"]
pub type EorsmeR = crate::BitReader;
#[doc = "Field `UPRSME` reader - Upstream Resume Interrupt Mask"]
pub type UprsmeR = crate::BitReader;
#[doc = "Field `PEP_0` reader - Endpoint 0 Interrupt Mask"]
pub type Pep0R = crate::BitReader;
#[doc = "Field `PEP_1` reader - Endpoint 1 Interrupt Mask"]
pub type Pep1R = crate::BitReader;
#[doc = "Field `PEP_2` reader - Endpoint 2 Interrupt Mask"]
pub type Pep2R = crate::BitReader;
#[doc = "Field `PEP_3` reader - Endpoint 3 Interrupt Mask"]
pub type Pep3R = crate::BitReader;
#[doc = "Field `PEP_4` reader - Endpoint 4 Interrupt Mask"]
pub type Pep4R = crate::BitReader;
#[doc = "Field `PEP_5` reader - Endpoint 5 Interrupt Mask"]
pub type Pep5R = crate::BitReader;
#[doc = "Field `PEP_6` reader - Endpoint 6 Interrupt Mask"]
pub type Pep6R = crate::BitReader;
#[doc = "Field `PEP_7` reader - Endpoint 7 Interrupt Mask"]
pub type Pep7R = crate::BitReader;
#[doc = "Field `PEP_8` reader - Endpoint 8 Interrupt Mask"]
pub type Pep8R = crate::BitReader;
#[doc = "Field `PEP_9` reader - Endpoint 9 Interrupt Mask"]
pub type Pep9R = crate::BitReader;
#[doc = "Field `DMA_1` reader - DMA Channel 1 Interrupt Mask"]
pub type Dma1R = crate::BitReader;
#[doc = "Field `DMA_2` reader - DMA Channel 2 Interrupt Mask"]
pub type Dma2R = crate::BitReader;
#[doc = "Field `DMA_3` reader - DMA Channel 3 Interrupt Mask"]
pub type Dma3R = crate::BitReader;
#[doc = "Field `DMA_4` reader - DMA Channel 4 Interrupt Mask"]
pub type Dma4R = crate::BitReader;
#[doc = "Field `DMA_5` reader - DMA Channel 5 Interrupt Mask"]
pub type Dma5R = crate::BitReader;
#[doc = "Field `DMA_6` reader - DMA Channel 6 Interrupt Mask"]
pub type Dma6R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Suspend Interrupt Mask"]
    #[inline(always)]
    pub fn suspe(&self) -> SuspeR {
        SuspeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Mask"]
    #[inline(always)]
    pub fn msofe(&self) -> MsofeR {
        MsofeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start of Frame Interrupt Mask"]
    #[inline(always)]
    pub fn sofe(&self) -> SofeR {
        SofeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Mask"]
    #[inline(always)]
    pub fn eorste(&self) -> EorsteR {
        EorsteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn wakeupe(&self) -> WakeupeR {
        WakeupeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Resume Interrupt Mask"]
    #[inline(always)]
    pub fn eorsme(&self) -> EorsmeR {
        EorsmeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Mask"]
    #[inline(always)]
    pub fn uprsme(&self) -> UprsmeR {
        UprsmeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Endpoint 0 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_0(&self) -> Pep0R {
        Pep0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Endpoint 1 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_1(&self) -> Pep1R {
        Pep1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Endpoint 2 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_2(&self) -> Pep2R {
        Pep2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Endpoint 3 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_3(&self) -> Pep3R {
        Pep3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint 4 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_4(&self) -> Pep4R {
        Pep4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Endpoint 5 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_5(&self) -> Pep5R {
        Pep5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Endpoint 6 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_6(&self) -> Pep6R {
        Pep6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Endpoint 7 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_7(&self) -> Pep7R {
        Pep7R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Endpoint 8 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_8(&self) -> Pep8R {
        Pep8R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Endpoint 9 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_9(&self) -> Pep9R {
        Pep9R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Mask"]
    #[inline(always)]
    pub fn dma_1(&self) -> Dma1R {
        Dma1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Mask"]
    #[inline(always)]
    pub fn dma_2(&self) -> Dma2R {
        Dma2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Mask"]
    #[inline(always)]
    pub fn dma_3(&self) -> Dma3R {
        Dma3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Mask"]
    #[inline(always)]
    pub fn dma_4(&self) -> Dma4R {
        Dma4R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Mask"]
    #[inline(always)]
    pub fn dma_5(&self) -> Dma5R {
        Dma5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Mask"]
    #[inline(always)]
    pub fn dma_6(&self) -> Dma6R {
        Dma6R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "Device Global Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devimr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevimrSpec;
impl crate::RegisterSpec for DevimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devimr::R`](R) reader structure"]
impl crate::Readable for DevimrSpec {}
#[doc = "`reset()` method sets DEVIMR to value 0"]
impl crate::Resettable for DevimrSpec {
    const RESET_VALUE: u32 = 0;
}
