#[doc = "Register `CPRD6` reader"]
pub struct R(crate::R<CPRD6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPRD6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPRD6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPRD6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPRD6` writer"]
pub struct W(crate::W<CPRD6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPRD6_SPEC>;
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
impl From<crate::W<CPRD6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPRD6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPRD` reader - Channel Period"]
pub struct CPRD_R(crate::FieldReader<u32, u32>);
impl CPRD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CPRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPRD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPRD` writer - Channel Period"]
pub struct CPRD_W<'a> {
    w: &'a mut W,
}
impl<'a> CPRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&self) -> CPRD_R {
        CPRD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&mut self) -> CPRD_W {
        CPRD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Period Register (ch_num = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprd6](index.html) module"]
pub struct CPRD6_SPEC;
impl crate::RegisterSpec for CPRD6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cprd6::R](R) reader structure"]
impl crate::Readable for CPRD6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cprd6::W](W) writer structure"]
impl crate::Writable for CPRD6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPRD6 to value 0"]
impl crate::Resettable for CPRD6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
