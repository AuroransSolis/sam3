#[doc = "Register `FIFO205` reader"]
pub type R = crate::R<Fifo205Spec>;
#[doc = "Register `FIFO205` writer"]
pub type W = crate::W<Fifo205Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<Fifo205Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "FIFO Memory Aperture0 205\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo205::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo205::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo205Spec;
impl crate::RegisterSpec for Fifo205Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo205::R`](R) reader structure"]
impl crate::Readable for Fifo205Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo205::W`](W) writer structure"]
impl crate::Writable for Fifo205Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
