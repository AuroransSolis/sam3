#[doc = "Register `IFDR` writer"]
pub type W = crate::W<IfdrSpec>;
#[doc = "Field `P0` writer - Input Filter Disable"]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Input Filter Disable"]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Input Filter Disable"]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Input Filter Disable"]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Input Filter Disable"]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Input Filter Disable"]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Input Filter Disable"]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Input Filter Disable"]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Input Filter Disable"]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Input Filter Disable"]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Input Filter Disable"]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Input Filter Disable"]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Input Filter Disable"]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Input Filter Disable"]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Input Filter Disable"]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Input Filter Disable"]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Input Filter Disable"]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Input Filter Disable"]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Input Filter Disable"]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Input Filter Disable"]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Input Filter Disable"]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Input Filter Disable"]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Input Filter Disable"]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Input Filter Disable"]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Input Filter Disable"]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Input Filter Disable"]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Input Filter Disable"]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Input Filter Disable"]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Input Filter Disable"]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Input Filter Disable"]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Input Filter Disable"]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Input Filter Disable"]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<IfdrSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<IfdrSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<IfdrSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<IfdrSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<IfdrSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<IfdrSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<IfdrSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<IfdrSpec> {
        P7W::new(self, 7)
    }
    #[doc = "Bit 8 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8W<IfdrSpec> {
        P8W::new(self, 8)
    }
    #[doc = "Bit 9 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9W<IfdrSpec> {
        P9W::new(self, 9)
    }
    #[doc = "Bit 10 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10W<IfdrSpec> {
        P10W::new(self, 10)
    }
    #[doc = "Bit 11 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11W<IfdrSpec> {
        P11W::new(self, 11)
    }
    #[doc = "Bit 12 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12W<IfdrSpec> {
        P12W::new(self, 12)
    }
    #[doc = "Bit 13 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13W<IfdrSpec> {
        P13W::new(self, 13)
    }
    #[doc = "Bit 14 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14W<IfdrSpec> {
        P14W::new(self, 14)
    }
    #[doc = "Bit 15 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15W<IfdrSpec> {
        P15W::new(self, 15)
    }
    #[doc = "Bit 16 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16W<IfdrSpec> {
        P16W::new(self, 16)
    }
    #[doc = "Bit 17 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17W<IfdrSpec> {
        P17W::new(self, 17)
    }
    #[doc = "Bit 18 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18W<IfdrSpec> {
        P18W::new(self, 18)
    }
    #[doc = "Bit 19 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19W<IfdrSpec> {
        P19W::new(self, 19)
    }
    #[doc = "Bit 20 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20W<IfdrSpec> {
        P20W::new(self, 20)
    }
    #[doc = "Bit 21 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21W<IfdrSpec> {
        P21W::new(self, 21)
    }
    #[doc = "Bit 22 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22W<IfdrSpec> {
        P22W::new(self, 22)
    }
    #[doc = "Bit 23 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23W<IfdrSpec> {
        P23W::new(self, 23)
    }
    #[doc = "Bit 24 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24W<IfdrSpec> {
        P24W::new(self, 24)
    }
    #[doc = "Bit 25 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25W<IfdrSpec> {
        P25W::new(self, 25)
    }
    #[doc = "Bit 26 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26W<IfdrSpec> {
        P26W::new(self, 26)
    }
    #[doc = "Bit 27 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27W<IfdrSpec> {
        P27W::new(self, 27)
    }
    #[doc = "Bit 28 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28W<IfdrSpec> {
        P28W::new(self, 28)
    }
    #[doc = "Bit 29 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29W<IfdrSpec> {
        P29W::new(self, 29)
    }
    #[doc = "Bit 30 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30W<IfdrSpec> {
        P30W::new(self, 30)
    }
    #[doc = "Bit 31 - Input Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31W<IfdrSpec> {
        P31W::new(self, 31)
    }
}
#[doc = "Glitch Input Filter Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfdrSpec;
impl crate::RegisterSpec for IfdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifdr::W`](W) writer structure"]
impl crate::Writable for IfdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
