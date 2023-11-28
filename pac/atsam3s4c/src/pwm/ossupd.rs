#[doc = "Register `OSSUPD` writer"]
pub type W = crate::W<OSSUPD_SPEC>;
#[doc = "Field `OSSUPH0` writer - Output Selection Set for PWMH output of the channel 0"]
pub type OSSUPH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPH1` writer - Output Selection Set for PWMH output of the channel 1"]
pub type OSSUPH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPH2` writer - Output Selection Set for PWMH output of the channel 2"]
pub type OSSUPH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPH3` writer - Output Selection Set for PWMH output of the channel 3"]
pub type OSSUPH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL0` writer - Output Selection Set for PWML output of the channel 0"]
pub type OSSUPL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL1` writer - Output Selection Set for PWML output of the channel 1"]
pub type OSSUPL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL2` writer - Output Selection Set for PWML output of the channel 2"]
pub type OSSUPL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSUPL3` writer - Output Selection Set for PWML output of the channel 3"]
pub type OSSUPL3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Output Selection Set for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph0(&mut self) -> OSSUPH0_W<OSSUPD_SPEC> {
        OSSUPH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Selection Set for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph1(&mut self) -> OSSUPH1_W<OSSUPD_SPEC> {
        OSSUPH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Output Selection Set for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph2(&mut self) -> OSSUPH2_W<OSSUPD_SPEC> {
        OSSUPH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output Selection Set for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph3(&mut self) -> OSSUPH3_W<OSSUPD_SPEC> {
        OSSUPH3_W::new(self, 3)
    }
    #[doc = "Bit 16 - Output Selection Set for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl0(&mut self) -> OSSUPL0_W<OSSUPD_SPEC> {
        OSSUPL0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Output Selection Set for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl1(&mut self) -> OSSUPL1_W<OSSUPD_SPEC> {
        OSSUPL1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Output Selection Set for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl2(&mut self) -> OSSUPL2_W<OSSUPD_SPEC> {
        OSSUPL2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Output Selection Set for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl3(&mut self) -> OSSUPL3_W<OSSUPD_SPEC> {
        OSSUPL3_W::new(self, 19)
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
#[doc = "PWM Output Selection Set Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ossupd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSSUPD_SPEC;
impl crate::RegisterSpec for OSSUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ossupd::W`](W) writer structure"]
impl crate::Writable for OSSUPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
