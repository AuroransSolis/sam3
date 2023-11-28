#[doc = "Register `SA4T` reader"]
pub type R = crate::R<SA4T_SPEC>;
#[doc = "Register `SA4T` writer"]
pub type W = crate::W<SA4T_SPEC>;
#[doc = "Field `ADDR` reader - "]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - "]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<SA4T_SPEC> {
        ADDR_W::new(self, 0)
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
#[doc = "Specific Address 4 Top Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa4t::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa4t::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SA4T_SPEC;
impl crate::RegisterSpec for SA4T_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa4t::R`](R) reader structure"]
impl crate::Readable for SA4T_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sa4t::W`](W) writer structure"]
impl crate::Writable for SA4T_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SA4T to value 0"]
impl crate::Resettable for SA4T_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
