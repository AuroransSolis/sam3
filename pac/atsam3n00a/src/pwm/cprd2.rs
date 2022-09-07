#[doc = "Register `CPRD2` reader"]
pub struct R(crate::R<CPRD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPRD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPRD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPRD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPRD2` writer"]
pub struct W(crate::W<CPRD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPRD2_SPEC>;
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
impl From<crate::W<CPRD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPRD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPRD` reader - Channel Period"]
pub type CPRD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CPRD` writer - Channel Period"]
pub type CPRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPRD2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&self) -> CPRD_R {
        CPRD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&mut self) -> CPRD_W<0> {
        CPRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Period Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprd2](index.html) module"]
pub struct CPRD2_SPEC;
impl crate::RegisterSpec for CPRD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cprd2::R](R) reader structure"]
impl crate::Readable for CPRD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cprd2::W](W) writer structure"]
impl crate::Writable for CPRD2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPRD2 to value 0"]
impl crate::Resettable for CPRD2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
