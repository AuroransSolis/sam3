#[doc = "Register `CDR2` reader"]
pub type R = crate::R<Cdr2Spec>;
#[doc = "Field `DATA` reader - Converted Data"]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Channel Data Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdr2Spec;
impl crate::RegisterSpec for Cdr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr2::R`](R) reader structure"]
impl crate::Readable for Cdr2Spec {}
