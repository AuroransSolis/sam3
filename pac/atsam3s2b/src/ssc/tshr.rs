#[doc = "Register `TSHR` reader"]
pub type R = crate::R<TshrSpec>;
#[doc = "Register `TSHR` writer"]
pub type W = crate::W<TshrSpec>;
#[doc = "Field `TSDAT` reader - Transmit Synchronization Data"]
pub type TsdatR = crate::FieldReader<u16>;
#[doc = "Field `TSDAT` writer - Transmit Synchronization Data"]
pub type TsdatW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit Synchronization Data"]
    #[inline(always)]
    pub fn tsdat(&self) -> TsdatR {
        TsdatR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Synchronization Data"]
    #[inline(always)]
    #[must_use]
    pub fn tsdat(&mut self) -> TsdatW<TshrSpec> {
        TsdatW::new(self, 0)
    }
}
#[doc = "Transmit Sync. Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tshr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tshr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TshrSpec;
impl crate::RegisterSpec for TshrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tshr::R`](R) reader structure"]
impl crate::Readable for TshrSpec {}
#[doc = "`write(|w| ..)` method takes [`tshr::W`](W) writer structure"]
impl crate::Writable for TshrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSHR to value 0"]
impl crate::Resettable for TshrSpec {
    const RESET_VALUE: u32 = 0;
}
