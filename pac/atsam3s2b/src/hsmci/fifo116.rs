#[doc = "Register `FIFO116` reader"]
pub type R = crate::R<Fifo116Spec>;
#[doc = "Register `FIFO116` writer"]
pub type W = crate::W<Fifo116Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<Fifo116Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 116\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo116::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo116::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo116Spec;
impl crate::RegisterSpec for Fifo116Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo116::R`](R) reader structure"]
impl crate::Readable for Fifo116Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo116::W`](W) writer structure"]
impl crate::Writable for Fifo116Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
