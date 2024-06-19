#[doc = "Register `DSCR` reader"]
pub type R = crate::R<DscrSpec>;
#[doc = "Register `DSCR` writer"]
pub type W = crate::W<DscrSpec>;
#[doc = "Field `DSCR` reader - Descriptor Base Address"]
pub type DscrR = crate::FieldReader<u32>;
#[doc = "Field `DSCR` writer - Descriptor Base Address"]
pub type DscrW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 9:31 - Descriptor Base Address"]
    #[inline(always)]
    pub fn dscr(&self) -> DscrR {
        DscrR::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 9:31 - Descriptor Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn dscr(&mut self) -> DscrW<DscrSpec> {
        DscrW::new(self, 9)
    }
}
#[doc = "CRCCU Descriptor Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DscrSpec;
impl crate::RegisterSpec for DscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscr::R`](R) reader structure"]
impl crate::Readable for DscrSpec {}
#[doc = "`write(|w| ..)` method takes [`dscr::W`](W) writer structure"]
impl crate::Writable for DscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSCR to value 0"]
impl crate::Resettable for DscrSpec {
    const RESET_VALUE: u32 = 0;
}
