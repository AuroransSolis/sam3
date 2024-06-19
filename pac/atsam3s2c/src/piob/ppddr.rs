#[doc = "Register `PPDDR` writer"]
pub type W = crate::W<PpddrSpec>;
#[doc = "Field `P0` writer - Pull Down Disable"]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Pull Down Disable"]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Pull Down Disable"]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Pull Down Disable"]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Pull Down Disable"]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Pull Down Disable"]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Pull Down Disable"]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Pull Down Disable"]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Pull Down Disable"]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Pull Down Disable"]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Pull Down Disable"]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Pull Down Disable"]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Pull Down Disable"]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Pull Down Disable"]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Pull Down Disable"]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Pull Down Disable"]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Pull Down Disable"]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Pull Down Disable"]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Pull Down Disable"]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Pull Down Disable"]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Pull Down Disable"]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Pull Down Disable"]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Pull Down Disable"]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Pull Down Disable"]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Pull Down Disable"]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Pull Down Disable"]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Pull Down Disable"]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Pull Down Disable"]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Pull Down Disable"]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Pull Down Disable"]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Pull Down Disable"]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Pull Down Disable"]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<PpddrSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<PpddrSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<PpddrSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<PpddrSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<PpddrSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<PpddrSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<PpddrSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<PpddrSpec> {
        P7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8W<PpddrSpec> {
        P8W::new(self, 8)
    }
    #[doc = "Bit 9 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9W<PpddrSpec> {
        P9W::new(self, 9)
    }
    #[doc = "Bit 10 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10W<PpddrSpec> {
        P10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11W<PpddrSpec> {
        P11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12W<PpddrSpec> {
        P12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13W<PpddrSpec> {
        P13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14W<PpddrSpec> {
        P14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15W<PpddrSpec> {
        P15W::new(self, 15)
    }
    #[doc = "Bit 16 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16W<PpddrSpec> {
        P16W::new(self, 16)
    }
    #[doc = "Bit 17 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17W<PpddrSpec> {
        P17W::new(self, 17)
    }
    #[doc = "Bit 18 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18W<PpddrSpec> {
        P18W::new(self, 18)
    }
    #[doc = "Bit 19 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19W<PpddrSpec> {
        P19W::new(self, 19)
    }
    #[doc = "Bit 20 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20W<PpddrSpec> {
        P20W::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21W<PpddrSpec> {
        P21W::new(self, 21)
    }
    #[doc = "Bit 22 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22W<PpddrSpec> {
        P22W::new(self, 22)
    }
    #[doc = "Bit 23 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23W<PpddrSpec> {
        P23W::new(self, 23)
    }
    #[doc = "Bit 24 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24W<PpddrSpec> {
        P24W::new(self, 24)
    }
    #[doc = "Bit 25 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25W<PpddrSpec> {
        P25W::new(self, 25)
    }
    #[doc = "Bit 26 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26W<PpddrSpec> {
        P26W::new(self, 26)
    }
    #[doc = "Bit 27 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27W<PpddrSpec> {
        P27W::new(self, 27)
    }
    #[doc = "Bit 28 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28W<PpddrSpec> {
        P28W::new(self, 28)
    }
    #[doc = "Bit 29 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29W<PpddrSpec> {
        P29W::new(self, 29)
    }
    #[doc = "Bit 30 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30W<PpddrSpec> {
        P30W::new(self, 30)
    }
    #[doc = "Bit 31 - Pull Down Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31W<PpddrSpec> {
        P31W::new(self, 31)
    }
}
#[doc = "Pad Pull-down Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppddr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpddrSpec;
impl crate::RegisterSpec for PpddrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ppddr::W`](W) writer structure"]
impl crate::Writable for PpddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
