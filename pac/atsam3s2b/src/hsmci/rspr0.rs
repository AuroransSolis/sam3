#[doc = "Register `RSPR0` reader"]
pub type R = crate::R<Rspr0Spec>;
#[doc = "Field `RSP` reader - Response"]
pub type RspR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Response"]
    #[inline(always)]
    pub fn rsp(&self) -> RspR {
        RspR::new(self.bits)
    }
}
#[doc = "Response Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rspr0Spec;
impl crate::RegisterSpec for Rspr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rspr0::R`](R) reader structure"]
impl crate::Readable for Rspr0Spec {}
