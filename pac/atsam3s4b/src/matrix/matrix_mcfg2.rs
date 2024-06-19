#[doc = "Register `MATRIX_MCFG2` reader"]
pub type R = crate::R<MatrixMcfg2Spec>;
#[doc = "Register `MATRIX_MCFG2` writer"]
pub type W = crate::W<MatrixMcfg2Spec>;
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub type UlbtR = crate::FieldReader;
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub type UlbtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> UlbtR {
        UlbtR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    #[must_use]
    pub fn ulbt(&mut self) -> UlbtW<MatrixMcfg2Spec> {
        UlbtW::new(self, 0)
    }
}
#[doc = "Master Configuration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_mcfg2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_mcfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatrixMcfg2Spec;
impl crate::RegisterSpec for MatrixMcfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`matrix_mcfg2::R`](R) reader structure"]
impl crate::Readable for MatrixMcfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`matrix_mcfg2::W`](W) writer structure"]
impl crate::Writable for MatrixMcfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
