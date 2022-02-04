#[doc = "Register `CPRDUPD2` writer"]
pub struct W(crate::W<CPRDUPD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPRDUPD2_SPEC>;
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
impl From<crate::W<CPRDUPD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPRDUPD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPRDUPD` writer - Channel Period Update"]
pub struct CPRDUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CPRDUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Period Update"]
    #[inline(always)]
    pub fn cprdupd(&mut self) -> CPRDUPD_W {
        CPRDUPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Period Update Register (ch_num = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprdupd2](index.html) module"]
pub struct CPRDUPD2_SPEC;
impl crate::RegisterSpec for CPRDUPD2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cprdupd2::W](W) writer structure"]
impl crate::Writable for CPRDUPD2_SPEC {
    type Writer = W;
}
