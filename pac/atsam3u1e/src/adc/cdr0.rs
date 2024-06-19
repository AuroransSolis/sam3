#[doc = "Register `CDR0` reader"]
pub type R = crate::R<Cdr0Spec>;
#[doc = "Field `DATA` reader - Converted Data"]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Channel Data Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdr0Spec;
impl crate::RegisterSpec for Cdr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr0::R`](R) reader structure"]
impl crate::Readable for Cdr0Spec {}
