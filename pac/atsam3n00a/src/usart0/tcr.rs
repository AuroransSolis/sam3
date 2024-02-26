#[doc = "Register `TCR` reader"]
pub type R = crate::R<TcrSpec>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TcrSpec>;
#[doc = "Field `TXCTR` reader - Transmit Counter Register"]
pub type TxctrR = crate::FieldReader<u16>;
#[doc = "Field `TXCTR` writer - Transmit Counter Register"]
pub type TxctrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit Counter Register"]
    #[inline(always)]
    pub fn txctr(&self) -> TxctrR {
        TxctrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Counter Register"]
    #[inline(always)]
    #[must_use]
    pub fn txctr(&mut self) -> TxctrW<TcrSpec> {
        TxctrW::new(self, 0)
    }
}
#[doc = "Transmit Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrSpec;
impl crate::RegisterSpec for TcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TcrSpec {
    const RESET_VALUE: u32 = 0;
}
