#[doc = "Register `TTGR` reader"]
pub type R = crate::R<TtgrSpec>;
#[doc = "Register `TTGR` writer"]
pub type W = crate::W<TtgrSpec>;
#[doc = "Field `TG` reader - Timeguard Value"]
pub type TgR = crate::FieldReader;
#[doc = "Field `TG` writer - Timeguard Value"]
pub type TgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeguard Value"]
    #[inline(always)]
    pub fn tg(&self) -> TgR {
        TgR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeguard Value"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TgW<TtgrSpec> {
        TgW::new(self, 0)
    }
}
#[doc = "Transmitter Timeguard Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ttgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TtgrSpec;
impl crate::RegisterSpec for TtgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ttgr::R`](R) reader structure"]
impl crate::Readable for TtgrSpec {}
#[doc = "`write(|w| ..)` method takes [`ttgr::W`](W) writer structure"]
impl crate::Writable for TtgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TTGR to value 0"]
impl crate::Resettable for TtgrSpec {
    const RESET_VALUE: u32 = 0;
}
