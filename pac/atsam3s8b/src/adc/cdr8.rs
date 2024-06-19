#[doc = "Register `CDR8` reader"]
pub type R = crate::R<Cdr8Spec>;
#[doc = "Field `DATA` reader - Converted Data"]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Channel Data Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdr8Spec;
impl crate::RegisterSpec for Cdr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr8::R`](R) reader structure"]
impl crate::Readable for Cdr8Spec {}
