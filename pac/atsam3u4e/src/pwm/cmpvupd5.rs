#[doc = "Register `CMPVUPD5` writer"]
pub struct W(crate::W<CMPVUPD5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPVUPD5_SPEC>;
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
impl From<crate::W<CMPVUPD5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPVUPD5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVUPD` writer - Comparison x Value Update"]
pub type CVUPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPVUPD5_SPEC, u32, u32, 24, O>;
#[doc = "Field `CVMUPD` writer - Comparison x Value Mode Update"]
pub type CVMUPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPVUPD5_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:23 - Comparison x Value Update"]
    #[inline(always)]
    pub fn cvupd(&mut self) -> CVUPD_W<0> {
        CVUPD_W::new(self)
    }
    #[doc = "Bit 24 - Comparison x Value Mode Update"]
    #[inline(always)]
    pub fn cvmupd(&mut self) -> CVMUPD_W<24> {
        CVMUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Comparison 5 Value Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpvupd5](index.html) module"]
pub struct CMPVUPD5_SPEC;
impl crate::RegisterSpec for CMPVUPD5_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmpvupd5::W](W) writer structure"]
impl crate::Writable for CMPVUPD5_SPEC {
    type Writer = W;
}
