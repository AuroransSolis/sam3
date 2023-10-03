#[doc = "Register `OOV` reader"]
pub type R = crate::R<OOV_SPEC>;
#[doc = "Register `OOV` writer"]
pub type W = crate::W<OOV_SPEC>;
#[doc = "Field `OOVH0` reader - Output Override Value for PWMH output of the channel 0"]
pub type OOVH0_R = crate::BitReader;
#[doc = "Field `OOVH0` writer - Output Override Value for PWMH output of the channel 0"]
pub type OOVH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVH1` reader - Output Override Value for PWMH output of the channel 1"]
pub type OOVH1_R = crate::BitReader;
#[doc = "Field `OOVH1` writer - Output Override Value for PWMH output of the channel 1"]
pub type OOVH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVH2` reader - Output Override Value for PWMH output of the channel 2"]
pub type OOVH2_R = crate::BitReader;
#[doc = "Field `OOVH2` writer - Output Override Value for PWMH output of the channel 2"]
pub type OOVH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVH3` reader - Output Override Value for PWMH output of the channel 3"]
pub type OOVH3_R = crate::BitReader;
#[doc = "Field `OOVH3` writer - Output Override Value for PWMH output of the channel 3"]
pub type OOVH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVH4` reader - Output Override Value for PWMH output of the channel 4"]
pub type OOVH4_R = crate::BitReader;
#[doc = "Field `OOVH4` writer - Output Override Value for PWMH output of the channel 4"]
pub type OOVH4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVH5` reader - Output Override Value for PWMH output of the channel 5"]
pub type OOVH5_R = crate::BitReader;
#[doc = "Field `OOVH5` writer - Output Override Value for PWMH output of the channel 5"]
pub type OOVH5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVH6` reader - Output Override Value for PWMH output of the channel 6"]
pub type OOVH6_R = crate::BitReader;
#[doc = "Field `OOVH6` writer - Output Override Value for PWMH output of the channel 6"]
pub type OOVH6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVH7` reader - Output Override Value for PWMH output of the channel 7"]
pub type OOVH7_R = crate::BitReader;
#[doc = "Field `OOVH7` writer - Output Override Value for PWMH output of the channel 7"]
pub type OOVH7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVL0` reader - Output Override Value for PWML output of the channel 0"]
pub type OOVL0_R = crate::BitReader;
#[doc = "Field `OOVL0` writer - Output Override Value for PWML output of the channel 0"]
pub type OOVL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVL1` reader - Output Override Value for PWML output of the channel 1"]
pub type OOVL1_R = crate::BitReader;
#[doc = "Field `OOVL1` writer - Output Override Value for PWML output of the channel 1"]
pub type OOVL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVL2` reader - Output Override Value for PWML output of the channel 2"]
pub type OOVL2_R = crate::BitReader;
#[doc = "Field `OOVL2` writer - Output Override Value for PWML output of the channel 2"]
pub type OOVL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVL3` reader - Output Override Value for PWML output of the channel 3"]
pub type OOVL3_R = crate::BitReader;
#[doc = "Field `OOVL3` writer - Output Override Value for PWML output of the channel 3"]
pub type OOVL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVL4` reader - Output Override Value for PWML output of the channel 4"]
pub type OOVL4_R = crate::BitReader;
#[doc = "Field `OOVL4` writer - Output Override Value for PWML output of the channel 4"]
pub type OOVL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVL5` reader - Output Override Value for PWML output of the channel 5"]
pub type OOVL5_R = crate::BitReader;
#[doc = "Field `OOVL5` writer - Output Override Value for PWML output of the channel 5"]
pub type OOVL5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVL6` reader - Output Override Value for PWML output of the channel 6"]
pub type OOVL6_R = crate::BitReader;
#[doc = "Field `OOVL6` writer - Output Override Value for PWML output of the channel 6"]
pub type OOVL6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OOVL7` reader - Output Override Value for PWML output of the channel 7"]
pub type OOVL7_R = crate::BitReader;
#[doc = "Field `OOVL7` writer - Output Override Value for PWML output of the channel 7"]
pub type OOVL7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Output Override Value for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn oovh0(&self) -> OOVH0_R {
        OOVH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Override Value for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn oovh1(&self) -> OOVH1_R {
        OOVH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Override Value for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn oovh2(&self) -> OOVH2_R {
        OOVH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Override Value for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn oovh3(&self) -> OOVH3_R {
        OOVH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output Override Value for PWMH output of the channel 4"]
    #[inline(always)]
    pub fn oovh4(&self) -> OOVH4_R {
        OOVH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output Override Value for PWMH output of the channel 5"]
    #[inline(always)]
    pub fn oovh5(&self) -> OOVH5_R {
        OOVH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output Override Value for PWMH output of the channel 6"]
    #[inline(always)]
    pub fn oovh6(&self) -> OOVH6_R {
        OOVH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output Override Value for PWMH output of the channel 7"]
    #[inline(always)]
    pub fn oovh7(&self) -> OOVH7_R {
        OOVH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Override Value for PWML output of the channel 0"]
    #[inline(always)]
    pub fn oovl0(&self) -> OOVL0_R {
        OOVL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output Override Value for PWML output of the channel 1"]
    #[inline(always)]
    pub fn oovl1(&self) -> OOVL1_R {
        OOVL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Override Value for PWML output of the channel 2"]
    #[inline(always)]
    pub fn oovl2(&self) -> OOVL2_R {
        OOVL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output Override Value for PWML output of the channel 3"]
    #[inline(always)]
    pub fn oovl3(&self) -> OOVL3_R {
        OOVL3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output Override Value for PWML output of the channel 4"]
    #[inline(always)]
    pub fn oovl4(&self) -> OOVL4_R {
        OOVL4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output Override Value for PWML output of the channel 5"]
    #[inline(always)]
    pub fn oovl5(&self) -> OOVL5_R {
        OOVL5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Output Override Value for PWML output of the channel 6"]
    #[inline(always)]
    pub fn oovl6(&self) -> OOVL6_R {
        OOVL6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Output Override Value for PWML output of the channel 7"]
    #[inline(always)]
    pub fn oovl7(&self) -> OOVL7_R {
        OOVL7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Override Value for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn oovh0(&mut self) -> OOVH0_W<OOV_SPEC, 0> {
        OOVH0_W::new(self)
    }
    #[doc = "Bit 1 - Output Override Value for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn oovh1(&mut self) -> OOVH1_W<OOV_SPEC, 1> {
        OOVH1_W::new(self)
    }
    #[doc = "Bit 2 - Output Override Value for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn oovh2(&mut self) -> OOVH2_W<OOV_SPEC, 2> {
        OOVH2_W::new(self)
    }
    #[doc = "Bit 3 - Output Override Value for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn oovh3(&mut self) -> OOVH3_W<OOV_SPEC, 3> {
        OOVH3_W::new(self)
    }
    #[doc = "Bit 4 - Output Override Value for PWMH output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn oovh4(&mut self) -> OOVH4_W<OOV_SPEC, 4> {
        OOVH4_W::new(self)
    }
    #[doc = "Bit 5 - Output Override Value for PWMH output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn oovh5(&mut self) -> OOVH5_W<OOV_SPEC, 5> {
        OOVH5_W::new(self)
    }
    #[doc = "Bit 6 - Output Override Value for PWMH output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn oovh6(&mut self) -> OOVH6_W<OOV_SPEC, 6> {
        OOVH6_W::new(self)
    }
    #[doc = "Bit 7 - Output Override Value for PWMH output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn oovh7(&mut self) -> OOVH7_W<OOV_SPEC, 7> {
        OOVH7_W::new(self)
    }
    #[doc = "Bit 16 - Output Override Value for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn oovl0(&mut self) -> OOVL0_W<OOV_SPEC, 16> {
        OOVL0_W::new(self)
    }
    #[doc = "Bit 17 - Output Override Value for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn oovl1(&mut self) -> OOVL1_W<OOV_SPEC, 17> {
        OOVL1_W::new(self)
    }
    #[doc = "Bit 18 - Output Override Value for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn oovl2(&mut self) -> OOVL2_W<OOV_SPEC, 18> {
        OOVL2_W::new(self)
    }
    #[doc = "Bit 19 - Output Override Value for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn oovl3(&mut self) -> OOVL3_W<OOV_SPEC, 19> {
        OOVL3_W::new(self)
    }
    #[doc = "Bit 20 - Output Override Value for PWML output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn oovl4(&mut self) -> OOVL4_W<OOV_SPEC, 20> {
        OOVL4_W::new(self)
    }
    #[doc = "Bit 21 - Output Override Value for PWML output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn oovl5(&mut self) -> OOVL5_W<OOV_SPEC, 21> {
        OOVL5_W::new(self)
    }
    #[doc = "Bit 22 - Output Override Value for PWML output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn oovl6(&mut self) -> OOVL6_W<OOV_SPEC, 22> {
        OOVL6_W::new(self)
    }
    #[doc = "Bit 23 - Output Override Value for PWML output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn oovl7(&mut self) -> OOVL7_W<OOV_SPEC, 23> {
        OOVL7_W::new(self)
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
#[doc = "PWM Output Override Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oov::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oov::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OOV_SPEC;
impl crate::RegisterSpec for OOV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oov::R`](R) reader structure"]
impl crate::Readable for OOV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oov::W`](W) writer structure"]
impl crate::Writable for OOV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OOV to value 0"]
impl crate::Resettable for OOV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
