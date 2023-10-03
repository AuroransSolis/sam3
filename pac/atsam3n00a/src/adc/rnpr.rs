#[doc = "Register `RNPR` reader"]
pub type R = crate::R<RNPR_SPEC>;
#[doc = "Register `RNPR` writer"]
pub type W = crate::W<RNPR_SPEC>;
#[doc = "Field `RXNPTR` reader - Receive Next Pointer"]
pub type RXNPTR_R = crate::FieldReader<u32>;
#[doc = "Field `RXNPTR` writer - Receive Next Pointer"]
pub type RXNPTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Next Pointer"]
    #[inline(always)]
    pub fn rxnptr(&self) -> RXNPTR_R {
        RXNPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Next Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn rxnptr(&mut self) -> RXNPTR_W<RNPR_SPEC, 0> {
        RXNPTR_W::new(self)
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
#[doc = "Receive Next Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNPR_SPEC;
impl crate::RegisterSpec for RNPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rnpr::R`](R) reader structure"]
impl crate::Readable for RNPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rnpr::W`](W) writer structure"]
impl crate::Writable for RNPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RNPR to value 0"]
impl crate::Resettable for RNPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
