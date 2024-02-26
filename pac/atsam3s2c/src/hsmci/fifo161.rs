#[doc = "Register `FIFO161` reader"]
pub type R = crate::R<Fifo161Spec>;
#[doc = "Register `FIFO161` writer"]
pub type W = crate::W<Fifo161Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<Fifo161Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 161\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo161::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo161::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo161Spec;
impl crate::RegisterSpec for Fifo161Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo161::R`](R) reader structure"]
impl crate::Readable for Fifo161Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo161::W`](W) writer structure"]
impl crate::Writable for Fifo161Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
