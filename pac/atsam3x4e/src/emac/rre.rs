#[doc = "Register `RRE` reader"]
pub type R = crate::R<RRE_SPEC>;
#[doc = "Register `RRE` writer"]
pub type W = crate::W<RRE_SPEC>;
#[doc = "Field `RRE` reader - Receive Resource Errors"]
pub type RRE_R = crate::FieldReader<u16>;
#[doc = "Field `RRE` writer - Receive Resource Errors"]
pub type RRE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Resource Errors"]
    #[inline(always)]
    pub fn rre(&self) -> RRE_R {
        RRE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Resource Errors"]
    #[inline(always)]
    #[must_use]
    pub fn rre(&mut self) -> RRE_W<RRE_SPEC> {
        RRE_W::new(self, 0)
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
#[doc = "Receive Resource Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rre::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rre::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RRE_SPEC;
impl crate::RegisterSpec for RRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rre::R`](R) reader structure"]
impl crate::Readable for RRE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rre::W`](W) writer structure"]
impl crate::Writable for RRE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RRE to value 0"]
impl crate::Resettable for RRE_SPEC {
    const RESET_VALUE: u32 = 0;
}
