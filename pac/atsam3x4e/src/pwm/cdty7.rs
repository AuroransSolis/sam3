#[doc = "Register `CDTY7` reader"]
pub struct R(crate::R<CDTY7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDTY7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDTY7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDTY7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDTY7` writer"]
pub struct W(crate::W<CDTY7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDTY7_SPEC>;
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
impl From<crate::W<CDTY7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDTY7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDTY` reader - Channel Duty-Cycle"]
pub type CDTY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CDTY` writer - Channel Duty-Cycle"]
pub type CDTY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CDTY7_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    pub fn cdty(&self) -> CDTY_R {
        CDTY_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    pub fn cdty(&mut self) -> CDTY_W<0> {
        CDTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdty7](index.html) module"]
pub struct CDTY7_SPEC;
impl crate::RegisterSpec for CDTY7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdty7::R](R) reader structure"]
impl crate::Readable for CDTY7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdty7::W](W) writer structure"]
impl crate::Writable for CDTY7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CDTY7 to value 0"]
impl crate::Resettable for CDTY7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
