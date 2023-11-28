#[doc = "Register `ELMR0` reader"]
pub type R = crate::R<ELMR0_SPEC>;
#[doc = "Register `ELMR0` writer"]
pub type W = crate::W<ELMR0_SPEC>;
#[doc = "Field `CSEL0` reader - Comparison 0 Selection"]
pub type CSEL0_R = crate::BitReader;
#[doc = "Field `CSEL0` writer - Comparison 0 Selection"]
pub type CSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEL1` reader - Comparison 1 Selection"]
pub type CSEL1_R = crate::BitReader;
#[doc = "Field `CSEL1` writer - Comparison 1 Selection"]
pub type CSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEL2` reader - Comparison 2 Selection"]
pub type CSEL2_R = crate::BitReader;
#[doc = "Field `CSEL2` writer - Comparison 2 Selection"]
pub type CSEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEL3` reader - Comparison 3 Selection"]
pub type CSEL3_R = crate::BitReader;
#[doc = "Field `CSEL3` writer - Comparison 3 Selection"]
pub type CSEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEL4` reader - Comparison 4 Selection"]
pub type CSEL4_R = crate::BitReader;
#[doc = "Field `CSEL4` writer - Comparison 4 Selection"]
pub type CSEL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEL5` reader - Comparison 5 Selection"]
pub type CSEL5_R = crate::BitReader;
#[doc = "Field `CSEL5` writer - Comparison 5 Selection"]
pub type CSEL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEL6` reader - Comparison 6 Selection"]
pub type CSEL6_R = crate::BitReader;
#[doc = "Field `CSEL6` writer - Comparison 6 Selection"]
pub type CSEL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEL7` reader - Comparison 7 Selection"]
pub type CSEL7_R = crate::BitReader;
#[doc = "Field `CSEL7` writer - Comparison 7 Selection"]
pub type CSEL7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparison 0 Selection"]
    #[inline(always)]
    pub fn csel0(&self) -> CSEL0_R {
        CSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparison 1 Selection"]
    #[inline(always)]
    pub fn csel1(&self) -> CSEL1_R {
        CSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparison 2 Selection"]
    #[inline(always)]
    pub fn csel2(&self) -> CSEL2_R {
        CSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparison 3 Selection"]
    #[inline(always)]
    pub fn csel3(&self) -> CSEL3_R {
        CSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparison 4 Selection"]
    #[inline(always)]
    pub fn csel4(&self) -> CSEL4_R {
        CSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparison 5 Selection"]
    #[inline(always)]
    pub fn csel5(&self) -> CSEL5_R {
        CSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparison 6 Selection"]
    #[inline(always)]
    pub fn csel6(&self) -> CSEL6_R {
        CSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparison 7 Selection"]
    #[inline(always)]
    pub fn csel7(&self) -> CSEL7_R {
        CSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparison 0 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel0(&mut self) -> CSEL0_W<ELMR0_SPEC> {
        CSEL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparison 1 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel1(&mut self) -> CSEL1_W<ELMR0_SPEC> {
        CSEL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Comparison 2 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel2(&mut self) -> CSEL2_W<ELMR0_SPEC> {
        CSEL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Comparison 3 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel3(&mut self) -> CSEL3_W<ELMR0_SPEC> {
        CSEL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Comparison 4 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel4(&mut self) -> CSEL4_W<ELMR0_SPEC> {
        CSEL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Comparison 5 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel5(&mut self) -> CSEL5_W<ELMR0_SPEC> {
        CSEL5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Comparison 6 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel6(&mut self) -> CSEL6_W<ELMR0_SPEC> {
        CSEL6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Comparison 7 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel7(&mut self) -> CSEL7_W<ELMR0_SPEC> {
        CSEL7_W::new(self, 7)
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
#[doc = "PWM Event Line 0 Mode Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`elmr0::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`elmr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ELMR0_SPEC;
impl crate::RegisterSpec for ELMR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elmr0::R`](R) reader structure"]
impl crate::Readable for ELMR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`elmr0::W`](W) writer structure"]
impl crate::Writable for ELMR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
