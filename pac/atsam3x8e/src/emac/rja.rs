#[doc = "Register `RJA` reader"]
pub type R = crate::R<RJA_SPEC>;
#[doc = "Register `RJA` writer"]
pub type W = crate::W<RJA_SPEC>;
#[doc = "Field `RJB` reader - Receive Jabbers"]
pub type RJB_R = crate::FieldReader;
#[doc = "Field `RJB` writer - Receive Jabbers"]
pub type RJB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive Jabbers"]
    #[inline(always)]
    pub fn rjb(&self) -> RJB_R {
        RJB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Jabbers"]
    #[inline(always)]
    #[must_use]
    pub fn rjb(&mut self) -> RJB_W<RJA_SPEC, 0> {
        RJB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive Jabbers Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rja::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rja::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RJA_SPEC;
impl crate::RegisterSpec for RJA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rja::R`](R) reader structure"]
impl crate::Readable for RJA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rja::W`](W) writer structure"]
impl crate::Writable for RJA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RJA to value 0"]
impl crate::Resettable for RJA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
