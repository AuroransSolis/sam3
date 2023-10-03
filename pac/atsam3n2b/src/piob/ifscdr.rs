#[doc = "Register `IFSCDR` writer"]
pub type W = crate::W<IFSCDR_SPEC>;
#[doc = "Field `P0` writer - PIO Clock Glitch Filtering Select."]
pub type P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P1` writer - PIO Clock Glitch Filtering Select."]
pub type P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2` writer - PIO Clock Glitch Filtering Select."]
pub type P2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P3` writer - PIO Clock Glitch Filtering Select."]
pub type P3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P4` writer - PIO Clock Glitch Filtering Select."]
pub type P4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P5` writer - PIO Clock Glitch Filtering Select."]
pub type P5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P6` writer - PIO Clock Glitch Filtering Select."]
pub type P6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P7` writer - PIO Clock Glitch Filtering Select."]
pub type P7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P8` writer - PIO Clock Glitch Filtering Select."]
pub type P8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P9` writer - PIO Clock Glitch Filtering Select."]
pub type P9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P10` writer - PIO Clock Glitch Filtering Select."]
pub type P10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P11` writer - PIO Clock Glitch Filtering Select."]
pub type P11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P12` writer - PIO Clock Glitch Filtering Select."]
pub type P12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P13` writer - PIO Clock Glitch Filtering Select."]
pub type P13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P14` writer - PIO Clock Glitch Filtering Select."]
pub type P14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P15` writer - PIO Clock Glitch Filtering Select."]
pub type P15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P16` writer - PIO Clock Glitch Filtering Select."]
pub type P16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P17` writer - PIO Clock Glitch Filtering Select."]
pub type P17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P18` writer - PIO Clock Glitch Filtering Select."]
pub type P18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P19` writer - PIO Clock Glitch Filtering Select."]
pub type P19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P20` writer - PIO Clock Glitch Filtering Select."]
pub type P20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P21` writer - PIO Clock Glitch Filtering Select."]
pub type P21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P22` writer - PIO Clock Glitch Filtering Select."]
pub type P22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P23` writer - PIO Clock Glitch Filtering Select."]
pub type P23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P24` writer - PIO Clock Glitch Filtering Select."]
pub type P24_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P25` writer - PIO Clock Glitch Filtering Select."]
pub type P25_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P26` writer - PIO Clock Glitch Filtering Select."]
pub type P26_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P27` writer - PIO Clock Glitch Filtering Select."]
pub type P27_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P28` writer - PIO Clock Glitch Filtering Select."]
pub type P28_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P29` writer - PIO Clock Glitch Filtering Select."]
pub type P29_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P30` writer - PIO Clock Glitch Filtering Select."]
pub type P30_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P31` writer - PIO Clock Glitch Filtering Select."]
pub type P31_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<IFSCDR_SPEC, 0> {
        P0_W::new(self)
    }
    #[doc = "Bit 1 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<IFSCDR_SPEC, 1> {
        P1_W::new(self)
    }
    #[doc = "Bit 2 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<IFSCDR_SPEC, 2> {
        P2_W::new(self)
    }
    #[doc = "Bit 3 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<IFSCDR_SPEC, 3> {
        P3_W::new(self)
    }
    #[doc = "Bit 4 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<IFSCDR_SPEC, 4> {
        P4_W::new(self)
    }
    #[doc = "Bit 5 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<IFSCDR_SPEC, 5> {
        P5_W::new(self)
    }
    #[doc = "Bit 6 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<IFSCDR_SPEC, 6> {
        P6_W::new(self)
    }
    #[doc = "Bit 7 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<IFSCDR_SPEC, 7> {
        P7_W::new(self)
    }
    #[doc = "Bit 8 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<IFSCDR_SPEC, 8> {
        P8_W::new(self)
    }
    #[doc = "Bit 9 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<IFSCDR_SPEC, 9> {
        P9_W::new(self)
    }
    #[doc = "Bit 10 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<IFSCDR_SPEC, 10> {
        P10_W::new(self)
    }
    #[doc = "Bit 11 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<IFSCDR_SPEC, 11> {
        P11_W::new(self)
    }
    #[doc = "Bit 12 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<IFSCDR_SPEC, 12> {
        P12_W::new(self)
    }
    #[doc = "Bit 13 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<IFSCDR_SPEC, 13> {
        P13_W::new(self)
    }
    #[doc = "Bit 14 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14_W<IFSCDR_SPEC, 14> {
        P14_W::new(self)
    }
    #[doc = "Bit 15 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15_W<IFSCDR_SPEC, 15> {
        P15_W::new(self)
    }
    #[doc = "Bit 16 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16_W<IFSCDR_SPEC, 16> {
        P16_W::new(self)
    }
    #[doc = "Bit 17 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17_W<IFSCDR_SPEC, 17> {
        P17_W::new(self)
    }
    #[doc = "Bit 18 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18_W<IFSCDR_SPEC, 18> {
        P18_W::new(self)
    }
    #[doc = "Bit 19 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19_W<IFSCDR_SPEC, 19> {
        P19_W::new(self)
    }
    #[doc = "Bit 20 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20_W<IFSCDR_SPEC, 20> {
        P20_W::new(self)
    }
    #[doc = "Bit 21 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21_W<IFSCDR_SPEC, 21> {
        P21_W::new(self)
    }
    #[doc = "Bit 22 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22_W<IFSCDR_SPEC, 22> {
        P22_W::new(self)
    }
    #[doc = "Bit 23 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23_W<IFSCDR_SPEC, 23> {
        P23_W::new(self)
    }
    #[doc = "Bit 24 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24_W<IFSCDR_SPEC, 24> {
        P24_W::new(self)
    }
    #[doc = "Bit 25 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25_W<IFSCDR_SPEC, 25> {
        P25_W::new(self)
    }
    #[doc = "Bit 26 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26_W<IFSCDR_SPEC, 26> {
        P26_W::new(self)
    }
    #[doc = "Bit 27 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27_W<IFSCDR_SPEC, 27> {
        P27_W::new(self)
    }
    #[doc = "Bit 28 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28_W<IFSCDR_SPEC, 28> {
        P28_W::new(self)
    }
    #[doc = "Bit 29 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29_W<IFSCDR_SPEC, 29> {
        P29_W::new(self)
    }
    #[doc = "Bit 30 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30_W<IFSCDR_SPEC, 30> {
        P30_W::new(self)
    }
    #[doc = "Bit 31 - PIO Clock Glitch Filtering Select."]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31_W<IFSCDR_SPEC, 31> {
        P31_W::new(self)
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
#[doc = "Input Filter Slow Clock Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifscdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFSCDR_SPEC;
impl crate::RegisterSpec for IFSCDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifscdr::W`](W) writer structure"]
impl crate::Writable for IFSCDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
