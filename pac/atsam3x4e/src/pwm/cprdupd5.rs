#[doc = "Register `CPRDUPD5` writer"]
pub type W = crate::W<Cprdupd5Spec>;
#[doc = "Field `CPRDUPD` writer - Channel Period Update"]
pub type CprdupdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bits 0:23 - Channel Period Update"]
    #[inline(always)]
    #[must_use]
    pub fn cprdupd(&mut self) -> CprdupdW<Cprdupd5Spec> {
        CprdupdW::new(self, 0)
    }
}
#[doc = "PWM Channel Period Update Register (ch_num = 5)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprdupd5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cprdupd5Spec;
impl crate::RegisterSpec for Cprdupd5Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cprdupd5::W`](W) writer structure"]
impl crate::Writable for Cprdupd5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
