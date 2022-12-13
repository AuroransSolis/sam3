#[doc = "Register `MATRIX_WPSR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<MATRIX_WPSR_SPEC>);
#[doc = "Field `WPVS` reader - Write Protect Violation Status"]
pub type WPVS_R = crate::BitReader<bool>;
#[doc = "Field `WPVSRC` reader - Write Protect Violation Source"]
pub type WPVSRC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Write Protect Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:23 - Write Protect Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[doc = "Write Protect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_wpsr](index.html) module"]
pub struct MATRIX_WPSR_SPEC;
impl crate::RegisterSpec for MATRIX_WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrix_wpsr::R](R) reader structure"]
impl crate::Readable for MATRIX_WPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MATRIX_WPSR to value 0"]
impl crate::Resettable for MATRIX_WPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
