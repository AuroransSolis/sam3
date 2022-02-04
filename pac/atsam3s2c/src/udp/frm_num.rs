#[doc = "Register `FRM_NUM` reader"]
pub struct R(crate::R<FRM_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRM_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRM_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRM_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRM_NUM` reader - Frame Number as Defined in the Packet Field Formats"]
pub struct FRM_NUM_R(crate::FieldReader<u16, u16>);
impl FRM_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FRM_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRM_NUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRM_ERR` reader - Frame Error"]
pub struct FRM_ERR_R(crate::FieldReader<bool, bool>);
impl FRM_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRM_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRM_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRM_OK` reader - Frame OK"]
pub struct FRM_OK_R(crate::FieldReader<bool, bool>);
impl FRM_OK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRM_OK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRM_OK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:10 - Frame Number as Defined in the Packet Field Formats"]
    #[inline(always)]
    pub fn frm_num(&self) -> FRM_NUM_R {
        FRM_NUM_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - Frame Error"]
    #[inline(always)]
    pub fn frm_err(&self) -> FRM_ERR_R {
        FRM_ERR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Frame OK"]
    #[inline(always)]
    pub fn frm_ok(&self) -> FRM_OK_R {
        FRM_OK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frm_num](index.html) module"]
pub struct FRM_NUM_SPEC;
impl crate::RegisterSpec for FRM_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frm_num::R](R) reader structure"]
impl crate::Readable for FRM_NUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRM_NUM to value 0"]
impl crate::Resettable for FRM_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
