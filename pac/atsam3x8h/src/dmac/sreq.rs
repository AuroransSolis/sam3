#[doc = "Register `SREQ` reader"]
pub type R = crate::R<SREQ_SPEC>;
#[doc = "Register `SREQ` writer"]
pub type W = crate::W<SREQ_SPEC>;
#[doc = "Field `SSREQ0` reader - Source Request"]
pub type SSREQ0_R = crate::BitReader;
#[doc = "Field `SSREQ0` writer - Source Request"]
pub type SSREQ0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSREQ0` reader - Destination Request"]
pub type DSREQ0_R = crate::BitReader;
#[doc = "Field `DSREQ0` writer - Destination Request"]
pub type DSREQ0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSREQ1` reader - Source Request"]
pub type SSREQ1_R = crate::BitReader;
#[doc = "Field `SSREQ1` writer - Source Request"]
pub type SSREQ1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSREQ1` reader - Destination Request"]
pub type DSREQ1_R = crate::BitReader;
#[doc = "Field `DSREQ1` writer - Destination Request"]
pub type DSREQ1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSREQ2` reader - Source Request"]
pub type SSREQ2_R = crate::BitReader;
#[doc = "Field `SSREQ2` writer - Source Request"]
pub type SSREQ2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSREQ2` reader - Destination Request"]
pub type DSREQ2_R = crate::BitReader;
#[doc = "Field `DSREQ2` writer - Destination Request"]
pub type DSREQ2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSREQ3` reader - Source Request"]
pub type SSREQ3_R = crate::BitReader;
#[doc = "Field `SSREQ3` writer - Source Request"]
pub type SSREQ3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSREQ3` reader - Destination Request"]
pub type DSREQ3_R = crate::BitReader;
#[doc = "Field `DSREQ3` writer - Destination Request"]
pub type DSREQ3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSREQ4` reader - Source Request"]
pub type SSREQ4_R = crate::BitReader;
#[doc = "Field `SSREQ4` writer - Source Request"]
pub type SSREQ4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSREQ4` reader - Destination Request"]
pub type DSREQ4_R = crate::BitReader;
#[doc = "Field `DSREQ4` writer - Destination Request"]
pub type DSREQ4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSREQ5` reader - Source Request"]
pub type SSREQ5_R = crate::BitReader;
#[doc = "Field `SSREQ5` writer - Source Request"]
pub type SSREQ5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSREQ5` reader - Destination Request"]
pub type DSREQ5_R = crate::BitReader;
#[doc = "Field `DSREQ5` writer - Destination Request"]
pub type DSREQ5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Source Request"]
    #[inline(always)]
    pub fn ssreq0(&self) -> SSREQ0_R {
        SSREQ0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Destination Request"]
    #[inline(always)]
    pub fn dsreq0(&self) -> DSREQ0_R {
        DSREQ0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source Request"]
    #[inline(always)]
    pub fn ssreq1(&self) -> SSREQ1_R {
        SSREQ1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination Request"]
    #[inline(always)]
    pub fn dsreq1(&self) -> DSREQ1_R {
        DSREQ1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Source Request"]
    #[inline(always)]
    pub fn ssreq2(&self) -> SSREQ2_R {
        SSREQ2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Destination Request"]
    #[inline(always)]
    pub fn dsreq2(&self) -> DSREQ2_R {
        DSREQ2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source Request"]
    #[inline(always)]
    pub fn ssreq3(&self) -> SSREQ3_R {
        SSREQ3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Destination Request"]
    #[inline(always)]
    pub fn dsreq3(&self) -> DSREQ3_R {
        DSREQ3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Source Request"]
    #[inline(always)]
    pub fn ssreq4(&self) -> SSREQ4_R {
        SSREQ4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Destination Request"]
    #[inline(always)]
    pub fn dsreq4(&self) -> DSREQ4_R {
        DSREQ4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Source Request"]
    #[inline(always)]
    pub fn ssreq5(&self) -> SSREQ5_R {
        SSREQ5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Destination Request"]
    #[inline(always)]
    pub fn dsreq5(&self) -> DSREQ5_R {
        DSREQ5_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Request"]
    #[inline(always)]
    #[must_use]
    pub fn ssreq0(&mut self) -> SSREQ0_W<SREQ_SPEC> {
        SSREQ0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Destination Request"]
    #[inline(always)]
    #[must_use]
    pub fn dsreq0(&mut self) -> DSREQ0_W<SREQ_SPEC> {
        DSREQ0_W::new(self, 1)
    }
    #[doc = "Bit 2 - Source Request"]
    #[inline(always)]
    #[must_use]
    pub fn ssreq1(&mut self) -> SSREQ1_W<SREQ_SPEC> {
        SSREQ1_W::new(self, 2)
    }
    #[doc = "Bit 3 - Destination Request"]
    #[inline(always)]
    #[must_use]
    pub fn dsreq1(&mut self) -> DSREQ1_W<SREQ_SPEC> {
        DSREQ1_W::new(self, 3)
    }
    #[doc = "Bit 4 - Source Request"]
    #[inline(always)]
    #[must_use]
    pub fn ssreq2(&mut self) -> SSREQ2_W<SREQ_SPEC> {
        SSREQ2_W::new(self, 4)
    }
    #[doc = "Bit 5 - Destination Request"]
    #[inline(always)]
    #[must_use]
    pub fn dsreq2(&mut self) -> DSREQ2_W<SREQ_SPEC> {
        DSREQ2_W::new(self, 5)
    }
    #[doc = "Bit 6 - Source Request"]
    #[inline(always)]
    #[must_use]
    pub fn ssreq3(&mut self) -> SSREQ3_W<SREQ_SPEC> {
        SSREQ3_W::new(self, 6)
    }
    #[doc = "Bit 7 - Destination Request"]
    #[inline(always)]
    #[must_use]
    pub fn dsreq3(&mut self) -> DSREQ3_W<SREQ_SPEC> {
        DSREQ3_W::new(self, 7)
    }
    #[doc = "Bit 8 - Source Request"]
    #[inline(always)]
    #[must_use]
    pub fn ssreq4(&mut self) -> SSREQ4_W<SREQ_SPEC> {
        SSREQ4_W::new(self, 8)
    }
    #[doc = "Bit 9 - Destination Request"]
    #[inline(always)]
    #[must_use]
    pub fn dsreq4(&mut self) -> DSREQ4_W<SREQ_SPEC> {
        DSREQ4_W::new(self, 9)
    }
    #[doc = "Bit 10 - Source Request"]
    #[inline(always)]
    #[must_use]
    pub fn ssreq5(&mut self) -> SSREQ5_W<SREQ_SPEC> {
        SSREQ5_W::new(self, 10)
    }
    #[doc = "Bit 11 - Destination Request"]
    #[inline(always)]
    #[must_use]
    pub fn dsreq5(&mut self) -> DSREQ5_W<SREQ_SPEC> {
        DSREQ5_W::new(self, 11)
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
#[doc = "DMAC Software Single Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SREQ_SPEC;
impl crate::RegisterSpec for SREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sreq::R`](R) reader structure"]
impl crate::Readable for SREQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sreq::W`](W) writer structure"]
impl crate::Writable for SREQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SREQ to value 0"]
impl crate::Resettable for SREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
