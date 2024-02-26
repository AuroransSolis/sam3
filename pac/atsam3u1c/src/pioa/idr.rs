#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `P0` writer - Input Change Interrupt Disable"]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Input Change Interrupt Disable"]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Input Change Interrupt Disable"]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Input Change Interrupt Disable"]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Input Change Interrupt Disable"]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Input Change Interrupt Disable"]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Input Change Interrupt Disable"]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Input Change Interrupt Disable"]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Input Change Interrupt Disable"]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Input Change Interrupt Disable"]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Input Change Interrupt Disable"]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Input Change Interrupt Disable"]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Input Change Interrupt Disable"]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Input Change Interrupt Disable"]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Input Change Interrupt Disable"]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Input Change Interrupt Disable"]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Input Change Interrupt Disable"]
pub type P16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Input Change Interrupt Disable"]
pub type P17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Input Change Interrupt Disable"]
pub type P18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Input Change Interrupt Disable"]
pub type P19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Input Change Interrupt Disable"]
pub type P20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Input Change Interrupt Disable"]
pub type P21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Input Change Interrupt Disable"]
pub type P22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Input Change Interrupt Disable"]
pub type P23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Input Change Interrupt Disable"]
pub type P24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Input Change Interrupt Disable"]
pub type P25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Input Change Interrupt Disable"]
pub type P26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Input Change Interrupt Disable"]
pub type P27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Input Change Interrupt Disable"]
pub type P28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Input Change Interrupt Disable"]
pub type P29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Input Change Interrupt Disable"]
pub type P30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Input Change Interrupt Disable"]
pub type P31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<IdrSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<IdrSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<IdrSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<IdrSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<IdrSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<IdrSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<IdrSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<IdrSpec> {
        P7W::new(self, 7)
    }
    #[doc = "Bit 8 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8W<IdrSpec> {
        P8W::new(self, 8)
    }
    #[doc = "Bit 9 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9W<IdrSpec> {
        P9W::new(self, 9)
    }
    #[doc = "Bit 10 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10W<IdrSpec> {
        P10W::new(self, 10)
    }
    #[doc = "Bit 11 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11W<IdrSpec> {
        P11W::new(self, 11)
    }
    #[doc = "Bit 12 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12W<IdrSpec> {
        P12W::new(self, 12)
    }
    #[doc = "Bit 13 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13W<IdrSpec> {
        P13W::new(self, 13)
    }
    #[doc = "Bit 14 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14W<IdrSpec> {
        P14W::new(self, 14)
    }
    #[doc = "Bit 15 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15W<IdrSpec> {
        P15W::new(self, 15)
    }
    #[doc = "Bit 16 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16W<IdrSpec> {
        P16W::new(self, 16)
    }
    #[doc = "Bit 17 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17W<IdrSpec> {
        P17W::new(self, 17)
    }
    #[doc = "Bit 18 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18W<IdrSpec> {
        P18W::new(self, 18)
    }
    #[doc = "Bit 19 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19W<IdrSpec> {
        P19W::new(self, 19)
    }
    #[doc = "Bit 20 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20W<IdrSpec> {
        P20W::new(self, 20)
    }
    #[doc = "Bit 21 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21W<IdrSpec> {
        P21W::new(self, 21)
    }
    #[doc = "Bit 22 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22W<IdrSpec> {
        P22W::new(self, 22)
    }
    #[doc = "Bit 23 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23W<IdrSpec> {
        P23W::new(self, 23)
    }
    #[doc = "Bit 24 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24W<IdrSpec> {
        P24W::new(self, 24)
    }
    #[doc = "Bit 25 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25W<IdrSpec> {
        P25W::new(self, 25)
    }
    #[doc = "Bit 26 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26W<IdrSpec> {
        P26W::new(self, 26)
    }
    #[doc = "Bit 27 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27W<IdrSpec> {
        P27W::new(self, 27)
    }
    #[doc = "Bit 28 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28W<IdrSpec> {
        P28W::new(self, 28)
    }
    #[doc = "Bit 29 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29W<IdrSpec> {
        P29W::new(self, 29)
    }
    #[doc = "Bit 30 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30W<IdrSpec> {
        P30W::new(self, 30)
    }
    #[doc = "Bit 31 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31W<IdrSpec> {
        P31W::new(self, 31)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
