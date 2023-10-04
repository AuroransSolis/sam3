#[doc = "Register `CDR8` reader"]
pub type R = crate::R<CDR8_SPEC>;
#[doc = "Field `DATA` reader - Converted Data"]
pub type DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Channel Data Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDR8_SPEC;
impl crate::RegisterSpec for CDR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr8::R`](R) reader structure"]
impl crate::Readable for CDR8_SPEC {}
