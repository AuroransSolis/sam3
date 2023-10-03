#[doc = "Register `CSE` reader"]
pub type R = crate::R<CSE_SPEC>;
#[doc = "Register `CSE` writer"]
pub type W = crate::W<CSE_SPEC>;
#[doc = "Field `CSE` reader - Carrier Sense Errors"]
pub type CSE_R = crate::FieldReader;
#[doc = "Field `CSE` writer - Carrier Sense Errors"]
pub type CSE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Carrier Sense Errors"]
    #[inline(always)]
    pub fn cse(&self) -> CSE_R {
        CSE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Carrier Sense Errors"]
    #[inline(always)]
    #[must_use]
    pub fn cse(&mut self) -> CSE_W<CSE_SPEC, 0> {
        CSE_W::new(self)
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
#[doc = "Carrier Sense Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSE_SPEC;
impl crate::RegisterSpec for CSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cse::R`](R) reader structure"]
impl crate::Readable for CSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cse::W`](W) writer structure"]
impl crate::Writable for CSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSE to value 0"]
impl crate::Resettable for CSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
