#[doc = "Register `REHLSR` writer"]
pub type W = crate::W<RehlsrSpec>;
#[doc = "Field `P0` writer - Rising Edge /High Level Interrupt Selection"]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Rising Edge /High Level Interrupt Selection"]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Rising Edge /High Level Interrupt Selection"]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Rising Edge /High Level Interrupt Selection"]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Rising Edge /High Level Interrupt Selection"]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Rising Edge /High Level Interrupt Selection"]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Rising Edge /High Level Interrupt Selection"]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Rising Edge /High Level Interrupt Selection"]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Rising Edge /High Level Interrupt Selection"]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Rising Edge /High Level Interrupt Selection"]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Rising Edge /High Level Interrupt Selection"]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Rising Edge /High Level Interrupt Selection"]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Rising Edge /High Level Interrupt Selection"]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Rising Edge /High Level Interrupt Selection"]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Rising Edge /High Level Interrupt Selection"]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Rising Edge /High Level Interrupt Selection"]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Rising Edge /High Level Interrupt Selection"]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Rising Edge /High Level Interrupt Selection"]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Rising Edge /High Level Interrupt Selection"]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Rising Edge /High Level Interrupt Selection"]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Rising Edge /High Level Interrupt Selection"]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Rising Edge /High Level Interrupt Selection"]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Rising Edge /High Level Interrupt Selection"]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Rising Edge /High Level Interrupt Selection"]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Rising Edge /High Level Interrupt Selection"]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Rising Edge /High Level Interrupt Selection"]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Rising Edge /High Level Interrupt Selection"]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Rising Edge /High Level Interrupt Selection"]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Rising Edge /High Level Interrupt Selection"]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Rising Edge /High Level Interrupt Selection"]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Rising Edge /High Level Interrupt Selection"]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Rising Edge /High Level Interrupt Selection"]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<RehlsrSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<RehlsrSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<RehlsrSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<RehlsrSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<RehlsrSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<RehlsrSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<RehlsrSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<RehlsrSpec> {
        P7W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8W<RehlsrSpec> {
        P8W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9W<RehlsrSpec> {
        P9W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10W<RehlsrSpec> {
        P10W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11W<RehlsrSpec> {
        P11W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12W<RehlsrSpec> {
        P12W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13W<RehlsrSpec> {
        P13W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14W<RehlsrSpec> {
        P14W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15W<RehlsrSpec> {
        P15W::new(self, 15)
    }
    #[doc = "Bit 16 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16W<RehlsrSpec> {
        P16W::new(self, 16)
    }
    #[doc = "Bit 17 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17W<RehlsrSpec> {
        P17W::new(self, 17)
    }
    #[doc = "Bit 18 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18W<RehlsrSpec> {
        P18W::new(self, 18)
    }
    #[doc = "Bit 19 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19W<RehlsrSpec> {
        P19W::new(self, 19)
    }
    #[doc = "Bit 20 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20W<RehlsrSpec> {
        P20W::new(self, 20)
    }
    #[doc = "Bit 21 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21W<RehlsrSpec> {
        P21W::new(self, 21)
    }
    #[doc = "Bit 22 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22W<RehlsrSpec> {
        P22W::new(self, 22)
    }
    #[doc = "Bit 23 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23W<RehlsrSpec> {
        P23W::new(self, 23)
    }
    #[doc = "Bit 24 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24W<RehlsrSpec> {
        P24W::new(self, 24)
    }
    #[doc = "Bit 25 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25W<RehlsrSpec> {
        P25W::new(self, 25)
    }
    #[doc = "Bit 26 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26W<RehlsrSpec> {
        P26W::new(self, 26)
    }
    #[doc = "Bit 27 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27W<RehlsrSpec> {
        P27W::new(self, 27)
    }
    #[doc = "Bit 28 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28W<RehlsrSpec> {
        P28W::new(self, 28)
    }
    #[doc = "Bit 29 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29W<RehlsrSpec> {
        P29W::new(self, 29)
    }
    #[doc = "Bit 30 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30W<RehlsrSpec> {
        P30W::new(self, 30)
    }
    #[doc = "Bit 31 - Rising Edge /High Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31W<RehlsrSpec> {
        P31W::new(self, 31)
    }
}
#[doc = "Rising Edge/ High Level Select Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rehlsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RehlsrSpec;
impl crate::RegisterSpec for RehlsrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rehlsr::W`](W) writer structure"]
impl crate::Writable for RehlsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
