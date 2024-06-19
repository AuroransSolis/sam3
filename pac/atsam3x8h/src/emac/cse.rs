#[doc = "Register `CSE` reader"]
pub type R = crate::R<CseSpec>;
#[doc = "Register `CSE` writer"]
pub type W = crate::W<CseSpec>;
#[doc = "Field `CSE` reader - Carrier Sense Errors"]
pub type CseR = crate::FieldReader;
#[doc = "Field `CSE` writer - Carrier Sense Errors"]
pub type CseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Carrier Sense Errors"]
    #[inline(always)]
    pub fn cse(&self) -> CseR {
        CseR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Carrier Sense Errors"]
    #[inline(always)]
    #[must_use]
    pub fn cse(&mut self) -> CseW<CseSpec> {
        CseW::new(self, 0)
    }
}
#[doc = "Carrier Sense Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CseSpec;
impl crate::RegisterSpec for CseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cse::R`](R) reader structure"]
impl crate::Readable for CseSpec {}
#[doc = "`write(|w| ..)` method takes [`cse::W`](W) writer structure"]
impl crate::Writable for CseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSE to value 0"]
impl crate::Resettable for CseSpec {
    const RESET_VALUE: u32 = 0;
}
