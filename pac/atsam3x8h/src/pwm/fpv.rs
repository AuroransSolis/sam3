#[doc = "Register `FPV` reader"]
pub struct R(crate::R<FPV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPV` writer"]
pub struct W(crate::W<FPV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPV_SPEC>;
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
impl From<crate::W<FPV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPVH0` reader - Fault Protection Value for PWMH output on channel 0"]
pub type FPVH0_R = crate::BitReader<bool>;
#[doc = "Field `FPVH0` writer - Fault Protection Value for PWMH output on channel 0"]
pub type FPVH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVH1` reader - Fault Protection Value for PWMH output on channel 1"]
pub type FPVH1_R = crate::BitReader<bool>;
#[doc = "Field `FPVH1` writer - Fault Protection Value for PWMH output on channel 1"]
pub type FPVH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVH2` reader - Fault Protection Value for PWMH output on channel 2"]
pub type FPVH2_R = crate::BitReader<bool>;
#[doc = "Field `FPVH2` writer - Fault Protection Value for PWMH output on channel 2"]
pub type FPVH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVH3` reader - Fault Protection Value for PWMH output on channel 3"]
pub type FPVH3_R = crate::BitReader<bool>;
#[doc = "Field `FPVH3` writer - Fault Protection Value for PWMH output on channel 3"]
pub type FPVH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVH4` reader - Fault Protection Value for PWMH output on channel 4"]
pub type FPVH4_R = crate::BitReader<bool>;
#[doc = "Field `FPVH4` writer - Fault Protection Value for PWMH output on channel 4"]
pub type FPVH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVH5` reader - Fault Protection Value for PWMH output on channel 5"]
pub type FPVH5_R = crate::BitReader<bool>;
#[doc = "Field `FPVH5` writer - Fault Protection Value for PWMH output on channel 5"]
pub type FPVH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVH6` reader - Fault Protection Value for PWMH output on channel 6"]
pub type FPVH6_R = crate::BitReader<bool>;
#[doc = "Field `FPVH6` writer - Fault Protection Value for PWMH output on channel 6"]
pub type FPVH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVH7` reader - Fault Protection Value for PWMH output on channel 7"]
pub type FPVH7_R = crate::BitReader<bool>;
#[doc = "Field `FPVH7` writer - Fault Protection Value for PWMH output on channel 7"]
pub type FPVH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVL0` reader - Fault Protection Value for PWML output on channel 0"]
pub type FPVL0_R = crate::BitReader<bool>;
#[doc = "Field `FPVL0` writer - Fault Protection Value for PWML output on channel 0"]
pub type FPVL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVL1` reader - Fault Protection Value for PWML output on channel 1"]
pub type FPVL1_R = crate::BitReader<bool>;
#[doc = "Field `FPVL1` writer - Fault Protection Value for PWML output on channel 1"]
pub type FPVL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVL2` reader - Fault Protection Value for PWML output on channel 2"]
pub type FPVL2_R = crate::BitReader<bool>;
#[doc = "Field `FPVL2` writer - Fault Protection Value for PWML output on channel 2"]
pub type FPVL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVL3` reader - Fault Protection Value for PWML output on channel 3"]
pub type FPVL3_R = crate::BitReader<bool>;
#[doc = "Field `FPVL3` writer - Fault Protection Value for PWML output on channel 3"]
pub type FPVL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVL4` reader - Fault Protection Value for PWML output on channel 4"]
pub type FPVL4_R = crate::BitReader<bool>;
#[doc = "Field `FPVL4` writer - Fault Protection Value for PWML output on channel 4"]
pub type FPVL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVL5` reader - Fault Protection Value for PWML output on channel 5"]
pub type FPVL5_R = crate::BitReader<bool>;
#[doc = "Field `FPVL5` writer - Fault Protection Value for PWML output on channel 5"]
pub type FPVL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVL6` reader - Fault Protection Value for PWML output on channel 6"]
pub type FPVL6_R = crate::BitReader<bool>;
#[doc = "Field `FPVL6` writer - Fault Protection Value for PWML output on channel 6"]
pub type FPVL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
#[doc = "Field `FPVL7` reader - Fault Protection Value for PWML output on channel 7"]
pub type FPVL7_R = crate::BitReader<bool>;
#[doc = "Field `FPVL7` writer - Fault Protection Value for PWML output on channel 7"]
pub type FPVL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPV_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpvh0(&self) -> FPVH0_R {
        FPVH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpvh1(&self) -> FPVH1_R {
        FPVH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpvh2(&self) -> FPVH2_R {
        FPVH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpvh3(&self) -> FPVH3_R {
        FPVH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault Protection Value for PWMH output on channel 4"]
    #[inline(always)]
    pub fn fpvh4(&self) -> FPVH4_R {
        FPVH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fault Protection Value for PWMH output on channel 5"]
    #[inline(always)]
    pub fn fpvh5(&self) -> FPVH5_R {
        FPVH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fault Protection Value for PWMH output on channel 6"]
    #[inline(always)]
    pub fn fpvh6(&self) -> FPVH6_R {
        FPVH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fault Protection Value for PWMH output on channel 7"]
    #[inline(always)]
    pub fn fpvh7(&self) -> FPVH7_R {
        FPVH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpvl0(&self) -> FPVL0_R {
        FPVL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpvl1(&self) -> FPVL1_R {
        FPVL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpvl2(&self) -> FPVL2_R {
        FPVL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpvl3(&self) -> FPVL3_R {
        FPVL3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Fault Protection Value for PWML output on channel 4"]
    #[inline(always)]
    pub fn fpvl4(&self) -> FPVL4_R {
        FPVL4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Fault Protection Value for PWML output on channel 5"]
    #[inline(always)]
    pub fn fpvl5(&self) -> FPVL5_R {
        FPVL5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Fault Protection Value for PWML output on channel 6"]
    #[inline(always)]
    pub fn fpvl6(&self) -> FPVL6_R {
        FPVL6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Fault Protection Value for PWML output on channel 7"]
    #[inline(always)]
    pub fn fpvl7(&self) -> FPVL7_R {
        FPVL7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpvh0(&mut self) -> FPVH0_W<0> {
        FPVH0_W::new(self)
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpvh1(&mut self) -> FPVH1_W<1> {
        FPVH1_W::new(self)
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpvh2(&mut self) -> FPVH2_W<2> {
        FPVH2_W::new(self)
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpvh3(&mut self) -> FPVH3_W<3> {
        FPVH3_W::new(self)
    }
    #[doc = "Bit 4 - Fault Protection Value for PWMH output on channel 4"]
    #[inline(always)]
    pub fn fpvh4(&mut self) -> FPVH4_W<4> {
        FPVH4_W::new(self)
    }
    #[doc = "Bit 5 - Fault Protection Value for PWMH output on channel 5"]
    #[inline(always)]
    pub fn fpvh5(&mut self) -> FPVH5_W<5> {
        FPVH5_W::new(self)
    }
    #[doc = "Bit 6 - Fault Protection Value for PWMH output on channel 6"]
    #[inline(always)]
    pub fn fpvh6(&mut self) -> FPVH6_W<6> {
        FPVH6_W::new(self)
    }
    #[doc = "Bit 7 - Fault Protection Value for PWMH output on channel 7"]
    #[inline(always)]
    pub fn fpvh7(&mut self) -> FPVH7_W<7> {
        FPVH7_W::new(self)
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpvl0(&mut self) -> FPVL0_W<16> {
        FPVL0_W::new(self)
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpvl1(&mut self) -> FPVL1_W<17> {
        FPVL1_W::new(self)
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpvl2(&mut self) -> FPVL2_W<18> {
        FPVL2_W::new(self)
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpvl3(&mut self) -> FPVL3_W<19> {
        FPVL3_W::new(self)
    }
    #[doc = "Bit 20 - Fault Protection Value for PWML output on channel 4"]
    #[inline(always)]
    pub fn fpvl4(&mut self) -> FPVL4_W<20> {
        FPVL4_W::new(self)
    }
    #[doc = "Bit 21 - Fault Protection Value for PWML output on channel 5"]
    #[inline(always)]
    pub fn fpvl5(&mut self) -> FPVL5_W<21> {
        FPVL5_W::new(self)
    }
    #[doc = "Bit 22 - Fault Protection Value for PWML output on channel 6"]
    #[inline(always)]
    pub fn fpvl6(&mut self) -> FPVL6_W<22> {
        FPVL6_W::new(self)
    }
    #[doc = "Bit 23 - Fault Protection Value for PWML output on channel 7"]
    #[inline(always)]
    pub fn fpvl7(&mut self) -> FPVL7_W<23> {
        FPVL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Fault Protection Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpv](index.html) module"]
pub struct FPV_SPEC;
impl crate::RegisterSpec for FPV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpv::R](R) reader structure"]
impl crate::Readable for FPV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpv::W](W) writer structure"]
impl crate::Writable for FPV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPV to value 0"]
impl crate::Resettable for FPV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
