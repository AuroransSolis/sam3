#[doc = "Register `RA0` reader"]
pub type R = crate::R<RA0_SPEC>;
#[doc = "Register `RA0` writer"]
pub type W = crate::W<RA0_SPEC>;
#[doc = "Field `RA` reader - Register A"]
pub type RA_R = crate::FieldReader<u32>;
#[doc = "Field `RA` writer - Register A"]
pub type RA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register A"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register A"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<RA0_SPEC> {
        RA_W::new(self, 0)
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
#[doc = "Register A (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ra0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ra0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RA0_SPEC;
impl crate::RegisterSpec for RA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ra0::R`](R) reader structure"]
impl crate::Readable for RA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ra0::W`](W) writer structure"]
impl crate::Writable for RA0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RA0 to value 0"]
impl crate::Resettable for RA0_SPEC {
    const RESET_VALUE: u32 = 0;
}
