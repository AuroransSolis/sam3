#[doc = "Register `MDDR` writer"]
pub type W = crate::W<MddrSpec>;
#[doc = "Field `P0` writer - Multi Drive Disable."]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Multi Drive Disable."]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Multi Drive Disable."]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Multi Drive Disable."]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Multi Drive Disable."]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Multi Drive Disable."]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Multi Drive Disable."]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Multi Drive Disable."]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Multi Drive Disable."]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Multi Drive Disable."]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Multi Drive Disable."]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Multi Drive Disable."]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Multi Drive Disable."]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Multi Drive Disable."]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Multi Drive Disable."]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Multi Drive Disable."]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Multi Drive Disable."]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Multi Drive Disable."]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Multi Drive Disable."]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Multi Drive Disable."]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Multi Drive Disable."]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Multi Drive Disable."]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Multi Drive Disable."]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Multi Drive Disable."]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Multi Drive Disable."]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Multi Drive Disable."]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Multi Drive Disable."]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Multi Drive Disable."]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Multi Drive Disable."]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Multi Drive Disable."]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Multi Drive Disable."]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Multi Drive Disable."]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<MddrSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<MddrSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<MddrSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<MddrSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<MddrSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<MddrSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<MddrSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<MddrSpec> {
        P7W::new(self, 7)
    }
    #[doc = "Bit 8 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8W<MddrSpec> {
        P8W::new(self, 8)
    }
    #[doc = "Bit 9 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9W<MddrSpec> {
        P9W::new(self, 9)
    }
    #[doc = "Bit 10 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10W<MddrSpec> {
        P10W::new(self, 10)
    }
    #[doc = "Bit 11 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11W<MddrSpec> {
        P11W::new(self, 11)
    }
    #[doc = "Bit 12 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12W<MddrSpec> {
        P12W::new(self, 12)
    }
    #[doc = "Bit 13 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13W<MddrSpec> {
        P13W::new(self, 13)
    }
    #[doc = "Bit 14 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14W<MddrSpec> {
        P14W::new(self, 14)
    }
    #[doc = "Bit 15 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15W<MddrSpec> {
        P15W::new(self, 15)
    }
    #[doc = "Bit 16 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16W<MddrSpec> {
        P16W::new(self, 16)
    }
    #[doc = "Bit 17 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17W<MddrSpec> {
        P17W::new(self, 17)
    }
    #[doc = "Bit 18 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18W<MddrSpec> {
        P18W::new(self, 18)
    }
    #[doc = "Bit 19 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19W<MddrSpec> {
        P19W::new(self, 19)
    }
    #[doc = "Bit 20 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20W<MddrSpec> {
        P20W::new(self, 20)
    }
    #[doc = "Bit 21 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21W<MddrSpec> {
        P21W::new(self, 21)
    }
    #[doc = "Bit 22 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22W<MddrSpec> {
        P22W::new(self, 22)
    }
    #[doc = "Bit 23 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23W<MddrSpec> {
        P23W::new(self, 23)
    }
    #[doc = "Bit 24 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24W<MddrSpec> {
        P24W::new(self, 24)
    }
    #[doc = "Bit 25 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25W<MddrSpec> {
        P25W::new(self, 25)
    }
    #[doc = "Bit 26 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26W<MddrSpec> {
        P26W::new(self, 26)
    }
    #[doc = "Bit 27 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27W<MddrSpec> {
        P27W::new(self, 27)
    }
    #[doc = "Bit 28 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28W<MddrSpec> {
        P28W::new(self, 28)
    }
    #[doc = "Bit 29 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29W<MddrSpec> {
        P29W::new(self, 29)
    }
    #[doc = "Bit 30 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30W<MddrSpec> {
        P30W::new(self, 30)
    }
    #[doc = "Bit 31 - Multi Drive Disable."]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31W<MddrSpec> {
        P31W::new(self, 31)
    }
}
#[doc = "Multi-driver Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mddr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MddrSpec;
impl crate::RegisterSpec for MddrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mddr::W`](W) writer structure"]
impl crate::Writable for MddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
