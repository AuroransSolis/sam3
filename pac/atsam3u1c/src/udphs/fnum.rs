#[doc = "Register `FNUM` reader"]
pub struct R(crate::R<FNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MICRO_FRAME_NUM` reader - Microframe Number"]
pub type MICRO_FRAME_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAME_NUMBER` reader - Frame Number as defined in the Packet Field Formats"]
pub type FRAME_NUMBER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FNUM_ERR` reader - Frame Number CRC Error"]
pub type FNUM_ERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - Microframe Number"]
    #[inline(always)]
    pub fn micro_frame_num(&self) -> MICRO_FRAME_NUM_R {
        MICRO_FRAME_NUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number as defined in the Packet Field Formats"]
    #[inline(always)]
    pub fn frame_number(&self) -> FRAME_NUMBER_R {
        FRAME_NUMBER_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - Frame Number CRC Error"]
    #[inline(always)]
    pub fn fnum_err(&self) -> FNUM_ERR_R {
        FNUM_ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "UDPHS Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fnum](index.html) module"]
pub struct FNUM_SPEC;
impl crate::RegisterSpec for FNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fnum::R](R) reader structure"]
impl crate::Readable for FNUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FNUM to value 0"]
impl crate::Resettable for FNUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
