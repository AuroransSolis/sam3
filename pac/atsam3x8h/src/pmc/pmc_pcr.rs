#[doc = "Register `PMC_PCR` reader"]
pub type R = crate::R<PmcPcrSpec>;
#[doc = "Register `PMC_PCR` writer"]
pub type W = crate::W<PmcPcrSpec>;
#[doc = "Field `PID` reader - Peripheral ID"]
pub type PidR = crate::FieldReader;
#[doc = "Field `PID` writer - Peripheral ID"]
pub type PidW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CMD` reader - Command"]
pub type CmdR = crate::BitReader;
#[doc = "Field `CMD` writer - Command"]
pub type CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Divisor Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Div {
    #[doc = "0: Peripheral clock is MCK"]
    PeriphDivMck = 0,
    #[doc = "1: Peripheral clock is MCK/2"]
    PeriphDiv2Mck = 1,
    #[doc = "2: Peripheral clock is MCK/4"]
    PeriphDiv4Mck = 2,
}
impl From<Div> for u8 {
    #[inline(always)]
    fn from(variant: Div) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Div {
    type Ux = u8;
}
impl crate::IsEnum for Div {}
#[doc = "Field `DIV` reader - Divisor Value"]
pub type DivR = crate::FieldReader<Div>;
impl DivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Div> {
        match self.bits {
            0 => Some(Div::PeriphDivMck),
            1 => Some(Div::PeriphDiv2Mck),
            2 => Some(Div::PeriphDiv4Mck),
            _ => None,
        }
    }
    #[doc = "Peripheral clock is MCK"]
    #[inline(always)]
    pub fn is_periph_div_mck(&self) -> bool {
        *self == Div::PeriphDivMck
    }
    #[doc = "Peripheral clock is MCK/2"]
    #[inline(always)]
    pub fn is_periph_div2_mck(&self) -> bool {
        *self == Div::PeriphDiv2Mck
    }
    #[doc = "Peripheral clock is MCK/4"]
    #[inline(always)]
    pub fn is_periph_div4_mck(&self) -> bool {
        *self == Div::PeriphDiv4Mck
    }
}
#[doc = "Field `DIV` writer - Divisor Value"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Div>;
impl<'a, REG> DivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral clock is MCK"]
    #[inline(always)]
    pub fn periph_div_mck(self) -> &'a mut crate::W<REG> {
        self.variant(Div::PeriphDivMck)
    }
    #[doc = "Peripheral clock is MCK/2"]
    #[inline(always)]
    pub fn periph_div2_mck(self) -> &'a mut crate::W<REG> {
        self.variant(Div::PeriphDiv2Mck)
    }
    #[doc = "Peripheral clock is MCK/4"]
    #[inline(always)]
    pub fn periph_div4_mck(self) -> &'a mut crate::W<REG> {
        self.variant(Div::PeriphDiv4Mck)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&self) -> PidR {
        PidR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Divisor Value"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Peripheral ID"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PidW<PmcPcrSpec> {
        PidW::new(self, 0)
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<PmcPcrSpec> {
        CmdW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Divisor Value"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<PmcPcrSpec> {
        DivW::new(self, 16)
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<PmcPcrSpec> {
        EnW::new(self, 28)
    }
}
#[doc = "Peripheral Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_pcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_pcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcPcrSpec;
impl crate::RegisterSpec for PmcPcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_pcr::R`](R) reader structure"]
impl crate::Readable for PmcPcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmc_pcr::W`](W) writer structure"]
impl crate::Writable for PmcPcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMC_PCR to value 0"]
impl crate::Resettable for PmcPcrSpec {
    const RESET_VALUE: u32 = 0;
}
