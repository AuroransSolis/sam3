#[doc = "Register `ESR` writer"]
pub type W = crate::W<EsrSpec>;
#[doc = "Field `P0` writer - Edge Interrupt Selection."]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Edge Interrupt Selection."]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Edge Interrupt Selection."]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Edge Interrupt Selection."]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Edge Interrupt Selection."]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Edge Interrupt Selection."]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Edge Interrupt Selection."]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Edge Interrupt Selection."]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Edge Interrupt Selection."]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Edge Interrupt Selection."]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Edge Interrupt Selection."]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Edge Interrupt Selection."]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Edge Interrupt Selection."]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Edge Interrupt Selection."]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Edge Interrupt Selection."]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Edge Interrupt Selection."]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Edge Interrupt Selection."]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Edge Interrupt Selection."]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Edge Interrupt Selection."]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Edge Interrupt Selection."]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Edge Interrupt Selection."]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Edge Interrupt Selection."]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Edge Interrupt Selection."]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Edge Interrupt Selection."]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Edge Interrupt Selection."]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Edge Interrupt Selection."]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Edge Interrupt Selection."]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Edge Interrupt Selection."]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Edge Interrupt Selection."]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Edge Interrupt Selection."]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Edge Interrupt Selection."]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Edge Interrupt Selection."]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<EsrSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<EsrSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<EsrSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<EsrSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<EsrSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<EsrSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<EsrSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<EsrSpec> {
        P7W::new(self, 7)
    }
    #[doc = "Bit 8 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8W<EsrSpec> {
        P8W::new(self, 8)
    }
    #[doc = "Bit 9 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9W<EsrSpec> {
        P9W::new(self, 9)
    }
    #[doc = "Bit 10 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10W<EsrSpec> {
        P10W::new(self, 10)
    }
    #[doc = "Bit 11 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11W<EsrSpec> {
        P11W::new(self, 11)
    }
    #[doc = "Bit 12 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12W<EsrSpec> {
        P12W::new(self, 12)
    }
    #[doc = "Bit 13 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13W<EsrSpec> {
        P13W::new(self, 13)
    }
    #[doc = "Bit 14 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14W<EsrSpec> {
        P14W::new(self, 14)
    }
    #[doc = "Bit 15 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15W<EsrSpec> {
        P15W::new(self, 15)
    }
    #[doc = "Bit 16 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16W<EsrSpec> {
        P16W::new(self, 16)
    }
    #[doc = "Bit 17 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17W<EsrSpec> {
        P17W::new(self, 17)
    }
    #[doc = "Bit 18 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18W<EsrSpec> {
        P18W::new(self, 18)
    }
    #[doc = "Bit 19 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19W<EsrSpec> {
        P19W::new(self, 19)
    }
    #[doc = "Bit 20 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20W<EsrSpec> {
        P20W::new(self, 20)
    }
    #[doc = "Bit 21 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21W<EsrSpec> {
        P21W::new(self, 21)
    }
    #[doc = "Bit 22 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22W<EsrSpec> {
        P22W::new(self, 22)
    }
    #[doc = "Bit 23 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23W<EsrSpec> {
        P23W::new(self, 23)
    }
    #[doc = "Bit 24 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24W<EsrSpec> {
        P24W::new(self, 24)
    }
    #[doc = "Bit 25 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25W<EsrSpec> {
        P25W::new(self, 25)
    }
    #[doc = "Bit 26 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26W<EsrSpec> {
        P26W::new(self, 26)
    }
    #[doc = "Bit 27 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27W<EsrSpec> {
        P27W::new(self, 27)
    }
    #[doc = "Bit 28 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28W<EsrSpec> {
        P28W::new(self, 28)
    }
    #[doc = "Bit 29 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29W<EsrSpec> {
        P29W::new(self, 29)
    }
    #[doc = "Bit 30 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30W<EsrSpec> {
        P30W::new(self, 30)
    }
    #[doc = "Bit 31 - Edge Interrupt Selection."]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31W<EsrSpec> {
        P31W::new(self, 31)
    }
}
#[doc = "Edge Select Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EsrSpec;
impl crate::RegisterSpec for EsrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`esr::W`](W) writer structure"]
impl crate::Writable for EsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
