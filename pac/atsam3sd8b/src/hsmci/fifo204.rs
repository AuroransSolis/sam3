#[doc = "Register `FIFO204` reader"]
pub type R = crate::R<Fifo204Spec>;
#[doc = "Register `FIFO204` writer"]
pub type W = crate::W<Fifo204Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 204\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo204::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo204::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo204Spec;
impl crate::RegisterSpec for Fifo204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo204::R`](R) reader structure"]
impl crate::Readable for Fifo204Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo204::W`](W) writer structure"]
impl crate::Writable for Fifo204Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
