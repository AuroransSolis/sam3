#[doc = "Register `RSPR2` reader"]
pub type R = crate::R<Rspr2Spec>;
#[doc = "Field `RSP` reader - Response"]
pub type RspR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Response"]
    #[inline(always)]
    pub fn rsp(&self) -> RspR {
        RspR::new(self.bits)
    }
}
#[doc = "Response Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rspr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rspr2Spec;
impl crate::RegisterSpec for Rspr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rspr2::R`](R) reader structure"]
impl crate::Readable for Rspr2Spec {}
