#[doc = "Register `OWDR` writer"]
pub type W = crate::W<OwdrSpec>;
#[doc = "Field `P0` writer - Output Write Disable"]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Output Write Disable"]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Output Write Disable"]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Output Write Disable"]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Output Write Disable"]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Output Write Disable"]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Output Write Disable"]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Output Write Disable"]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Output Write Disable"]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Output Write Disable"]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Output Write Disable"]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Output Write Disable"]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Output Write Disable"]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Output Write Disable"]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Output Write Disable"]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Output Write Disable"]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Output Write Disable"]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Output Write Disable"]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Output Write Disable"]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Output Write Disable"]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Output Write Disable"]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Output Write Disable"]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Output Write Disable"]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Output Write Disable"]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Output Write Disable"]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Output Write Disable"]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Output Write Disable"]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Output Write Disable"]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Output Write Disable"]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Output Write Disable"]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Output Write Disable"]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Output Write Disable"]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<OwdrSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<OwdrSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<OwdrSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<OwdrSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<OwdrSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<OwdrSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<OwdrSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<OwdrSpec> {
        P7W::new(self, 7)
    }
    #[doc = "Bit 8 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8W<OwdrSpec> {
        P8W::new(self, 8)
    }
    #[doc = "Bit 9 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9W<OwdrSpec> {
        P9W::new(self, 9)
    }
    #[doc = "Bit 10 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10W<OwdrSpec> {
        P10W::new(self, 10)
    }
    #[doc = "Bit 11 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11W<OwdrSpec> {
        P11W::new(self, 11)
    }
    #[doc = "Bit 12 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12W<OwdrSpec> {
        P12W::new(self, 12)
    }
    #[doc = "Bit 13 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13W<OwdrSpec> {
        P13W::new(self, 13)
    }
    #[doc = "Bit 14 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14W<OwdrSpec> {
        P14W::new(self, 14)
    }
    #[doc = "Bit 15 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15W<OwdrSpec> {
        P15W::new(self, 15)
    }
    #[doc = "Bit 16 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16W<OwdrSpec> {
        P16W::new(self, 16)
    }
    #[doc = "Bit 17 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17W<OwdrSpec> {
        P17W::new(self, 17)
    }
    #[doc = "Bit 18 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18W<OwdrSpec> {
        P18W::new(self, 18)
    }
    #[doc = "Bit 19 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19W<OwdrSpec> {
        P19W::new(self, 19)
    }
    #[doc = "Bit 20 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20W<OwdrSpec> {
        P20W::new(self, 20)
    }
    #[doc = "Bit 21 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21W<OwdrSpec> {
        P21W::new(self, 21)
    }
    #[doc = "Bit 22 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22W<OwdrSpec> {
        P22W::new(self, 22)
    }
    #[doc = "Bit 23 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23W<OwdrSpec> {
        P23W::new(self, 23)
    }
    #[doc = "Bit 24 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24W<OwdrSpec> {
        P24W::new(self, 24)
    }
    #[doc = "Bit 25 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25W<OwdrSpec> {
        P25W::new(self, 25)
    }
    #[doc = "Bit 26 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26W<OwdrSpec> {
        P26W::new(self, 26)
    }
    #[doc = "Bit 27 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27W<OwdrSpec> {
        P27W::new(self, 27)
    }
    #[doc = "Bit 28 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28W<OwdrSpec> {
        P28W::new(self, 28)
    }
    #[doc = "Bit 29 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29W<OwdrSpec> {
        P29W::new(self, 29)
    }
    #[doc = "Bit 30 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30W<OwdrSpec> {
        P30W::new(self, 30)
    }
    #[doc = "Bit 31 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31W<OwdrSpec> {
        P31W::new(self, 31)
    }
}
#[doc = "Output Write Disable\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OwdrSpec;
impl crate::RegisterSpec for OwdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`owdr::W`](W) writer structure"]
impl crate::Writable for OwdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
