#[doc = "Register `AR` reader"]
pub type R = crate::R<ArSpec>;
#[doc = "Register `AR` writer"]
pub type W = crate::W<ArSpec>;
#[doc = "Field `ALMV` reader - Alarm Value"]
pub type AlmvR = crate::FieldReader<u32>;
#[doc = "Field `ALMV` writer - Alarm Value"]
pub type AlmvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alarm Value"]
    #[inline(always)]
    pub fn almv(&self) -> AlmvR {
        AlmvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alarm Value"]
    #[inline(always)]
    #[must_use]
    pub fn almv(&mut self) -> AlmvW<ArSpec> {
        AlmvW::new(self, 0)
    }
}
#[doc = "Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArSpec;
impl crate::RegisterSpec for ArSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ar::R`](R) reader structure"]
impl crate::Readable for ArSpec {}
#[doc = "`write(|w| ..)` method takes [`ar::W`](W) writer structure"]
impl crate::Writable for ArSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AR to value 0xffff_ffff"]
impl crate::Resettable for ArSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
