#[doc = "Register `ALE` reader"]
pub type R = crate::R<ALE_SPEC>;
#[doc = "Register `ALE` writer"]
pub type W = crate::W<ALE_SPEC>;
#[doc = "Field `ALE` reader - Alignment Errors"]
pub type ALE_R = crate::FieldReader;
#[doc = "Field `ALE` writer - Alignment Errors"]
pub type ALE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Alignment Errors"]
    #[inline(always)]
    pub fn ale(&self) -> ALE_R {
        ALE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alignment Errors"]
    #[inline(always)]
    #[must_use]
    pub fn ale(&mut self) -> ALE_W<ALE_SPEC, 0> {
        ALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Alignment Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ale::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ale::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALE_SPEC;
impl crate::RegisterSpec for ALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale::R`](R) reader structure"]
impl crate::Readable for ALE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ale::W`](W) writer structure"]
impl crate::Writable for ALE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALE to value 0"]
impl crate::Resettable for ALE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
