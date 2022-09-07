#[doc = "Register `TXVC` reader"]
pub struct R(crate::R<TXVC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXVC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXVC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXVC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXVC` writer"]
pub struct W(crate::W<TXVC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXVC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TXVC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXVC_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn txvdis(&mut self) -> TXVDIS_W<8> {
        TXVDIS_W::new(self)
    }
    #[doc = "Bit 9 - Pull-up On"]
    #[inline(always)]
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
}
#[doc = "`reset()` method sets TXVC to value 0x0100"]
impl crate::Resettable for TXVC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
