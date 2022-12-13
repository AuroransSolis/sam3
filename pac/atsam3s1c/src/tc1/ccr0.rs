#[doc = "Register `CCR0` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CCR0_SPEC>);
#[doc = "Field `CLKEN` writer - Counter Clock Enable Command"]
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR0_SPEC, bool, O>;
#[doc = "Field `CLKDIS` writer - Counter Clock Disable Command"]
pub type CLKDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR0_SPEC, bool, O>;
#[doc = "Field `SWTRG` writer - Software Trigger Command"]
pub type SWTRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR0_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Counter Clock Enable Command"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<0> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 1 - Counter Clock Disable Command"]
    #[inline(always)]
    #[must_use]
    pub fn clkdis(&mut self) -> CLKDIS_W<1> {
        CLKDIS_W::new(self)
    }
    #[doc = "Bit 2 - Software Trigger Command"]
    #[inline(always)]
    #[must_use]
    pub fn swtrg(&mut self) -> SWTRG_W<2> {
        SWTRG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Control Register (channel = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr0](index.html) module"]
pub struct CCR0_SPEC;
impl crate::RegisterSpec for CCR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ccr0::W](W) writer structure"]
impl crate::Writable for CCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
