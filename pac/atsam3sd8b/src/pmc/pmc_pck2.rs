#[doc = "Register `PMC_PCK2` reader"]
pub type R = crate::R<PmcPck2Spec>;
#[doc = "Register `PMC_PCK2` writer"]
pub type W = crate::W<PmcPck2Spec>;
#[doc = "Master Clock Source Selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Css {
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
impl From<Css> for u8 {
    #[inline(always)]
    fn from(variant: Css) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Css {
    type Ux = u8;
}
impl crate::IsEnum for Css {}
#[doc = "Field `CSS` reader - Master Clock Source Selection"]
pub type CssR = crate::FieldReader<Css>;
impl CssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Css> {
        match self.bits {
            0 => Some(Css::SlowClk),
            1 => Some(Css::MainClk),
            2 => Some(Css::PllaClk),
            3 => Some(Css::PllbClk),
            4 => Some(Css::Mck),
            _ => None,
        }
    }
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == Css::SlowClk
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == Css::MainClk
    }
    #[doc = "PLLA Clock is selected"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == Css::PllaClk
    }
    #[doc = "PLLB Clock is selected"]
    #[inline(always)]
    pub fn is_pllb_clk(&self) -> bool {
        *self == Css::PllbClk
    }
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Css::Mck
    }
}
#[doc = "Field `CSS` writer - Master Clock Source Selection"]
pub type CssW<'a, REG> = crate::FieldWriter<'a, REG, 3, Css>;
impl<'a, REG> CssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Css::SlowClk)
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Css::MainClk)
    }
    #[doc = "PLLA Clock is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Css::PllaClk)
    }
    #[doc = "PLLB Clock is selected"]
    #[inline(always)]
    pub fn pllb_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Css::PllbClk)
    }
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(Css::Mck)
    }
}
#[doc = "Programmable Clock Prescaler"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pres {
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
impl From<Pres> for u8 {
    #[inline(always)]
    fn from(variant: Pres) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pres {
    type Ux = u8;
}
impl crate::IsEnum for Pres {}
#[doc = "Field `PRES` reader - Programmable Clock Prescaler"]
pub type PresR = crate::FieldReader<Pres>;
impl PresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pres> {
        match self.bits {
            0 => Some(Pres::Clk1),
            1 => Some(Pres::Clk2),
            2 => Some(Pres::Clk4),
            3 => Some(Pres::Clk8),
            4 => Some(Pres::Clk16),
            5 => Some(Pres::Clk32),
            6 => Some(Pres::Clk64),
            _ => None,
        }
    }
    #[doc = "Selected clock"]
    #[inline(always)]
    pub fn is_clk_1(&self) -> bool {
        *self == Pres::Clk1
    }
    #[doc = "Selected clock divided by 2"]
    #[inline(always)]
    pub fn is_clk_2(&self) -> bool {
        *self == Pres::Clk2
    }
    #[doc = "Selected clock divided by 4"]
    #[inline(always)]
    pub fn is_clk_4(&self) -> bool {
        *self == Pres::Clk4
    }
    #[doc = "Selected clock divided by 8"]
    #[inline(always)]
    pub fn is_clk_8(&self) -> bool {
        *self == Pres::Clk8
    }
    #[doc = "Selected clock divided by 16"]
    #[inline(always)]
    pub fn is_clk_16(&self) -> bool {
        *self == Pres::Clk16
    }
    #[doc = "Selected clock divided by 32"]
    #[inline(always)]
    pub fn is_clk_32(&self) -> bool {
        *self == Pres::Clk32
    }
    #[doc = "Selected clock divided by 64"]
    #[inline(always)]
    pub fn is_clk_64(&self) -> bool {
        *self == Pres::Clk64
    }
}
#[doc = "Field `PRES` writer - Programmable Clock Prescaler"]
pub type PresW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pres>;
impl<'a, REG> PresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selected clock"]
    #[inline(always)]
    pub fn clk_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Clk1)
    }
    #[doc = "Selected clock divided by 2"]
    #[inline(always)]
    pub fn clk_2(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Clk2)
    }
    #[doc = "Selected clock divided by 4"]
    #[inline(always)]
    pub fn clk_4(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Clk4)
    }
    #[doc = "Selected clock divided by 8"]
    #[inline(always)]
    pub fn clk_8(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Clk8)
    }
    #[doc = "Selected clock divided by 16"]
    #[inline(always)]
    pub fn clk_16(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Clk16)
    }
    #[doc = "Selected clock divided by 32"]
    #[inline(always)]
    pub fn clk_32(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Clk32)
    }
    #[doc = "Selected clock divided by 64"]
    #[inline(always)]
    pub fn clk_64(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Clk64)
    }
}
impl R {
    #[doc = "Bits 0:2 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CssR {
        CssR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PresR {
        PresR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Master Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CssW<PmcPck2Spec> {
        CssW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Programmable Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pres(&mut self) -> PresW<PmcPck2Spec> {
        PresW::new(self, 4)
    }
}
#[doc = "Programmable Clock 0 Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pmc_pck2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc_pck2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcPck2Spec;
impl crate::RegisterSpec for PmcPck2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_pck2::R`](R) reader structure"]
impl crate::Readable for PmcPck2Spec {}
#[doc = "`write(|w| ..)` method takes [`pmc_pck2::W`](W) writer structure"]
impl crate::Writable for PmcPck2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
