#[doc = "Register `RBQP` reader"]
pub type R = crate::R<RBQP_SPEC>;
#[doc = "Register `RBQP` writer"]
pub type W = crate::W<RBQP_SPEC>;
#[doc = "Field `ADDR` reader - Receive buffer queue pointer address"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Receive buffer queue pointer address"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Receive buffer queue pointer address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive buffer queue pointer address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<RBQP_SPEC> {
        ADDR_W::new(self, 2)
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
#[doc = "Receive Buffer Queue Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbqp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbqp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RBQP_SPEC;
impl crate::RegisterSpec for RBQP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbqp::R`](R) reader structure"]
impl crate::Readable for RBQP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rbqp::W`](W) writer structure"]
impl crate::Writable for RBQP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RBQP to value 0"]
impl crate::Resettable for RBQP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
