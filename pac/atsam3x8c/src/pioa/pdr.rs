#[doc = "Register `PDR` writer"]
pub type W = crate::W<PDR_SPEC>;
#[doc = "Field `P0` writer - PIO Disable"]
pub type P0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - PIO Disable"]
pub type P1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - PIO Disable"]
pub type P2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - PIO Disable"]
pub type P3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - PIO Disable"]
pub type P4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - PIO Disable"]
pub type P5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - PIO Disable"]
pub type P6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - PIO Disable"]
pub type P7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - PIO Disable"]
pub type P8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - PIO Disable"]
pub type P9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - PIO Disable"]
pub type P10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - PIO Disable"]
pub type P11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - PIO Disable"]
pub type P12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - PIO Disable"]
pub type P13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - PIO Disable"]
pub type P14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - PIO Disable"]
pub type P15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - PIO Disable"]
pub type P16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - PIO Disable"]
pub type P17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - PIO Disable"]
pub type P18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - PIO Disable"]
pub type P19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - PIO Disable"]
pub type P20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - PIO Disable"]
pub type P21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - PIO Disable"]
pub type P22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - PIO Disable"]
pub type P23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - PIO Disable"]
pub type P24_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - PIO Disable"]
pub type P25_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - PIO Disable"]
pub type P26_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - PIO Disable"]
pub type P27_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - PIO Disable"]
pub type P28_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - PIO Disable"]
pub type P29_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - PIO Disable"]
pub type P30_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - PIO Disable"]
pub type P31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<PDR_SPEC> {
        P0_W::new(self, 0)
    }
    #[doc = "Bit 1 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<PDR_SPEC> {
        P1_W::new(self, 1)
    }
    #[doc = "Bit 2 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<PDR_SPEC> {
        P2_W::new(self, 2)
    }
    #[doc = "Bit 3 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<PDR_SPEC> {
        P3_W::new(self, 3)
    }
    #[doc = "Bit 4 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<PDR_SPEC> {
        P4_W::new(self, 4)
    }
    #[doc = "Bit 5 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<PDR_SPEC> {
        P5_W::new(self, 5)
    }
    #[doc = "Bit 6 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<PDR_SPEC> {
        P6_W::new(self, 6)
    }
    #[doc = "Bit 7 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<PDR_SPEC> {
        P7_W::new(self, 7)
    }
    #[doc = "Bit 8 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<PDR_SPEC> {
        P8_W::new(self, 8)
    }
    #[doc = "Bit 9 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<PDR_SPEC> {
        P9_W::new(self, 9)
    }
    #[doc = "Bit 10 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<PDR_SPEC> {
        P10_W::new(self, 10)
    }
    #[doc = "Bit 11 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<PDR_SPEC> {
        P11_W::new(self, 11)
    }
    #[doc = "Bit 12 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<PDR_SPEC> {
        P12_W::new(self, 12)
    }
    #[doc = "Bit 13 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<PDR_SPEC> {
        P13_W::new(self, 13)
    }
    #[doc = "Bit 14 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14_W<PDR_SPEC> {
        P14_W::new(self, 14)
    }
    #[doc = "Bit 15 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15_W<PDR_SPEC> {
        P15_W::new(self, 15)
    }
    #[doc = "Bit 16 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16_W<PDR_SPEC> {
        P16_W::new(self, 16)
    }
    #[doc = "Bit 17 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17_W<PDR_SPEC> {
        P17_W::new(self, 17)
    }
    #[doc = "Bit 18 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18_W<PDR_SPEC> {
        P18_W::new(self, 18)
    }
    #[doc = "Bit 19 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19_W<PDR_SPEC> {
        P19_W::new(self, 19)
    }
    #[doc = "Bit 20 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20_W<PDR_SPEC> {
        P20_W::new(self, 20)
    }
    #[doc = "Bit 21 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21_W<PDR_SPEC> {
        P21_W::new(self, 21)
    }
    #[doc = "Bit 22 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22_W<PDR_SPEC> {
        P22_W::new(self, 22)
    }
    #[doc = "Bit 23 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23_W<PDR_SPEC> {
        P23_W::new(self, 23)
    }
    #[doc = "Bit 24 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24_W<PDR_SPEC> {
        P24_W::new(self, 24)
    }
    #[doc = "Bit 25 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25_W<PDR_SPEC> {
        P25_W::new(self, 25)
    }
    #[doc = "Bit 26 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26_W<PDR_SPEC> {
        P26_W::new(self, 26)
    }
    #[doc = "Bit 27 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27_W<PDR_SPEC> {
        P27_W::new(self, 27)
    }
    #[doc = "Bit 28 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28_W<PDR_SPEC> {
        P28_W::new(self, 28)
    }
    #[doc = "Bit 29 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29_W<PDR_SPEC> {
        P29_W::new(self, 29)
    }
    #[doc = "Bit 30 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30_W<PDR_SPEC> {
        P30_W::new(self, 30)
    }
    #[doc = "Bit 31 - PIO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31_W<PDR_SPEC> {
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
#[doc = "PIO Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDR_SPEC;
impl crate::RegisterSpec for PDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pdr::W`](W) writer structure"]
impl crate::Writable for PDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
