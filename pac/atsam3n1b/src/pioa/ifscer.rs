#[doc = "Register `IFSCER` writer"]
pub type W = crate::W<IfscerSpec>;
#[doc = "Field `P0` writer - Debouncing Filtering Select."]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Debouncing Filtering Select."]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Debouncing Filtering Select."]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Debouncing Filtering Select."]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Debouncing Filtering Select."]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Debouncing Filtering Select."]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Debouncing Filtering Select."]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Debouncing Filtering Select."]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Debouncing Filtering Select."]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Debouncing Filtering Select."]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Debouncing Filtering Select."]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Debouncing Filtering Select."]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Debouncing Filtering Select."]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Debouncing Filtering Select."]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Debouncing Filtering Select."]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Debouncing Filtering Select."]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Debouncing Filtering Select."]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Debouncing Filtering Select."]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Debouncing Filtering Select."]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Debouncing Filtering Select."]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Debouncing Filtering Select."]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Debouncing Filtering Select."]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Debouncing Filtering Select."]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Debouncing Filtering Select."]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Debouncing Filtering Select."]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Debouncing Filtering Select."]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Debouncing Filtering Select."]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Debouncing Filtering Select."]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Debouncing Filtering Select."]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Debouncing Filtering Select."]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Debouncing Filtering Select."]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Debouncing Filtering Select."]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<IfscerSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<IfscerSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<IfscerSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<IfscerSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<IfscerSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<IfscerSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<IfscerSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<IfscerSpec> {
        P7W::new(self, 7)
    }
    #[doc = "Bit 8 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8W<IfscerSpec> {
        P8W::new(self, 8)
    }
    #[doc = "Bit 9 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9W<IfscerSpec> {
        P9W::new(self, 9)
    }
    #[doc = "Bit 10 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10W<IfscerSpec> {
        P10W::new(self, 10)
    }
    #[doc = "Bit 11 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11W<IfscerSpec> {
        P11W::new(self, 11)
    }
    #[doc = "Bit 12 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12W<IfscerSpec> {
        P12W::new(self, 12)
    }
    #[doc = "Bit 13 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13W<IfscerSpec> {
        P13W::new(self, 13)
    }
    #[doc = "Bit 14 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14W<IfscerSpec> {
        P14W::new(self, 14)
    }
    #[doc = "Bit 15 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15W<IfscerSpec> {
        P15W::new(self, 15)
    }
    #[doc = "Bit 16 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16W<IfscerSpec> {
        P16W::new(self, 16)
    }
    #[doc = "Bit 17 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17W<IfscerSpec> {
        P17W::new(self, 17)
    }
    #[doc = "Bit 18 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18W<IfscerSpec> {
        P18W::new(self, 18)
    }
    #[doc = "Bit 19 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19W<IfscerSpec> {
        P19W::new(self, 19)
    }
    #[doc = "Bit 20 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20W<IfscerSpec> {
        P20W::new(self, 20)
    }
    #[doc = "Bit 21 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21W<IfscerSpec> {
        P21W::new(self, 21)
    }
    #[doc = "Bit 22 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22W<IfscerSpec> {
        P22W::new(self, 22)
    }
    #[doc = "Bit 23 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23W<IfscerSpec> {
        P23W::new(self, 23)
    }
    #[doc = "Bit 24 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24W<IfscerSpec> {
        P24W::new(self, 24)
    }
    #[doc = "Bit 25 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25W<IfscerSpec> {
        P25W::new(self, 25)
    }
    #[doc = "Bit 26 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26W<IfscerSpec> {
        P26W::new(self, 26)
    }
    #[doc = "Bit 27 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27W<IfscerSpec> {
        P27W::new(self, 27)
    }
    #[doc = "Bit 28 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28W<IfscerSpec> {
        P28W::new(self, 28)
    }
    #[doc = "Bit 29 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29W<IfscerSpec> {
        P29W::new(self, 29)
    }
    #[doc = "Bit 30 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30W<IfscerSpec> {
        P30W::new(self, 30)
    }
    #[doc = "Bit 31 - Debouncing Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31W<IfscerSpec> {
        P31W::new(self, 31)
    }
}
#[doc = "Input Filter Slow Clock Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifscer::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfscerSpec;
impl crate::RegisterSpec for IfscerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifscer::W`](W) writer structure"]
impl crate::Writable for IfscerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
