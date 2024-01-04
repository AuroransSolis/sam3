#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Field `ERRIMR` reader - CRC Error Interrupt Mask"]
pub type ERRIMR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CRC Error Interrupt Mask"]
    #[inline(always)]
    pub fn errimr(&self) -> ERRIMR_R {
        ERRIMR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "CRCCU Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
