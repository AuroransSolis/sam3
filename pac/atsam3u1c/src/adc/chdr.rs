#[doc = "Register `CHDR` writer"]
pub type W = crate::W<CHDR_SPEC>;
#[doc = "Field `CH0` writer - Channel 0 Disable"]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Channel 1 Disable"]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Channel 2 Disable"]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Channel 3 Disable"]
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` writer - Channel 4 Disable"]
pub type CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` writer - Channel 5 Disable"]
pub type CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` writer - Channel 6 Disable"]
pub type CH6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` writer - Channel 7 Disable"]
pub type CH7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<CHDR_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<CHDR_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<CHDR_SPEC> {
        CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<CHDR_SPEC> {
        CH3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<CHDR_SPEC> {
        CH4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<CHDR_SPEC> {
        CH5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<CHDR_SPEC> {
        CH6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<CHDR_SPEC> {
        CH7_W::new(self, 7)
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
#[doc = "Channel Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHDR_SPEC;
impl crate::RegisterSpec for CHDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chdr::W`](W) writer structure"]
impl crate::Writable for CHDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
