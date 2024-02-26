#[doc = "Register `RSPR1` reader"]
pub type R = crate::R<Rspr1Spec>;
#[doc = "Field `RSP` reader - Response"]
pub type RspR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Response"]
    #[inline(always)]
    pub fn rsp(&self) -> RspR {
        RspR::new(self.bits)
    }
}
#[doc = "Response Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rspr1Spec;
impl crate::RegisterSpec for Rspr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rspr1::R`](R) reader structure"]
impl crate::Readable for Rspr1Spec {}
