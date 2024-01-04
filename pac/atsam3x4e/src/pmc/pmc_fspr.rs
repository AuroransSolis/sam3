#[doc = "Register `PMC_FSPR` reader"]
pub type R = crate::R<PMC_FSPR_SPEC>;
#[doc = "Register `PMC_FSPR` writer"]
pub type W = crate::W<PMC_FSPR_SPEC>;
#[doc = "Field `FSTP0` reader - Fast Start-up Input Polarityx"]
pub type FSTP0_R = crate::BitReader;
#[doc = "Field `FSTP0` writer - Fast Start-up Input Polarityx"]
pub type FSTP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP1` reader - Fast Start-up Input Polarityx"]
pub type FSTP1_R = crate::BitReader;
#[doc = "Field `FSTP1` writer - Fast Start-up Input Polarityx"]
pub type FSTP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP2` reader - Fast Start-up Input Polarityx"]
pub type FSTP2_R = crate::BitReader;
#[doc = "Field `FSTP2` writer - Fast Start-up Input Polarityx"]
pub type FSTP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP3` reader - Fast Start-up Input Polarityx"]
pub type FSTP3_R = crate::BitReader;
#[doc = "Field `FSTP3` writer - Fast Start-up Input Polarityx"]
pub type FSTP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP4` reader - Fast Start-up Input Polarityx"]
pub type FSTP4_R = crate::BitReader;
#[doc = "Field `FSTP4` writer - Fast Start-up Input Polarityx"]
pub type FSTP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP5` reader - Fast Start-up Input Polarityx"]
pub type FSTP5_R = crate::BitReader;
#[doc = "Field `FSTP5` writer - Fast Start-up Input Polarityx"]
pub type FSTP5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP6` reader - Fast Start-up Input Polarityx"]
pub type FSTP6_R = crate::BitReader;
#[doc = "Field `FSTP6` writer - Fast Start-up Input Polarityx"]
pub type FSTP6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP7` reader - Fast Start-up Input Polarityx"]
pub type FSTP7_R = crate::BitReader;
#[doc = "Field `FSTP7` writer - Fast Start-up Input Polarityx"]
pub type FSTP7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP8` reader - Fast Start-up Input Polarityx"]
pub type FSTP8_R = crate::BitReader;
#[doc = "Field `FSTP8` writer - Fast Start-up Input Polarityx"]
pub type FSTP8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP9` reader - Fast Start-up Input Polarityx"]
pub type FSTP9_R = crate::BitReader;
#[doc = "Field `FSTP9` writer - Fast Start-up Input Polarityx"]
pub type FSTP9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP10` reader - Fast Start-up Input Polarityx"]
pub type FSTP10_R = crate::BitReader;
#[doc = "Field `FSTP10` writer - Fast Start-up Input Polarityx"]
pub type FSTP10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP11` reader - Fast Start-up Input Polarityx"]
pub type FSTP11_R = crate::BitReader;
#[doc = "Field `FSTP11` writer - Fast Start-up Input Polarityx"]
pub type FSTP11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP12` reader - Fast Start-up Input Polarityx"]
pub type FSTP12_R = crate::BitReader;
#[doc = "Field `FSTP12` writer - Fast Start-up Input Polarityx"]
pub type FSTP12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP13` reader - Fast Start-up Input Polarityx"]
pub type FSTP13_R = crate::BitReader;
#[doc = "Field `FSTP13` writer - Fast Start-up Input Polarityx"]
pub type FSTP13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP14` reader - Fast Start-up Input Polarityx"]
pub type FSTP14_R = crate::BitReader;
#[doc = "Field `FSTP14` writer - Fast Start-up Input Polarityx"]
pub type FSTP14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTP15` reader - Fast Start-up Input Polarityx"]
pub type FSTP15_R = crate::BitReader;
#[doc = "Field `FSTP15` writer - Fast Start-up Input Polarityx"]
pub type FSTP15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp0(&self) -> FSTP0_R {
        FSTP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp1(&self) -> FSTP1_R {
        FSTP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp2(&self) -> FSTP2_R {
        FSTP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp3(&self) -> FSTP3_R {
        FSTP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp4(&self) -> FSTP4_R {
        FSTP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp5(&self) -> FSTP5_R {
        FSTP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp6(&self) -> FSTP6_R {
        FSTP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp7(&self) -> FSTP7_R {
        FSTP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp8(&self) -> FSTP8_R {
        FSTP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp9(&self) -> FSTP9_R {
        FSTP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp10(&self) -> FSTP10_R {
        FSTP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp11(&self) -> FSTP11_R {
        FSTP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp12(&self) -> FSTP12_R {
        FSTP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp13(&self) -> FSTP13_R {
        FSTP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp14(&self) -> FSTP14_R {
        FSTP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    pub fn fstp15(&self) -> FSTP15_R {
        FSTP15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp0(&mut self) -> FSTP0_W<PMC_FSPR_SPEC> {
        FSTP0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp1(&mut self) -> FSTP1_W<PMC_FSPR_SPEC> {
        FSTP1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp2(&mut self) -> FSTP2_W<PMC_FSPR_SPEC> {
        FSTP2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp3(&mut self) -> FSTP3_W<PMC_FSPR_SPEC> {
        FSTP3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp4(&mut self) -> FSTP4_W<PMC_FSPR_SPEC> {
        FSTP4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp5(&mut self) -> FSTP5_W<PMC_FSPR_SPEC> {
        FSTP5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp6(&mut self) -> FSTP6_W<PMC_FSPR_SPEC> {
        FSTP6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp7(&mut self) -> FSTP7_W<PMC_FSPR_SPEC> {
        FSTP7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp8(&mut self) -> FSTP8_W<PMC_FSPR_SPEC> {
        FSTP8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp9(&mut self) -> FSTP9_W<PMC_FSPR_SPEC> {
        FSTP9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp10(&mut self) -> FSTP10_W<PMC_FSPR_SPEC> {
        FSTP10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp11(&mut self) -> FSTP11_W<PMC_FSPR_SPEC> {
        FSTP11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp12(&mut self) -> FSTP12_W<PMC_FSPR_SPEC> {
        FSTP12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp13(&mut self) -> FSTP13_W<PMC_FSPR_SPEC> {
        FSTP13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp14(&mut self) -> FSTP14_W<PMC_FSPR_SPEC> {
        FSTP14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Fast Start-up Input Polarityx"]
    #[inline(always)]
    #[must_use]
    pub fn fstp15(&mut self) -> FSTP15_W<PMC_FSPR_SPEC> {
        FSTP15_W::new(self, 15)
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
#[doc = "Fast Start-up Polarity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_fspr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_fspr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMC_FSPR_SPEC;
impl crate::RegisterSpec for PMC_FSPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_fspr::R`](R) reader structure"]
impl crate::Readable for PMC_FSPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmc_fspr::W`](W) writer structure"]
impl crate::Writable for PMC_FSPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMC_FSPR to value 0"]
impl crate::Resettable for PMC_FSPR_SPEC {
    const RESET_VALUE: u32 = 0;
}
