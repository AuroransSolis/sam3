#[doc = "Register `LINBRR` reader"]
pub type R = crate::R<LinbrrSpec>;
#[doc = "Field `LINCD` reader - Clock Divider after Synchronization"]
pub type LincdR = crate::FieldReader<u16>;
#[doc = "Field `LINFP` reader - Fractional Part after Synchronization"]
pub type LinfpR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Clock Divider after Synchronization"]
    #[inline(always)]
    pub fn lincd(&self) -> LincdR {
        LincdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Fractional Part after Synchronization"]
    #[inline(always)]
    pub fn linfp(&self) -> LinfpR {
        LinfpR::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "LIN Baud Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`linbrr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinbrrSpec;
impl crate::RegisterSpec for LinbrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linbrr::R`](R) reader structure"]
impl crate::Readable for LinbrrSpec {}
#[doc = "`reset()` method sets LINBRR to value 0"]
impl crate::Resettable for LinbrrSpec {
    const RESET_VALUE: u32 = 0;
}
