#[doc = "Register `HSTIER` writer"]
pub type W = crate::W<HSTIER_SPEC>;
#[doc = "Field `DCONNIES` writer - Device Connection Interrupt Enable"]
pub type DCONNIES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDISCIES` writer - Device Disconnection Interrupt Enable"]
pub type DDISCIES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIES` writer - USB Reset Sent Interrupt Enable"]
pub type RSTIES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSMEDIES` writer - Downstream Resume Sent Interrupt Enable"]
pub type RSMEDIES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRSMIES` writer - Upstream Resume Received Interrupt Enable"]
pub type RXRSMIES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSOFIES` writer - Host Start of Frame Interrupt Enable"]
pub type HSOFIES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWUPIES` writer - Host Wake-Up Interrupt Enable"]
pub type HWUPIES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_0` writer - Pipe 0 Interrupt Enable"]
pub type PEP_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_1` writer - Pipe 1 Interrupt Enable"]
pub type PEP_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_2` writer - Pipe 2 Interrupt Enable"]
pub type PEP_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_3` writer - Pipe 3 Interrupt Enable"]
pub type PEP_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_4` writer - Pipe 4 Interrupt Enable"]
pub type PEP_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_5` writer - Pipe 5 Interrupt Enable"]
pub type PEP_5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_6` writer - Pipe 6 Interrupt Enable"]
pub type PEP_6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_7` writer - Pipe 7 Interrupt Enable"]
pub type PEP_7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_8` writer - Pipe 8 Interrupt Enable"]
pub type PEP_8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEP_9` writer - Pipe 9 Interrupt Enable"]
pub type PEP_9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Enable"]
pub type DMA_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Enable"]
pub type DMA_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Enable"]
pub type DMA_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Enable"]
pub type DMA_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Enable"]
pub type DMA_5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Enable"]
pub type DMA_6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Device Connection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dconnies(&mut self) -> DCONNIES_W<HSTIER_SPEC> {
        DCONNIES_W::new(self, 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ddiscies(&mut self) -> DDISCIES_W<HSTIER_SPEC> {
        DDISCIES_W::new(self, 1)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsties(&mut self) -> RSTIES_W<HSTIER_SPEC> {
        RSTIES_W::new(self, 2)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsmedies(&mut self) -> RSMEDIES_W<HSTIER_SPEC> {
        RSMEDIES_W::new(self, 3)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsmies(&mut self) -> RXRSMIES_W<HSTIER_SPEC> {
        RXRSMIES_W::new(self, 4)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsofies(&mut self) -> HSOFIES_W<HSTIER_SPEC> {
        HSOFIES_W::new(self, 5)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hwupies(&mut self) -> HWUPIES_W<HSTIER_SPEC> {
        HWUPIES_W::new(self, 6)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_0(&mut self) -> PEP_0_W<HSTIER_SPEC> {
        PEP_0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_1(&mut self) -> PEP_1_W<HSTIER_SPEC> {
        PEP_1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_2(&mut self) -> PEP_2_W<HSTIER_SPEC> {
        PEP_2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_3(&mut self) -> PEP_3_W<HSTIER_SPEC> {
        PEP_3_W::new(self, 11)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_4(&mut self) -> PEP_4_W<HSTIER_SPEC> {
        PEP_4_W::new(self, 12)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_5(&mut self) -> PEP_5_W<HSTIER_SPEC> {
        PEP_5_W::new(self, 13)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_6(&mut self) -> PEP_6_W<HSTIER_SPEC> {
        PEP_6_W::new(self, 14)
    }
    #[doc = "Bit 15 - Pipe 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_7(&mut self) -> PEP_7_W<HSTIER_SPEC> {
        PEP_7_W::new(self, 15)
    }
    #[doc = "Bit 16 - Pipe 8 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_8(&mut self) -> PEP_8_W<HSTIER_SPEC> {
        PEP_8_W::new(self, 16)
    }
    #[doc = "Bit 17 - Pipe 9 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_9(&mut self) -> PEP_9_W<HSTIER_SPEC> {
        PEP_9_W::new(self, 17)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> DMA_1_W<HSTIER_SPEC> {
        DMA_1_W::new(self, 25)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> DMA_2_W<HSTIER_SPEC> {
        DMA_2_W::new(self, 26)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> DMA_3_W<HSTIER_SPEC> {
        DMA_3_W::new(self, 27)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> DMA_4_W<HSTIER_SPEC> {
        DMA_4_W::new(self, 28)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> DMA_5_W<HSTIER_SPEC> {
        DMA_5_W::new(self, 29)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> DMA_6_W<HSTIER_SPEC> {
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
#[doc = "Host Global Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTIER_SPEC;
impl crate::RegisterSpec for HSTIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstier::W`](W) writer structure"]
impl crate::Writable for HSTIER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
