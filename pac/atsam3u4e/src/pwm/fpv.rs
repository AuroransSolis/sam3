#[doc = "Register `FPV` reader"]
pub type R = crate::R<FPV_SPEC>;
#[doc = "Register `FPV` writer"]
pub type W = crate::W<FPV_SPEC>;
#[doc = "Field `FPVH0` reader - Fault Protection Value for PWMH output on channel 0"]
pub type FPVH0_R = crate::BitReader;
#[doc = "Field `FPVH0` writer - Fault Protection Value for PWMH output on channel 0"]
pub type FPVH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVH1` reader - Fault Protection Value for PWMH output on channel 1"]
pub type FPVH1_R = crate::BitReader;
#[doc = "Field `FPVH1` writer - Fault Protection Value for PWMH output on channel 1"]
pub type FPVH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVH2` reader - Fault Protection Value for PWMH output on channel 2"]
pub type FPVH2_R = crate::BitReader;
#[doc = "Field `FPVH2` writer - Fault Protection Value for PWMH output on channel 2"]
pub type FPVH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVH3` reader - Fault Protection Value for PWMH output on channel 3"]
pub type FPVH3_R = crate::BitReader;
#[doc = "Field `FPVH3` writer - Fault Protection Value for PWMH output on channel 3"]
pub type FPVH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVL0` reader - Fault Protection Value for PWML output on channel 0"]
pub type FPVL0_R = crate::BitReader;
#[doc = "Field `FPVL0` writer - Fault Protection Value for PWML output on channel 0"]
pub type FPVL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVL1` reader - Fault Protection Value for PWML output on channel 1"]
pub type FPVL1_R = crate::BitReader;
#[doc = "Field `FPVL1` writer - Fault Protection Value for PWML output on channel 1"]
pub type FPVL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVL2` reader - Fault Protection Value for PWML output on channel 2"]
pub type FPVL2_R = crate::BitReader;
#[doc = "Field `FPVL2` writer - Fault Protection Value for PWML output on channel 2"]
pub type FPVL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPVL3` reader - Fault Protection Value for PWML output on channel 3"]
pub type FPVL3_R = crate::BitReader;
#[doc = "Field `FPVL3` writer - Fault Protection Value for PWML output on channel 3"]
pub type FPVL3_W<'a, REG> = crate::BitWriter<'a, REG>;
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
}
impl W {
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh0(&mut self) -> FPVH0_W<FPV_SPEC> {
        FPVH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh1(&mut self) -> FPVH1_W<FPV_SPEC> {
        FPVH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh2(&mut self) -> FPVH2_W<FPV_SPEC> {
        FPVH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh3(&mut self) -> FPVH3_W<FPV_SPEC> {
        FPVH3_W::new(self, 3)
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl0(&mut self) -> FPVL0_W<FPV_SPEC> {
        FPVL0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl1(&mut self) -> FPVL1_W<FPV_SPEC> {
        FPVL1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl2(&mut self) -> FPVL2_W<FPV_SPEC> {
        FPVL2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl3(&mut self) -> FPVL3_W<FPV_SPEC> {
        FPVL3_W::new(self, 19)
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
