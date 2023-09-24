#[doc = "Register `CHER` writer"]
pub type W = crate::W<CHER_SPEC>;
#[doc = "Field `CH0` writer - Channel 0 Enable"]
pub type CH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1` writer - Channel 1 Enable"]
pub type CH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH2` writer - Channel 2 Enable"]
pub type CH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH3` writer - Channel 3 Enable"]
pub type CH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH4` writer - Channel 4 Enable"]
pub type CH4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH5` writer - Channel 5 Enable"]
pub type CH5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH6` writer - Channel 6 Enable"]
pub type CH6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH7` writer - Channel 7 Enable"]
pub type CH7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH8` writer - Channel 8 Enable"]
pub type CH8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH9` writer - Channel 9 Enable"]
pub type CH9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH10` writer - Channel 10 Enable"]
pub type CH10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH11` writer - Channel 11 Enable"]
pub type CH11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH12` writer - Channel 12 Enable"]
pub type CH12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH13` writer - Channel 13 Enable"]
pub type CH13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH14` writer - Channel 14 Enable"]
pub type CH14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH15` writer - Channel 15 Enable"]
pub type CH15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<CHER_SPEC, 0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<CHER_SPEC, 1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<CHER_SPEC, 2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<CHER_SPEC, 3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<CHER_SPEC, 4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<CHER_SPEC, 5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<CHER_SPEC, 6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<CHER_SPEC, 7> {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> CH8_W<CHER_SPEC, 8> {
        CH8_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> CH9_W<CHER_SPEC, 9> {
        CH9_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> CH10_W<CHER_SPEC, 10> {
        CH10_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> CH11_W<CHER_SPEC, 11> {
        CH11_W::new(self)
    }
    #[doc = "Bit 12 - Channel 12 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> CH12_W<CHER_SPEC, 12> {
        CH12_W::new(self)
    }
    #[doc = "Bit 13 - Channel 13 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> CH13_W<CHER_SPEC, 13> {
        CH13_W::new(self)
    }
    #[doc = "Bit 14 - Channel 14 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> CH14_W<CHER_SPEC, 14> {
        CH14_W::new(self)
    }
    #[doc = "Bit 15 - Channel 15 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> CH15_W<CHER_SPEC, 15> {
        CH15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
