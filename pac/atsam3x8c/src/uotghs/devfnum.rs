#[doc = "Register `DEVFNUM` reader"]
pub type R = crate::R<DevfnumSpec>;
#[doc = "Field `MFNUM` reader - Micro Frame Number"]
pub type MfnumR = crate::FieldReader;
#[doc = "Field `FNUM` reader - Frame Number"]
pub type FnumR = crate::FieldReader<u16>;
#[doc = "Field `FNCERR` reader - Frame Number CRC Error"]
pub type FncerrR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MfnumR {
        MfnumR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FnumR {
        FnumR::new(((self.bits >> 3) & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Frame Number CRC Error"]
    #[inline(always)]
    pub fn fncerr(&self) -> FncerrR {
        FncerrR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Device Frame Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devfnum::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevfnumSpec;
impl crate::RegisterSpec for DevfnumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devfnum::R`](R) reader structure"]
impl crate::Readable for DevfnumSpec {}
#[doc = "`reset()` method sets DEVFNUM to value 0"]
impl crate::Resettable for DevfnumSpec {
    const RESET_VALUE: u32 = 0;
}
