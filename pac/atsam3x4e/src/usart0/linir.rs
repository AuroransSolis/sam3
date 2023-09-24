#[doc = "Register `LINIR` reader"]
pub type R = crate::R<LINIR_SPEC>;
#[doc = "Register `LINIR` writer"]
pub type W = crate::W<LINIR_SPEC>;
#[doc = "Field `IDCHR` reader - Identifier Character"]
pub type IDCHR_R = crate::FieldReader;
#[doc = "Field `IDCHR` writer - Identifier Character"]
pub type IDCHR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Identifier Character"]
    #[inline(always)]
    pub fn idchr(&self) -> IDCHR_R {
        IDCHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Identifier Character"]
    #[inline(always)]
    #[must_use]
    pub fn idchr(&mut self) -> IDCHR_W<LINIR_SPEC, 0> {
        IDCHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LIN Identifier Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINIR_SPEC;
impl crate::RegisterSpec for LINIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linir::R`](R) reader structure"]
impl crate::Readable for LINIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`linir::W`](W) writer structure"]
impl crate::Writable for LINIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINIR to value 0"]
impl crate::Resettable for LINIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
