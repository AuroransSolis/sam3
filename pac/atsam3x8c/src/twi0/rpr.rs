#[doc = "Register `RPR` reader"]
pub type R = crate::R<RprSpec>;
#[doc = "Register `RPR` writer"]
pub type W = crate::W<RprSpec>;
#[doc = "Field `RXPTR` reader - Receive Pointer Register"]
pub type RxptrR = crate::FieldReader<u32>;
#[doc = "Field `RXPTR` writer - Receive Pointer Register"]
pub type RxptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Pointer Register"]
    #[inline(always)]
    pub fn rxptr(&self) -> RxptrR {
        RxptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Pointer Register"]
    #[inline(always)]
    #[must_use]
    pub fn rxptr(&mut self) -> RxptrW<RprSpec> {
        RxptrW::new(self, 0)
    }
}
#[doc = "Receive Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RprSpec;
impl crate::RegisterSpec for RprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpr::R`](R) reader structure"]
impl crate::Readable for RprSpec {}
#[doc = "`write(|w| ..)` method takes [`rpr::W`](W) writer structure"]
impl crate::Writable for RprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RPR to value 0"]
impl crate::Resettable for RprSpec {
    const RESET_VALUE: u32 = 0;
}
