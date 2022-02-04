#[doc = "Register `SA2T` reader"]
pub struct R(crate::R<SA2T_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SA2T_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SA2T_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SA2T_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SA2T` writer"]
pub struct W(crate::W<SA2T_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SA2T_SPEC>;
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
impl From<crate::W<SA2T_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SA2T_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - "]
pub struct ADDR_R(crate::FieldReader<u16, u16>);
impl ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - "]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Specific Address 2 Top Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa2t](index.html) module"]
pub struct SA2T_SPEC;
impl crate::RegisterSpec for SA2T_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sa2t::R](R) reader structure"]
impl crate::Readable for SA2T_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sa2t::W](W) writer structure"]
impl crate::Writable for SA2T_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SA2T to value 0"]
impl crate::Resettable for SA2T_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
