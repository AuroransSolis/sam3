#[doc = "Register `DTUPD3` writer"]
pub type W = crate::W<DTUPD3_SPEC>;
#[doc = "Field `DTHUPD` writer - Dead-Time Value Update for PWMHx Output"]
pub type DTHUPD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DTLUPD` writer - Dead-Time Value Update for PWMLx Output"]
pub type DTLUPD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Dead-Time Value Update for PWMHx Output"]
    #[inline(always)]
    #[must_use]
    pub fn dthupd(&mut self) -> DTHUPD_W<DTUPD3_SPEC> {
        DTHUPD_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Dead-Time Value Update for PWMLx Output"]
    #[inline(always)]
    #[must_use]
    pub fn dtlupd(&mut self) -> DTLUPD_W<DTUPD3_SPEC> {
        DTLUPD_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtupd3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTUPD3_SPEC;
impl crate::RegisterSpec for DTUPD3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dtupd3::W`](W) writer structure"]
impl crate::Writable for DTUPD3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
