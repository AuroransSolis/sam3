#[doc = "Register `CDR3` reader"]
pub type R = crate::R<CDR3_SPEC>;
#[doc = "Field `DATA` reader - Converted Data"]
pub type DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Channel Data Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDR3_SPEC;
impl crate::RegisterSpec for CDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr3::R`](R) reader structure"]
impl crate::Readable for CDR3_SPEC {}
