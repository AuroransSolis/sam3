#[doc = "Register `FIFO246` reader"]
pub type R = crate::R<Fifo246Spec>;
#[doc = "Register `FIFO246` writer"]
pub type W = crate::W<Fifo246Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<Fifo246Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 246\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo246::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo246::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo246Spec;
impl crate::RegisterSpec for Fifo246Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo246::R`](R) reader structure"]
impl crate::Readable for Fifo246Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo246::W`](W) writer structure"]
impl crate::Writable for Fifo246Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
