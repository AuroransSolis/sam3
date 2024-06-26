#[doc = "Register `CDR14` reader"]
pub type R = crate::R<Cdr14Spec>;
#[doc = "Field `DATA` reader - Converted Data"]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Channel Data Register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdr14Spec;
impl crate::RegisterSpec for Cdr14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr14::R`](R) reader structure"]
impl crate::Readable for Cdr14Spec {}
