#[doc = "Register `RSE` reader"]
pub type R = crate::R<RSE_SPEC>;
#[doc = "Register `RSE` writer"]
pub type W = crate::W<RSE_SPEC>;
#[doc = "Field `RSE` reader - Receive Symbol Errors"]
pub type RSE_R = crate::FieldReader;
#[doc = "Field `RSE` writer - Receive Symbol Errors"]
pub type RSE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive Symbol Errors"]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Symbol Errors"]
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RSE_W<RSE_SPEC> {
        RSE_W::new(self, 0)
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
#[doc = "Receive Symbol Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSE_SPEC;
impl crate::RegisterSpec for RSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rse::R`](R) reader structure"]
impl crate::Readable for RSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rse::W`](W) writer structure"]
impl crate::Writable for RSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSE to value 0"]
impl crate::Resettable for RSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
