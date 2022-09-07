#[doc = "Register `CMPMUPD0` writer"]
pub struct W(crate::W<CMPMUPD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPMUPD0_SPEC>;
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
impl From<crate::W<CMPMUPD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPMUPD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CENUPD` writer - Comparison x Enable Update"]
pub type CENUPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPMUPD0_SPEC, bool, O>;
#[doc = "Field `CTRUPD` writer - Comparison x Trigger Update"]
pub type CTRUPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPMUPD0_SPEC, u8, u8, 4, O>;
#[doc = "Field `CPRUPD` writer - Comparison x Period Update"]
pub type CPRUPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPMUPD0_SPEC, u8, u8, 4, O>;
#[doc = "Field `CUPRUPD` writer - Comparison x Update Period Update"]
pub type CUPRUPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPMUPD0_SPEC, u8, u8, 4, O>;
impl W {
    #[doc = "Bit 0 - Comparison x Enable Update"]
    #[inline(always)]
    pub fn cenupd(&mut self) -> CENUPD_W<0> {
        CENUPD_W::new(self)
    }
    #[doc = "Bits 4:7 - Comparison x Trigger Update"]
    #[inline(always)]
    pub fn ctrupd(&mut self) -> CTRUPD_W<4> {
        CTRUPD_W::new(self)
    }
    #[doc = "Bits 8:11 - Comparison x Period Update"]
    #[inline(always)]
    pub fn cprupd(&mut self) -> CPRUPD_W<8> {
        CPRUPD_W::new(self)
    }
    #[doc = "Bits 16:19 - Comparison x Update Period Update"]
    #[inline(always)]
    pub fn cuprupd(&mut self) -> CUPRUPD_W<16> {
        CUPRUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Comparison 0 Mode Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpmupd0](index.html) module"]
pub struct CMPMUPD0_SPEC;
impl crate::RegisterSpec for CMPMUPD0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmpmupd0::W](W) writer structure"]
impl crate::Writable for CMPMUPD0_SPEC {
    type Writer = W;
}
