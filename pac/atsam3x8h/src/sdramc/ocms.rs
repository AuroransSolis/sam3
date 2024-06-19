#[doc = "Register `OCMS` reader"]
pub type R = crate::R<OcmsSpec>;
#[doc = "Register `OCMS` writer"]
pub type W = crate::W<OcmsSpec>;
#[doc = "Field `SDR_SE` reader - SDRAM Memory Controller Scrambling Enable"]
pub type SdrSeR = crate::BitReader;
#[doc = "Field `SDR_SE` writer - SDRAM Memory Controller Scrambling Enable"]
pub type SdrSeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SDRAM Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn sdr_se(&self) -> SdrSeR {
        SdrSeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDRAM Memory Controller Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdr_se(&mut self) -> SdrSeW<OcmsSpec> {
        SdrSeW::new(self, 0)
    }
}
#[doc = "SDRAMC OCMS Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ocms::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocms::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
