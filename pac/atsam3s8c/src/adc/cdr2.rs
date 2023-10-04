#[doc = "Register `CDR2` reader"]
pub type R = crate::R<CDR2_SPEC>;
#[doc = "Field `DATA` reader - Converted Data"]
pub type DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Channel Data Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDR2_SPEC;
impl crate::RegisterSpec for CDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr2::R`](R) reader structure"]
impl crate::Readable for CDR2_SPEC {}
