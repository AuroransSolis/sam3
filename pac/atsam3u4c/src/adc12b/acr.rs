#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Field `GAIN` reader - Input Gain"]
pub type GainR = crate::FieldReader;
#[doc = "Field `GAIN` writer - Input Gain"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IBCTL` reader - Bias Current Control"]
pub type IbctlR = crate::FieldReader;
#[doc = "Field `IBCTL` writer - Bias Current Control"]
pub type IbctlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIFF` reader - Differential Mode"]
pub type DiffR = crate::BitReader;
#[doc = "Field `DIFF` writer - Differential Mode"]
pub type DiffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSET` reader - Input OFFSET"]
pub type OffsetR = crate::BitReader;
#[doc = "Field `OFFSET` writer - Input OFFSET"]
pub type OffsetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Input Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Bias Current Control"]
    #[inline(always)]
    pub fn ibctl(&self) -> IbctlR {
        IbctlR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input OFFSET"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Gain"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GainW<AcrSpec> {
        GainW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Bias Current Control"]
    #[inline(always)]
    #[must_use]
    pub fn ibctl(&mut self) -> IbctlW<AcrSpec> {
        IbctlW::new(self, 8)
    }
    #[doc = "Bit 16 - Differential Mode"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DiffW<AcrSpec> {
        DiffW::new(self, 16)
    }
    #[doc = "Bit 17 - Input OFFSET"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OffsetW<AcrSpec> {
        OffsetW::new(self, 17)
    }
}
#[doc = "Analog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcrSpec;
impl crate::RegisterSpec for AcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for AcrSpec {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for AcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for AcrSpec {
    const RESET_VALUE: u32 = 0;
}
