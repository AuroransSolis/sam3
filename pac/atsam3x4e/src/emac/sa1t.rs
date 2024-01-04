#[doc = "Register `SA1T` reader"]
pub type R = crate::R<SA1T_SPEC>;
#[doc = "Register `SA1T` writer"]
pub type W = crate::W<SA1T_SPEC>;
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
    pub fn addr(&mut self) -> ADDR_W<SA1T_SPEC> {
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
#[doc = "Specific Address 1 Top Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa1t::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa1t::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SA1T_SPEC;
impl crate::RegisterSpec for SA1T_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa1t::R`](R) reader structure"]
impl crate::Readable for SA1T_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sa1t::W`](W) writer structure"]
impl crate::Writable for SA1T_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SA1T to value 0"]
impl crate::Resettable for SA1T_SPEC {
    const RESET_VALUE: u32 = 0;
}
