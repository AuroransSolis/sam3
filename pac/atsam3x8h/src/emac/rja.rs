#[doc = "Register `RJA` reader"]
pub type R = crate::R<RjaSpec>;
#[doc = "Register `RJA` writer"]
pub type W = crate::W<RjaSpec>;
#[doc = "Field `RJB` reader - Receive Jabbers"]
pub type RjbR = crate::FieldReader;
#[doc = "Field `RJB` writer - Receive Jabbers"]
pub type RjbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive Jabbers"]
    #[inline(always)]
    pub fn rjb(&self) -> RjbR {
        RjbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Jabbers"]
    #[inline(always)]
    #[must_use]
    pub fn rjb(&mut self) -> RjbW<RjaSpec> {
        RjbW::new(self, 0)
    }
}
#[doc = "Receive Jabbers Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rja::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rja::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RjaSpec;
impl crate::RegisterSpec for RjaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rja::R`](R) reader structure"]
impl crate::Readable for RjaSpec {}
#[doc = "`write(|w| ..)` method takes [`rja::W`](W) writer structure"]
impl crate::Writable for RjaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RJA to value 0"]
impl crate::Resettable for RjaSpec {
    const RESET_VALUE: u32 = 0;
}
