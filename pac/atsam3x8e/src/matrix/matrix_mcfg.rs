#[doc = "Register `MATRIX_MCFG[%s]` reader"]
pub type R = crate::R<MATRIX_MCFG_SPEC>;
#[doc = "Register `MATRIX_MCFG[%s]` writer"]
pub type W = crate::W<MATRIX_MCFG_SPEC>;
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub type ULBT_R = crate::FieldReader;
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub type ULBT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> ULBT_R {
        ULBT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    #[must_use]
    pub fn ulbt(&mut self) -> ULBT_W<MATRIX_MCFG_SPEC, 0> {
        ULBT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Master Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_mcfg::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_mcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MATRIX_MCFG_SPEC;
impl crate::RegisterSpec for MATRIX_MCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`matrix_mcfg::R`](R) reader structure"]
impl crate::Readable for MATRIX_MCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`matrix_mcfg::W`](W) writer structure"]
impl crate::Writable for MATRIX_MCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
