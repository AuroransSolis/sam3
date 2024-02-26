#[doc = "Register `FIFO104` reader"]
pub type R = crate::R<Fifo104Spec>;
#[doc = "Register `FIFO104` writer"]
pub type W = crate::W<Fifo104Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<Fifo104Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 104\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo104::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo104::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo104Spec;
impl crate::RegisterSpec for Fifo104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo104::R`](R) reader structure"]
impl crate::Readable for Fifo104Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo104::W`](W) writer structure"]
impl crate::Writable for Fifo104Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
