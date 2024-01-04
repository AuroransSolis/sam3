#[doc = "Register `OSCUPD` writer"]
pub type W = crate::W<OSCUPD_SPEC>;
#[doc = "Field `OSCUPH0` writer - Output Selection Clear for PWMH output of the channel 0"]
pub type OSCUPH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPH1` writer - Output Selection Clear for PWMH output of the channel 1"]
pub type OSCUPH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPH2` writer - Output Selection Clear for PWMH output of the channel 2"]
pub type OSCUPH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPH3` writer - Output Selection Clear for PWMH output of the channel 3"]
pub type OSCUPH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL0` writer - Output Selection Clear for PWML output of the channel 0"]
pub type OSCUPL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL1` writer - Output Selection Clear for PWML output of the channel 1"]
pub type OSCUPL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL2` writer - Output Selection Clear for PWML output of the channel 2"]
pub type OSCUPL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCUPL3` writer - Output Selection Clear for PWML output of the channel 3"]
pub type OSCUPL3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Output Selection Clear for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph0(&mut self) -> OSCUPH0_W<OSCUPD_SPEC> {
        OSCUPH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Selection Clear for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph1(&mut self) -> OSCUPH1_W<OSCUPD_SPEC> {
        OSCUPH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Output Selection Clear for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph2(&mut self) -> OSCUPH2_W<OSCUPD_SPEC> {
        OSCUPH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output Selection Clear for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn oscuph3(&mut self) -> OSCUPH3_W<OSCUPD_SPEC> {
        OSCUPH3_W::new(self, 3)
    }
    #[doc = "Bit 16 - Output Selection Clear for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl0(&mut self) -> OSCUPL0_W<OSCUPD_SPEC> {
        OSCUPL0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Output Selection Clear for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl1(&mut self) -> OSCUPL1_W<OSCUPD_SPEC> {
        OSCUPL1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Output Selection Clear for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl2(&mut self) -> OSCUPL2_W<OSCUPD_SPEC> {
        OSCUPL2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Output Selection Clear for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn oscupl3(&mut self) -> OSCUPL3_W<OSCUPD_SPEC> {
        OSCUPL3_W::new(self, 19)
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
#[doc = "PWM Output Selection Clear Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscupd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCUPD_SPEC;
impl crate::RegisterSpec for OSCUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`oscupd::W`](W) writer structure"]
impl crate::Writable for OSCUPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
