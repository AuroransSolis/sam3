#[doc = "Register `CDR` writer"]
pub type W = crate::W<CdrSpec>;
#[doc = "Field `DATA` writer - Data to Convert"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Data to Convert"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<CdrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Conversion Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdrSpec;
impl crate::RegisterSpec for CdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cdr::W`](W) writer structure"]
impl crate::Writable for CdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CdrSpec {
    const RESET_VALUE: u32 = 0;
}
