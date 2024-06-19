#[doc = "Register `IFSCDR` writer"]
pub type W = crate::W<IfscdrSpec>;
#[doc = "Field `P0` writer - PIO Clock Glitch Filtering Select."]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - PIO Clock Glitch Filtering Select."]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - PIO Clock Glitch Filtering Select."]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - PIO Clock Glitch Filtering Select."]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - PIO Clock Glitch Filtering Select."]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - PIO Clock Glitch Filtering Select."]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - PIO Clock Glitch Filtering Select."]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - PIO Clock Glitch Filtering Select."]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - PIO Clock Glitch Filtering Select."]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - PIO Clock Glitch Filtering Select."]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - PIO Clock Glitch Filtering Select."]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - PIO Clock Glitch Filtering Select."]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - PIO Clock Glitch Filtering Select."]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - PIO Clock Glitch Filtering Select."]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - PIO Clock Glitch Filtering Select."]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - PIO Clock Glitch Filtering Select."]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - PIO Clock Glitch Filtering Select."]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - PIO Clock Glitch Filtering Select."]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - PIO Clock Glitch Filtering Select."]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - PIO Clock Glitch Filtering Select."]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - PIO Clock Glitch Filtering Select."]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - PIO Clock Glitch Filtering Select."]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - PIO Clock Glitch Filtering Select."]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - PIO Clock Glitch Filtering Select."]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - PIO Clock Glitch Filtering Select."]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - PIO Clock Glitch Filtering Select."]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - PIO Clock Glitch Filtering Select."]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - PIO Clock Glitch Filtering Select."]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - PIO Clock Glitch Filtering Select."]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - PIO Clock Glitch Filtering Select."]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - PIO Clock Glitch Filtering Select."]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - PIO Clock Glitch Filtering Select."]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<IfscdrSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<IfscdrSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<IfscdrSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<IfscdrSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<IfscdrSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<IfscdrSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<IfscdrSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<IfscdrSpec> {
        P7W::new(self, 7)
    }
    #[doc = "Bit 8 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8W<IfscdrSpec> {
        P8W::new(self, 8)
    }
    #[doc = "Bit 9 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9W<IfscdrSpec> {
        P9W::new(self, 9)
    }
    #[doc = "Bit 10 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10W<IfscdrSpec> {
        P10W::new(self, 10)
    }
    #[doc = "Bit 11 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11W<IfscdrSpec> {
        P11W::new(self, 11)
    }
    #[doc = "Bit 12 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12W<IfscdrSpec> {
        P12W::new(self, 12)
    }
    #[doc = "Bit 13 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13W<IfscdrSpec> {
        P13W::new(self, 13)
    }
    #[doc = "Bit 14 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14W<IfscdrSpec> {
        P14W::new(self, 14)
    }
    #[doc = "Bit 15 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15W<IfscdrSpec> {
        P15W::new(self, 15)
    }
    #[doc = "Bit 16 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16W<IfscdrSpec> {
        P16W::new(self, 16)
    }
    #[doc = "Bit 17 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17W<IfscdrSpec> {
        P17W::new(self, 17)
    }
    #[doc = "Bit 18 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18W<IfscdrSpec> {
        P18W::new(self, 18)
    }
    #[doc = "Bit 19 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19W<IfscdrSpec> {
        P19W::new(self, 19)
    }
    #[doc = "Bit 20 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20W<IfscdrSpec> {
        P20W::new(self, 20)
    }
    #[doc = "Bit 21 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21W<IfscdrSpec> {
        P21W::new(self, 21)
    }
    #[doc = "Bit 22 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22W<IfscdrSpec> {
        P22W::new(self, 22)
    }
    #[doc = "Bit 23 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23W<IfscdrSpec> {
        P23W::new(self, 23)
    }
    #[doc = "Bit 24 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24W<IfscdrSpec> {
        P24W::new(self, 24)
    }
    #[doc = "Bit 25 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25W<IfscdrSpec> {
        P25W::new(self, 25)
    }
    #[doc = "Bit 26 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26W<IfscdrSpec> {
        P26W::new(self, 26)
    }
    #[doc = "Bit 27 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27W<IfscdrSpec> {
        P27W::new(self, 27)
    }
    #[doc = "Bit 28 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28W<IfscdrSpec> {
        P28W::new(self, 28)
    }
    #[doc = "Bit 29 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29W<IfscdrSpec> {
        P29W::new(self, 29)
    }
    #[doc = "Bit 30 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30W<IfscdrSpec> {
        P30W::new(self, 30)
    }
    #[doc = "Bit 31 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31W<IfscdrSpec> {
        P31W::new(self, 31)
    }
}
#[doc = "Input Filter Slow Clock Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifscdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfscdrSpec;
impl crate::RegisterSpec for IfscdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifscdr::W`](W) writer structure"]
impl crate::Writable for IfscdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
