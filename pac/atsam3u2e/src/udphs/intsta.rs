#[doc = "Register `INTSTA` reader"]
pub struct R(crate::R<INTSTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPEED` reader - Speed Status"]
pub struct SPEED_R(crate::FieldReader<bool, bool>);
impl SPEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPEED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DET_SUSPD` reader - Suspend Interrupt"]
pub struct DET_SUSPD_R(crate::FieldReader<bool, bool>);
impl DET_SUSPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DET_SUSPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DET_SUSPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MICRO_SOF` reader - Micro Start Of Frame Interrupt"]
pub struct MICRO_SOF_R(crate::FieldReader<bool, bool>);
impl MICRO_SOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MICRO_SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MICRO_SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_SOF` reader - Start Of Frame Interrupt"]
pub struct INT_SOF_R(crate::FieldReader<bool, bool>);
impl INT_SOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDRESET` reader - End Of Reset Interrupt"]
pub struct ENDRESET_R(crate::FieldReader<bool, bool>);
impl ENDRESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKE_UP` reader - Wake Up CPU Interrupt"]
pub struct WAKE_UP_R(crate::FieldReader<bool, bool>);
impl WAKE_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKE_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKE_UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDOFRSM` reader - End Of Resume Interrupt"]
pub struct ENDOFRSM_R(crate::FieldReader<bool, bool>);
impl ENDOFRSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDOFRSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDOFRSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPSTR_RES` reader - Upstream Resume Interrupt"]
pub struct UPSTR_RES_R(crate::FieldReader<bool, bool>);
impl UPSTR_RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPSTR_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPSTR_RES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPT_0` reader - Endpoint 0 Interrupt"]
pub struct EPT_0_R(crate::FieldReader<bool, bool>);
impl EPT_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPT_1` reader - Endpoint 1 Interrupt"]
pub struct EPT_1_R(crate::FieldReader<bool, bool>);
impl EPT_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPT_2` reader - Endpoint 2 Interrupt"]
pub struct EPT_2_R(crate::FieldReader<bool, bool>);
impl EPT_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPT_3` reader - Endpoint 3 Interrupt"]
pub struct EPT_3_R(crate::FieldReader<bool, bool>);
impl EPT_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPT_4` reader - Endpoint 4 Interrupt"]
pub struct EPT_4_R(crate::FieldReader<bool, bool>);
impl EPT_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPT_5` reader - Endpoint 5 Interrupt"]
pub struct EPT_5_R(crate::FieldReader<bool, bool>);
impl EPT_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPT_6` reader - Endpoint 6 Interrupt"]
pub struct EPT_6_R(crate::FieldReader<bool, bool>);
impl EPT_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPT_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPT_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_1` reader - DMA Channel 1 Interrupt"]
pub struct DMA_1_R(crate::FieldReader<bool, bool>);
impl DMA_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_2` reader - DMA Channel 2 Interrupt"]
pub struct DMA_2_R(crate::FieldReader<bool, bool>);
impl DMA_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_3` reader - DMA Channel 3 Interrupt"]
pub struct DMA_3_R(crate::FieldReader<bool, bool>);
impl DMA_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_4` reader - DMA Channel 4 Interrupt"]
pub struct DMA_4_R(crate::FieldReader<bool, bool>);
impl DMA_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_5` reader - DMA Channel 5 Interrupt"]
pub struct DMA_5_R(crate::FieldReader<bool, bool>);
impl DMA_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_6` reader - DMA Channel 6 Interrupt"]
pub struct DMA_6_R(crate::FieldReader<bool, bool>);
impl DMA_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Speed Status"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Suspend Interrupt"]
    #[inline(always)]
    pub fn det_suspd(&self) -> DET_SUSPD_R {
        DET_SUSPD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Micro Start Of Frame Interrupt"]
    #[inline(always)]
    pub fn micro_sof(&self) -> MICRO_SOF_R {
        MICRO_SOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start Of Frame Interrupt"]
    #[inline(always)]
    pub fn int_sof(&self) -> INT_SOF_R {
        INT_SOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End Of Reset Interrupt"]
    #[inline(always)]
    pub fn endreset(&self) -> ENDRESET_R {
        ENDRESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wake Up CPU Interrupt"]
    #[inline(always)]
    pub fn wake_up(&self) -> WAKE_UP_R {
        WAKE_UP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - End Of Resume Interrupt"]
    #[inline(always)]
    pub fn endofrsm(&self) -> ENDOFRSM_R {
        ENDOFRSM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Upstream Resume Interrupt"]
    #[inline(always)]
    pub fn upstr_res(&self) -> UPSTR_RES_R {
        UPSTR_RES_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint 0 Interrupt"]
    #[inline(always)]
    pub fn ept_0(&self) -> EPT_0_R {
        EPT_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn ept_1(&self) -> EPT_1_R {
        EPT_1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn ept_2(&self) -> EPT_2_R {
        EPT_2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn ept_3(&self) -> EPT_3_R {
        EPT_3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn ept_4(&self) -> EPT_4_R {
        EPT_4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn ept_5(&self) -> EPT_5_R {
        EPT_5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn ept_6(&self) -> EPT_6_R {
        EPT_6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
#[doc = "UDPHS Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsta](index.html) module"]
pub struct INTSTA_SPEC;
impl crate::RegisterSpec for INTSTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intsta::R](R) reader structure"]
impl crate::Readable for INTSTA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTA to value 0"]
impl crate::Resettable for INTSTA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
