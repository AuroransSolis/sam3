#[doc = "Register `CMPMUPD7` writer"]
pub type W = crate::W<Cmpmupd7Spec>;
#[doc = "Field `CENUPD` writer - Comparison x Enable Update"]
pub type CenupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRUPD` writer - Comparison x Trigger Update"]
pub type CtrupdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CPRUPD` writer - Comparison x Period Update"]
pub type CprupdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CUPRUPD` writer - Comparison x Update Period Update"]
pub type CuprupdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bit 0 - Comparison x Enable Update"]
    #[inline(always)]
    #[must_use]
    pub fn cenupd(&mut self) -> CenupdW<Cmpmupd7Spec> {
        CenupdW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Comparison x Trigger Update"]
    #[inline(always)]
    #[must_use]
    pub fn ctrupd(&mut self) -> CtrupdW<Cmpmupd7Spec> {
        CtrupdW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Comparison x Period Update"]
    #[inline(always)]
    #[must_use]
    pub fn cprupd(&mut self) -> CprupdW<Cmpmupd7Spec> {
        CprupdW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Comparison x Update Period Update"]
    #[inline(always)]
    #[must_use]
    pub fn cuprupd(&mut self) -> CuprupdW<Cmpmupd7Spec> {
        CuprupdW::new(self, 16)
    }
}
#[doc = "PWM Comparison 7 Mode Update Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpmupd7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmpmupd7Spec;
impl crate::RegisterSpec for Cmpmupd7Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmpmupd7::W`](W) writer structure"]
impl crate::Writable for Cmpmupd7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
