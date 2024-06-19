#[doc = "Register `CHSR` reader"]
pub type R = crate::R<ChsrSpec>;
#[doc = "Field `CH0` reader - Channel 0 Status"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH1` reader - Channel 1 Status"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH2` reader - Channel 2 Status"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH3` reader - Channel 3 Status"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH4` reader - Channel 4 Status"]
pub type Ch4R = crate::BitReader;
#[doc = "Field `CH5` reader - Channel 5 Status"]
pub type Ch5R = crate::BitReader;
#[doc = "Field `CH6` reader - Channel 6 Status"]
pub type Ch6R = crate::BitReader;
#[doc = "Field `CH7` reader - Channel 7 Status"]
pub type Ch7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Status"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Status"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Status"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Status"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Status"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Status"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Status"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Status"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChsrSpec;
impl crate::RegisterSpec for ChsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsr::R`](R) reader structure"]
impl crate::Readable for ChsrSpec {}
#[doc = "`reset()` method sets CHSR to value 0"]
impl crate::Resettable for ChsrSpec {
    const RESET_VALUE: u32 = 0;
}
