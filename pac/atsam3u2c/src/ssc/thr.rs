#[doc = "Register `THR` writer"]
pub type W = crate::W<ThrSpec>;
#[doc = "Field `TDAT` writer - Transmit Data"]
pub type TdatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Transmit Data"]
    #[inline(always)]
    #[must_use]
    pub fn tdat(&mut self) -> TdatW<ThrSpec> {
        TdatW::new(self, 0)
    }
}
#[doc = "Transmit Holding Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThrSpec;
impl crate::RegisterSpec for ThrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`thr::W`](W) writer structure"]
impl crate::Writable for ThrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
