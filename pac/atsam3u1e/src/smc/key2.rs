#[doc = "Register `KEY2` writer"]
pub type W = crate::W<Key2Spec>;
#[doc = "Field `KEY2` writer - Off Chip Memory Scrambling (OCMS) Key Part 2"]
pub type Key2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Off Chip Memory Scrambling (OCMS) Key Part 2"]
    #[inline(always)]
    #[must_use]
    pub fn key2(&mut self) -> Key2W<Key2Spec> {
        Key2W::new(self, 0)
    }
}
#[doc = "SMC OCMS KEY2 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key2Spec;
impl crate::RegisterSpec for Key2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key2::W`](W) writer structure"]
impl crate::Writable for Key2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY2 to value 0"]
impl crate::Resettable for Key2Spec {
    const RESET_VALUE: u32 = 0;
}
