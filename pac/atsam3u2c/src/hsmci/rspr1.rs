#[doc = "Register `RSPR1` reader"]
pub type R = crate::R<RSPR1_SPEC>;
#[doc = "Field `RSP` reader - Response"]
pub type RSP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Response"]
    #[inline(always)]
    pub fn rsp(&self) -> RSP_R {
        RSP_R::new(self.bits)
    }
}
#[doc = "Response Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSPR1_SPEC;
impl crate::RegisterSpec for RSPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rspr1::R`](R) reader structure"]
impl crate::Readable for RSPR1_SPEC {}
