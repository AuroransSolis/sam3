#[doc = "Register `FPV` reader"]
pub type R = crate::R<FPV_SPEC>;
#[doc = "Register `FPV` writer"]
pub type W = crate::W<FPV_SPEC>;
#[doc = "Field `FPVH0` reader - Fault Protection Value for PWMH output on channel 0"]
pub type FPVH0_R = crate::BitReader;
#[doc = "Field `FPVH0` writer - Fault Protection Value for PWMH output on channel 0"]
pub type FPVH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVH1` reader - Fault Protection Value for PWMH output on channel 1"]
pub type FPVH1_R = crate::BitReader;
#[doc = "Field `FPVH1` writer - Fault Protection Value for PWMH output on channel 1"]
pub type FPVH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVH2` reader - Fault Protection Value for PWMH output on channel 2"]
pub type FPVH2_R = crate::BitReader;
#[doc = "Field `FPVH2` writer - Fault Protection Value for PWMH output on channel 2"]
pub type FPVH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVH3` reader - Fault Protection Value for PWMH output on channel 3"]
pub type FPVH3_R = crate::BitReader;
#[doc = "Field `FPVH3` writer - Fault Protection Value for PWMH output on channel 3"]
pub type FPVH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVH4` reader - Fault Protection Value for PWMH output on channel 4"]
pub type FPVH4_R = crate::BitReader;
#[doc = "Field `FPVH4` writer - Fault Protection Value for PWMH output on channel 4"]
pub type FPVH4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVH5` reader - Fault Protection Value for PWMH output on channel 5"]
pub type FPVH5_R = crate::BitReader;
#[doc = "Field `FPVH5` writer - Fault Protection Value for PWMH output on channel 5"]
pub type FPVH5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVH6` reader - Fault Protection Value for PWMH output on channel 6"]
pub type FPVH6_R = crate::BitReader;
#[doc = "Field `FPVH6` writer - Fault Protection Value for PWMH output on channel 6"]
pub type FPVH6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVH7` reader - Fault Protection Value for PWMH output on channel 7"]
pub type FPVH7_R = crate::BitReader;
#[doc = "Field `FPVH7` writer - Fault Protection Value for PWMH output on channel 7"]
pub type FPVH7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVL0` reader - Fault Protection Value for PWML output on channel 0"]
pub type FPVL0_R = crate::BitReader;
#[doc = "Field `FPVL0` writer - Fault Protection Value for PWML output on channel 0"]
pub type FPVL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVL1` reader - Fault Protection Value for PWML output on channel 1"]
pub type FPVL1_R = crate::BitReader;
#[doc = "Field `FPVL1` writer - Fault Protection Value for PWML output on channel 1"]
pub type FPVL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVL2` reader - Fault Protection Value for PWML output on channel 2"]
pub type FPVL2_R = crate::BitReader;
#[doc = "Field `FPVL2` writer - Fault Protection Value for PWML output on channel 2"]
pub type FPVL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVL3` reader - Fault Protection Value for PWML output on channel 3"]
pub type FPVL3_R = crate::BitReader;
#[doc = "Field `FPVL3` writer - Fault Protection Value for PWML output on channel 3"]
pub type FPVL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVL4` reader - Fault Protection Value for PWML output on channel 4"]
pub type FPVL4_R = crate::BitReader;
#[doc = "Field `FPVL4` writer - Fault Protection Value for PWML output on channel 4"]
pub type FPVL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVL5` reader - Fault Protection Value for PWML output on channel 5"]
pub type FPVL5_R = crate::BitReader;
#[doc = "Field `FPVL5` writer - Fault Protection Value for PWML output on channel 5"]
pub type FPVL5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVL6` reader - Fault Protection Value for PWML output on channel 6"]
pub type FPVL6_R = crate::BitReader;
#[doc = "Field `FPVL6` writer - Fault Protection Value for PWML output on channel 6"]
pub type FPVL6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPVL7` reader - Fault Protection Value for PWML output on channel 7"]
pub type FPVL7_R = crate::BitReader;
#[doc = "Field `FPVL7` writer - Fault Protection Value for PWML output on channel 7"]
pub type FPVL7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[must_use]
    pub fn fpvh0(&mut self) -> FPVH0_W<FPV_SPEC, 0> {
        FPVH0_W::new(self)
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh1(&mut self) -> FPVH1_W<FPV_SPEC, 1> {
        FPVH1_W::new(self)
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh2(&mut self) -> FPVH2_W<FPV_SPEC, 2> {
        FPVH2_W::new(self)
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh3(&mut self) -> FPVH3_W<FPV_SPEC, 3> {
        FPVH3_W::new(self)
    }
    #[doc = "Bit 4 - Fault Protection Value for PWMH output on channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh4(&mut self) -> FPVH4_W<FPV_SPEC, 4> {
        FPVH4_W::new(self)
    }
    #[doc = "Bit 5 - Fault Protection Value for PWMH output on channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh5(&mut self) -> FPVH5_W<FPV_SPEC, 5> {
        FPVH5_W::new(self)
    }
    #[doc = "Bit 6 - Fault Protection Value for PWMH output on channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh6(&mut self) -> FPVH6_W<FPV_SPEC, 6> {
        FPVH6_W::new(self)
    }
    #[doc = "Bit 7 - Fault Protection Value for PWMH output on channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh7(&mut self) -> FPVH7_W<FPV_SPEC, 7> {
        FPVH7_W::new(self)
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl0(&mut self) -> FPVL0_W<FPV_SPEC, 16> {
        FPVL0_W::new(self)
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl1(&mut self) -> FPVL1_W<FPV_SPEC, 17> {
        FPVL1_W::new(self)
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl2(&mut self) -> FPVL2_W<FPV_SPEC, 18> {
        FPVL2_W::new(self)
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl3(&mut self) -> FPVL3_W<FPV_SPEC, 19> {
        FPVL3_W::new(self)
    }
    #[doc = "Bit 20 - Fault Protection Value for PWML output on channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl4(&mut self) -> FPVL4_W<FPV_SPEC, 20> {
        FPVL4_W::new(self)
    }
    #[doc = "Bit 21 - Fault Protection Value for PWML output on channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl5(&mut self) -> FPVL5_W<FPV_SPEC, 21> {
        FPVL5_W::new(self)
    }
    #[doc = "Bit 22 - Fault Protection Value for PWML output on channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl6(&mut self) -> FPVL6_W<FPV_SPEC, 22> {
        FPVL6_W::new(self)
    }
    #[doc = "Bit 23 - Fault Protection Value for PWML output on channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl7(&mut self) -> FPVL7_W<FPV_SPEC, 23> {
        FPVL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Fault Protection Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPV_SPEC;
impl crate::RegisterSpec for FPV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpv::R`](R) reader structure"]
impl crate::Readable for FPV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpv::W`](W) writer structure"]
impl crate::Writable for FPV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPV to value 0"]
impl crate::Resettable for FPV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
