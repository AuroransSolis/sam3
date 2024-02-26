#[doc = "Register `RSE` reader"]
pub type R = crate::R<RseSpec>;
#[doc = "Register `RSE` writer"]
pub type W = crate::W<RseSpec>;
#[doc = "Field `RSE` reader - Receive Symbol Errors"]
pub type RseR = crate::FieldReader;
#[doc = "Field `RSE` writer - Receive Symbol Errors"]
pub type RseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive Symbol Errors"]
    #[inline(always)]
    pub fn rse(&self) -> RseR {
        RseR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Symbol Errors"]
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RseW<RseSpec> {
        RseW::new(self, 0)
    }
}
#[doc = "Receive Symbol Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RseSpec;
impl crate::RegisterSpec for RseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rse::R`](R) reader structure"]
impl crate::Readable for RseSpec {}
#[doc = "`write(|w| ..)` method takes [`rse::W`](W) writer structure"]
impl crate::Writable for RseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSE to value 0"]
impl crate::Resettable for RseSpec {
    const RESET_VALUE: u32 = 0;
}
