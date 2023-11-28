#[doc = "Register `SCIFSR` writer"]
pub type W = crate::W<SCIFSR_SPEC>;
#[doc = "Field `P0` writer - System Clock Glitch Filtering Select."]
pub type P0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - System Clock Glitch Filtering Select."]
pub type P1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - System Clock Glitch Filtering Select."]
pub type P2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - System Clock Glitch Filtering Select."]
pub type P3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - System Clock Glitch Filtering Select."]
pub type P4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - System Clock Glitch Filtering Select."]
pub type P5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - System Clock Glitch Filtering Select."]
pub type P6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - System Clock Glitch Filtering Select."]
pub type P7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - System Clock Glitch Filtering Select."]
pub type P8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - System Clock Glitch Filtering Select."]
pub type P9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - System Clock Glitch Filtering Select."]
pub type P10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - System Clock Glitch Filtering Select."]
pub type P11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - System Clock Glitch Filtering Select."]
pub type P12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - System Clock Glitch Filtering Select."]
pub type P13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - System Clock Glitch Filtering Select."]
pub type P14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - System Clock Glitch Filtering Select."]
pub type P15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - System Clock Glitch Filtering Select."]
pub type P16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - System Clock Glitch Filtering Select."]
pub type P17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - System Clock Glitch Filtering Select."]
pub type P18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - System Clock Glitch Filtering Select."]
pub type P19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - System Clock Glitch Filtering Select."]
pub type P20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - System Clock Glitch Filtering Select."]
pub type P21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - System Clock Glitch Filtering Select."]
pub type P22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - System Clock Glitch Filtering Select."]
pub type P23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - System Clock Glitch Filtering Select."]
pub type P24_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - System Clock Glitch Filtering Select."]
pub type P25_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - System Clock Glitch Filtering Select."]
pub type P26_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - System Clock Glitch Filtering Select."]
pub type P27_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - System Clock Glitch Filtering Select."]
pub type P28_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - System Clock Glitch Filtering Select."]
pub type P29_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - System Clock Glitch Filtering Select."]
pub type P30_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - System Clock Glitch Filtering Select."]
pub type P31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<SCIFSR_SPEC> {
        P0_W::new(self, 0)
    }
    #[doc = "Bit 1 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<SCIFSR_SPEC> {
        P1_W::new(self, 1)
    }
    #[doc = "Bit 2 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<SCIFSR_SPEC> {
        P2_W::new(self, 2)
    }
    #[doc = "Bit 3 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<SCIFSR_SPEC> {
        P3_W::new(self, 3)
    }
    #[doc = "Bit 4 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<SCIFSR_SPEC> {
        P4_W::new(self, 4)
    }
    #[doc = "Bit 5 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<SCIFSR_SPEC> {
        P5_W::new(self, 5)
    }
    #[doc = "Bit 6 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<SCIFSR_SPEC> {
        P6_W::new(self, 6)
    }
    #[doc = "Bit 7 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<SCIFSR_SPEC> {
        P7_W::new(self, 7)
    }
    #[doc = "Bit 8 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<SCIFSR_SPEC> {
        P8_W::new(self, 8)
    }
    #[doc = "Bit 9 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<SCIFSR_SPEC> {
        P9_W::new(self, 9)
    }
    #[doc = "Bit 10 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<SCIFSR_SPEC> {
        P10_W::new(self, 10)
    }
    #[doc = "Bit 11 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<SCIFSR_SPEC> {
        P11_W::new(self, 11)
    }
    #[doc = "Bit 12 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<SCIFSR_SPEC> {
        P12_W::new(self, 12)
    }
    #[doc = "Bit 13 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<SCIFSR_SPEC> {
        P13_W::new(self, 13)
    }
    #[doc = "Bit 14 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14_W<SCIFSR_SPEC> {
        P14_W::new(self, 14)
    }
    #[doc = "Bit 15 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15_W<SCIFSR_SPEC> {
        P15_W::new(self, 15)
    }
    #[doc = "Bit 16 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16_W<SCIFSR_SPEC> {
        P16_W::new(self, 16)
    }
    #[doc = "Bit 17 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17_W<SCIFSR_SPEC> {
        P17_W::new(self, 17)
    }
    #[doc = "Bit 18 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18_W<SCIFSR_SPEC> {
        P18_W::new(self, 18)
    }
    #[doc = "Bit 19 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19_W<SCIFSR_SPEC> {
        P19_W::new(self, 19)
    }
    #[doc = "Bit 20 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20_W<SCIFSR_SPEC> {
        P20_W::new(self, 20)
    }
    #[doc = "Bit 21 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21_W<SCIFSR_SPEC> {
        P21_W::new(self, 21)
    }
    #[doc = "Bit 22 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22_W<SCIFSR_SPEC> {
        P22_W::new(self, 22)
    }
    #[doc = "Bit 23 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23_W<SCIFSR_SPEC> {
        P23_W::new(self, 23)
    }
    #[doc = "Bit 24 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24_W<SCIFSR_SPEC> {
        P24_W::new(self, 24)
    }
    #[doc = "Bit 25 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25_W<SCIFSR_SPEC> {
        P25_W::new(self, 25)
    }
    #[doc = "Bit 26 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26_W<SCIFSR_SPEC> {
        P26_W::new(self, 26)
    }
    #[doc = "Bit 27 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27_W<SCIFSR_SPEC> {
        P27_W::new(self, 27)
    }
    #[doc = "Bit 28 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28_W<SCIFSR_SPEC> {
        P28_W::new(self, 28)
    }
    #[doc = "Bit 29 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29_W<SCIFSR_SPEC> {
        P29_W::new(self, 29)
    }
    #[doc = "Bit 30 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30_W<SCIFSR_SPEC> {
        P30_W::new(self, 30)
    }
    #[doc = "Bit 31 - System Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31_W<SCIFSR_SPEC> {
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
#[doc = "System Clock Glitch Input Filter Select Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scifsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCIFSR_SPEC;
impl crate::RegisterSpec for SCIFSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scifsr::W`](W) writer structure"]
impl crate::Writable for SCIFSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
