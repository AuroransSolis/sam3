#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DET_SUSPD` reader - Suspend Interrupt Enable"]
pub type DET_SUSPD_R = crate::BitReader<bool>;
#[doc = "Field `DET_SUSPD` writer - Suspend Interrupt Enable"]
pub type DET_SUSPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `MICRO_SOF` reader - Micro-SOF Interrupt Enable"]
pub type MICRO_SOF_R = crate::BitReader<bool>;
#[doc = "Field `MICRO_SOF` writer - Micro-SOF Interrupt Enable"]
pub type MICRO_SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `INT_SOF` reader - SOF Interrupt Enable"]
pub type INT_SOF_R = crate::BitReader<bool>;
#[doc = "Field `INT_SOF` writer - SOF Interrupt Enable"]
pub type INT_SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ENDRESET` reader - End Of Reset Interrupt Enable"]
pub type ENDRESET_R = crate::BitReader<bool>;
#[doc = "Field `ENDRESET` writer - End Of Reset Interrupt Enable"]
pub type ENDRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `WAKE_UP` reader - Wake Up CPU Interrupt Enable"]
pub type WAKE_UP_R = crate::BitReader<bool>;
#[doc = "Field `WAKE_UP` writer - Wake Up CPU Interrupt Enable"]
pub type WAKE_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ENDOFRSM` reader - End Of Resume Interrupt Enable"]
pub type ENDOFRSM_R = crate::BitReader<bool>;
#[doc = "Field `ENDOFRSM` writer - End Of Resume Interrupt Enable"]
pub type ENDOFRSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `UPSTR_RES` reader - Upstream Resume Interrupt Enable"]
pub type UPSTR_RES_R = crate::BitReader<bool>;
#[doc = "Field `UPSTR_RES` writer - Upstream Resume Interrupt Enable"]
pub type UPSTR_RES_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EPT_0` reader - Endpoint 0 Interrupt Enable"]
pub type EPT_0_R = crate::BitReader<bool>;
#[doc = "Field `EPT_0` writer - Endpoint 0 Interrupt Enable"]
pub type EPT_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EPT_1` reader - Endpoint 1 Interrupt Enable"]
pub type EPT_1_R = crate::BitReader<bool>;
#[doc = "Field `EPT_1` writer - Endpoint 1 Interrupt Enable"]
pub type EPT_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EPT_2` reader - Endpoint 2 Interrupt Enable"]
pub type EPT_2_R = crate::BitReader<bool>;
#[doc = "Field `EPT_2` writer - Endpoint 2 Interrupt Enable"]
pub type EPT_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EPT_3` reader - Endpoint 3 Interrupt Enable"]
pub type EPT_3_R = crate::BitReader<bool>;
#[doc = "Field `EPT_3` writer - Endpoint 3 Interrupt Enable"]
pub type EPT_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EPT_4` reader - Endpoint 4 Interrupt Enable"]
pub type EPT_4_R = crate::BitReader<bool>;
#[doc = "Field `EPT_4` writer - Endpoint 4 Interrupt Enable"]
pub type EPT_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EPT_5` reader - Endpoint 5 Interrupt Enable"]
pub type EPT_5_R = crate::BitReader<bool>;
#[doc = "Field `EPT_5` writer - Endpoint 5 Interrupt Enable"]
pub type EPT_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EPT_6` reader - Endpoint 6 Interrupt Enable"]
pub type EPT_6_R = crate::BitReader<bool>;
#[doc = "Field `EPT_6` writer - Endpoint 6 Interrupt Enable"]
pub type EPT_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DMA_1` reader - DMA Channel 1 Interrupt Enable"]
pub type DMA_1_R = crate::BitReader<bool>;
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Enable"]
pub type DMA_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DMA_2` reader - DMA Channel 2 Interrupt Enable"]
pub type DMA_2_R = crate::BitReader<bool>;
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Enable"]
pub type DMA_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DMA_3` reader - DMA Channel 3 Interrupt Enable"]
pub type DMA_3_R = crate::BitReader<bool>;
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Enable"]
pub type DMA_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DMA_4` reader - DMA Channel 4 Interrupt Enable"]
pub type DMA_4_R = crate::BitReader<bool>;
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Enable"]
pub type DMA_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DMA_5` reader - DMA Channel 5 Interrupt Enable"]
pub type DMA_5_R = crate::BitReader<bool>;
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Enable"]
pub type DMA_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DMA_6` reader - DMA Channel 6 Interrupt Enable"]
pub type DMA_6_R = crate::BitReader<bool>;
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Enable"]
pub type DMA_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn det_suspd(&self) -> DET_SUSPD_R {
        DET_SUSPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Micro-SOF Interrupt Enable"]
    #[inline(always)]
    pub fn micro_sof(&self) -> MICRO_SOF_R {
        MICRO_SOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOF Interrupt Enable"]
    #[inline(always)]
    pub fn int_sof(&self) -> INT_SOF_R {
        INT_SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End Of Reset Interrupt Enable"]
    #[inline(always)]
    pub fn endreset(&self) -> ENDRESET_R {
        ENDRESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wake Up CPU Interrupt Enable"]
    #[inline(always)]
    pub fn wake_up(&self) -> WAKE_UP_R {
        WAKE_UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End Of Resume Interrupt Enable"]
    #[inline(always)]
    pub fn endofrsm(&self) -> ENDOFRSM_R {
        ENDOFRSM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    pub fn upstr_res(&self) -> UPSTR_RES_R {
        UPSTR_RES_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_0(&self) -> EPT_0_R {
        EPT_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_1(&self) -> EPT_1_R {
        EPT_1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_2(&self) -> EPT_2_R {
        EPT_2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_3(&self) -> EPT_3_R {
        EPT_3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_4(&self) -> EPT_4_R {
        EPT_4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_5(&self) -> EPT_5_R {
        EPT_5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Endpoint 6 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_6(&self) -> EPT_6_R {
        EPT_6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn det_suspd(&mut self) -> DET_SUSPD_W<1> {
        DET_SUSPD_W::new(self)
    }
    #[doc = "Bit 2 - Micro-SOF Interrupt Enable"]
    #[inline(always)]
    pub fn micro_sof(&mut self) -> MICRO_SOF_W<2> {
        MICRO_SOF_W::new(self)
    }
    #[doc = "Bit 3 - SOF Interrupt Enable"]
    #[inline(always)]
    pub fn int_sof(&mut self) -> INT_SOF_W<3> {
        INT_SOF_W::new(self)
    }
    #[doc = "Bit 4 - End Of Reset Interrupt Enable"]
    #[inline(always)]
    pub fn endreset(&mut self) -> ENDRESET_W<4> {
        ENDRESET_W::new(self)
    }
    #[doc = "Bit 5 - Wake Up CPU Interrupt Enable"]
    #[inline(always)]
    pub fn wake_up(&mut self) -> WAKE_UP_W<5> {
        WAKE_UP_W::new(self)
    }
    #[doc = "Bit 6 - End Of Resume Interrupt Enable"]
    #[inline(always)]
    pub fn endofrsm(&mut self) -> ENDOFRSM_W<6> {
        ENDOFRSM_W::new(self)
    }
    #[doc = "Bit 7 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    pub fn upstr_res(&mut self) -> UPSTR_RES_W<7> {
        UPSTR_RES_W::new(self)
    }
    #[doc = "Bit 8 - Endpoint 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_0(&mut self) -> EPT_0_W<8> {
        EPT_0_W::new(self)
    }
    #[doc = "Bit 9 - Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_1(&mut self) -> EPT_1_W<9> {
        EPT_1_W::new(self)
    }
    #[doc = "Bit 10 - Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_2(&mut self) -> EPT_2_W<10> {
        EPT_2_W::new(self)
    }
    #[doc = "Bit 11 - Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_3(&mut self) -> EPT_3_W<11> {
        EPT_3_W::new(self)
    }
    #[doc = "Bit 12 - Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_4(&mut self) -> EPT_4_W<12> {
        EPT_4_W::new(self)
    }
    #[doc = "Bit 13 - Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_5(&mut self) -> EPT_5_W<13> {
        EPT_5_W::new(self)
    }
    #[doc = "Bit 14 - Endpoint 6 Interrupt Enable"]
    #[inline(always)]
    pub fn ept_6(&mut self) -> EPT_6_W<14> {
        EPT_6_W::new(self)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_1(&mut self) -> DMA_1_W<25> {
        DMA_1_W::new(self)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_2(&mut self) -> DMA_2_W<26> {
        DMA_2_W::new(self)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_3(&mut self) -> DMA_3_W<27> {
        DMA_3_W::new(self)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_4(&mut self) -> DMA_4_W<28> {
        DMA_4_W::new(self)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_5(&mut self) -> DMA_5_W<29> {
        DMA_5_W::new(self)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_6(&mut self) -> DMA_6_W<30> {
        DMA_6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0x10"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
