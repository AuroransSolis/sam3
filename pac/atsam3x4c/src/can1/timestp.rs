#[doc = "Register `TIMESTP` reader"]
pub type R = crate::R<TIMESTP_SPEC>;
#[doc = "Field `MTIMESTAMP` reader - Timestamp"]
pub type MTIMESTAMP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp"]
    #[inline(always)]
    pub fn mtimestamp(&self) -> MTIMESTAMP_R {
        MTIMESTAMP_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timestamp Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMESTP_SPEC;
impl crate::RegisterSpec for TIMESTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestp::R`](R) reader structure"]
impl crate::Readable for TIMESTP_SPEC {}
#[doc = "`reset()` method sets TIMESTP to value 0"]
impl crate::Resettable for TIMESTP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
