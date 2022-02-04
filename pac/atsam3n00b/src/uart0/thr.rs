#[doc = "Register `THR` writer"]
pub struct W(crate::W<THR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THR_SPEC>;
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
impl From<crate::W<THR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXCHR` writer - Character to be Transmitted"]
pub struct TXCHR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Character to be Transmitted"]
    #[inline(always)]
    pub fn txchr(&mut self) -> TXCHR_W {
        TXCHR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Holding Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr](index.html) module"]
pub struct THR_SPEC;
impl crate::RegisterSpec for THR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [thr::W](W) writer structure"]
impl crate::Writable for THR_SPEC {
    type Writer = W;
}
