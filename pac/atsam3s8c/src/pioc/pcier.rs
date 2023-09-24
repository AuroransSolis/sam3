#[doc = "Register `PCIER` writer"]
pub type W = crate::W<PCIER_SPEC>;
#[doc = "Field `DRDY` writer - Parallel Capture Mode Data Ready Interrupt Enable"]
pub type DRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVRE` writer - Parallel Capture Mode Overrun Error Interrupt Enable"]
pub type OVRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENDRX` writer - End of Reception Transfer Interrupt Enable"]
pub type ENDRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBUFF` writer - Reception Buffer Full Interrupt Enable"]
pub type RXBUFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Parallel Capture Mode Data Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DRDY_W<PCIER_SPEC, 0> {
        DRDY_W::new(self)
    }
    #[doc = "Bit 1 - Parallel Capture Mode Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<PCIER_SPEC, 1> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 2 - End of Reception Transfer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<PCIER_SPEC, 2> {
        ENDRX_W::new(self)
    }
    #[doc = "Bit 3 - Reception Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<PCIER_SPEC, 3> {
        RXBUFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Parallel Capture Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCIER_SPEC;
impl crate::RegisterSpec for PCIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pcier::W`](W) writer structure"]
impl crate::Writable for PCIER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
