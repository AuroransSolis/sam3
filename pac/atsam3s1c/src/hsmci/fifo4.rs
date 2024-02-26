#[doc = "Register `FIFO4` reader"]
pub type R = crate::R<Fifo4Spec>;
#[doc = "Register `FIFO4` writer"]
pub type W = crate::W<Fifo4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<Fifo4Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo4::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo4Spec;
impl crate::RegisterSpec for Fifo4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo4::R`](R) reader structure"]
impl crate::Readable for Fifo4Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo4::W`](W) writer structure"]
impl crate::Writable for Fifo4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
