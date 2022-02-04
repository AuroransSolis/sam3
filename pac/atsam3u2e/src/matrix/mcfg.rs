#[doc = "Register `MCFG[%s]` reader"]
pub struct R(crate::R<MCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCFG[%s]` writer"]
pub struct W(crate::W<MCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCFG_SPEC>;
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
impl From<crate::W<MCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub struct ULBT_R(crate::FieldReader<u8, u8>);
impl ULBT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ULBT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULBT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub struct ULBT_W<'a> {
    w: &'a mut W,
}
impl<'a> ULBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> ULBT_R {
        ULBT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&mut self) -> ULBT_W {
        ULBT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfg](index.html) module"]
pub struct MCFG_SPEC;
impl crate::RegisterSpec for MCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcfg::R](R) reader structure"]
impl crate::Readable for MCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcfg::W](W) writer structure"]
impl crate::Writable for MCFG_SPEC {
    type Writer = W;
}
