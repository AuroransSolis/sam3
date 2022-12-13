#[doc = "Register `PMC_MCKR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<PMC_MCKR_SPEC>);
#[doc = "Register `PMC_MCKR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<PMC_MCKR_SPEC>);
#[doc = "Field `CSS` reader - Master Clock Source Selection"]
pub type CSS_R = crate::FieldReader<u8, CSS_A>;
#[doc = "Master Clock Source Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSS_A {
    #[doc = "0: Slow Clock is selected"]
    SlowClk = 0,
    #[doc = "1: Main Clock is selected"]
    MainClk = 1,
    #[doc = "2: PLLA Clock is selected"]
    PllaClk = 2,
    #[doc = "3: UPLLClock is selected"]
    UpllClk = 3,
}
impl From<CSS_A> for u8 {
    #[inline(always)]
    fn from(variant: CSS_A) -> Self {
        variant as _
    }
}
impl CSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSS_A {
        match self.bits {
            0 => CSS_A::SlowClk,
            1 => CSS_A::MainClk,
            2 => CSS_A::PllaClk,
            3 => CSS_A::UpllClk,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SlowClk`"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == CSS_A::SlowClk
    }
    #[doc = "Checks if the value of the field is `MainClk`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == CSS_A::MainClk
    }
    #[doc = "Checks if the value of the field is `PllaClk`"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == CSS_A::PllaClk
    }
    #[doc = "Checks if the value of the field is `UpllClk`"]
    #[inline(always)]
    pub fn is_upll_clk(&self) -> bool {
        *self == CSS_A::UpllClk
    }
}
#[doc = "Field `CSS` writer - Master Clock Source Selection"]
pub type CSS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PMC_MCKR_SPEC, u8, CSS_A, 2, O>;
impl<'a, const O: u8> CSS_W<'a, O> {
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut W {
        self.variant(CSS_A::SlowClk)
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(CSS_A::MainClk)
    }
    #[doc = "PLLA Clock is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut W {
        self.variant(CSS_A::PllaClk)
    }
    #[doc = "UPLLClock is selected"]
    #[inline(always)]
    pub fn upll_clk(self) -> &'a mut W {
        self.variant(CSS_A::UpllClk)
    }
}
#[doc = "Field `PRES` reader - Processor Clock Prescaler"]
pub type PRES_R = crate::FieldReader<u8, PRES_A>;
#[doc = "Processor Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRES_A {
    #[doc = "0: Selected clock"]
    Clk1 = 0,
    #[doc = "1: Selected clock divided by 2"]
    Clk2 = 1,
    #[doc = "2: Selected clock divided by 4"]
    Clk4 = 2,
    #[doc = "3: Selected clock divided by 8"]
    Clk8 = 3,
    #[doc = "4: Selected clock divided by 16"]
    Clk16 = 4,
    #[doc = "5: Selected clock divided by 32"]
    Clk32 = 5,
    #[doc = "6: Selected clock divided by 64"]
    Clk64 = 6,
    #[doc = "7: Selected clock divided by 3"]
    Clk3 = 7,
}
impl From<PRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PRES_A) -> Self {
        variant as _
    }
}
impl PRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRES_A {
        match self.bits {
            0 => PRES_A::Clk1,
            1 => PRES_A::Clk2,
            2 => PRES_A::Clk4,
            3 => PRES_A::Clk8,
            4 => PRES_A::Clk16,
            5 => PRES_A::Clk32,
            6 => PRES_A::Clk64,
            7 => PRES_A::Clk3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Clk1`"]
    #[inline(always)]
    pub fn is_clk_1(&self) -> bool {
        *self == PRES_A::Clk1
    }
    #[doc = "Checks if the value of the field is `Clk2`"]
    #[inline(always)]
    pub fn is_clk_2(&self) -> bool {
        *self == PRES_A::Clk2
    }
    #[doc = "Checks if the value of the field is `Clk4`"]
    #[inline(always)]
    pub fn is_clk_4(&self) -> bool {
        *self == PRES_A::Clk4
    }
    #[doc = "Checks if the value of the field is `Clk8`"]
    #[inline(always)]
    pub fn is_clk_8(&self) -> bool {
        *self == PRES_A::Clk8
    }
    #[doc = "Checks if the value of the field is `Clk16`"]
    #[inline(always)]
    pub fn is_clk_16(&self) -> bool {
        *self == PRES_A::Clk16
    }
    #[doc = "Checks if the value of the field is `Clk32`"]
    #[inline(always)]
    pub fn is_clk_32(&self) -> bool {
        *self == PRES_A::Clk32
    }
    #[doc = "Checks if the value of the field is `Clk64`"]
    #[inline(always)]
    pub fn is_clk_64(&self) -> bool {
        *self == PRES_A::Clk64
    }
    #[doc = "Checks if the value of the field is `Clk3`"]
    #[inline(always)]
    pub fn is_clk_3(&self) -> bool {
        *self == PRES_A::Clk3
    }
}
#[doc = "Field `PRES` writer - Processor Clock Prescaler"]
pub type PRES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PMC_MCKR_SPEC, u8, PRES_A, 3, O>;
impl<'a, const O: u8> PRES_W<'a, O> {
    #[doc = "Selected clock"]
    #[inline(always)]
    pub fn clk_1(self) -> &'a mut W {
        self.variant(PRES_A::Clk1)
    }
    #[doc = "Selected clock divided by 2"]
    #[inline(always)]
    pub fn clk_2(self) -> &'a mut W {
        self.variant(PRES_A::Clk2)
    }
    #[doc = "Selected clock divided by 4"]
    #[inline(always)]
    pub fn clk_4(self) -> &'a mut W {
        self.variant(PRES_A::Clk4)
    }
    #[doc = "Selected clock divided by 8"]
    #[inline(always)]
    pub fn clk_8(self) -> &'a mut W {
        self.variant(PRES_A::Clk8)
    }
    #[doc = "Selected clock divided by 16"]
    #[inline(always)]
    pub fn clk_16(self) -> &'a mut W {
        self.variant(PRES_A::Clk16)
    }
    #[doc = "Selected clock divided by 32"]
    #[inline(always)]
    pub fn clk_32(self) -> &'a mut W {
        self.variant(PRES_A::Clk32)
    }
    #[doc = "Selected clock divided by 64"]
    #[inline(always)]
    pub fn clk_64(self) -> &'a mut W {
        self.variant(PRES_A::Clk64)
    }
    #[doc = "Selected clock divided by 3"]
    #[inline(always)]
    pub fn clk_3(self) -> &'a mut W {
        self.variant(PRES_A::Clk3)
    }
}
#[doc = "Field `PLLADIV2` reader - PLLA Divisor by 2"]
pub type PLLADIV2_R = crate::BitReader<bool>;
#[doc = "Field `PLLADIV2` writer - PLLA Divisor by 2"]
pub type PLLADIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_MCKR_SPEC, bool, O>;
#[doc = "Field `UPLLDIV2` reader - UPLL Divisor by 2"]
pub type UPLLDIV2_R = crate::BitReader<bool>;
#[doc = "Field `UPLLDIV2` writer - UPLL Divisor by 2"]
pub type UPLLDIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_MCKR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Processor Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 12 - PLLA Divisor by 2"]
    #[inline(always)]
    pub fn plladiv2(&self) -> PLLADIV2_R {
        PLLADIV2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - UPLL Divisor by 2"]
    #[inline(always)]
    pub fn uplldiv2(&self) -> UPLLDIV2_R {
        UPLLDIV2_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CSS_W<0> {
        CSS_W::new(self)
    }
    #[doc = "Bits 4:6 - Processor Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pres(&mut self) -> PRES_W<4> {
        PRES_W::new(self)
    }
    #[doc = "Bit 12 - PLLA Divisor by 2"]
    #[inline(always)]
    #[must_use]
    pub fn plladiv2(&mut self) -> PLLADIV2_W<12> {
        PLLADIV2_W::new(self)
    }
    #[doc = "Bit 13 - UPLL Divisor by 2"]
    #[inline(always)]
    #[must_use]
    pub fn uplldiv2(&mut self) -> UPLLDIV2_W<13> {
        UPLLDIV2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_mckr](index.html) module"]
pub struct PMC_MCKR_SPEC;
impl crate::RegisterSpec for PMC_MCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_mckr::R](R) reader structure"]
impl crate::Readable for PMC_MCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_mckr::W](W) writer structure"]
impl crate::Writable for PMC_MCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMC_MCKR to value 0x01"]
impl crate::Resettable for PMC_MCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
