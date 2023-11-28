#[doc = "Register `TBQP` reader"]
pub type R = crate::R<TBQP_SPEC>;
#[doc = "Register `TBQP` writer"]
pub type W = crate::W<TBQP_SPEC>;
#[doc = "Field `ADDR` reader - Transmit buffer queue pointer address"]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Transmit buffer queue pointer address"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Transmit buffer queue pointer address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit buffer queue pointer address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<TBQP_SPEC> {
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
#[doc = "Transmit Buffer Queue Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbqp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbqp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBQP_SPEC;
impl crate::RegisterSpec for TBQP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbqp::R`](R) reader structure"]
impl crate::Readable for TBQP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbqp::W`](W) writer structure"]
impl crate::Writable for TBQP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBQP to value 0"]
impl crate::Resettable for TBQP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
