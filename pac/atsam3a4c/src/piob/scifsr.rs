#[doc = "Register `SCIFSR` writer"]
pub type W = crate::W<ScifsrSpec>;
#[doc = "Field `P0` writer - System Clock Glitch Filtering Select."]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - System Clock Glitch Filtering Select."]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - System Clock Glitch Filtering Select."]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - System Clock Glitch Filtering Select."]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - System Clock Glitch Filtering Select."]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - System Clock Glitch Filtering Select."]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - System Clock Glitch Filtering Select."]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - System Clock Glitch Filtering Select."]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - System Clock Glitch Filtering Select."]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - System Clock Glitch Filtering Select."]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - System Clock Glitch Filtering Select."]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - System Clock Glitch Filtering Select."]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - System Clock Glitch Filtering Select."]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - System Clock Glitch Filtering Select."]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - System Clock Glitch Filtering Select."]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - System Clock Glitch Filtering Select."]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - System Clock Glitch Filtering Select."]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - System Clock Glitch Filtering Select."]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - System Clock Glitch Filtering Select."]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - System Clock Glitch Filtering Select."]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - System Clock Glitch Filtering Select."]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - System Clock Glitch Filtering Select."]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - System Clock Glitch Filtering Select."]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - System Clock Glitch Filtering Select."]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - System Clock Glitch Filtering Select."]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - System Clock Glitch Filtering Select."]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - System Clock Glitch Filtering Select."]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - System Clock Glitch Filtering Select."]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - System Clock Glitch Filtering Select."]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - System Clock Glitch Filtering Select."]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - System Clock Glitch Filtering Select."]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - System Clock Glitch Filtering Select."]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<ScifsrSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<ScifsrSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<ScifsrSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<ScifsrSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<ScifsrSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<ScifsrSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<ScifsrSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<ScifsrSpec> {
        P7W::new(self, 7)
    }
    #[doc = "Bit 8 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8W<ScifsrSpec> {
        P8W::new(self, 8)
    }
    #[doc = "Bit 9 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9W<ScifsrSpec> {
        P9W::new(self, 9)
    }
    #[doc = "Bit 10 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10W<ScifsrSpec> {
        P10W::new(self, 10)
    }
    #[doc = "Bit 11 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11W<ScifsrSpec> {
        P11W::new(self, 11)
    }
    #[doc = "Bit 12 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12W<ScifsrSpec> {
        P12W::new(self, 12)
    }
    #[doc = "Bit 13 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13W<ScifsrSpec> {
        P13W::new(self, 13)
    }
    #[doc = "Bit 14 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14W<ScifsrSpec> {
        P14W::new(self, 14)
    }
    #[doc = "Bit 15 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15W<ScifsrSpec> {
        P15W::new(self, 15)
    }
    #[doc = "Bit 16 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16W<ScifsrSpec> {
        P16W::new(self, 16)
    }
    #[doc = "Bit 17 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17W<ScifsrSpec> {
        P17W::new(self, 17)
    }
    #[doc = "Bit 18 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18W<ScifsrSpec> {
        P18W::new(self, 18)
    }
    #[doc = "Bit 19 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19W<ScifsrSpec> {
        P19W::new(self, 19)
    }
    #[doc = "Bit 20 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20W<ScifsrSpec> {
        P20W::new(self, 20)
    }
    #[doc = "Bit 21 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21W<ScifsrSpec> {
        P21W::new(self, 21)
    }
    #[doc = "Bit 22 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22W<ScifsrSpec> {
        P22W::new(self, 22)
    }
    #[doc = "Bit 23 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23W<ScifsrSpec> {
        P23W::new(self, 23)
    }
    #[doc = "Bit 24 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24W<ScifsrSpec> {
        P24W::new(self, 24)
    }
    #[doc = "Bit 25 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25W<ScifsrSpec> {
        P25W::new(self, 25)
    }
    #[doc = "Bit 26 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26W<ScifsrSpec> {
        P26W::new(self, 26)
    }
    #[doc = "Bit 27 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27W<ScifsrSpec> {
        P27W::new(self, 27)
    }
    #[doc = "Bit 28 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28W<ScifsrSpec> {
        P28W::new(self, 28)
    }
    #[doc = "Bit 29 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29W<ScifsrSpec> {
        P29W::new(self, 29)
    }
    #[doc = "Bit 30 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30W<ScifsrSpec> {
        P30W::new(self, 30)
    }
    #[doc = "Bit 31 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31W<ScifsrSpec> {
        P31W::new(self, 31)
    }
}
#[doc = "System Clock Glitch Input Filter Select Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scifsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScifsrSpec;
impl crate::RegisterSpec for ScifsrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scifsr::W`](W) writer structure"]
impl crate::Writable for ScifsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
