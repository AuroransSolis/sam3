#[doc = "Register `AIMER` writer"]
pub type W = crate::W<AIMER_SPEC>;
#[doc = "Field `P0` writer - Additional Interrupt Modes Enable"]
pub type P0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` writer - Additional Interrupt Modes Enable"]
pub type P1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` writer - Additional Interrupt Modes Enable"]
pub type P2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` writer - Additional Interrupt Modes Enable"]
pub type P3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` writer - Additional Interrupt Modes Enable"]
pub type P4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` writer - Additional Interrupt Modes Enable"]
pub type P5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` writer - Additional Interrupt Modes Enable"]
pub type P6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` writer - Additional Interrupt Modes Enable"]
pub type P7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` writer - Additional Interrupt Modes Enable"]
pub type P8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` writer - Additional Interrupt Modes Enable"]
pub type P9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` writer - Additional Interrupt Modes Enable"]
pub type P10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` writer - Additional Interrupt Modes Enable"]
pub type P11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` writer - Additional Interrupt Modes Enable"]
pub type P12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` writer - Additional Interrupt Modes Enable"]
pub type P13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` writer - Additional Interrupt Modes Enable"]
pub type P14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` writer - Additional Interrupt Modes Enable"]
pub type P15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` writer - Additional Interrupt Modes Enable"]
pub type P16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` writer - Additional Interrupt Modes Enable"]
pub type P17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` writer - Additional Interrupt Modes Enable"]
pub type P18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` writer - Additional Interrupt Modes Enable"]
pub type P19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P20` writer - Additional Interrupt Modes Enable"]
pub type P20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` writer - Additional Interrupt Modes Enable"]
pub type P21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` writer - Additional Interrupt Modes Enable"]
pub type P22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` writer - Additional Interrupt Modes Enable"]
pub type P23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` writer - Additional Interrupt Modes Enable"]
pub type P24_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` writer - Additional Interrupt Modes Enable"]
pub type P25_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` writer - Additional Interrupt Modes Enable"]
pub type P26_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` writer - Additional Interrupt Modes Enable"]
pub type P27_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` writer - Additional Interrupt Modes Enable"]
pub type P28_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` writer - Additional Interrupt Modes Enable"]
pub type P29_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30` writer - Additional Interrupt Modes Enable"]
pub type P30_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` writer - Additional Interrupt Modes Enable"]
pub type P31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<AIMER_SPEC> {
        P0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<AIMER_SPEC> {
        P1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<AIMER_SPEC> {
        P2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<AIMER_SPEC> {
        P3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<AIMER_SPEC> {
        P4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<AIMER_SPEC> {
        P5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<AIMER_SPEC> {
        P6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<AIMER_SPEC> {
        P7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<AIMER_SPEC> {
        P8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<AIMER_SPEC> {
        P9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<AIMER_SPEC> {
        P10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<AIMER_SPEC> {
        P11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<AIMER_SPEC> {
        P12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<AIMER_SPEC> {
        P13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14_W<AIMER_SPEC> {
        P14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15_W<AIMER_SPEC> {
        P15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16_W<AIMER_SPEC> {
        P16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17_W<AIMER_SPEC> {
        P17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18_W<AIMER_SPEC> {
        P18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19_W<AIMER_SPEC> {
        P19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20_W<AIMER_SPEC> {
        P20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21_W<AIMER_SPEC> {
        P21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22_W<AIMER_SPEC> {
        P22_W::new(self, 22)
    }
    #[doc = "Bit 23 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23_W<AIMER_SPEC> {
        P23_W::new(self, 23)
    }
    #[doc = "Bit 24 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24_W<AIMER_SPEC> {
        P24_W::new(self, 24)
    }
    #[doc = "Bit 25 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25_W<AIMER_SPEC> {
        P25_W::new(self, 25)
    }
    #[doc = "Bit 26 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26_W<AIMER_SPEC> {
        P26_W::new(self, 26)
    }
    #[doc = "Bit 27 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27_W<AIMER_SPEC> {
        P27_W::new(self, 27)
    }
    #[doc = "Bit 28 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28_W<AIMER_SPEC> {
        P28_W::new(self, 28)
    }
    #[doc = "Bit 29 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29_W<AIMER_SPEC> {
        P29_W::new(self, 29)
    }
    #[doc = "Bit 30 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30_W<AIMER_SPEC> {
        P30_W::new(self, 30)
    }
    #[doc = "Bit 31 - Additional Interrupt Modes Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31_W<AIMER_SPEC> {
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
#[doc = "Additional Interrupt Modes Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aimer::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AIMER_SPEC;
impl crate::RegisterSpec for AIMER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aimer::W`](W) writer structure"]
impl crate::Writable for AIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
