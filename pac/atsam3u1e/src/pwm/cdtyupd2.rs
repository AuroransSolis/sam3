#[doc = "Register `CDTYUPD2` writer"]
pub type W = crate::W<Cdtyupd2Spec>;
#[doc = "Field `CDTYUPD` writer - Channel Duty-Cycle Update"]
pub type CdtyupdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle Update"]
    #[inline(always)]
    #[must_use]
    pub fn cdtyupd(&mut self) -> CdtyupdW<Cdtyupd2Spec> {
        CdtyupdW::new(self, 0)
    }
}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 2)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdtyupd2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdtyupd2Spec;
impl crate::RegisterSpec for Cdtyupd2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cdtyupd2::W`](W) writer structure"]
impl crate::Writable for Cdtyupd2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
