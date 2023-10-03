#[doc = "Register `DEVIDR` writer"]
pub type W = crate::W<DEVIDR_SPEC>;
#[doc = "Field `SUSPEC` writer - Suspend Interrupt Disable"]
pub type SUSPEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSOFEC` writer - Micro Start of Frame Interrupt Disable"]
pub type MSOFEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFEC` writer - Start of Frame Interrupt Disable"]
pub type SOFEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EORSTEC` writer - End of Reset Interrupt Disable"]
pub type EORSTEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPEC` writer - Wake-Up Interrupt Disable"]
pub type WAKEUPEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EORSMEC` writer - End of Resume Interrupt Disable"]
pub type EORSMEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPRSMEC` writer - Upstream Resume Interrupt Disable"]
pub type UPRSMEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_0` writer - Endpoint 0 Interrupt Disable"]
pub type PEP_0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_1` writer - Endpoint 1 Interrupt Disable"]
pub type PEP_1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_2` writer - Endpoint 2 Interrupt Disable"]
pub type PEP_2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_3` writer - Endpoint 3 Interrupt Disable"]
pub type PEP_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_4` writer - Endpoint 4 Interrupt Disable"]
pub type PEP_4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_5` writer - Endpoint 5 Interrupt Disable"]
pub type PEP_5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_6` writer - Endpoint 6 Interrupt Disable"]
pub type PEP_6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_7` writer - Endpoint 7 Interrupt Disable"]
pub type PEP_7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_8` writer - Endpoint 8 Interrupt Disable"]
pub type PEP_8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_9` writer - Endpoint 9 Interrupt Disable"]
pub type PEP_9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Disable"]
pub type DMA_1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Disable"]
pub type DMA_2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Disable"]
pub type DMA_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Disable"]
pub type DMA_4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Disable"]
pub type DMA_5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Disable"]
pub type DMA_6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn suspec(&mut self) -> SUSPEC_W<DEVIDR_SPEC, 0> {
        SUSPEC_W::new(self)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn msofec(&mut self) -> MSOFEC_W<DEVIDR_SPEC, 1> {
        MSOFEC_W::new(self)
    }
    #[doc = "Bit 2 - Start of Frame Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sofec(&mut self) -> SOFEC_W<DEVIDR_SPEC, 2> {
        SOFEC_W::new(self)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eorstec(&mut self) -> EORSTEC_W<DEVIDR_SPEC, 3> {
        EORSTEC_W::new(self)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupec(&mut self) -> WAKEUPEC_W<DEVIDR_SPEC, 4> {
        WAKEUPEC_W::new(self)
    }
    #[doc = "Bit 5 - End of Resume Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eorsmec(&mut self) -> EORSMEC_W<DEVIDR_SPEC, 5> {
        EORSMEC_W::new(self)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn uprsmec(&mut self) -> UPRSMEC_W<DEVIDR_SPEC, 6> {
        UPRSMEC_W::new(self)
    }
    #[doc = "Bit 12 - Endpoint 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_0(&mut self) -> PEP_0_W<DEVIDR_SPEC, 12> {
        PEP_0_W::new(self)
    }
    #[doc = "Bit 13 - Endpoint 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_1(&mut self) -> PEP_1_W<DEVIDR_SPEC, 13> {
        PEP_1_W::new(self)
    }
    #[doc = "Bit 14 - Endpoint 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_2(&mut self) -> PEP_2_W<DEVIDR_SPEC, 14> {
        PEP_2_W::new(self)
    }
    #[doc = "Bit 15 - Endpoint 3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_3(&mut self) -> PEP_3_W<DEVIDR_SPEC, 15> {
        PEP_3_W::new(self)
    }
    #[doc = "Bit 16 - Endpoint 4 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_4(&mut self) -> PEP_4_W<DEVIDR_SPEC, 16> {
        PEP_4_W::new(self)
    }
    #[doc = "Bit 17 - Endpoint 5 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_5(&mut self) -> PEP_5_W<DEVIDR_SPEC, 17> {
        PEP_5_W::new(self)
    }
    #[doc = "Bit 18 - Endpoint 6 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_6(&mut self) -> PEP_6_W<DEVIDR_SPEC, 18> {
        PEP_6_W::new(self)
    }
    #[doc = "Bit 19 - Endpoint 7 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_7(&mut self) -> PEP_7_W<DEVIDR_SPEC, 19> {
        PEP_7_W::new(self)
    }
    #[doc = "Bit 20 - Endpoint 8 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_8(&mut self) -> PEP_8_W<DEVIDR_SPEC, 20> {
        PEP_8_W::new(self)
    }
    #[doc = "Bit 21 - Endpoint 9 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_9(&mut self) -> PEP_9_W<DEVIDR_SPEC, 21> {
        PEP_9_W::new(self)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> DMA_1_W<DEVIDR_SPEC, 25> {
        DMA_1_W::new(self)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> DMA_2_W<DEVIDR_SPEC, 26> {
        DMA_2_W::new(self)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> DMA_3_W<DEVIDR_SPEC, 27> {
        DMA_3_W::new(self)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> DMA_4_W<DEVIDR_SPEC, 28> {
        DMA_4_W::new(self)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> DMA_5_W<DEVIDR_SPEC, 29> {
        DMA_5_W::new(self)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> DMA_6_W<DEVIDR_SPEC, 30> {
        DMA_6_W::new(self)
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
#[doc = "Device Global Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devidr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVIDR_SPEC;
impl crate::RegisterSpec for DEVIDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devidr::W`](W) writer structure"]
impl crate::Writable for DEVIDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
