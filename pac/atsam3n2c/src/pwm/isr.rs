#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `CHID0` reader - Channel ID"]
pub type Chid0R = crate::BitReader;
#[doc = "Field `CHID1` reader - Channel ID"]
pub type Chid1R = crate::BitReader;
#[doc = "Field `CHID2` reader - Channel ID"]
pub type Chid2R = crate::BitReader;
#[doc = "Field `CHID3` reader - Channel ID"]
pub type Chid3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel ID"]
    #[inline(always)]
    pub fn chid0(&self) -> Chid0R {
        Chid0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel ID"]
    #[inline(always)]
    pub fn chid1(&self) -> Chid1R {
        Chid1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel ID"]
    #[inline(always)]
    pub fn chid2(&self) -> Chid2R {
        Chid2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel ID"]
    #[inline(always)]
    pub fn chid3(&self) -> Chid3R {
        Chid3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "PWM Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
