#[doc = "Register `FIFO146` reader"]
pub type R = crate::R<FIFO146_SPEC>;
#[doc = "Register `FIFO146` writer"]
pub type W = crate::W<FIFO146_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<FIFO146_SPEC> {
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
#[doc = "FIFO Memory Aperture0 146\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo146::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo146::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO146_SPEC;
impl crate::RegisterSpec for FIFO146_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo146::R`](R) reader structure"]
impl crate::Readable for FIFO146_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo146::W`](W) writer structure"]
impl crate::Writable for FIFO146_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
