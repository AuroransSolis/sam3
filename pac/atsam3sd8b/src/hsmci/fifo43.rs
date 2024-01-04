#[doc = "Register `FIFO43` reader"]
pub type R = crate::R<FIFO43_SPEC>;
#[doc = "Register `FIFO43` writer"]
pub type W = crate::W<FIFO43_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<FIFO43_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FIFO Memory Aperture0 43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo43::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo43::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO43_SPEC;
impl crate::RegisterSpec for FIFO43_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo43::R`](R) reader structure"]
impl crate::Readable for FIFO43_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo43::W`](W) writer structure"]
impl crate::Writable for FIFO43_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
