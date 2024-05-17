#[doc = "Register `FIFO35` reader"]
pub type R = crate::R<Fifo35Spec>;
#[doc = "Register `FIFO35` writer"]
pub type W = crate::W<Fifo35Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo35::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo35Spec;
impl crate::RegisterSpec for Fifo35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo35::R`](R) reader structure"]
impl crate::Readable for Fifo35Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo35::W`](W) writer structure"]
impl crate::Writable for Fifo35Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
