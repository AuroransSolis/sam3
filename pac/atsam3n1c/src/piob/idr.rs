#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `P0` writer - Input Change Interrupt Disable"]
pub type P0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Input Change Interrupt Disable"]
pub type P1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Input Change Interrupt Disable"]
pub type P2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Input Change Interrupt Disable"]
pub type P3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Input Change Interrupt Disable"]
pub type P4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Input Change Interrupt Disable"]
pub type P5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Input Change Interrupt Disable"]
pub type P6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Input Change Interrupt Disable"]
pub type P7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Input Change Interrupt Disable"]
pub type P8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Input Change Interrupt Disable"]
pub type P9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Input Change Interrupt Disable"]
pub type P10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Input Change Interrupt Disable"]
pub type P11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Input Change Interrupt Disable"]
pub type P12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Input Change Interrupt Disable"]
pub type P13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Input Change Interrupt Disable"]
pub type P14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Input Change Interrupt Disable"]
pub type P15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Input Change Interrupt Disable"]
pub type P16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Input Change Interrupt Disable"]
pub type P17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Input Change Interrupt Disable"]
pub type P18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Input Change Interrupt Disable"]
pub type P19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Input Change Interrupt Disable"]
pub type P20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Input Change Interrupt Disable"]
pub type P21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Input Change Interrupt Disable"]
pub type P22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Input Change Interrupt Disable"]
pub type P23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Input Change Interrupt Disable"]
pub type P24_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Input Change Interrupt Disable"]
pub type P25_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Input Change Interrupt Disable"]
pub type P26_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Input Change Interrupt Disable"]
pub type P27_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Input Change Interrupt Disable"]
pub type P28_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Input Change Interrupt Disable"]
pub type P29_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Input Change Interrupt Disable"]
pub type P30_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Input Change Interrupt Disable"]
pub type P31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<IDR_SPEC> {
        P0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<IDR_SPEC> {
        P1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<IDR_SPEC> {
        P2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<IDR_SPEC> {
        P3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<IDR_SPEC> {
        P4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<IDR_SPEC> {
        P5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<IDR_SPEC> {
        P6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<IDR_SPEC> {
        P7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<IDR_SPEC> {
        P8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<IDR_SPEC> {
        P9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<IDR_SPEC> {
        P10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<IDR_SPEC> {
        P11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<IDR_SPEC> {
        P12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<IDR_SPEC> {
        P13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14_W<IDR_SPEC> {
        P14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15_W<IDR_SPEC> {
        P15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16_W<IDR_SPEC> {
        P16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17_W<IDR_SPEC> {
        P17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18_W<IDR_SPEC> {
        P18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19_W<IDR_SPEC> {
        P19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20_W<IDR_SPEC> {
        P20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21_W<IDR_SPEC> {
        P21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22_W<IDR_SPEC> {
        P22_W::new(self, 22)
    }
    #[doc = "Bit 23 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23_W<IDR_SPEC> {
        P23_W::new(self, 23)
    }
    #[doc = "Bit 24 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24_W<IDR_SPEC> {
        P24_W::new(self, 24)
    }
    #[doc = "Bit 25 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25_W<IDR_SPEC> {
        P25_W::new(self, 25)
    }
    #[doc = "Bit 26 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26_W<IDR_SPEC> {
        P26_W::new(self, 26)
    }
    #[doc = "Bit 27 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27_W<IDR_SPEC> {
        P27_W::new(self, 27)
    }
    #[doc = "Bit 28 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28_W<IDR_SPEC> {
        P28_W::new(self, 28)
    }
    #[doc = "Bit 29 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29_W<IDR_SPEC> {
        P29_W::new(self, 29)
    }
    #[doc = "Bit 30 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30_W<IDR_SPEC> {
        P30_W::new(self, 30)
    }
    #[doc = "Bit 31 - Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31_W<IDR_SPEC> {
        P31_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
