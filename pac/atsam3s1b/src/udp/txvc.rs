#[doc = "Register `TXVC` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<TXVC_SPEC>);
#[doc = "Register `TXVC` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<TXVC_SPEC>);
#[doc = "Field `TXVDIS` reader - Transceiver Disable"]
pub type TXVDIS_R = crate::BitReader<bool>;
#[doc = "Field `TXVDIS` writer - Transceiver Disable"]
pub type TXVDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXVC_SPEC, bool, O>;
#[doc = "Field `PUON` reader - Pull-up On"]
pub type PUON_R = crate::BitReader<bool>;
#[doc = "Field `PUON` writer - Pull-up On"]
pub type PUON_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXVC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - Transceiver Disable"]
    #[inline(always)]
    pub fn txvdis(&self) -> TXVDIS_R {
        TXVDIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pull-up On"]
    #[inline(always)]
    pub fn puon(&self) -> PUON_R {
        PUON_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Transceiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txvdis(&mut self) -> TXVDIS_W<8> {
        TXVDIS_W::new(self)
    }
    #[doc = "Bit 9 - Pull-up On"]
    #[inline(always)]
    #[must_use]
    pub fn puon(&mut self) -> PUON_W<9> {
        PUON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transceiver Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txvc](index.html) module"]
pub struct TXVC_SPEC;
impl crate::RegisterSpec for TXVC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txvc::R](R) reader structure"]
impl crate::Readable for TXVC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txvc::W](W) writer structure"]
impl crate::Writable for TXVC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXVC to value 0x0100"]
impl crate::Resettable for TXVC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
