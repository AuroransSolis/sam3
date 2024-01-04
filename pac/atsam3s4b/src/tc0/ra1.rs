#[doc = "Register `RA1` reader"]
pub type R = crate::R<RA1_SPEC>;
#[doc = "Register `RA1` writer"]
pub type W = crate::W<RA1_SPEC>;
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
    pub fn ra(&mut self) -> RA_W<RA1_SPEC> {
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
#[doc = "Register A (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ra1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ra1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RA1_SPEC;
impl crate::RegisterSpec for RA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ra1::R`](R) reader structure"]
impl crate::Readable for RA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ra1::W`](W) writer structure"]
impl crate::Writable for RA1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RA1 to value 0"]
impl crate::Resettable for RA1_SPEC {
    const RESET_VALUE: u32 = 0;
}
