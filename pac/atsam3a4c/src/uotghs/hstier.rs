#[doc = "Register `HSTIER` writer"]
pub type W = crate::W<HstierSpec>;
#[doc = "Field `DCONNIES` writer - Device Connection Interrupt Enable"]
pub type DconniesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDISCIES` writer - Device Disconnection Interrupt Enable"]
pub type DdisciesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIES` writer - USB Reset Sent Interrupt Enable"]
pub type RstiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSMEDIES` writer - Downstream Resume Sent Interrupt Enable"]
pub type RsmediesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRSMIES` writer - Upstream Resume Received Interrupt Enable"]
pub type RxrsmiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSOFIES` writer - Host Start of Frame Interrupt Enable"]
pub type HsofiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWUPIES` writer - Host Wake-Up Interrupt Enable"]
pub type HwupiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_0` writer - Pipe 0 Interrupt Enable"]
pub type Pep0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_1` writer - Pipe 1 Interrupt Enable"]
pub type Pep1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_2` writer - Pipe 2 Interrupt Enable"]
pub type Pep2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_3` writer - Pipe 3 Interrupt Enable"]
pub type Pep3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_4` writer - Pipe 4 Interrupt Enable"]
pub type Pep4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_5` writer - Pipe 5 Interrupt Enable"]
pub type Pep5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_6` writer - Pipe 6 Interrupt Enable"]
pub type Pep6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_7` writer - Pipe 7 Interrupt Enable"]
pub type Pep7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_8` writer - Pipe 8 Interrupt Enable"]
pub type Pep8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_9` writer - Pipe 9 Interrupt Enable"]
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
    #[doc = "Bit 0 - Device Connection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dconnies(&mut self) -> DconniesW<HstierSpec> {
        DconniesW::new(self, 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddiscies(&mut self) -> DdisciesW<HstierSpec> {
        DdisciesW::new(self, 1)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsties(&mut self) -> RstiesW<HstierSpec> {
        RstiesW::new(self, 2)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsmedies(&mut self) -> RsmediesW<HstierSpec> {
        RsmediesW::new(self, 3)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsmies(&mut self) -> RxrsmiesW<HstierSpec> {
        RxrsmiesW::new(self, 4)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsofies(&mut self) -> HsofiesW<HstierSpec> {
        HsofiesW::new(self, 5)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hwupies(&mut self) -> HwupiesW<HstierSpec> {
        HwupiesW::new(self, 6)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_0(&mut self) -> Pep0W<HstierSpec> {
        Pep0W::new(self, 8)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_1(&mut self) -> Pep1W<HstierSpec> {
        Pep1W::new(self, 9)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_2(&mut self) -> Pep2W<HstierSpec> {
        Pep2W::new(self, 10)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_3(&mut self) -> Pep3W<HstierSpec> {
        Pep3W::new(self, 11)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_4(&mut self) -> Pep4W<HstierSpec> {
        Pep4W::new(self, 12)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_5(&mut self) -> Pep5W<HstierSpec> {
        Pep5W::new(self, 13)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_6(&mut self) -> Pep6W<HstierSpec> {
        Pep6W::new(self, 14)
    }
    #[doc = "Bit 15 - Pipe 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_7(&mut self) -> Pep7W<HstierSpec> {
        Pep7W::new(self, 15)
    }
    #[doc = "Bit 16 - Pipe 8 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_8(&mut self) -> Pep8W<HstierSpec> {
        Pep8W::new(self, 16)
    }
    #[doc = "Bit 17 - Pipe 9 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_9(&mut self) -> Pep9W<HstierSpec> {
        Pep9W::new(self, 17)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> Dma1W<HstierSpec> {
        Dma1W::new(self, 25)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> Dma2W<HstierSpec> {
        Dma2W::new(self, 26)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> Dma3W<HstierSpec> {
        Dma3W::new(self, 27)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> Dma4W<HstierSpec> {
        Dma4W::new(self, 28)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> Dma5W<HstierSpec> {
        Dma5W::new(self, 29)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> Dma6W<HstierSpec> {
        Dma6W::new(self, 30)
    }
}
#[doc = "Host Global Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstierSpec;
impl crate::RegisterSpec for HstierSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstier::W`](W) writer structure"]
impl crate::Writable for HstierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
