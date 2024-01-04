#[doc = "Register `DIS` writer"]
pub type W = crate::W<DIS_SPEC>;
#[doc = "Field `CHID0` writer - Channel ID"]
pub type CHID0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID1` writer - Channel ID"]
pub type CHID1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID2` writer - Channel ID"]
pub type CHID2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID3` writer - Channel ID"]
pub type CHID3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID4` writer - Channel ID"]
pub type CHID4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID5` writer - Channel ID"]
pub type CHID5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID6` writer - Channel ID"]
pub type CHID6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID7` writer - Channel ID"]
pub type CHID7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn chid0(&mut self) -> CHID0_W<DIS_SPEC> {
        CHID0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn chid1(&mut self) -> CHID1_W<DIS_SPEC> {
        CHID1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn chid2(&mut self) -> CHID2_W<DIS_SPEC> {
        CHID2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn chid3(&mut self) -> CHID3_W<DIS_SPEC> {
        CHID3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn chid4(&mut self) -> CHID4_W<DIS_SPEC> {
        CHID4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn chid5(&mut self) -> CHID5_W<DIS_SPEC> {
        CHID5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn chid6(&mut self) -> CHID6_W<DIS_SPEC> {
        CHID6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn chid7(&mut self) -> CHID7_W<DIS_SPEC> {
        CHID7_W::new(self, 7)
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
#[doc = "PWM Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dis::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIS_SPEC;
impl crate::RegisterSpec for DIS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dis::W`](W) writer structure"]
impl crate::Writable for DIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
