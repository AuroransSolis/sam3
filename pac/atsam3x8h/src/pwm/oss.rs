#[doc = "Register `OSS` writer"]
pub struct W(crate::W<OSS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSS_SPEC>;
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
impl From<crate::W<OSS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSSH0` writer - Output Selection Set for PWMH output of the channel 0"]
pub type OSSH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSH1` writer - Output Selection Set for PWMH output of the channel 1"]
pub type OSSH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSH2` writer - Output Selection Set for PWMH output of the channel 2"]
pub type OSSH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSH3` writer - Output Selection Set for PWMH output of the channel 3"]
pub type OSSH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSH4` writer - Output Selection Set for PWMH output of the channel 4"]
pub type OSSH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSH5` writer - Output Selection Set for PWMH output of the channel 5"]
pub type OSSH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSH6` writer - Output Selection Set for PWMH output of the channel 6"]
pub type OSSH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSH7` writer - Output Selection Set for PWMH output of the channel 7"]
pub type OSSH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSL0` writer - Output Selection Set for PWML output of the channel 0"]
pub type OSSL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSL1` writer - Output Selection Set for PWML output of the channel 1"]
pub type OSSL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSL2` writer - Output Selection Set for PWML output of the channel 2"]
pub type OSSL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSL3` writer - Output Selection Set for PWML output of the channel 3"]
pub type OSSL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSL4` writer - Output Selection Set for PWML output of the channel 4"]
pub type OSSL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSL5` writer - Output Selection Set for PWML output of the channel 5"]
pub type OSSL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSL6` writer - Output Selection Set for PWML output of the channel 6"]
pub type OSSL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
#[doc = "Field `OSSL7` writer - Output Selection Set for PWML output of the channel 7"]
pub type OSSL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Output Selection Set for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn ossh0(&mut self) -> OSSH0_W<0> {
        OSSH0_W::new(self)
    }
    #[doc = "Bit 1 - Output Selection Set for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn ossh1(&mut self) -> OSSH1_W<1> {
        OSSH1_W::new(self)
    }
    #[doc = "Bit 2 - Output Selection Set for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn ossh2(&mut self) -> OSSH2_W<2> {
        OSSH2_W::new(self)
    }
    #[doc = "Bit 3 - Output Selection Set for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn ossh3(&mut self) -> OSSH3_W<3> {
        OSSH3_W::new(self)
    }
    #[doc = "Bit 4 - Output Selection Set for PWMH output of the channel 4"]
    #[inline(always)]
    pub fn ossh4(&mut self) -> OSSH4_W<4> {
        OSSH4_W::new(self)
    }
    #[doc = "Bit 5 - Output Selection Set for PWMH output of the channel 5"]
    #[inline(always)]
    pub fn ossh5(&mut self) -> OSSH5_W<5> {
        OSSH5_W::new(self)
    }
    #[doc = "Bit 6 - Output Selection Set for PWMH output of the channel 6"]
    #[inline(always)]
    pub fn ossh6(&mut self) -> OSSH6_W<6> {
        OSSH6_W::new(self)
    }
    #[doc = "Bit 7 - Output Selection Set for PWMH output of the channel 7"]
    #[inline(always)]
    pub fn ossh7(&mut self) -> OSSH7_W<7> {
        OSSH7_W::new(self)
    }
    #[doc = "Bit 16 - Output Selection Set for PWML output of the channel 0"]
    #[inline(always)]
    pub fn ossl0(&mut self) -> OSSL0_W<16> {
        OSSL0_W::new(self)
    }
    #[doc = "Bit 17 - Output Selection Set for PWML output of the channel 1"]
    #[inline(always)]
    pub fn ossl1(&mut self) -> OSSL1_W<17> {
        OSSL1_W::new(self)
    }
    #[doc = "Bit 18 - Output Selection Set for PWML output of the channel 2"]
    #[inline(always)]
    pub fn ossl2(&mut self) -> OSSL2_W<18> {
        OSSL2_W::new(self)
    }
    #[doc = "Bit 19 - Output Selection Set for PWML output of the channel 3"]
    #[inline(always)]
    pub fn ossl3(&mut self) -> OSSL3_W<19> {
        OSSL3_W::new(self)
    }
    #[doc = "Bit 20 - Output Selection Set for PWML output of the channel 4"]
    #[inline(always)]
    pub fn ossl4(&mut self) -> OSSL4_W<20> {
        OSSL4_W::new(self)
    }
    #[doc = "Bit 21 - Output Selection Set for PWML output of the channel 5"]
    #[inline(always)]
    pub fn ossl5(&mut self) -> OSSL5_W<21> {
        OSSL5_W::new(self)
    }
    #[doc = "Bit 22 - Output Selection Set for PWML output of the channel 6"]
    #[inline(always)]
    pub fn ossl6(&mut self) -> OSSL6_W<22> {
        OSSL6_W::new(self)
    }
    #[doc = "Bit 23 - Output Selection Set for PWML output of the channel 7"]
    #[inline(always)]
    pub fn ossl7(&mut self) -> OSSL7_W<23> {
        OSSL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Selection Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oss](index.html) module"]
pub struct OSS_SPEC;
impl crate::RegisterSpec for OSS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [oss::W](W) writer structure"]
impl crate::Writable for OSS_SPEC {
    type Writer = W;
}
