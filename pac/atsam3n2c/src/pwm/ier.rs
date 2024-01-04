#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `CHID0` writer - Channel ID."]
pub type CHID0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID1` writer - Channel ID."]
pub type CHID1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID2` writer - Channel ID."]
pub type CHID2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID3` writer - Channel ID."]
pub type CHID3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel ID."]
    #[inline(always)]
    #[must_use]
    pub fn chid0(&mut self) -> CHID0_W<IER_SPEC> {
        CHID0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel ID."]
    #[inline(always)]
    #[must_use]
    pub fn chid1(&mut self) -> CHID1_W<IER_SPEC> {
        CHID1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel ID."]
    #[inline(always)]
    #[must_use]
    pub fn chid2(&mut self) -> CHID2_W<IER_SPEC> {
        CHID2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel ID."]
    #[inline(always)]
    #[must_use]
    pub fn chid3(&mut self) -> CHID3_W<IER_SPEC> {
        CHID3_W::new(self, 3)
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
#[doc = "PWM Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
