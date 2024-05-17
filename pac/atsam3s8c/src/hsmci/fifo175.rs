#[doc = "Register `FIFO175` reader"]
pub type R = crate::R<Fifo175Spec>;
#[doc = "Register `FIFO175` writer"]
pub type W = crate::W<Fifo175Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 175\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo175::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo175::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo175Spec;
impl crate::RegisterSpec for Fifo175Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo175::R`](R) reader structure"]
impl crate::Readable for Fifo175Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo175::W`](W) writer structure"]
impl crate::Writable for Fifo175Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
