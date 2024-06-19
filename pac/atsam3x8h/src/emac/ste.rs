#[doc = "Register `STE` reader"]
pub type R = crate::R<SteSpec>;
#[doc = "Register `STE` writer"]
pub type W = crate::W<SteSpec>;
#[doc = "Field `SQER` reader - SQE test errors"]
pub type SqerR = crate::FieldReader;
#[doc = "Field `SQER` writer - SQE test errors"]
pub type SqerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SQE test errors"]
    #[inline(always)]
    pub fn sqer(&self) -> SqerR {
        SqerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SQE test errors"]
    #[inline(always)]
    #[must_use]
    pub fn sqer(&mut self) -> SqerW<SteSpec> {
        SqerW::new(self, 0)
    }
}
#[doc = "SQE Test Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ste::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ste::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SteSpec;
impl crate::RegisterSpec for SteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ste::R`](R) reader structure"]
impl crate::Readable for SteSpec {}
#[doc = "`write(|w| ..)` method takes [`ste::W`](W) writer structure"]
impl crate::Writable for SteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STE to value 0"]
impl crate::Resettable for SteSpec {
    const RESET_VALUE: u32 = 0;
}
