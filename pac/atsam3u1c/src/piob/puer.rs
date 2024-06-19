#[doc = "Register `PUER` writer"]
pub type W = crate::W<PuerSpec>;
#[doc = "Field `P0` writer - Pull Up Enable."]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Pull Up Enable."]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Pull Up Enable."]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Pull Up Enable."]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Pull Up Enable."]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Pull Up Enable."]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Pull Up Enable."]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Pull Up Enable."]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Pull Up Enable."]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Pull Up Enable."]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Pull Up Enable."]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Pull Up Enable."]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Pull Up Enable."]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Pull Up Enable."]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Pull Up Enable."]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Pull Up Enable."]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Pull Up Enable."]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Pull Up Enable."]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Pull Up Enable."]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Pull Up Enable."]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Pull Up Enable."]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Pull Up Enable."]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Pull Up Enable."]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Pull Up Enable."]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Pull Up Enable."]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Pull Up Enable."]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Pull Up Enable."]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Pull Up Enable."]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Pull Up Enable."]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Pull Up Enable."]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Pull Up Enable."]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Pull Up Enable."]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<PuerSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<PuerSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<PuerSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<PuerSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<PuerSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<PuerSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<PuerSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<PuerSpec> {
        P7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8W<PuerSpec> {
        P8W::new(self, 8)
    }
    #[doc = "Bit 9 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9W<PuerSpec> {
        P9W::new(self, 9)
    }
    #[doc = "Bit 10 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10W<PuerSpec> {
        P10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11W<PuerSpec> {
        P11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12W<PuerSpec> {
        P12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13W<PuerSpec> {
        P13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14W<PuerSpec> {
        P14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15W<PuerSpec> {
        P15W::new(self, 15)
    }
    #[doc = "Bit 16 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16W<PuerSpec> {
        P16W::new(self, 16)
    }
    #[doc = "Bit 17 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17W<PuerSpec> {
        P17W::new(self, 17)
    }
    #[doc = "Bit 18 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18W<PuerSpec> {
        P18W::new(self, 18)
    }
    #[doc = "Bit 19 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19W<PuerSpec> {
        P19W::new(self, 19)
    }
    #[doc = "Bit 20 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20W<PuerSpec> {
        P20W::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21W<PuerSpec> {
        P21W::new(self, 21)
    }
    #[doc = "Bit 22 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22W<PuerSpec> {
        P22W::new(self, 22)
    }
    #[doc = "Bit 23 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23W<PuerSpec> {
        P23W::new(self, 23)
    }
    #[doc = "Bit 24 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24W<PuerSpec> {
        P24W::new(self, 24)
    }
    #[doc = "Bit 25 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25W<PuerSpec> {
        P25W::new(self, 25)
    }
    #[doc = "Bit 26 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26W<PuerSpec> {
        P26W::new(self, 26)
    }
    #[doc = "Bit 27 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27W<PuerSpec> {
        P27W::new(self, 27)
    }
    #[doc = "Bit 28 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28W<PuerSpec> {
        P28W::new(self, 28)
    }
    #[doc = "Bit 29 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29W<PuerSpec> {
        P29W::new(self, 29)
    }
    #[doc = "Bit 30 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30W<PuerSpec> {
        P30W::new(self, 30)
    }
    #[doc = "Bit 31 - Pull Up Enable."]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31W<PuerSpec> {
        P31W::new(self, 31)
    }
}
#[doc = "Pull-up Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`puer::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PuerSpec;
impl crate::RegisterSpec for PuerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`puer::W`](W) writer structure"]
impl crate::Writable for PuerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
