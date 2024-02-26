#[doc = "Register `FRO` reader"]
pub type R = crate::R<FroSpec>;
#[doc = "Register `FRO` writer"]
pub type W = crate::W<FroSpec>;
#[doc = "Field `FROK` reader - Frames Received OK"]
pub type FrokR = crate::FieldReader<u32>;
#[doc = "Field `FROK` writer - Frames Received OK"]
pub type FrokW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Frames Received OK"]
    #[inline(always)]
    pub fn frok(&self) -> FrokR {
        FrokR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Frames Received OK"]
    #[inline(always)]
    #[must_use]
    pub fn frok(&mut self) -> FrokW<FroSpec> {
        FrokW::new(self, 0)
    }
}
#[doc = "Frames Received Ok Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fro::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fro::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FroSpec;
impl crate::RegisterSpec for FroSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fro::R`](R) reader structure"]
impl crate::Readable for FroSpec {}
#[doc = "`write(|w| ..)` method takes [`fro::W`](W) writer structure"]
impl crate::Writable for FroSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRO to value 0"]
impl crate::Resettable for FroSpec {
    const RESET_VALUE: u32 = 0;
}
