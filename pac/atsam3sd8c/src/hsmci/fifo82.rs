#[doc = "Register `FIFO82` reader"]
pub type R = crate::R<Fifo82Spec>;
#[doc = "Register `FIFO82` writer"]
pub type W = crate::W<Fifo82Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<Fifo82Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 82\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo82::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo82::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo82Spec;
impl crate::RegisterSpec for Fifo82Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo82::R`](R) reader structure"]
impl crate::Readable for Fifo82Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo82::W`](W) writer structure"]
impl crate::Writable for Fifo82Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
