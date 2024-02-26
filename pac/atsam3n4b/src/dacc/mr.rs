#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `TRGEN` reader - Trigger Enable"]
pub type TrgenR = crate::BitReader;
#[doc = "Field `TRGEN` writer - Trigger Enable"]
pub type TrgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsel {
    #[doc = "0: External trigger"]
    Trgsel0 = 0,
    #[doc = "1: TIO Output of the Timer Counter Channel 0"]
    Trgsel1 = 1,
    #[doc = "2: TIO Output of the Timer Counter Channel 1"]
    Trgsel2 = 2,
    #[doc = "3: TIO Output of the Timer Counter Channel 2"]
    Trgsel3 = 3,
}
impl From<Trgsel> for u8 {
    #[inline(always)]
    fn from(variant: Trgsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsel {
    type Ux = u8;
}
#[doc = "Field `TRGSEL` reader - Trigger Selection"]
pub type TrgselR = crate::FieldReader<Trgsel>;
impl TrgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trgsel> {
        match self.bits {
            0 => Some(Trgsel::Trgsel0),
            1 => Some(Trgsel::Trgsel1),
            2 => Some(Trgsel::Trgsel2),
            3 => Some(Trgsel::Trgsel3),
            _ => None,
        }
    }
    #[doc = "External trigger"]
    #[inline(always)]
    pub fn is_trgsel0(&self) -> bool {
        *self == Trgsel::Trgsel0
    }
    #[doc = "TIO Output of the Timer Counter Channel 0"]
    #[inline(always)]
    pub fn is_trgsel1(&self) -> bool {
        *self == Trgsel::Trgsel1
    }
    #[doc = "TIO Output of the Timer Counter Channel 1"]
    #[inline(always)]
    pub fn is_trgsel2(&self) -> bool {
        *self == Trgsel::Trgsel2
    }
    #[doc = "TIO Output of the Timer Counter Channel 2"]
    #[inline(always)]
    pub fn is_trgsel3(&self) -> bool {
        *self == Trgsel::Trgsel3
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Selection"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trgsel>;
impl<'a, REG> TrgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External trigger"]
    #[inline(always)]
    pub fn trgsel0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::Trgsel0)
    }
    #[doc = "TIO Output of the Timer Counter Channel 0"]
    #[inline(always)]
    pub fn trgsel1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::Trgsel1)
    }
    #[doc = "TIO Output of the Timer Counter Channel 1"]
    #[inline(always)]
    pub fn trgsel2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::Trgsel2)
    }
    #[doc = "TIO Output of the Timer Counter Channel 2"]
    #[inline(always)]
    pub fn trgsel3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::Trgsel3)
    }
}
#[doc = "Field `DACEN` reader - DAC enable"]
pub type DacenR = crate::BitReader;
#[doc = "Field `DACEN` writer - DAC enable"]
pub type DacenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORD` reader - Word Transfer"]
pub type WordR = crate::BitReader;
#[doc = "Field `WORD` writer - Word Transfer"]
pub type WordW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTUP` reader - Startup Time Selection"]
pub type StartupR = crate::FieldReader;
#[doc = "Field `STARTUP` writer - Startup Time Selection"]
pub type StartupW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLKDIV` reader - DAC Clock Divider for Internal Trigger"]
pub type ClkdivR = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV` writer - DAC Clock Divider for Internal Trigger"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TrgenR {
        TrgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - DAC enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DacenR {
        DacenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Word Transfer"]
    #[inline(always)]
    pub fn word(&self) -> WordR {
        WordR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Startup Time Selection"]
    #[inline(always)]
    pub fn startup(&self) -> StartupR {
        StartupR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - DAC Clock Divider for Internal Trigger"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgen(&mut self) -> TrgenW<MrSpec> {
        TrgenW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TrgselW<MrSpec> {
        TrgselW::new(self, 1)
    }
    #[doc = "Bit 4 - DAC enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DacenW<MrSpec> {
        DacenW::new(self, 4)
    }
    #[doc = "Bit 5 - Word Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn word(&mut self) -> WordW<MrSpec> {
        WordW::new(self, 5)
    }
    #[doc = "Bits 8:15 - Startup Time Selection"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> StartupW<MrSpec> {
        StartupW::new(self, 8)
    }
    #[doc = "Bits 16:31 - DAC Clock Divider for Internal Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> ClkdivW<MrSpec> {
        ClkdivW::new(self, 16)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0;
}
