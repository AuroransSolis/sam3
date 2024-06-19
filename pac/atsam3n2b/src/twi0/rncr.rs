#[doc = "Register `RNCR` reader"]
pub type R = crate::R<RncrSpec>;
#[doc = "Register `RNCR` writer"]
pub type W = crate::W<RncrSpec>;
#[doc = "Field `RXNCTR` reader - Receive Next Counter"]
pub type RxnctrR = crate::FieldReader<u16>;
#[doc = "Field `RXNCTR` writer - Receive Next Counter"]
pub type RxnctrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Next Counter"]
    #[inline(always)]
    pub fn rxnctr(&self) -> RxnctrR {
        RxnctrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Next Counter"]
    #[inline(always)]
    #[must_use]
    pub fn rxnctr(&mut self) -> RxnctrW<RncrSpec> {
        RxnctrW::new(self, 0)
    }
}
#[doc = "Receive Next Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rncr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rncr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RncrSpec;
impl crate::RegisterSpec for RncrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rncr::R`](R) reader structure"]
impl crate::Readable for RncrSpec {}
#[doc = "`write(|w| ..)` method takes [`rncr::W`](W) writer structure"]
impl crate::Writable for RncrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNCR to value 0"]
impl crate::Resettable for RncrSpec {
    const RESET_VALUE: u32 = 0;
}
