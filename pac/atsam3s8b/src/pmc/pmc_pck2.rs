#[doc = "Register `PMC_PCK2` reader"]
pub type R = crate::R<PMC_PCK2_SPEC>;
#[doc = "Register `PMC_PCK2` writer"]
pub type W = crate::W<PMC_PCK2_SPEC>;
#[doc = "Field `CSS` reader - Master Clock Source Selection"]
pub type CSS_R = crate::FieldReader<CSS_A>;
#[doc = "Master Clock Source Selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSS_A {
    #[doc = "0: Slow Clock is selected"]
    SlowClk = 0,
    #[doc = "1: Main Clock is selected"]
    MainClk = 1,
    #[doc = "2: PLLA Clock is selected"]
    PllaClk = 2,
    #[doc = "3: PLLB Clock is selected"]
    PllbClk = 3,
    #[doc = "4: Master Clock is selected"]
    Mck = 4,
}
impl From<CSS_A> for u8 {
    #[inline(always)]
    fn from(variant: CSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSS_A {
    type Ux = u8;
}
impl CSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSS_A> {
        match self.bits {
            0 => Some(CSS_A::SlowClk),
            1 => Some(CSS_A::MainClk),
            2 => Some(CSS_A::PllaClk),
            3 => Some(CSS_A::PllbClk),
            4 => Some(CSS_A::Mck),
            _ => None,
        }
    }
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == CSS_A::SlowClk
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == CSS_A::MainClk
    }
    #[doc = "PLLA Clock is selected"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == CSS_A::PllaClk
    }
    #[doc = "PLLB Clock is selected"]
    #[inline(always)]
    pub fn is_pllb_clk(&self) -> bool {
        *self == CSS_A::PllbClk
    }
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CSS_A::Mck
    }
}
#[doc = "Field `CSS` writer - Master Clock Source Selection"]
pub type CSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CSS_A>;
impl<'a, REG, const O: u8> CSS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut crate::W<REG> {
        self.variant(CSS_A::SlowClk)
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut crate::W<REG> {
        self.variant(CSS_A::MainClk)
    }
    #[doc = "PLLA Clock is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut crate::W<REG> {
        self.variant(CSS_A::PllaClk)
    }
    #[doc = "PLLB Clock is selected"]
    #[inline(always)]
    pub fn pllb_clk(self) -> &'a mut crate::W<REG> {
        self.variant(CSS_A::PllbClk)
    }
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(CSS_A::Mck)
    }
}
#[doc = "Field `PRES` reader - Programmable Clock Prescaler"]
pub type PRES_R = crate::FieldReader<PRES_A>;
#[doc = "Programmable Clock Prescaler"]
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
}
impl From<PRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PRES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRES_A {
    type Ux = u8;
}
impl PRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRES_A> {
        match self.bits {
            0 => Some(PRES_A::Clk1),
            1 => Some(PRES_A::Clk2),
            2 => Some(PRES_A::Clk4),
            3 => Some(PRES_A::Clk8),
            4 => Some(PRES_A::Clk16),
            5 => Some(PRES_A::Clk32),
            6 => Some(PRES_A::Clk64),
            _ => None,
        }
    }
    #[doc = "Selected clock"]
    #[inline(always)]
    pub fn is_clk_1(&self) -> bool {
        *self == PRES_A::Clk1
    }
    #[doc = "Selected clock divided by 2"]
    #[inline(always)]
    pub fn is_clk_2(&self) -> bool {
        *self == PRES_A::Clk2
    }
    #[doc = "Selected clock divided by 4"]
    #[inline(always)]
    pub fn is_clk_4(&self) -> bool {
        *self == PRES_A::Clk4
    }
    #[doc = "Selected clock divided by 8"]
    #[inline(always)]
    pub fn is_clk_8(&self) -> bool {
        *self == PRES_A::Clk8
    }
    #[doc = "Selected clock divided by 16"]
    #[inline(always)]
    pub fn is_clk_16(&self) -> bool {
        *self == PRES_A::Clk16
    }
    #[doc = "Selected clock divided by 32"]
    #[inline(always)]
    pub fn is_clk_32(&self) -> bool {
        *self == PRES_A::Clk32
    }
    #[doc = "Selected clock divided by 64"]
    #[inline(always)]
    pub fn is_clk_64(&self) -> bool {
        *self == PRES_A::Clk64
    }
}
#[doc = "Field `PRES` writer - Programmable Clock Prescaler"]
pub type PRES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, PRES_A>;
impl<'a, REG, const O: u8> PRES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selected clock"]
    #[inline(always)]
    pub fn clk_1(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::Clk1)
    }
    #[doc = "Selected clock divided by 2"]
    #[inline(always)]
    pub fn clk_2(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::Clk2)
    }
    #[doc = "Selected clock divided by 4"]
    #[inline(always)]
    pub fn clk_4(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::Clk4)
    }
    #[doc = "Selected clock divided by 8"]
    #[inline(always)]
    pub fn clk_8(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::Clk8)
    }
    #[doc = "Selected clock divided by 16"]
    #[inline(always)]
    pub fn clk_16(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::Clk16)
    }
    #[doc = "Selected clock divided by 32"]
    #[inline(always)]
    pub fn clk_32(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::Clk32)
    }
    #[doc = "Selected clock divided by 64"]
    #[inline(always)]
    pub fn clk_64(self) -> &'a mut crate::W<REG> {
        self.variant(PRES_A::Clk64)
    }
}
impl R {
    #[doc = "Bits 0:2 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Master Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CSS_W<PMC_PCK2_SPEC, 0> {
        CSS_W::new(self)
    }
    #[doc = "Bits 4:6 - Programmable Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pres(&mut self) -> PRES_W<PMC_PCK2_SPEC, 4> {
        PRES_W::new(self)
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
#[doc = "Programmable Clock 0 Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_pck2::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_pck2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMC_PCK2_SPEC;
impl crate::RegisterSpec for PMC_PCK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_pck2::R`](R) reader structure"]
impl crate::Readable for PMC_PCK2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmc_pck2::W`](W) writer structure"]
impl crate::Writable for PMC_PCK2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
