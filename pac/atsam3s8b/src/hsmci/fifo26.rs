#[doc = "Register `FIFO26` reader"]
pub type R = crate::R<Fifo26Spec>;
#[doc = "Register `FIFO26` writer"]
pub type W = crate::W<Fifo26Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo26::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo26Spec;
impl crate::RegisterSpec for Fifo26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo26::R`](R) reader structure"]
impl crate::Readable for Fifo26Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo26::W`](W) writer structure"]
impl crate::Writable for Fifo26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
