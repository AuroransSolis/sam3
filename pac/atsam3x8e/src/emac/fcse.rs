#[doc = "Register `FCSE` reader"]
pub type R = crate::R<FcseSpec>;
#[doc = "Register `FCSE` writer"]
pub type W = crate::W<FcseSpec>;
#[doc = "Field `FCSE` reader - Frame Check Sequence Errors"]
pub type FcseR = crate::FieldReader;
#[doc = "Field `FCSE` writer - Frame Check Sequence Errors"]
pub type FcseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Check Sequence Errors"]
    #[inline(always)]
    pub fn fcse(&self) -> FcseR {
        FcseR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Check Sequence Errors"]
    #[inline(always)]
    #[must_use]
    pub fn fcse(&mut self) -> FcseW<FcseSpec> {
        FcseW::new(self, 0)
    }
}
#[doc = "Frame Check Sequence Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcseSpec;
impl crate::RegisterSpec for FcseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcse::R`](R) reader structure"]
impl crate::Readable for FcseSpec {}
#[doc = "`write(|w| ..)` method takes [`fcse::W`](W) writer structure"]
impl crate::Writable for FcseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCSE to value 0"]
impl crate::Resettable for FcseSpec {
    const RESET_VALUE: u32 = 0;
}
