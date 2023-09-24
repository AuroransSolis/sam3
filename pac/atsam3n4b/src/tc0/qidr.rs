#[doc = "Register `QIDR` writer"]
pub type W = crate::W<QIDR_SPEC>;
#[doc = "Field `IDX` writer - Index"]
pub type IDX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIRCHG` writer - Direction Change"]
pub type DIRCHG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QERR` writer - Quadrature Error"]
pub type QERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Index"]
    #[inline(always)]
    #[must_use]
    pub fn idx(&mut self) -> IDX_W<QIDR_SPEC, 0> {
        IDX_W::new(self)
    }
    #[doc = "Bit 1 - Direction Change"]
    #[inline(always)]
    #[must_use]
    pub fn dirchg(&mut self) -> DIRCHG_W<QIDR_SPEC, 1> {
        DIRCHG_W::new(self)
    }
    #[doc = "Bit 2 - Quadrature Error"]
    #[inline(always)]
    #[must_use]
    pub fn qerr(&mut self) -> QERR_W<QIDR_SPEC, 2> {
        QERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "QDEC Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qidr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QIDR_SPEC;
impl crate::RegisterSpec for QIDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`qidr::W`](W) writer structure"]
impl crate::Writable for QIDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
