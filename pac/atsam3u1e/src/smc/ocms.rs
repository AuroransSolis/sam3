#[doc = "Register `OCMS` reader"]
pub type R = crate::R<OcmsSpec>;
#[doc = "Register `OCMS` writer"]
pub type W = crate::W<OcmsSpec>;
#[doc = "Field `SMSE` reader - Static Memory Controller Scrambling Enable"]
pub type SmseR = crate::BitReader;
#[doc = "Field `SMSE` writer - Static Memory Controller Scrambling Enable"]
pub type SmseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRSE` reader - SRAM Scrambling Enable"]
pub type SrseR = crate::BitReader;
#[doc = "Field `SRSE` writer - SRAM Scrambling Enable"]
pub type SrseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn smse(&self) -> SmseR {
        SmseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM Scrambling Enable"]
    #[inline(always)]
    pub fn srse(&self) -> SrseR {
        SrseR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smse(&mut self) -> SmseW<OcmsSpec> {
        SmseW::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srse(&mut self) -> SrseW<OcmsSpec> {
        SrseW::new(self, 1)
    }
}
#[doc = "SMC OCMS Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocms::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocms::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcmsSpec;
impl crate::RegisterSpec for OcmsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ocms::R`](R) reader structure"]
impl crate::Readable for OcmsSpec {}
#[doc = "`write(|w| ..)` method takes [`ocms::W`](W) writer structure"]
impl crate::Writable for OcmsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OCMS to value 0"]
impl crate::Resettable for OcmsSpec {
    const RESET_VALUE: u32 = 0;
}
