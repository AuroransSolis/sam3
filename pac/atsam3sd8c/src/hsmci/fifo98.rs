#[doc = "Register `FIFO98` reader"]
pub type R = crate::R<Fifo98Spec>;
#[doc = "Register `FIFO98` writer"]
pub type W = crate::W<Fifo98Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 98\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo98::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo98::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo98Spec;
impl crate::RegisterSpec for Fifo98Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo98::R`](R) reader structure"]
impl crate::Readable for Fifo98Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo98::W`](W) writer structure"]
impl crate::Writable for Fifo98Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
