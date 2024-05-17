#[doc = "Register `FIFO5` reader"]
pub type R = crate::R<Fifo5Spec>;
#[doc = "Register `FIFO5` writer"]
pub type W = crate::W<Fifo5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo5::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo5Spec;
impl crate::RegisterSpec for Fifo5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo5::R`](R) reader structure"]
impl crate::Readable for Fifo5Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo5::W`](W) writer structure"]
impl crate::Writable for Fifo5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
