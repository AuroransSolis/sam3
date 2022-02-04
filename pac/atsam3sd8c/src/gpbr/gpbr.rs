#[doc = "Register `GPBR[%s]` reader"]
pub struct R(crate::R<GPBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPBR[%s]` writer"]
pub struct W(crate::W<GPBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPBR_SPEC>;
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
impl From<crate::W<GPBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPBR_VALUE` reader - Value of GPBR x"]
pub struct GPBR_VALUE_R(crate::FieldReader<u32, u32>);
impl GPBR_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GPBR_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPBR_VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPBR_VALUE` writer - Value of GPBR x"]
pub struct GPBR_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPBR_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value of GPBR x"]
    #[inline(always)]
    pub fn gpbr_value(&self) -> GPBR_VALUE_R {
        GPBR_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of GPBR x"]
    #[inline(always)]
    pub fn gpbr_value(&mut self) -> GPBR_VALUE_W {
        GPBR_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Backup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpbr](index.html) module"]
pub struct GPBR_SPEC;
impl crate::RegisterSpec for GPBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpbr::R](R) reader structure"]
impl crate::Readable for GPBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpbr::W](W) writer structure"]
impl crate::Writable for GPBR_SPEC {
    type Writer = W;
}
