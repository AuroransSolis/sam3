#[doc = "Register `OSC` writer"]
pub type W = crate::W<OSC_SPEC>;
#[doc = "Field `OSCH0` writer - Output Selection Clear for PWMH output of the channel 0"]
pub type OSCH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCH1` writer - Output Selection Clear for PWMH output of the channel 1"]
pub type OSCH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCH2` writer - Output Selection Clear for PWMH output of the channel 2"]
pub type OSCH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCH3` writer - Output Selection Clear for PWMH output of the channel 3"]
pub type OSCH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCH4` writer - Output Selection Clear for PWMH output of the channel 4"]
pub type OSCH4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCH5` writer - Output Selection Clear for PWMH output of the channel 5"]
pub type OSCH5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCH6` writer - Output Selection Clear for PWMH output of the channel 6"]
pub type OSCH6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCH7` writer - Output Selection Clear for PWMH output of the channel 7"]
pub type OSCH7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCL0` writer - Output Selection Clear for PWML output of the channel 0"]
pub type OSCL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCL1` writer - Output Selection Clear for PWML output of the channel 1"]
pub type OSCL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCL2` writer - Output Selection Clear for PWML output of the channel 2"]
pub type OSCL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCL3` writer - Output Selection Clear for PWML output of the channel 3"]
pub type OSCL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCL4` writer - Output Selection Clear for PWML output of the channel 4"]
pub type OSCL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCL5` writer - Output Selection Clear for PWML output of the channel 5"]
pub type OSCL5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCL6` writer - Output Selection Clear for PWML output of the channel 6"]
pub type OSCL6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCL7` writer - Output Selection Clear for PWML output of the channel 7"]
pub type OSCL7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Output Selection Clear for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn osch0(&mut self) -> OSCH0_W<OSC_SPEC, 0> {
        OSCH0_W::new(self)
    }
    #[doc = "Bit 1 - Output Selection Clear for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn osch1(&mut self) -> OSCH1_W<OSC_SPEC, 1> {
        OSCH1_W::new(self)
    }
    #[doc = "Bit 2 - Output Selection Clear for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn osch2(&mut self) -> OSCH2_W<OSC_SPEC, 2> {
        OSCH2_W::new(self)
    }
    #[doc = "Bit 3 - Output Selection Clear for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn osch3(&mut self) -> OSCH3_W<OSC_SPEC, 3> {
        OSCH3_W::new(self)
    }
    #[doc = "Bit 4 - Output Selection Clear for PWMH output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn osch4(&mut self) -> OSCH4_W<OSC_SPEC, 4> {
        OSCH4_W::new(self)
    }
    #[doc = "Bit 5 - Output Selection Clear for PWMH output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn osch5(&mut self) -> OSCH5_W<OSC_SPEC, 5> {
        OSCH5_W::new(self)
    }
    #[doc = "Bit 6 - Output Selection Clear for PWMH output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn osch6(&mut self) -> OSCH6_W<OSC_SPEC, 6> {
        OSCH6_W::new(self)
    }
    #[doc = "Bit 7 - Output Selection Clear for PWMH output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn osch7(&mut self) -> OSCH7_W<OSC_SPEC, 7> {
        OSCH7_W::new(self)
    }
    #[doc = "Bit 16 - Output Selection Clear for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn oscl0(&mut self) -> OSCL0_W<OSC_SPEC, 16> {
        OSCL0_W::new(self)
    }
    #[doc = "Bit 17 - Output Selection Clear for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn oscl1(&mut self) -> OSCL1_W<OSC_SPEC, 17> {
        OSCL1_W::new(self)
    }
    #[doc = "Bit 18 - Output Selection Clear for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn oscl2(&mut self) -> OSCL2_W<OSC_SPEC, 18> {
        OSCL2_W::new(self)
    }
    #[doc = "Bit 19 - Output Selection Clear for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn oscl3(&mut self) -> OSCL3_W<OSC_SPEC, 19> {
        OSCL3_W::new(self)
    }
    #[doc = "Bit 20 - Output Selection Clear for PWML output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn oscl4(&mut self) -> OSCL4_W<OSC_SPEC, 20> {
        OSCL4_W::new(self)
    }
    #[doc = "Bit 21 - Output Selection Clear for PWML output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn oscl5(&mut self) -> OSCL5_W<OSC_SPEC, 21> {
        OSCL5_W::new(self)
    }
    #[doc = "Bit 22 - Output Selection Clear for PWML output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn oscl6(&mut self) -> OSCL6_W<OSC_SPEC, 22> {
        OSCL6_W::new(self)
    }
    #[doc = "Bit 23 - Output Selection Clear for PWML output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn oscl7(&mut self) -> OSCL7_W<OSC_SPEC, 23> {
        OSCL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Output Selection Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSC_SPEC;
impl crate::RegisterSpec for OSC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`osc::W`](W) writer structure"]
impl crate::Writable for OSC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
