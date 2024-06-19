#[doc = "Register `FIFO106` reader"]
pub type R = crate::R<Fifo106Spec>;
#[doc = "Register `FIFO106` writer"]
pub type W = crate::W<Fifo106Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 106\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo106::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo106::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo106Spec;
impl crate::RegisterSpec for Fifo106Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo106::R`](R) reader structure"]
impl crate::Readable for Fifo106Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo106::W`](W) writer structure"]
impl crate::Writable for Fifo106Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
