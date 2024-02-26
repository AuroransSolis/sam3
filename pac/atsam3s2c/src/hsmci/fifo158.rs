#[doc = "Register `FIFO158` reader"]
pub type R = crate::R<Fifo158Spec>;
#[doc = "Register `FIFO158` writer"]
pub type W = crate::W<Fifo158Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<Fifo158Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 158\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo158::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo158::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo158Spec;
impl crate::RegisterSpec for Fifo158Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo158::R`](R) reader structure"]
impl crate::Readable for Fifo158Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo158::W`](W) writer structure"]
impl crate::Writable for Fifo158Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
