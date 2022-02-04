#[doc = "Register `BANK` reader"]
pub struct R(crate::R<BANK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BANK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BANK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BANK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BANK` writer"]
pub struct W(crate::W<BANK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BANK_SPEC>;
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
impl From<crate::W<BANK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BANK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BANK` reader - Bank Identifier"]
pub struct BANK_R(crate::FieldReader<u8, u8>);
impl BANK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANK` writer - Bank Identifier"]
pub struct BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Bank Identifier"]
    #[inline(always)]
    pub fn bank(&self) -> BANK_R {
        BANK_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bank Identifier"]
    #[inline(always)]
    pub fn bank(&mut self) -> BANK_W {
        BANK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Bank Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bank](index.html) module"]
pub struct BANK_SPEC;
impl crate::RegisterSpec for BANK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bank::R](R) reader structure"]
impl crate::Readable for BANK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bank::W](W) writer structure"]
impl crate::Writable for BANK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BANK to value 0"]
impl crate::Resettable for BANK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
