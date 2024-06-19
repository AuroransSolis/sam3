#[doc = "Register `FIFO107` reader"]
pub type R = crate::R<Fifo107Spec>;
#[doc = "Register `FIFO107` writer"]
pub type W = crate::W<Fifo107Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 107\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo107::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo107::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo107Spec;
impl crate::RegisterSpec for Fifo107Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo107::R`](R) reader structure"]
impl crate::Readable for Fifo107Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo107::W`](W) writer structure"]
impl crate::Writable for Fifo107Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
