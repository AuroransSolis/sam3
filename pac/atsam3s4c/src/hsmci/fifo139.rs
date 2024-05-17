#[doc = "Register `FIFO139` reader"]
pub type R = crate::R<Fifo139Spec>;
#[doc = "Register `FIFO139` writer"]
pub type W = crate::W<Fifo139Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 139\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo139::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo139::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo139Spec;
impl crate::RegisterSpec for Fifo139Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo139::R`](R) reader structure"]
impl crate::Readable for Fifo139Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo139::W`](W) writer structure"]
impl crate::Writable for Fifo139Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
