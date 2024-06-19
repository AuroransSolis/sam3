#[doc = "Register `FIFO75` reader"]
pub type R = crate::R<Fifo75Spec>;
#[doc = "Register `FIFO75` writer"]
pub type W = crate::W<Fifo75Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 75\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo75::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo75::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo75Spec;
impl crate::RegisterSpec for Fifo75Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo75::R`](R) reader structure"]
impl crate::Readable for Fifo75Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo75::W`](W) writer structure"]
impl crate::Writable for Fifo75Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
