#[doc = "Register `CDTYUPD0` writer"]
pub type W = crate::W<Cdtyupd0Spec>;
#[doc = "Field `CDTYUPD` writer - Channel Duty-Cycle Update"]
pub type CdtyupdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle Update"]
    #[inline(always)]
    #[must_use]
    pub fn cdtyupd(&mut self) -> CdtyupdW<Cdtyupd0Spec> {
        CdtyupdW::new(self, 0)
    }
}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdtyupd0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdtyupd0Spec;
impl crate::RegisterSpec for Cdtyupd0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cdtyupd0::W`](W) writer structure"]
impl crate::Writable for Cdtyupd0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
