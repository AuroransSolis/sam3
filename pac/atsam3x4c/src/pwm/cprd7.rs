#[doc = "Register `CPRD7` reader"]
pub type R = crate::R<CPRD7_SPEC>;
#[doc = "Register `CPRD7` writer"]
pub type W = crate::W<CPRD7_SPEC>;
#[doc = "Field `CPRD` reader - Channel Period"]
pub type CPRD_R = crate::FieldReader<u32>;
#[doc = "Field `CPRD` writer - Channel Period"]
pub type CPRD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&self) -> CPRD_R {
        CPRD_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    #[must_use]
    pub fn cprd(&mut self) -> CPRD_W<CPRD7_SPEC> {
        CPRD_W::new(self, 0)
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
#[doc = "PWM Channel Period Register (ch_num = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPRD7_SPEC;
impl crate::RegisterSpec for CPRD7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cprd7::R`](R) reader structure"]
impl crate::Readable for CPRD7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cprd7::W`](W) writer structure"]
impl crate::Writable for CPRD7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPRD7 to value 0"]
impl crate::Resettable for CPRD7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
