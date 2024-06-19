#[doc = "Register `TUND` reader"]
pub type R = crate::R<TundSpec>;
#[doc = "Register `TUND` writer"]
pub type W = crate::W<TundSpec>;
#[doc = "Field `TUND` reader - Transmit Underruns"]
pub type TundR = crate::FieldReader;
#[doc = "Field `TUND` writer - Transmit Underruns"]
pub type TundW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Underruns"]
    #[inline(always)]
    pub fn tund(&self) -> TundR {
        TundR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Underruns"]
    #[inline(always)]
    #[must_use]
    pub fn tund(&mut self) -> TundW<TundSpec> {
        TundW::new(self, 0)
    }
}
#[doc = "Transmit Underrun Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tund::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tund::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TundSpec;
impl crate::RegisterSpec for TundSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tund::R`](R) reader structure"]
impl crate::Readable for TundSpec {}
#[doc = "`write(|w| ..)` method takes [`tund::W`](W) writer structure"]
impl crate::Writable for TundSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TUND to value 0"]
impl crate::Resettable for TundSpec {
    const RESET_VALUE: u32 = 0;
}
