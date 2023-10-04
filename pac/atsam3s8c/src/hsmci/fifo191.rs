#[doc = "Register `FIFO191` reader"]
pub type R = crate::R<FIFO191_SPEC>;
#[doc = "Register `FIFO191` writer"]
pub type W = crate::W<FIFO191_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<FIFO191_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
#[doc = "FIFO Memory Aperture0 191\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo191::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo191::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO191_SPEC;
impl crate::RegisterSpec for FIFO191_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo191::R`](R) reader structure"]
impl crate::Readable for FIFO191_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo191::W`](W) writer structure"]
impl crate::Writable for FIFO191_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
