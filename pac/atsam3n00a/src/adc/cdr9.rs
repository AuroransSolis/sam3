#[doc = "Register `CDR9` reader"]
pub type R = crate::R<Cdr9Spec>;
#[doc = "Field `DATA` reader - Converted Data"]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Channel Data Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr9::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdr9Spec;
impl crate::RegisterSpec for Cdr9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr9::R`](R) reader structure"]
impl crate::Readable for Cdr9Spec {}
