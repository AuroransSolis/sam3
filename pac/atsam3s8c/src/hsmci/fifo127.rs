#[doc = "Register `FIFO127` reader"]
pub type R = crate::R<Fifo127Spec>;
#[doc = "Register `FIFO127` writer"]
pub type W = crate::W<Fifo127Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 127\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo127::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo127::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo127Spec;
impl crate::RegisterSpec for Fifo127Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo127::R`](R) reader structure"]
impl crate::Readable for Fifo127Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo127::W`](W) writer structure"]
impl crate::Writable for Fifo127Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
