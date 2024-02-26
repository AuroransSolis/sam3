#[doc = "Register `CPRD0` reader"]
pub type R = crate::R<Cprd0Spec>;
#[doc = "Register `CPRD0` writer"]
pub type W = crate::W<Cprd0Spec>;
#[doc = "Field `CPRD` reader - Channel Period"]
pub type CprdR = crate::FieldReader<u32>;
#[doc = "Field `CPRD` writer - Channel Period"]
pub type CprdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&self) -> CprdR {
        CprdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Period"]
    #[inline(always)]
    #[must_use]
    pub fn cprd(&mut self) -> CprdW<Cprd0Spec> {
        CprdW::new(self, 0)
    }
}
#[doc = "PWM Channel Period Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cprd0Spec;
impl crate::RegisterSpec for Cprd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cprd0::R`](R) reader structure"]
impl crate::Readable for Cprd0Spec {}
#[doc = "`write(|w| ..)` method takes [`cprd0::W`](W) writer structure"]
impl crate::Writable for Cprd0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPRD0 to value 0"]
impl crate::Resettable for Cprd0Spec {
    const RESET_VALUE: u32 = 0;
}
