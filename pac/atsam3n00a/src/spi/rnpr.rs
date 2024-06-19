#[doc = "Register `RNPR` reader"]
pub type R = crate::R<RnprSpec>;
#[doc = "Register `RNPR` writer"]
pub type W = crate::W<RnprSpec>;
#[doc = "Field `RXNPTR` reader - Receive Next Pointer"]
pub type RxnptrR = crate::FieldReader<u32>;
#[doc = "Field `RXNPTR` writer - Receive Next Pointer"]
pub type RxnptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Next Pointer"]
    #[inline(always)]
    pub fn rxnptr(&self) -> RxnptrR {
        RxnptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Next Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn rxnptr(&mut self) -> RxnptrW<RnprSpec> {
        RxnptrW::new(self, 0)
    }
}
#[doc = "Receive Next Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RnprSpec;
impl crate::RegisterSpec for RnprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rnpr::R`](R) reader structure"]
impl crate::Readable for RnprSpec {}
#[doc = "`write(|w| ..)` method takes [`rnpr::W`](W) writer structure"]
impl crate::Writable for RnprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNPR to value 0"]
impl crate::Resettable for RnprSpec {
    const RESET_VALUE: u32 = 0;
}
