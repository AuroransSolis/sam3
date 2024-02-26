#[doc = "Register `SCUC` reader"]
pub type R = crate::R<ScucSpec>;
#[doc = "Register `SCUC` writer"]
pub type W = crate::W<ScucSpec>;
#[doc = "Field `UPDULOCK` reader - Synchronous Channels Update Unlock"]
pub type UpdulockR = crate::BitReader;
#[doc = "Field `UPDULOCK` writer - Synchronous Channels Update Unlock"]
pub type UpdulockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Synchronous Channels Update Unlock"]
    #[inline(always)]
    pub fn updulock(&self) -> UpdulockR {
        UpdulockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Channels Update Unlock"]
    #[inline(always)]
    #[must_use]
    pub fn updulock(&mut self) -> UpdulockW<ScucSpec> {
        UpdulockW::new(self, 0)
    }
}
#[doc = "PWM Sync Channels Update Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scuc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scuc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScucSpec;
impl crate::RegisterSpec for ScucSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuc::R`](R) reader structure"]
impl crate::Readable for ScucSpec {}
#[doc = "`write(|w| ..)` method takes [`scuc::W`](W) writer structure"]
impl crate::Writable for ScucSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCUC to value 0"]
impl crate::Resettable for ScucSpec {
    const RESET_VALUE: u32 = 0;
}
