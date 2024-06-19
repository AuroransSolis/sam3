#[doc = "Register `FIFO237` reader"]
pub type R = crate::R<Fifo237Spec>;
#[doc = "Register `FIFO237` writer"]
pub type W = crate::W<Fifo237Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 237\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo237::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo237::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo237Spec;
impl crate::RegisterSpec for Fifo237Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo237::R`](R) reader structure"]
impl crate::Readable for Fifo237Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo237::W`](W) writer structure"]
impl crate::Writable for Fifo237Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
