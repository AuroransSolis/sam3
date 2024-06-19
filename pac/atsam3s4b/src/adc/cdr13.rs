#[doc = "Register `CDR13` reader"]
pub type R = crate::R<Cdr13Spec>;
#[doc = "Field `DATA` reader - Converted Data"]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Channel Data Register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdr13Spec;
impl crate::RegisterSpec for Cdr13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr13::R`](R) reader structure"]
impl crate::Readable for Cdr13Spec {}
