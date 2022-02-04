#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFD` reader - Management Frame sent"]
pub struct MFD_R(crate::FieldReader<bool, bool>);
impl MFD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCOMP` reader - Receive Complete"]
pub struct RCOMP_R(crate::FieldReader<bool, bool>);
impl RCOMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCOMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUBR` reader - Receive Used Bit Read"]
pub struct RXUBR_R(crate::FieldReader<bool, bool>);
impl RXUBR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXUBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUBR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUBR` reader - Transmit Used Bit Read"]
pub struct TXUBR_R(crate::FieldReader<bool, bool>);
impl TXUBR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXUBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUBR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TUND` reader - Ethernet Transmit Buffer Underrun"]
pub struct TUND_R(crate::FieldReader<bool, bool>);
impl TUND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TUND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TUND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLE` reader - Retry Limit Exceeded"]
pub struct RLE_R(crate::FieldReader<bool, bool>);
impl RLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXERR` reader - "]
pub struct TXERR_R(crate::FieldReader<bool, bool>);
impl TXERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCOMP` reader - Transmit Complete"]
pub struct TCOMP_R(crate::FieldReader<bool, bool>);
impl TCOMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCOMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub struct ROVR_R(crate::FieldReader<bool, bool>);
impl ROVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRESP` reader - Hresp not OK"]
pub struct HRESP_R(crate::FieldReader<bool, bool>);
impl HRESP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRESP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PFR` reader - Pause Frame Received"]
pub struct PFR_R(crate::FieldReader<bool, bool>);
impl PFR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PFR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTZ` reader - Pause Time Zero"]
pub struct PTZ_R(crate::FieldReader<bool, bool>);
impl PTZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PTZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Management Frame sent"]
    #[inline(always)]
    pub fn mfd(&self) -> MFD_R {
        MFD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&self) -> RCOMP_R {
        RCOMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&self) -> RXUBR_R {
        RXUBR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Used Bit Read"]
    #[inline(always)]
    pub fn txubr(&self) -> TXUBR_R {
        TXUBR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Ethernet Transmit Buffer Underrun"]
    #[inline(always)]
    pub fn tund(&self) -> TUND_R {
        TUND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded"]
    #[inline(always)]
    pub fn rle(&self) -> RLE_R {
        RLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TCOMP_R {
        TCOMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Hresp not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HRESP_R {
        HRESP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pause Frame Received"]
    #[inline(always)]
    pub fn pfr(&self) -> PFR_R {
        PFR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    pub fn ptz(&self) -> PTZ_R {
        PTZ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0x3fff"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fff
    }
}
