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
#[doc = "Field `CH8` reader - Channel 8 Status"]
pub type Ch8R = crate::BitReader;
#[doc = "Field `CH9` reader - Channel 9 Status"]
pub type Ch9R = crate::BitReader;
#[doc = "Field `CH10` reader - Channel 10 Status"]
pub type Ch10R = crate::BitReader;
#[doc = "Field `CH11` reader - Channel 11 Status"]
pub type Ch11R = crate::BitReader;
#[doc = "Field `CH12` reader - Channel 12 Status"]
pub type Ch12R = crate::BitReader;
#[doc = "Field `CH13` reader - Channel 13 Status"]
pub type Ch13R = crate::BitReader;
#[doc = "Field `CH14` reader - Channel 14 Status"]
pub type Ch14R = crate::BitReader;
#[doc = "Field `CH15` reader - Channel 15 Status"]
pub type Ch15R = crate::BitReader;
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
    #[doc = "Bit 8 - Channel 8 Status"]
    #[inline(always)]
    pub fn ch8(&self) -> Ch8R {
        Ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Status"]
    #[inline(always)]
    pub fn ch9(&self) -> Ch9R {
        Ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Status"]
    #[inline(always)]
    pub fn ch10(&self) -> Ch10R {
        Ch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Status"]
    #[inline(always)]
    pub fn ch11(&self) -> Ch11R {
        Ch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 12 Status"]
    #[inline(always)]
    pub fn ch12(&self) -> Ch12R {
        Ch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 13 Status"]
    #[inline(always)]
    pub fn ch13(&self) -> Ch13R {
        Ch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 14 Status"]
    #[inline(always)]
    pub fn ch14(&self) -> Ch14R {
        Ch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 15 Status"]
    #[inline(always)]
    pub fn ch15(&self) -> Ch15R {
        Ch15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
