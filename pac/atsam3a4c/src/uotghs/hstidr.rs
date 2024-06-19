#[doc = "Register `HSTIDR` writer"]
pub type W = crate::W<HstidrSpec>;
#[doc = "Field `DCONNIEC` writer - Device Connection Interrupt Disable"]
pub type DconniecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDISCIEC` writer - Device Disconnection Interrupt Disable"]
pub type DdisciecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIEC` writer - USB Reset Sent Interrupt Disable"]
pub type RstiecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSMEDIEC` writer - Downstream Resume Sent Interrupt Disable"]
pub type RsmediecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRSMIEC` writer - Upstream Resume Received Interrupt Disable"]
pub type RxrsmiecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSOFIEC` writer - Host Start of Frame Interrupt Disable"]
pub type HsofiecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWUPIEC` writer - Host Wake-Up Interrupt Disable"]
pub type HwupiecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_0` writer - Pipe 0 Interrupt Disable"]
pub type Pep0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_1` writer - Pipe 1 Interrupt Disable"]
pub type Pep1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_2` writer - Pipe 2 Interrupt Disable"]
pub type Pep2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_3` writer - Pipe 3 Interrupt Disable"]
pub type Pep3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_4` writer - Pipe 4 Interrupt Disable"]
pub type Pep4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_5` writer - Pipe 5 Interrupt Disable"]
pub type Pep5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_6` writer - Pipe 6 Interrupt Disable"]
pub type Pep6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_7` writer - Pipe 7 Interrupt Disable"]
pub type Pep7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_8` writer - Pipe 8 Interrupt Disable"]
pub type Pep8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_9` writer - Pipe 9 Interrupt Disable"]
pub type Pep9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Disable"]
pub type Dma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Disable"]
pub type Dma2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Disable"]
pub type Dma3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Disable"]
pub type Dma4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Disable"]
pub type Dma5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Disable"]
pub type Dma6W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Device Connection Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dconniec(&mut self) -> DconniecW<HstidrSpec> {
        DconniecW::new(self, 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ddisciec(&mut self) -> DdisciecW<HstidrSpec> {
        DdisciecW::new(self, 1)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rstiec(&mut self) -> RstiecW<HstidrSpec> {
        RstiecW::new(self, 2)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rsmediec(&mut self) -> RsmediecW<HstidrSpec> {
        RsmediecW::new(self, 3)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsmiec(&mut self) -> RxrsmiecW<HstidrSpec> {
        RxrsmiecW::new(self, 4)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hsofiec(&mut self) -> HsofiecW<HstidrSpec> {
        HsofiecW::new(self, 5)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hwupiec(&mut self) -> HwupiecW<HstidrSpec> {
        HwupiecW::new(self, 6)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_0(&mut self) -> Pep0W<HstidrSpec> {
        Pep0W::new(self, 8)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_1(&mut self) -> Pep1W<HstidrSpec> {
        Pep1W::new(self, 9)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_2(&mut self) -> Pep2W<HstidrSpec> {
        Pep2W::new(self, 10)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_3(&mut self) -> Pep3W<HstidrSpec> {
        Pep3W::new(self, 11)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_4(&mut self) -> Pep4W<HstidrSpec> {
        Pep4W::new(self, 12)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_5(&mut self) -> Pep5W<HstidrSpec> {
        Pep5W::new(self, 13)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_6(&mut self) -> Pep6W<HstidrSpec> {
        Pep6W::new(self, 14)
    }
    #[doc = "Bit 15 - Pipe 7 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_7(&mut self) -> Pep7W<HstidrSpec> {
        Pep7W::new(self, 15)
    }
    #[doc = "Bit 16 - Pipe 8 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_8(&mut self) -> Pep8W<HstidrSpec> {
        Pep8W::new(self, 16)
    }
    #[doc = "Bit 17 - Pipe 9 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_9(&mut self) -> Pep9W<HstidrSpec> {
        Pep9W::new(self, 17)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> Dma1W<HstidrSpec> {
        Dma1W::new(self, 25)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> Dma2W<HstidrSpec> {
        Dma2W::new(self, 26)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> Dma3W<HstidrSpec> {
        Dma3W::new(self, 27)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> Dma4W<HstidrSpec> {
        Dma4W::new(self, 28)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> Dma5W<HstidrSpec> {
        Dma5W::new(self, 29)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> Dma6W<HstidrSpec> {
        Dma6W::new(self, 30)
    }
}
#[doc = "Host Global Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstidr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstidrSpec;
impl crate::RegisterSpec for HstidrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstidr::W`](W) writer structure"]
impl crate::Writable for HstidrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
