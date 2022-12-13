#[doc = "Register `PCIER` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<PCIER_SPEC>);
#[doc = "Field `DRDY` writer - Parallel Capture Mode Data Ready Interrupt Enable"]
pub type DRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCIER_SPEC, bool, O>;
#[doc = "Field `OVRE` writer - Parallel Capture Mode Overrun Error Interrupt Enable"]
pub type OVRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCIER_SPEC, bool, O>;
#[doc = "Field `ENDRX` writer - End of Reception Transfer Interrupt Enable"]
pub type ENDRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCIER_SPEC, bool, O>;
#[doc = "Field `RXBUFF` writer - Reception Buffer Full Interrupt Enable"]
pub type RXBUFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCIER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Parallel Capture Mode Data Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DRDY_W<0> {
        DRDY_W::new(self)
    }
    #[doc = "Bit 1 - Parallel Capture Mode Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<1> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 2 - End of Reception Transfer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<2> {
        ENDRX_W::new(self)
    }
    #[doc = "Bit 3 - Reception Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<3> {
        RXBUFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel Capture Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcier](index.html) module"]
pub struct PCIER_SPEC;
impl crate::RegisterSpec for PCIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pcier::W](W) writer structure"]
impl crate::Writable for PCIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
