#[doc = "Register `SA1B` reader"]
pub type R = crate::R<SA1B_SPEC>;
#[doc = "Register `SA1B` writer"]
pub type W = crate::W<SA1B_SPEC>;
#[doc = "Field `ADDR` reader - "]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - "]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<SA1B_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Specific Address 1 Bottom Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa1b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa1b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SA1B_SPEC;
impl crate::RegisterSpec for SA1B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa1b::R`](R) reader structure"]
impl crate::Readable for SA1B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sa1b::W`](W) writer structure"]
impl crate::Writable for SA1B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SA1B to value 0"]
impl crate::Resettable for SA1B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
