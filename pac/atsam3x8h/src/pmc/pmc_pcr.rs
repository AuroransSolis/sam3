#[doc = "Register `PMC_PCR` reader"]
pub type R = crate::R<PMC_PCR_SPEC>;
#[doc = "Register `PMC_PCR` writer"]
pub type W = crate::W<PMC_PCR_SPEC>;
#[doc = "Field `PID` reader - Peripheral ID"]
pub type PID_R = crate::FieldReader;
#[doc = "Field `PID` writer - Peripheral ID"]
pub type PID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CMD` reader - Command"]
pub type CMD_R = crate::BitReader;
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIV` reader - Divisor Value"]
pub type DIV_R = crate::FieldReader<DIV_A>;
#[doc = "Divisor Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIV_A {
    #[doc = "0: Peripheral clock is MCK"]
    PeriphDivMck = 0,
    #[doc = "1: Peripheral clock is MCK/2"]
    PeriphDiv2Mck = 1,
    #[doc = "2: Peripheral clock is MCK/4"]
    PeriphDiv4Mck = 2,
}
impl From<DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: DIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIV_A {
    type Ux = u8;
}
impl DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DIV_A> {
        match self.bits {
            0 => Some(DIV_A::PeriphDivMck),
            1 => Some(DIV_A::PeriphDiv2Mck),
            2 => Some(DIV_A::PeriphDiv4Mck),
            _ => None,
        }
    }
    #[doc = "Peripheral clock is MCK"]
    #[inline(always)]
    pub fn is_periph_div_mck(&self) -> bool {
        *self == DIV_A::PeriphDivMck
    }
    #[doc = "Peripheral clock is MCK/2"]
    #[inline(always)]
    pub fn is_periph_div2_mck(&self) -> bool {
        *self == DIV_A::PeriphDiv2Mck
    }
    #[doc = "Peripheral clock is MCK/4"]
    #[inline(always)]
    pub fn is_periph_div4_mck(&self) -> bool {
        *self == DIV_A::PeriphDiv4Mck
    }
}
#[doc = "Field `DIV` writer - Divisor Value"]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, DIV_A>;
impl<'a, REG, const O: u8> DIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral clock is MCK"]
    #[inline(always)]
    pub fn periph_div_mck(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::PeriphDivMck)
    }
    #[doc = "Peripheral clock is MCK/2"]
    #[inline(always)]
    pub fn periph_div2_mck(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::PeriphDiv2Mck)
    }
    #[doc = "Peripheral clock is MCK/4"]
    #[inline(always)]
    pub fn periph_div4_mck(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::PeriphDiv4Mck)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Divisor Value"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Peripheral ID"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<PMC_PCR_SPEC, 0> {
        PID_W::new(self)
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<PMC_PCR_SPEC, 12> {
        CMD_W::new(self)
    }
    #[doc = "Bits 16:17 - Divisor Value"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<PMC_PCR_SPEC, 16> {
        DIV_W::new(self)
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<PMC_PCR_SPEC, 28> {
        EN_W::new(self)
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
#[doc = "Peripheral Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_pcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_pcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMC_PCR_SPEC;
impl crate::RegisterSpec for PMC_PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_pcr::R`](R) reader structure"]
impl crate::Readable for PMC_PCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmc_pcr::W`](W) writer structure"]
impl crate::Writable for PMC_PCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMC_PCR to value 0"]
impl crate::Resettable for PMC_PCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
