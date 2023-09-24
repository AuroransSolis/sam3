#[doc = "Register `STE` reader"]
pub type R = crate::R<STE_SPEC>;
#[doc = "Register `STE` writer"]
pub type W = crate::W<STE_SPEC>;
#[doc = "Field `SQER` reader - SQE test errors"]
pub type SQER_R = crate::FieldReader;
#[doc = "Field `SQER` writer - SQE test errors"]
pub type SQER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SQE test errors"]
    #[inline(always)]
    pub fn sqer(&self) -> SQER_R {
        SQER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SQE test errors"]
    #[inline(always)]
    #[must_use]
    pub fn sqer(&mut self) -> SQER_W<STE_SPEC, 0> {
        SQER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SQE Test Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ste::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ste::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STE_SPEC;
impl crate::RegisterSpec for STE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ste::R`](R) reader structure"]
impl crate::Readable for STE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ste::W`](W) writer structure"]
impl crate::Writable for STE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STE to value 0"]
impl crate::Resettable for STE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
