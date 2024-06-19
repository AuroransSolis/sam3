#[doc = "Register `SCUPUPD` writer"]
pub type W = crate::W<ScupupdSpec>;
#[doc = "Field `UPRUPD` writer - Update Period Update"]
pub type UprupdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - Update Period Update"]
    #[inline(always)]
    #[must_use]
    pub fn uprupd(&mut self) -> UprupdW<ScupupdSpec> {
        UprupdW::new(self, 0)
    }
}
#[doc = "PWM Sync Channels Update Period Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scupupd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScupupdSpec;
impl crate::RegisterSpec for ScupupdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scupupd::W`](W) writer structure"]
impl crate::Writable for ScupupdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCUPUPD to value 0"]
impl crate::Resettable for ScupupdSpec {
    const RESET_VALUE: u32 = 0;
}
