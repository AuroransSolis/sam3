#[doc = "Register `MATRIX_WPMR` reader"]
pub type R = crate::R<MatrixWpmrSpec>;
#[doc = "Register `MATRIX_WPMR` writer"]
pub type W = crate::W<MatrixWpmrSpec>;
#[doc = "Field `WPEN` reader - Write Protect ENable"]
pub type WpenR = crate::BitReader;
#[doc = "Field `WPEN` writer - Write Protect ENable"]
pub type WpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPKEY` reader - Write Protect KEY (Write-only)"]
pub type WpkeyR = crate::FieldReader<u32>;
#[doc = "Field `WPKEY` writer - Write Protect KEY (Write-only)"]
pub type WpkeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Write Protect ENable"]
    #[inline(always)]
    pub fn wpen(&self) -> WpenR {
        WpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY (Write-only)"]
    #[inline(always)]
    pub fn wpkey(&self) -> WpkeyR {
        WpkeyR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protect ENable"]
    #[inline(always)]
    #[must_use]
    pub fn wpen(&mut self) -> WpenW<MatrixWpmrSpec> {
        WpenW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY (Write-only)"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WpkeyW<MatrixWpmrSpec> {
        WpkeyW::new(self, 8)
    }
}
#[doc = "Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_wpmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_wpmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatrixWpmrSpec;
impl crate::RegisterSpec for MatrixWpmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`matrix_wpmr::R`](R) reader structure"]
impl crate::Readable for MatrixWpmrSpec {}
#[doc = "`write(|w| ..)` method takes [`matrix_wpmr::W`](W) writer structure"]
impl crate::Writable for MatrixWpmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MATRIX_WPMR to value 0"]
impl crate::Resettable for MatrixWpmrSpec {
    const RESET_VALUE: u32 = 0;
}
