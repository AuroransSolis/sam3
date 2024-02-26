#[doc = "Register `PFR` reader"]
pub type R = crate::R<PfrSpec>;
#[doc = "Register `PFR` writer"]
pub type W = crate::W<PfrSpec>;
#[doc = "Field `FROK` reader - Pause Frames received OK"]
pub type FrokR = crate::FieldReader<u16>;
#[doc = "Field `FROK` writer - Pause Frames received OK"]
pub type FrokW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Pause Frames received OK"]
    #[inline(always)]
    pub fn frok(&self) -> FrokR {
        FrokR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pause Frames received OK"]
    #[inline(always)]
    #[must_use]
    pub fn frok(&mut self) -> FrokW<PfrSpec> {
        FrokW::new(self, 0)
    }
}
#[doc = "Pause Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfrSpec;
impl crate::RegisterSpec for PfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr::R`](R) reader structure"]
impl crate::Readable for PfrSpec {}
#[doc = "`write(|w| ..)` method takes [`pfr::W`](W) writer structure"]
impl crate::Writable for PfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFR to value 0"]
impl crate::Resettable for PfrSpec {
    const RESET_VALUE: u32 = 0;
}
