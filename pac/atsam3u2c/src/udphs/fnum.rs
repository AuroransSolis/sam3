#[doc = "Register `FNUM` reader"]
pub type R = crate::R<FnumSpec>;
#[doc = "Field `MICRO_FRAME_NUM` reader - Microframe Number"]
pub type MicroFrameNumR = crate::FieldReader;
#[doc = "Field `FRAME_NUMBER` reader - Frame Number as defined in the Packet Field Formats"]
pub type FrameNumberR = crate::FieldReader<u16>;
#[doc = "Field `FNUM_ERR` reader - Frame Number CRC Error"]
pub type FnumErrR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Microframe Number"]
    #[inline(always)]
    pub fn micro_frame_num(&self) -> MicroFrameNumR {
        MicroFrameNumR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number as defined in the Packet Field Formats"]
    #[inline(always)]
    pub fn frame_number(&self) -> FrameNumberR {
        FrameNumberR::new(((self.bits >> 3) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - Frame Number CRC Error"]
    #[inline(always)]
    pub fn fnum_err(&self) -> FnumErrR {
        FnumErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "UDPHS Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnum::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FnumSpec;
impl crate::RegisterSpec for FnumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fnum::R`](R) reader structure"]
impl crate::Readable for FnumSpec {}
#[doc = "`reset()` method sets FNUM to value 0"]
impl crate::Resettable for FnumSpec {
    const RESET_VALUE: u32 = 0;
}
