#[doc = "Register `SEQR0` reader"]
pub type R = crate::R<Seqr0Spec>;
#[doc = "Register `SEQR0` writer"]
pub type W = crate::W<Seqr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<Seqr0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Channel Sequence Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqr0::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seqr0Spec;
impl crate::RegisterSpec for Seqr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqr0::R`](R) reader structure"]
impl crate::Readable for Seqr0Spec {}
#[doc = "`write(|w| ..)` method takes [`seqr0::W`](W) writer structure"]
impl crate::Writable for Seqr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
