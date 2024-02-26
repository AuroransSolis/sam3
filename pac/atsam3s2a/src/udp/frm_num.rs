#[doc = "Register `FRM_NUM` reader"]
pub type R = crate::R<FrmNumSpec>;
#[doc = "Field `FRM_NUM` reader - Frame Number as Defined in the Packet Field Formats"]
pub type FrmNumR = crate::FieldReader<u16>;
#[doc = "Field `FRM_ERR` reader - Frame Error"]
pub type FrmErrR = crate::BitReader;
#[doc = "Field `FRM_OK` reader - Frame OK"]
pub type FrmOkR = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - Frame Number as Defined in the Packet Field Formats"]
    #[inline(always)]
    pub fn frm_num(&self) -> FrmNumR {
        FrmNumR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - Frame Error"]
    #[inline(always)]
    pub fn frm_err(&self) -> FrmErrR {
        FrmErrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Frame OK"]
    #[inline(always)]
    pub fn frm_ok(&self) -> FrmOkR {
        FrmOkR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frm_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrmNumSpec;
impl crate::RegisterSpec for FrmNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frm_num::R`](R) reader structure"]
impl crate::Readable for FrmNumSpec {}
#[doc = "`reset()` method sets FRM_NUM to value 0"]
impl crate::Resettable for FrmNumSpec {
    const RESET_VALUE: u32 = 0;
}
