#[doc = "Register `AIMER` writer"]
pub type W = crate::W<AimerSpec>;
#[doc = "Field `P0` writer - Additional Interrupt Modes Enable"]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Additional Interrupt Modes Enable"]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Additional Interrupt Modes Enable"]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Additional Interrupt Modes Enable"]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Additional Interrupt Modes Enable"]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Additional Interrupt Modes Enable"]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Additional Interrupt Modes Enable"]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Additional Interrupt Modes Enable"]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Additional Interrupt Modes Enable"]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Additional Interrupt Modes Enable"]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Additional Interrupt Modes Enable"]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Additional Interrupt Modes Enable"]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Additional Interrupt Modes Enable"]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Additional Interrupt Modes Enable"]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Additional Interrupt Modes Enable"]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Additional Interrupt Modes Enable"]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Additional Interrupt Modes Enable"]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Additional Interrupt Modes Enable"]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Additional Interrupt Modes Enable"]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Additional Interrupt Modes Enable"]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Additional Interrupt Modes Enable"]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Additional Interrupt Modes Enable"]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Additional Interrupt Modes Enable"]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Additional Interrupt Modes Enable"]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Additional Interrupt Modes Enable"]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Additional Interrupt Modes Enable"]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Additional Interrupt Modes Enable"]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Additional Interrupt Modes Enable"]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Additional Interrupt Modes Enable"]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Additional Interrupt Modes Enable"]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Additional Interrupt Modes Enable"]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Additional Interrupt Modes Enable"]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<AimerSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<AimerSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<AimerSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<AimerSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<AimerSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<AimerSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<AimerSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<AimerSpec> {
        P7W::new(self, 7)
    }
    #[doc = "Bit 8 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8W<AimerSpec> {
        P8W::new(self, 8)
    }
    #[doc = "Bit 9 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9W<AimerSpec> {
        P9W::new(self, 9)
    }
    #[doc = "Bit 10 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10W<AimerSpec> {
        P10W::new(self, 10)
    }
    #[doc = "Bit 11 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11W<AimerSpec> {
        P11W::new(self, 11)
    }
    #[doc = "Bit 12 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12W<AimerSpec> {
        P12W::new(self, 12)
    }
    #[doc = "Bit 13 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13W<AimerSpec> {
        P13W::new(self, 13)
    }
    #[doc = "Bit 14 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14W<AimerSpec> {
        P14W::new(self, 14)
    }
    #[doc = "Bit 15 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15W<AimerSpec> {
        P15W::new(self, 15)
    }
    #[doc = "Bit 16 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16W<AimerSpec> {
        P16W::new(self, 16)
    }
    #[doc = "Bit 17 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17W<AimerSpec> {
        P17W::new(self, 17)
    }
    #[doc = "Bit 18 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18W<AimerSpec> {
        P18W::new(self, 18)
    }
    #[doc = "Bit 19 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19W<AimerSpec> {
        P19W::new(self, 19)
    }
    #[doc = "Bit 20 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20W<AimerSpec> {
        P20W::new(self, 20)
    }
    #[doc = "Bit 21 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21W<AimerSpec> {
        P21W::new(self, 21)
    }
    #[doc = "Bit 22 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22W<AimerSpec> {
        P22W::new(self, 22)
    }
    #[doc = "Bit 23 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23W<AimerSpec> {
        P23W::new(self, 23)
    }
    #[doc = "Bit 24 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24W<AimerSpec> {
        P24W::new(self, 24)
    }
    #[doc = "Bit 25 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25W<AimerSpec> {
        P25W::new(self, 25)
    }
    #[doc = "Bit 26 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26W<AimerSpec> {
        P26W::new(self, 26)
    }
    #[doc = "Bit 27 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27W<AimerSpec> {
        P27W::new(self, 27)
    }
    #[doc = "Bit 28 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28W<AimerSpec> {
        P28W::new(self, 28)
    }
    #[doc = "Bit 29 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29W<AimerSpec> {
        P29W::new(self, 29)
    }
    #[doc = "Bit 30 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30W<AimerSpec> {
        P30W::new(self, 30)
    }
    #[doc = "Bit 31 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31W<AimerSpec> {
        P31W::new(self, 31)
    }
}
#[doc = "Additional Interrupt Modes Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aimer::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AimerSpec;
impl crate::RegisterSpec for AimerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aimer::W`](W) writer structure"]
impl crate::Writable for AimerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
