#[doc = "Register `RSPR3` reader"]
pub type R = crate::R<RSPR3_SPEC>;
#[doc = "Field `RSP` reader - Response"]
pub type RSP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Response"]
    #[inline(always)]
    pub fn rsp(&self) -> RSP_R {
        RSP_R::new(self.bits)
    }
}
#[doc = "Response Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSPR3_SPEC;
impl crate::RegisterSpec for RSPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rspr3::R`](R) reader structure"]
impl crate::Readable for RSPR3_SPEC {}
