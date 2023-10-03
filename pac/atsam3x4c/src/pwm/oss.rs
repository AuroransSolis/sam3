#[doc = "Register `OSS` writer"]
pub type W = crate::W<OSS_SPEC>;
#[doc = "Field `OSSH0` writer - Output Selection Set for PWMH output of the channel 0"]
pub type OSSH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSH1` writer - Output Selection Set for PWMH output of the channel 1"]
pub type OSSH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSH2` writer - Output Selection Set for PWMH output of the channel 2"]
pub type OSSH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSH3` writer - Output Selection Set for PWMH output of the channel 3"]
pub type OSSH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSH4` writer - Output Selection Set for PWMH output of the channel 4"]
pub type OSSH4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSH5` writer - Output Selection Set for PWMH output of the channel 5"]
pub type OSSH5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSH6` writer - Output Selection Set for PWMH output of the channel 6"]
pub type OSSH6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSH7` writer - Output Selection Set for PWMH output of the channel 7"]
pub type OSSH7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSL0` writer - Output Selection Set for PWML output of the channel 0"]
pub type OSSL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSL1` writer - Output Selection Set for PWML output of the channel 1"]
pub type OSSL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSL2` writer - Output Selection Set for PWML output of the channel 2"]
pub type OSSL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSL3` writer - Output Selection Set for PWML output of the channel 3"]
pub type OSSL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSL4` writer - Output Selection Set for PWML output of the channel 4"]
pub type OSSL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSL5` writer - Output Selection Set for PWML output of the channel 5"]
pub type OSSL5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSL6` writer - Output Selection Set for PWML output of the channel 6"]
pub type OSSL6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSL7` writer - Output Selection Set for PWML output of the channel 7"]
pub type OSSL7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Output Selection Set for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossh0(&mut self) -> OSSH0_W<OSS_SPEC, 0> {
        OSSH0_W::new(self)
    }
    #[doc = "Bit 1 - Output Selection Set for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossh1(&mut self) -> OSSH1_W<OSS_SPEC, 1> {
        OSSH1_W::new(self)
    }
    #[doc = "Bit 2 - Output Selection Set for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossh2(&mut self) -> OSSH2_W<OSS_SPEC, 2> {
        OSSH2_W::new(self)
    }
    #[doc = "Bit 3 - Output Selection Set for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossh3(&mut self) -> OSSH3_W<OSS_SPEC, 3> {
        OSSH3_W::new(self)
    }
    #[doc = "Bit 4 - Output Selection Set for PWMH output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ossh4(&mut self) -> OSSH4_W<OSS_SPEC, 4> {
        OSSH4_W::new(self)
    }
    #[doc = "Bit 5 - Output Selection Set for PWMH output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ossh5(&mut self) -> OSSH5_W<OSS_SPEC, 5> {
        OSSH5_W::new(self)
    }
    #[doc = "Bit 6 - Output Selection Set for PWMH output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ossh6(&mut self) -> OSSH6_W<OSS_SPEC, 6> {
        OSSH6_W::new(self)
    }
    #[doc = "Bit 7 - Output Selection Set for PWMH output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ossh7(&mut self) -> OSSH7_W<OSS_SPEC, 7> {
        OSSH7_W::new(self)
    }
    #[doc = "Bit 16 - Output Selection Set for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossl0(&mut self) -> OSSL0_W<OSS_SPEC, 16> {
        OSSL0_W::new(self)
    }
    #[doc = "Bit 17 - Output Selection Set for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossl1(&mut self) -> OSSL1_W<OSS_SPEC, 17> {
        OSSL1_W::new(self)
    }
    #[doc = "Bit 18 - Output Selection Set for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossl2(&mut self) -> OSSL2_W<OSS_SPEC, 18> {
        OSSL2_W::new(self)
    }
    #[doc = "Bit 19 - Output Selection Set for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossl3(&mut self) -> OSSL3_W<OSS_SPEC, 19> {
        OSSL3_W::new(self)
    }
    #[doc = "Bit 20 - Output Selection Set for PWML output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ossl4(&mut self) -> OSSL4_W<OSS_SPEC, 20> {
        OSSL4_W::new(self)
    }
    #[doc = "Bit 21 - Output Selection Set for PWML output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ossl5(&mut self) -> OSSL5_W<OSS_SPEC, 21> {
        OSSL5_W::new(self)
    }
    #[doc = "Bit 22 - Output Selection Set for PWML output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ossl6(&mut self) -> OSSL6_W<OSS_SPEC, 22> {
        OSSL6_W::new(self)
    }
    #[doc = "Bit 23 - Output Selection Set for PWML output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ossl7(&mut self) -> OSSL7_W<OSS_SPEC, 23> {
        OSSL7_W::new(self)
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
#[doc = "PWM Output Selection Set Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oss::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSS_SPEC;
impl crate::RegisterSpec for OSS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`oss::W`](W) writer structure"]
impl crate::Writable for OSS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
