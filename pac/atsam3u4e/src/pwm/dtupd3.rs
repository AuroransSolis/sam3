#[doc = "Register `DTUPD3` writer"]
pub type W = crate::W<Dtupd3Spec>;
#[doc = "Field `DTHUPD` writer - Dead-Time Value Update for PWMHx Output"]
pub type DthupdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DTLUPD` writer - Dead-Time Value Update for PWMLx Output"]
pub type DtlupdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Dead-Time Value Update for PWMHx Output"]
    #[inline(always)]
    #[must_use]
    pub fn dthupd(&mut self) -> DthupdW<Dtupd3Spec> {
        DthupdW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Dead-Time Value Update for PWMLx Output"]
    #[inline(always)]
    #[must_use]
    pub fn dtlupd(&mut self) -> DtlupdW<Dtupd3Spec> {
        DtlupdW::new(self, 16)
    }
}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 3)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtupd3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dtupd3Spec;
impl crate::RegisterSpec for Dtupd3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dtupd3::W`](W) writer structure"]
impl crate::Writable for Dtupd3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
