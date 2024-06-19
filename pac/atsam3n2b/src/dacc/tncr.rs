#[doc = "Register `TNCR` reader"]
pub type R = crate::R<TncrSpec>;
#[doc = "Register `TNCR` writer"]
pub type W = crate::W<TncrSpec>;
#[doc = "Field `TXNCTR` reader - Transmit Counter Next"]
pub type TxnctrR = crate::FieldReader<u16>;
#[doc = "Field `TXNCTR` writer - Transmit Counter Next"]
pub type TxnctrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit Counter Next"]
    #[inline(always)]
    pub fn txnctr(&self) -> TxnctrR {
        TxnctrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Counter Next"]
    #[inline(always)]
    #[must_use]
    pub fn txnctr(&mut self) -> TxnctrW<TncrSpec> {
        TxnctrW::new(self, 0)
    }
}
#[doc = "Transmit Next Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tncr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tncr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TncrSpec;
impl crate::RegisterSpec for TncrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tncr::R`](R) reader structure"]
impl crate::Readable for TncrSpec {}
#[doc = "`write(|w| ..)` method takes [`tncr::W`](W) writer structure"]
impl crate::Writable for TncrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TNCR to value 0"]
impl crate::Resettable for TncrSpec {
    const RESET_VALUE: u32 = 0;
}
