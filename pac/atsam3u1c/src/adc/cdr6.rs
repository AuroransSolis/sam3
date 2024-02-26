#[doc = "Register `CDR6` reader"]
pub type R = crate::R<Cdr6Spec>;
#[doc = "Field `DATA` reader - Converted Data"]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Channel Data Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdr6Spec;
impl crate::RegisterSpec for Cdr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr6::R`](R) reader structure"]
impl crate::Readable for Cdr6Spec {}
