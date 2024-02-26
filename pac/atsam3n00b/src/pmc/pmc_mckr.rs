#[doc = "Register `PMC_MCKR` reader"]
pub type R = crate::R<PmcMckrSpec>;
#[doc = "Register `PMC_MCKR` writer"]
pub type W = crate::W<PmcMckrSpec>;
#[doc = "Master Clock Source Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Css {
    #[doc = "0: Slow Clock is selected"]
    SlowClk = 0,
    #[doc = "1: Main Clock is selected"]
    MainClk = 1,
    #[doc = "2: PLLA Clock is selected"]
    PllaClk = 2,
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
}
#[doc = "Field `CSS` writer - Master Clock Source Selection"]
pub type CssW<'a, REG> = crate::FieldWriter<'a, REG, 2, Css>;
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
}
#[doc = "Processor Clock Prescaler\n\nValue on reset: 0"]
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
    #[doc = "7: Selected clock divided by 3"]
    Clk3 = 7,
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
#[doc = "Field `PRES` reader - Processor Clock Prescaler"]
pub type PresR = crate::FieldReader<Pres>;
impl PresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pres {
        match self.bits {
            0 => Pres::Clk1,
            1 => Pres::Clk2,
            2 => Pres::Clk4,
            3 => Pres::Clk8,
            4 => Pres::Clk16,
            5 => Pres::Clk32,
            6 => Pres::Clk64,
            7 => Pres::Clk3,
            _ => unreachable!(),
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
    #[doc = "Selected clock divided by 3"]
    #[inline(always)]
    pub fn is_clk_3(&self) -> bool {
        *self == Pres::Clk3
    }
}
#[doc = "Field `PRES` writer - Processor Clock Prescaler"]
pub type PresW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Pres>;
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
    #[doc = "Selected clock divided by 3"]
    #[inline(always)]
    pub fn clk_3(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Clk3)
    }
}
#[doc = "Field `PLLADIV2` reader - PLLA Divisor by 2"]
pub type Plladiv2R = crate::BitReader;
#[doc = "Field `PLLADIV2` writer - PLLA Divisor by 2"]
pub type Plladiv2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CssR {
        CssR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Processor Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PresR {
        PresR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 12 - PLLA Divisor by 2"]
    #[inline(always)]
    pub fn plladiv2(&self) -> Plladiv2R {
        Plladiv2R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CssW<PmcMckrSpec> {
        CssW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Processor Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pres(&mut self) -> PresW<PmcMckrSpec> {
        PresW::new(self, 4)
    }
    #[doc = "Bit 12 - PLLA Divisor by 2"]
    #[inline(always)]
    #[must_use]
    pub fn plladiv2(&mut self) -> Plladiv2W<PmcMckrSpec> {
        Plladiv2W::new(self, 12)
    }
}
#[doc = "Master Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_mckr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_mckr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcMckrSpec;
impl crate::RegisterSpec for PmcMckrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_mckr::R`](R) reader structure"]
impl crate::Readable for PmcMckrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmc_mckr::W`](W) writer structure"]
impl crate::Writable for PmcMckrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMC_MCKR to value 0x01"]
impl crate::Resettable for PmcMckrSpec {
    const RESET_VALUE: u32 = 0x01;
}
