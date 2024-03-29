#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `CHID0` reader - Channel ID"]
pub type Chid0R = crate::BitReader;
#[doc = "Field `CHID1` reader - Channel ID"]
pub type Chid1R = crate::BitReader;
#[doc = "Field `CHID2` reader - Channel ID"]
pub type Chid2R = crate::BitReader;
#[doc = "Field `CHID3` reader - Channel ID"]
pub type Chid3R = crate::BitReader;
#[doc = "Field `CHID4` reader - Channel ID"]
pub type Chid4R = crate::BitReader;
#[doc = "Field `CHID5` reader - Channel ID"]
pub type Chid5R = crate::BitReader;
#[doc = "Field `CHID6` reader - Channel ID"]
pub type Chid6R = crate::BitReader;
#[doc = "Field `CHID7` reader - Channel ID"]
pub type Chid7R = crate::BitReader;
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
    #[doc = "Bit 4 - Channel ID"]
    #[inline(always)]
    pub fn chid4(&self) -> Chid4R {
        Chid4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel ID"]
    #[inline(always)]
    pub fn chid5(&self) -> Chid5R {
        Chid5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel ID"]
    #[inline(always)]
    pub fn chid6(&self) -> Chid6R {
        Chid6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel ID"]
    #[inline(always)]
    pub fn chid7(&self) -> Chid7R {
        Chid7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "PWM Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
