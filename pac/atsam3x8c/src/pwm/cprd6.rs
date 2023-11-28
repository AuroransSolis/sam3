#[doc = "Register `CPRD6` reader"]
pub type R = crate::R<CPRD6_SPEC>;
#[doc = "Register `CPRD6` writer"]
pub type W = crate::W<CPRD6_SPEC>;
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
    pub fn cprd(&mut self) -> CPRD_W<CPRD6_SPEC> {
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
#[doc = "PWM Channel Period Register (ch_num = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPRD6_SPEC;
impl crate::RegisterSpec for CPRD6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cprd6::R`](R) reader structure"]
impl crate::Readable for CPRD6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cprd6::W`](W) writer structure"]
impl crate::Writable for CPRD6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPRD6 to value 0"]
impl crate::Resettable for CPRD6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
