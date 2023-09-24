#[doc = "Register `LINBRR` reader"]
pub type R = crate::R<LINBRR_SPEC>;
#[doc = "Field `LINCD` reader - Clock Divider after Synchronization"]
pub type LINCD_R = crate::FieldReader<u16>;
#[doc = "Field `LINFP` reader - Fractional Part after Synchronization"]
pub type LINFP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Clock Divider after Synchronization"]
    #[inline(always)]
    pub fn lincd(&self) -> LINCD_R {
        LINCD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Fractional Part after Synchronization"]
    #[inline(always)]
    pub fn linfp(&self) -> LINFP_R {
        LINFP_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "LIN Baud Rate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linbrr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINBRR_SPEC;
impl crate::RegisterSpec for LINBRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linbrr::R`](R) reader structure"]
impl crate::Readable for LINBRR_SPEC {}
#[doc = "`reset()` method sets LINBRR to value 0"]
impl crate::Resettable for LINBRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
