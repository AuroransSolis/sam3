#[doc = "Register `OSCUPD` writer"]
pub struct W(crate::W<OSCUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCUPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OSCUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSCUPH0` writer - Output Selection Clear for PWMH output of the channel 0"]
pub type OSCUPH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPH1` writer - Output Selection Clear for PWMH output of the channel 1"]
pub type OSCUPH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPH2` writer - Output Selection Clear for PWMH output of the channel 2"]
pub type OSCUPH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPH3` writer - Output Selection Clear for PWMH output of the channel 3"]
pub type OSCUPH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPH4` writer - Output Selection Clear for PWMH output of the channel 4"]
pub type OSCUPH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPH5` writer - Output Selection Clear for PWMH output of the channel 5"]
pub type OSCUPH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPH6` writer - Output Selection Clear for PWMH output of the channel 6"]
pub type OSCUPH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPH7` writer - Output Selection Clear for PWMH output of the channel 7"]
pub type OSCUPH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPL0` writer - Output Selection Clear for PWML output of the channel 0"]
pub type OSCUPL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPL1` writer - Output Selection Clear for PWML output of the channel 1"]
pub type OSCUPL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPL2` writer - Output Selection Clear for PWML output of the channel 2"]
pub type OSCUPL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPL3` writer - Output Selection Clear for PWML output of the channel 3"]
pub type OSCUPL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPL4` writer - Output Selection Clear for PWML output of the channel 4"]
pub type OSCUPL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPL5` writer - Output Selection Clear for PWML output of the channel 5"]
pub type OSCUPL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPL6` writer - Output Selection Clear for PWML output of the channel 6"]
pub type OSCUPL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
#[doc = "Field `OSCUPL7` writer - Output Selection Clear for PWML output of the channel 7"]
pub type OSCUPL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCUPD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Output Selection Clear for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn oscuph0(&mut self) -> OSCUPH0_W<0> {
        OSCUPH0_W::new(self)
    }
    #[doc = "Bit 1 - Output Selection Clear for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn oscuph1(&mut self) -> OSCUPH1_W<1> {
        OSCUPH1_W::new(self)
    }
    #[doc = "Bit 2 - Output Selection Clear for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn oscuph2(&mut self) -> OSCUPH2_W<2> {
        OSCUPH2_W::new(self)
    }
    #[doc = "Bit 3 - Output Selection Clear for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn oscuph3(&mut self) -> OSCUPH3_W<3> {
        OSCUPH3_W::new(self)
    }
    #[doc = "Bit 4 - Output Selection Clear for PWMH output of the channel 4"]
    #[inline(always)]
    pub fn oscuph4(&mut self) -> OSCUPH4_W<4> {
        OSCUPH4_W::new(self)
    }
    #[doc = "Bit 5 - Output Selection Clear for PWMH output of the channel 5"]
    #[inline(always)]
    pub fn oscuph5(&mut self) -> OSCUPH5_W<5> {
        OSCUPH5_W::new(self)
    }
    #[doc = "Bit 6 - Output Selection Clear for PWMH output of the channel 6"]
    #[inline(always)]
    pub fn oscuph6(&mut self) -> OSCUPH6_W<6> {
        OSCUPH6_W::new(self)
    }
    #[doc = "Bit 7 - Output Selection Clear for PWMH output of the channel 7"]
    #[inline(always)]
    pub fn oscuph7(&mut self) -> OSCUPH7_W<7> {
        OSCUPH7_W::new(self)
    }
    #[doc = "Bit 16 - Output Selection Clear for PWML output of the channel 0"]
    #[inline(always)]
    pub fn oscupl0(&mut self) -> OSCUPL0_W<16> {
        OSCUPL0_W::new(self)
    }
    #[doc = "Bit 17 - Output Selection Clear for PWML output of the channel 1"]
    #[inline(always)]
    pub fn oscupl1(&mut self) -> OSCUPL1_W<17> {
        OSCUPL1_W::new(self)
    }
    #[doc = "Bit 18 - Output Selection Clear for PWML output of the channel 2"]
    #[inline(always)]
    pub fn oscupl2(&mut self) -> OSCUPL2_W<18> {
        OSCUPL2_W::new(self)
    }
    #[doc = "Bit 19 - Output Selection Clear for PWML output of the channel 3"]
    #[inline(always)]
    pub fn oscupl3(&mut self) -> OSCUPL3_W<19> {
        OSCUPL3_W::new(self)
    }
    #[doc = "Bit 20 - Output Selection Clear for PWML output of the channel 4"]
    #[inline(always)]
    pub fn oscupl4(&mut self) -> OSCUPL4_W<20> {
        OSCUPL4_W::new(self)
    }
    #[doc = "Bit 21 - Output Selection Clear for PWML output of the channel 5"]
    #[inline(always)]
    pub fn oscupl5(&mut self) -> OSCUPL5_W<21> {
        OSCUPL5_W::new(self)
    }
    #[doc = "Bit 22 - Output Selection Clear for PWML output of the channel 6"]
    #[inline(always)]
    pub fn oscupl6(&mut self) -> OSCUPL6_W<22> {
        OSCUPL6_W::new(self)
    }
    #[doc = "Bit 23 - Output Selection Clear for PWML output of the channel 7"]
    #[inline(always)]
    pub fn oscupl7(&mut self) -> OSCUPL7_W<23> {
        OSCUPL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Selection Clear Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscupd](index.html) module"]
pub struct OSCUPD_SPEC;
impl crate::RegisterSpec for OSCUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [oscupd::W](W) writer structure"]
impl crate::Writable for OSCUPD_SPEC {
    type Writer = W;
}
