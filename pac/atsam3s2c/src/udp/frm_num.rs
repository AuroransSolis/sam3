#[doc = "Register `FRM_NUM` reader"]
pub type R = crate::R<FRM_NUM_SPEC>;
#[doc = "Field `FRM_NUM` reader - Frame Number as Defined in the Packet Field Formats"]
pub type FRM_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `FRM_ERR` reader - Frame Error"]
pub type FRM_ERR_R = crate::BitReader;
#[doc = "Field `FRM_OK` reader - Frame OK"]
pub type FRM_OK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - Frame Number as Defined in the Packet Field Formats"]
    #[inline(always)]
    pub fn frm_num(&self) -> FRM_NUM_R {
        FRM_NUM_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - Frame Error"]
    #[inline(always)]
    pub fn frm_err(&self) -> FRM_ERR_R {
        FRM_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Frame OK"]
    #[inline(always)]
    pub fn frm_ok(&self) -> FRM_OK_R {
        FRM_OK_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frm_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRM_NUM_SPEC;
impl crate::RegisterSpec for FRM_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frm_num::R`](R) reader structure"]
impl crate::Readable for FRM_NUM_SPEC {}
#[doc = "`reset()` method sets FRM_NUM to value 0"]
impl crate::Resettable for FRM_NUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
