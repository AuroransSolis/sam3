#[doc = "Register `MATRIX_WPSR` reader"]
pub type R = crate::R<MatrixWpsrSpec>;
#[doc = "Field `WPVS` reader - Write Protect Violation Status"]
pub type WpvsR = crate::BitReader;
#[doc = "Field `WPVSRC` reader - Write Protect Violation Source"]
pub type WpvsrcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Write Protect Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WpvsR {
        WpvsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:23 - Write Protect Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WpvsrcR {
        WpvsrcR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[doc = "Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_wpsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatrixWpsrSpec;
impl crate::RegisterSpec for MatrixWpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`matrix_wpsr::R`](R) reader structure"]
impl crate::Readable for MatrixWpsrSpec {}
#[doc = "`reset()` method sets MATRIX_WPSR to value 0"]
impl crate::Resettable for MatrixWpsrSpec {
    const RESET_VALUE: u32 = 0;
}
