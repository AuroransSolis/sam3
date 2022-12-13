#[doc = "Register `LINBRR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<LINBRR_SPEC>);
#[doc = "Field `LINCD` reader - Clock Divider after Synchronization"]
pub type LINCD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINFP` reader - Fractional Part after Synchronization"]
pub type LINFP_R = crate::FieldReader<u8, u8>;
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
#[doc = "LIN Baud Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linbrr](index.html) module"]
pub struct LINBRR_SPEC;
impl crate::RegisterSpec for LINBRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [linbrr::R](R) reader structure"]
impl crate::Readable for LINBRR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LINBRR to value 0"]
impl crate::Resettable for LINBRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
