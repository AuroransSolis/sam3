#[doc = "Register `TPR` reader"]
pub type R = crate::R<TPR_SPEC>;
#[doc = "Register `TPR` writer"]
pub type W = crate::W<TPR_SPEC>;
#[doc = "Field `TXPTR` reader - Transmit Counter Register"]
pub type TXPTR_R = crate::FieldReader<u32>;
#[doc = "Field `TXPTR` writer - Transmit Counter Register"]
pub type TXPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Counter Register"]
    #[inline(always)]
    pub fn txptr(&self) -> TXPTR_R {
        TXPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Counter Register"]
    #[inline(always)]
    #[must_use]
    pub fn txptr(&mut self) -> TXPTR_W<TPR_SPEC> {
        TXPTR_W::new(self, 0)
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
#[doc = "Transmit Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPR_SPEC;
impl crate::RegisterSpec for TPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpr::R`](R) reader structure"]
impl crate::Readable for TPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpr::W`](W) writer structure"]
impl crate::Writable for TPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TPR to value 0"]
impl crate::Resettable for TPR_SPEC {
    const RESET_VALUE: u32 = 0;
}
