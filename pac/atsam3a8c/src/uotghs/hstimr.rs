#[doc = "Register `HSTIMR` reader"]
pub type R = crate::R<HstimrSpec>;
#[doc = "Field `DCONNIE` reader - Device Connection Interrupt Enable"]
pub type DconnieR = crate::BitReader;
#[doc = "Field `DDISCIE` reader - Device Disconnection Interrupt Enable"]
pub type DdiscieR = crate::BitReader;
#[doc = "Field `RSTIE` reader - USB Reset Sent Interrupt Enable"]
pub type RstieR = crate::BitReader;
#[doc = "Field `RSMEDIE` reader - Downstream Resume Sent Interrupt Enable"]
pub type RsmedieR = crate::BitReader;
#[doc = "Field `RXRSMIE` reader - Upstream Resume Received Interrupt Enable"]
pub type RxrsmieR = crate::BitReader;
#[doc = "Field `HSOFIE` reader - Host Start of Frame Interrupt Enable"]
pub type HsofieR = crate::BitReader;
#[doc = "Field `HWUPIE` reader - Host Wake-Up Interrupt Enable"]
pub type HwupieR = crate::BitReader;
#[doc = "Field `PEP_0` reader - Pipe 0 Interrupt Enable"]
pub type Pep0R = crate::BitReader;
#[doc = "Field `PEP_1` reader - Pipe 1 Interrupt Enable"]
pub type Pep1R = crate::BitReader;
#[doc = "Field `PEP_2` reader - Pipe 2 Interrupt Enable"]
pub type Pep2R = crate::BitReader;
#[doc = "Field `PEP_3` reader - Pipe 3 Interrupt Enable"]
pub type Pep3R = crate::BitReader;
#[doc = "Field `PEP_4` reader - Pipe 4 Interrupt Enable"]
pub type Pep4R = crate::BitReader;
#[doc = "Field `PEP_5` reader - Pipe 5 Interrupt Enable"]
pub type Pep5R = crate::BitReader;
#[doc = "Field `PEP_6` reader - Pipe 6 Interrupt Enable"]
pub type Pep6R = crate::BitReader;
#[doc = "Field `PEP_7` reader - Pipe 7 Interrupt Enable"]
pub type Pep7R = crate::BitReader;
#[doc = "Field `PEP_8` reader - Pipe 8 Interrupt Enable"]
pub type Pep8R = crate::BitReader;
#[doc = "Field `PEP_9` reader - Pipe 9 Interrupt Enable"]
pub type Pep9R = crate::BitReader;
#[doc = "Field `DMA_1` reader - DMA Channel 1 Interrupt Enable"]
pub type Dma1R = crate::BitReader;
#[doc = "Field `DMA_2` reader - DMA Channel 2 Interrupt Enable"]
pub type Dma2R = crate::BitReader;
#[doc = "Field `DMA_3` reader - DMA Channel 3 Interrupt Enable"]
pub type Dma3R = crate::BitReader;
#[doc = "Field `DMA_4` reader - DMA Channel 4 Interrupt Enable"]
pub type Dma4R = crate::BitReader;
#[doc = "Field `DMA_5` reader - DMA Channel 5 Interrupt Enable"]
pub type Dma5R = crate::BitReader;
#[doc = "Field `DMA_6` reader - DMA Channel 6 Interrupt Enable"]
pub type Dma6R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Device Connection Interrupt Enable"]
    #[inline(always)]
    pub fn dconnie(&self) -> DconnieR {
        DconnieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Enable"]
    #[inline(always)]
    pub fn ddiscie(&self) -> DdiscieR {
        DdiscieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Enable"]
    #[inline(always)]
    pub fn rstie(&self) -> RstieR {
        RstieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Enable"]
    #[inline(always)]
    pub fn rsmedie(&self) -> RsmedieR {
        RsmedieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Enable"]
    #[inline(always)]
    pub fn rxrsmie(&self) -> RxrsmieR {
        RxrsmieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn hsofie(&self) -> HsofieR {
        HsofieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Enable"]
    #[inline(always)]
    pub fn hwupie(&self) -> HwupieR {
        HwupieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_0(&self) -> Pep0R {
        Pep0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_1(&self) -> Pep1R {
        Pep1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_2(&self) -> Pep2R {
        Pep2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_3(&self) -> Pep3R {
        Pep3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_4(&self) -> Pep4R {
        Pep4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_5(&self) -> Pep5R {
        Pep5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_6(&self) -> Pep6R {
        Pep6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pipe 7 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_7(&self) -> Pep7R {
        Pep7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pipe 8 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_8(&self) -> Pep8R {
        Pep8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pipe 9 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_9(&self) -> Pep9R {
        Pep9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_1(&self) -> Dma1R {
        Dma1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_2(&self) -> Dma2R {
        Dma2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_3(&self) -> Dma3R {
        Dma3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_4(&self) -> Dma4R {
        Dma4R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_5(&self) -> Dma5R {
        Dma5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_6(&self) -> Dma6R {
        Dma6R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "Host Global Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstimr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstimrSpec;
impl crate::RegisterSpec for HstimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstimr::R`](R) reader structure"]
impl crate::Readable for HstimrSpec {}
#[doc = "`reset()` method sets HSTIMR to value 0"]
impl crate::Resettable for HstimrSpec {
    const RESET_VALUE: u32 = 0;
}
