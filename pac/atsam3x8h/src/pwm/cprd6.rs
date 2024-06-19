#[doc = "Register `CPRD6` reader"]
pub type R = crate::R<Cprd6Spec>;
#[doc = "Register `CPRD6` writer"]
pub type W = crate::W<Cprd6Spec>;
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
    pub fn cprd(&mut self) -> CprdW<Cprd6Spec> {
        CprdW::new(self, 0)
    }
}
#[doc = "PWM Channel Period Register (ch_num = 6)\n\nYou can [`read`](crate::Reg::read) this register and get [`cprd6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cprd6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cprd6Spec;
impl crate::RegisterSpec for Cprd6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cprd6::R`](R) reader structure"]
impl crate::Readable for Cprd6Spec {}
#[doc = "`write(|w| ..)` method takes [`cprd6::W`](W) writer structure"]
impl crate::Writable for Cprd6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPRD6 to value 0"]
impl crate::Resettable for Cprd6Spec {
    const RESET_VALUE: u32 = 0;
}
