#[doc = "Register `CDTYUPD3` writer"]
pub struct W(crate::W<CDTYUPD3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDTYUPD3_SPEC>;
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
impl From<crate::W<CDTYUPD3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDTYUPD3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDTYUPD` writer - Channel Duty-Cycle Update"]
pub struct CDTYUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTYUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle Update"]
    #[inline(always)]
    pub fn cdtyupd(&mut self) -> CDTYUPD_W {
        CDTYUPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 3)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdtyupd3](index.html) module"]
pub struct CDTYUPD3_SPEC;
impl crate::RegisterSpec for CDTYUPD3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cdtyupd3::W](W) writer structure"]
impl crate::Writable for CDTYUPD3_SPEC {
    type Writer = W;
}
