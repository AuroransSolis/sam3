#[doc = "Register `WPSR` reader"]
pub type R = crate::R<WPSR_SPEC>;
#[doc = "Field `WPROTERR` reader - Write PROTection ERRor"]
pub type WPROTERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write PROTection ERRor"]
    #[inline(always)]
    pub fn wproterr(&self) -> WPROTERR_R {
        WPROTERR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPSR_SPEC;
impl crate::RegisterSpec for WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsr::R`](R) reader structure"]
impl crate::Readable for WPSR_SPEC {}
#[doc = "`reset()` method sets WPSR to value 0"]
impl crate::Resettable for WPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
