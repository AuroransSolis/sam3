#[doc = "Register `RSPR3` reader"]
pub type R = crate::R<Rspr3Spec>;
#[doc = "Field `RSP` reader - Response"]
pub type RspR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Response"]
    #[inline(always)]
    pub fn rsp(&self) -> RspR {
        RspR::new(self.bits)
    }
}
#[doc = "Response Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rspr3Spec;
impl crate::RegisterSpec for Rspr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rspr3::R`](R) reader structure"]
impl crate::Readable for Rspr3Spec {}
