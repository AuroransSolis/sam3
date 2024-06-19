#[doc = "Register `SEQR1` reader"]
pub type R = crate::R<Seqr1Spec>;
#[doc = "Register `SEQR1` writer"]
pub type W = crate::W<Seqr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel Sequence Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`seqr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seqr1Spec;
impl crate::RegisterSpec for Seqr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqr1::R`](R) reader structure"]
impl crate::Readable for Seqr1Spec {}
#[doc = "`write(|w| ..)` method takes [`seqr1::W`](W) writer structure"]
impl crate::Writable for Seqr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
