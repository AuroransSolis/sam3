#[doc = "Register `CPRD5` reader"]
pub type R = crate::R<Cprd5Spec>;
#[doc = "Register `CPRD5` writer"]
pub type W = crate::W<Cprd5Spec>;
#[doc = "Field `CPRD` reader - Channel Period"]
pub type CprdR = crate::FieldReader<u32>;
#[doc = "Field `CPRD` writer - Channel Period"]
pub type CprdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&self) -> CprdR {
        CprdR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    #[must_use]
    pub fn cprd(&mut self) -> CprdW<Cprd5Spec> {
        CprdW::new(self, 0)
    }
}
#[doc = "PWM Channel Period Register (ch_num = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cprd5Spec;
impl crate::RegisterSpec for Cprd5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cprd5::R`](R) reader structure"]
impl crate::Readable for Cprd5Spec {}
#[doc = "`write(|w| ..)` method takes [`cprd5::W`](W) writer structure"]
impl crate::Writable for Cprd5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPRD5 to value 0"]
impl crate::Resettable for Cprd5Spec {
    const RESET_VALUE: u32 = 0;
}
