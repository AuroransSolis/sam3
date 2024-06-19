#[doc = "Register `CDR4` reader"]
pub type R = crate::R<Cdr4Spec>;
#[doc = "Field `DATA` reader - Converted Data"]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Channel Data Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdr4Spec;
impl crate::RegisterSpec for Cdr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr4::R`](R) reader structure"]
impl crate::Readable for Cdr4Spec {}
