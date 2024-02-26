#[doc = "Register `TNPR` reader"]
pub type R = crate::R<TnprSpec>;
#[doc = "Register `TNPR` writer"]
pub type W = crate::W<TnprSpec>;
#[doc = "Field `TXNPTR` reader - Transmit Next Pointer"]
pub type TxnptrR = crate::FieldReader<u32>;
#[doc = "Field `TXNPTR` writer - Transmit Next Pointer"]
pub type TxnptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Next Pointer"]
    #[inline(always)]
    pub fn txnptr(&self) -> TxnptrR {
        TxnptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Next Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn txnptr(&mut self) -> TxnptrW<TnprSpec> {
        TxnptrW::new(self, 0)
    }
}
#[doc = "Transmit Next Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tnpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tnpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnprSpec;
impl crate::RegisterSpec for TnprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tnpr::R`](R) reader structure"]
impl crate::Readable for TnprSpec {}
#[doc = "`write(|w| ..)` method takes [`tnpr::W`](W) writer structure"]
impl crate::Writable for TnprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TNPR to value 0"]
impl crate::Resettable for TnprSpec {
    const RESET_VALUE: u32 = 0;
}
