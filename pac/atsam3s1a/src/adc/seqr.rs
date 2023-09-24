#[doc = "Register `SEQR[%s]` reader"]
pub type R = crate::R<SEQR_SPEC>;
#[doc = "Register `SEQR[%s]` writer"]
pub type W = crate::W<SEQR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SEQR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Sequence Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqr::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQR_SPEC;
impl crate::RegisterSpec for SEQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqr::R`](R) reader structure"]
impl crate::Readable for SEQR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seqr::W`](W) writer structure"]
impl crate::Writable for SEQR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
