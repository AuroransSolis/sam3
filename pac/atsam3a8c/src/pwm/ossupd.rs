#[doc = "Register `OSSUPD` writer"]
pub type W = crate::W<OSSUPD_SPEC>;
#[doc = "Field `OSSUPH0` writer - Output Selection Set for PWMH output of the channel 0"]
pub type OSSUPH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPH1` writer - Output Selection Set for PWMH output of the channel 1"]
pub type OSSUPH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPH2` writer - Output Selection Set for PWMH output of the channel 2"]
pub type OSSUPH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPH3` writer - Output Selection Set for PWMH output of the channel 3"]
pub type OSSUPH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPH4` writer - Output Selection Set for PWMH output of the channel 4"]
pub type OSSUPH4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPH5` writer - Output Selection Set for PWMH output of the channel 5"]
pub type OSSUPH5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPH6` writer - Output Selection Set for PWMH output of the channel 6"]
pub type OSSUPH6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPH7` writer - Output Selection Set for PWMH output of the channel 7"]
pub type OSSUPH7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPL0` writer - Output Selection Set for PWML output of the channel 0"]
pub type OSSUPL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPL1` writer - Output Selection Set for PWML output of the channel 1"]
pub type OSSUPL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPL2` writer - Output Selection Set for PWML output of the channel 2"]
pub type OSSUPL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPL3` writer - Output Selection Set for PWML output of the channel 3"]
pub type OSSUPL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPL4` writer - Output Selection Set for PWML output of the channel 4"]
pub type OSSUPL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPL5` writer - Output Selection Set for PWML output of the channel 5"]
pub type OSSUPL5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPL6` writer - Output Selection Set for PWML output of the channel 6"]
pub type OSSUPL6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSUPL7` writer - Output Selection Set for PWML output of the channel 7"]
pub type OSSUPL7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Output Selection Set for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph0(&mut self) -> OSSUPH0_W<OSSUPD_SPEC, 0> {
        OSSUPH0_W::new(self)
    }
    #[doc = "Bit 1 - Output Selection Set for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph1(&mut self) -> OSSUPH1_W<OSSUPD_SPEC, 1> {
        OSSUPH1_W::new(self)
    }
    #[doc = "Bit 2 - Output Selection Set for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph2(&mut self) -> OSSUPH2_W<OSSUPD_SPEC, 2> {
        OSSUPH2_W::new(self)
    }
    #[doc = "Bit 3 - Output Selection Set for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph3(&mut self) -> OSSUPH3_W<OSSUPD_SPEC, 3> {
        OSSUPH3_W::new(self)
    }
    #[doc = "Bit 4 - Output Selection Set for PWMH output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph4(&mut self) -> OSSUPH4_W<OSSUPD_SPEC, 4> {
        OSSUPH4_W::new(self)
    }
    #[doc = "Bit 5 - Output Selection Set for PWMH output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph5(&mut self) -> OSSUPH5_W<OSSUPD_SPEC, 5> {
        OSSUPH5_W::new(self)
    }
    #[doc = "Bit 6 - Output Selection Set for PWMH output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph6(&mut self) -> OSSUPH6_W<OSSUPD_SPEC, 6> {
        OSSUPH6_W::new(self)
    }
    #[doc = "Bit 7 - Output Selection Set for PWMH output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph7(&mut self) -> OSSUPH7_W<OSSUPD_SPEC, 7> {
        OSSUPH7_W::new(self)
    }
    #[doc = "Bit 16 - Output Selection Set for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl0(&mut self) -> OSSUPL0_W<OSSUPD_SPEC, 16> {
        OSSUPL0_W::new(self)
    }
    #[doc = "Bit 17 - Output Selection Set for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl1(&mut self) -> OSSUPL1_W<OSSUPD_SPEC, 17> {
        OSSUPL1_W::new(self)
    }
    #[doc = "Bit 18 - Output Selection Set for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl2(&mut self) -> OSSUPL2_W<OSSUPD_SPEC, 18> {
        OSSUPL2_W::new(self)
    }
    #[doc = "Bit 19 - Output Selection Set for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl3(&mut self) -> OSSUPL3_W<OSSUPD_SPEC, 19> {
        OSSUPL3_W::new(self)
    }
    #[doc = "Bit 20 - Output Selection Set for PWML output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl4(&mut self) -> OSSUPL4_W<OSSUPD_SPEC, 20> {
        OSSUPL4_W::new(self)
    }
    #[doc = "Bit 21 - Output Selection Set for PWML output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl5(&mut self) -> OSSUPL5_W<OSSUPD_SPEC, 21> {
        OSSUPL5_W::new(self)
    }
    #[doc = "Bit 22 - Output Selection Set for PWML output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl6(&mut self) -> OSSUPL6_W<OSSUPD_SPEC, 22> {
        OSSUPL6_W::new(self)
    }
    #[doc = "Bit 23 - Output Selection Set for PWML output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl7(&mut self) -> OSSUPL7_W<OSSUPD_SPEC, 23> {
        OSSUPL7_W::new(self)
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
