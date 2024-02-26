#[doc = "Register `CMPVUPD5` writer"]
pub type W = crate::W<Cmpvupd5Spec>;
#[doc = "Field `CVUPD` writer - Comparison x Value Update"]
pub type CvupdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `CVMUPD` writer - Comparison x Value Mode Update"]
pub type CvmupdW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:23 - Comparison x Value Update"]
    #[inline(always)]
    #[must_use]
    pub fn cvupd(&mut self) -> CvupdW<Cmpvupd5Spec> {
        CvupdW::new(self, 0)
    }
    #[doc = "Bit 24 - Comparison x Value Mode Update"]
    #[inline(always)]
    #[must_use]
    pub fn cvmupd(&mut self) -> CvmupdW<Cmpvupd5Spec> {
        CvmupdW::new(self, 24)
    }
}
#[doc = "PWM Comparison 5 Value Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpvupd5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmpvupd5Spec;
impl crate::RegisterSpec for Cmpvupd5Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmpvupd5::W`](W) writer structure"]
impl crate::Writable for Cmpvupd5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
