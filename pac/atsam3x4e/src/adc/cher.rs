#[doc = "Register `CHER` writer"]
pub type W = crate::W<CHER_SPEC>;
#[doc = "Field `CH0` writer - Channel 0 Enable"]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Channel 1 Enable"]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Channel 2 Enable"]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Channel 3 Enable"]
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` writer - Channel 4 Enable"]
pub type CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` writer - Channel 5 Enable"]
pub type CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` writer - Channel 6 Enable"]
pub type CH6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` writer - Channel 7 Enable"]
pub type CH7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` writer - Channel 8 Enable"]
pub type CH8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` writer - Channel 9 Enable"]
pub type CH9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` writer - Channel 10 Enable"]
pub type CH10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` writer - Channel 11 Enable"]
pub type CH11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` writer - Channel 12 Enable"]
pub type CH12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` writer - Channel 13 Enable"]
pub type CH13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` writer - Channel 14 Enable"]
pub type CH14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` writer - Channel 15 Enable"]
pub type CH15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<CHER_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<CHER_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<CHER_SPEC> {
        CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<CHER_SPEC> {
        CH3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<CHER_SPEC> {
        CH4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<CHER_SPEC> {
        CH5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<CHER_SPEC> {
        CH6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<CHER_SPEC> {
        CH7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 8 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> CH8_W<CHER_SPEC> {
        CH8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 9 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> CH9_W<CHER_SPEC> {
        CH9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 10 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> CH10_W<CHER_SPEC> {
        CH10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 11 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> CH11_W<CHER_SPEC> {
        CH11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 12 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> CH12_W<CHER_SPEC> {
        CH12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 13 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> CH13_W<CHER_SPEC> {
        CH13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel 14 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> CH14_W<CHER_SPEC> {
        CH14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel 15 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> CH15_W<CHER_SPEC> {
        CH15_W::new(self, 15)
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
#[doc = "Channel Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cher::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHER_SPEC;
impl crate::RegisterSpec for CHER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cher::W`](W) writer structure"]
impl crate::Writable for CHER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
