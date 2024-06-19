#[doc = "Register `PPDSR` reader"]
pub type R = crate::R<PpdsrSpec>;
#[doc = "Field `P0` reader - Pull Down Status."]
pub type P0R = crate::BitReader;
#[doc = "Field `P1` reader - Pull Down Status."]
pub type P1R = crate::BitReader;
#[doc = "Field `P2` reader - Pull Down Status."]
pub type P2R = crate::BitReader;
#[doc = "Field `P3` reader - Pull Down Status."]
pub type P3R = crate::BitReader;
#[doc = "Field `P4` reader - Pull Down Status."]
pub type P4R = crate::BitReader;
#[doc = "Field `P5` reader - Pull Down Status."]
pub type P5R = crate::BitReader;
#[doc = "Field `P6` reader - Pull Down Status."]
pub type P6R = crate::BitReader;
#[doc = "Field `P7` reader - Pull Down Status."]
pub type P7R = crate::BitReader;
#[doc = "Field `P8` reader - Pull Down Status."]
pub type P8R = crate::BitReader;
#[doc = "Field `P9` reader - Pull Down Status."]
pub type P9R = crate::BitReader;
#[doc = "Field `P10` reader - Pull Down Status."]
pub type P10R = crate::BitReader;
#[doc = "Field `P11` reader - Pull Down Status."]
pub type P11R = crate::BitReader;
#[doc = "Field `P12` reader - Pull Down Status."]
pub type P12R = crate::BitReader;
#[doc = "Field `P13` reader - Pull Down Status."]
pub type P13R = crate::BitReader;
#[doc = "Field `P14` reader - Pull Down Status."]
pub type P14R = crate::BitReader;
#[doc = "Field `P15` reader - Pull Down Status."]
pub type P15R = crate::BitReader;
#[doc = "Field `P16` reader - Pull Down Status."]
pub type P16R = crate::BitReader;
#[doc = "Field `P17` reader - Pull Down Status."]
pub type P17R = crate::BitReader;
#[doc = "Field `P18` reader - Pull Down Status."]
pub type P18R = crate::BitReader;
#[doc = "Field `P19` reader - Pull Down Status."]
pub type P19R = crate::BitReader;
#[doc = "Field `P20` reader - Pull Down Status."]
pub type P20R = crate::BitReader;
#[doc = "Field `P21` reader - Pull Down Status."]
pub type P21R = crate::BitReader;
#[doc = "Field `P22` reader - Pull Down Status."]
pub type P22R = crate::BitReader;
#[doc = "Field `P23` reader - Pull Down Status."]
pub type P23R = crate::BitReader;
#[doc = "Field `P24` reader - Pull Down Status."]
pub type P24R = crate::BitReader;
#[doc = "Field `P25` reader - Pull Down Status."]
pub type P25R = crate::BitReader;
#[doc = "Field `P26` reader - Pull Down Status."]
pub type P26R = crate::BitReader;
#[doc = "Field `P27` reader - Pull Down Status."]
pub type P27R = crate::BitReader;
#[doc = "Field `P28` reader - Pull Down Status."]
pub type P28R = crate::BitReader;
#[doc = "Field `P29` reader - Pull Down Status."]
pub type P29R = crate::BitReader;
#[doc = "Field `P30` reader - Pull Down Status."]
pub type P30R = crate::BitReader;
#[doc = "Field `P31` reader - Pull Down Status."]
pub type P31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Pull Down Status."]
    #[inline(always)]
    pub fn p0(&self) -> P0R {
        P0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pull Down Status."]
    #[inline(always)]
    pub fn p1(&self) -> P1R {
        P1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull Down Status."]
    #[inline(always)]
    pub fn p2(&self) -> P2R {
        P2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pull Down Status."]
    #[inline(always)]
    pub fn p3(&self) -> P3R {
        P3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pull Down Status."]
    #[inline(always)]
    pub fn p4(&self) -> P4R {
        P4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Status."]
    #[inline(always)]
    pub fn p5(&self) -> P5R {
        P5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pull Down Status."]
    #[inline(always)]
    pub fn p6(&self) -> P6R {
        P6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pull Down Status."]
    #[inline(always)]
    pub fn p7(&self) -> P7R {
        P7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull Down Status."]
    #[inline(always)]
    pub fn p8(&self) -> P8R {
        P8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pull Down Status."]
    #[inline(always)]
    pub fn p9(&self) -> P9R {
        P9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pull Down Status."]
    #[inline(always)]
    pub fn p10(&self) -> P10R {
        P10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pull Down Status."]
    #[inline(always)]
    pub fn p11(&self) -> P11R {
        P11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pull Down Status."]
    #[inline(always)]
    pub fn p12(&self) -> P12R {
        P12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pull Down Status."]
    #[inline(always)]
    pub fn p13(&self) -> P13R {
        P13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pull Down Status."]
    #[inline(always)]
    pub fn p14(&self) -> P14R {
        P14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pull Down Status."]
    #[inline(always)]
    pub fn p15(&self) -> P15R {
        P15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pull Down Status."]
    #[inline(always)]
    pub fn p16(&self) -> P16R {
        P16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pull Down Status."]
    #[inline(always)]
    pub fn p17(&self) -> P17R {
        P17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pull Down Status."]
    #[inline(always)]
    pub fn p18(&self) -> P18R {
        P18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Pull Down Status."]
    #[inline(always)]
    pub fn p19(&self) -> P19R {
        P19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pull Down Status."]
    #[inline(always)]
    pub fn p20(&self) -> P20R {
        P20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Status."]
    #[inline(always)]
    pub fn p21(&self) -> P21R {
        P21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pull Down Status."]
    #[inline(always)]
    pub fn p22(&self) -> P22R {
        P22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Pull Down Status."]
    #[inline(always)]
    pub fn p23(&self) -> P23R {
        P23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Pull Down Status."]
    #[inline(always)]
    pub fn p24(&self) -> P24R {
        P24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Pull Down Status."]
    #[inline(always)]
    pub fn p25(&self) -> P25R {
        P25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pull Down Status."]
    #[inline(always)]
    pub fn p26(&self) -> P26R {
        P26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Pull Down Status."]
    #[inline(always)]
    pub fn p27(&self) -> P27R {
        P27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pull Down Status."]
    #[inline(always)]
    pub fn p28(&self) -> P28R {
        P28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Pull Down Status."]
    #[inline(always)]
    pub fn p29(&self) -> P29R {
        P29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Pull Down Status."]
    #[inline(always)]
    pub fn p30(&self) -> P30R {
        P30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Pull Down Status."]
    #[inline(always)]
    pub fn p31(&self) -> P31R {
        P31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Pad Pull-down Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ppdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpdsrSpec;
impl crate::RegisterSpec for PpdsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppdsr::R`](R) reader structure"]
impl crate::Readable for PpdsrSpec {}
