#[doc = "Register `FTO` reader"]
pub type R = crate::R<FtoSpec>;
#[doc = "Register `FTO` writer"]
pub type W = crate::W<FtoSpec>;
#[doc = "Field `FTOK` reader - Frames Transmitted OK"]
pub type FtokR = crate::FieldReader<u32>;
#[doc = "Field `FTOK` writer - Frames Transmitted OK"]
pub type FtokW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Frames Transmitted OK"]
    #[inline(always)]
    pub fn ftok(&self) -> FtokR {
        FtokR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Frames Transmitted OK"]
    #[inline(always)]
    #[must_use]
    pub fn ftok(&mut self) -> FtokW<FtoSpec> {
        FtokW::new(self, 0)
    }
}
#[doc = "Frames Transmitted Ok Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtoSpec;
impl crate::RegisterSpec for FtoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fto::R`](R) reader structure"]
impl crate::Readable for FtoSpec {}
#[doc = "`write(|w| ..)` method takes [`fto::W`](W) writer structure"]
impl crate::Writable for FtoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FTO to value 0"]
impl crate::Resettable for FtoSpec {
    const RESET_VALUE: u32 = 0;
}
