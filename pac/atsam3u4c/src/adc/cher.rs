#[doc = "Register `CHER` writer"]
pub type W = crate::W<CherSpec>;
#[doc = "Field `CH0` writer - Channel 0 Enable"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Channel 1 Enable"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Channel 2 Enable"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Channel 3 Enable"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` writer - Channel 4 Enable"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` writer - Channel 5 Enable"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` writer - Channel 6 Enable"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` writer - Channel 7 Enable"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<CherSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<CherSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<CherSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<CherSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> Ch4W<CherSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> Ch5W<CherSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> Ch6W<CherSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> Ch7W<CherSpec> {
        Ch7W::new(self, 7)
    }
}
#[doc = "Channel Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cher::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CherSpec;
impl crate::RegisterSpec for CherSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cher::W`](W) writer structure"]
impl crate::Writable for CherSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
