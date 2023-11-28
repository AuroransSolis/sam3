#[doc = "Register `HSTIFR` writer"]
pub type W = crate::W<HSTIFR_SPEC>;
#[doc = "Field `DCONNIS` writer - Device Connection Interrupt Set"]
pub type DCONNIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDISCIS` writer - Device Disconnection Interrupt Set"]
pub type DDISCIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIS` writer - USB Reset Sent Interrupt Set"]
pub type RSTIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSMEDIS` writer - Downstream Resume Sent Interrupt Set"]
pub type RSMEDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRSMIS` writer - Upstream Resume Received Interrupt Set"]
pub type RXRSMIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSOFIS` writer - Host Start of Frame Interrupt Set"]
pub type HSOFIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWUPIS` writer - Host Wake-Up Interrupt Set"]
pub type HWUPIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Set"]
pub type DMA_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Set"]
pub type DMA_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Set"]
pub type DMA_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Set"]
pub type DMA_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Set"]
pub type DMA_5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Set"]
pub type DMA_6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Device Connection Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dconnis(&mut self) -> DCONNIS_W<HSTIFR_SPEC> {
        DCONNIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn ddiscis(&mut self) -> DDISCIS_W<HSTIFR_SPEC> {
        DDISCIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rstis(&mut self) -> RSTIS_W<HSTIFR_SPEC> {
        RSTIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rsmedis(&mut self) -> RSMEDIS_W<HSTIFR_SPEC> {
        RSMEDIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsmis(&mut self) -> RXRSMIS_W<HSTIFR_SPEC> {
        RXRSMIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn hsofis(&mut self) -> HSOFIS_W<HSTIFR_SPEC> {
        HSOFIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn hwupis(&mut self) -> HWUPIS_W<HSTIFR_SPEC> {
        HWUPIS_W::new(self, 6)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> DMA_1_W<HSTIFR_SPEC> {
        DMA_1_W::new(self, 25)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> DMA_2_W<HSTIFR_SPEC> {
        DMA_2_W::new(self, 26)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> DMA_3_W<HSTIFR_SPEC> {
        DMA_3_W::new(self, 27)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> DMA_4_W<HSTIFR_SPEC> {
        DMA_4_W::new(self, 28)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> DMA_5_W<HSTIFR_SPEC> {
        DMA_5_W::new(self, 29)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> DMA_6_W<HSTIFR_SPEC> {
        DMA_6_W::new(self, 30)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host Global Interrupt Set Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstifr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTIFR_SPEC;
impl crate::RegisterSpec for HSTIFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstifr::W`](W) writer structure"]
impl crate::Writable for HSTIFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
