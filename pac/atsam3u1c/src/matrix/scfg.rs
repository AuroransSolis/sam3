#[doc = "Register `SCFG[%s]` reader"]
pub struct R(crate::R<SCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCFG[%s]` writer"]
pub struct W(crate::W<SCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCFG_SPEC>;
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
impl From<crate::W<SCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOT_CYCLE` reader - Maximum Number of Allowed Cycles for a Burst"]
pub struct SLOT_CYCLE_R(crate::FieldReader<u8, u8>);
impl SLOT_CYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLOT_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOT_CYCLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOT_CYCLE` writer - Maximum Number of Allowed Cycles for a Burst"]
pub struct SLOT_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOT_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DEFMSTR_TYPE` reader - Default Master Type"]
pub struct DEFMSTR_TYPE_R(crate::FieldReader<u8, u8>);
impl DEFMSTR_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEFMSTR_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEFMSTR_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEFMSTR_TYPE` writer - Default Master Type"]
pub struct DEFMSTR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEFMSTR_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `FIXED_DEFMSTR` reader - Fixed Default Master"]
pub struct FIXED_DEFMSTR_R(crate::FieldReader<u8, u8>);
impl FIXED_DEFMSTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIXED_DEFMSTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIXED_DEFMSTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIXED_DEFMSTR` writer - Fixed Default Master"]
pub struct FIXED_DEFMSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXED_DEFMSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `ARBT` reader - Arbitration Type"]
pub struct ARBT_R(crate::FieldReader<u8, u8>);
impl ARBT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ARBT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARBT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBT` writer - Arbitration Type"]
pub struct ARBT_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    pub fn slot_cycle(&self) -> SLOT_CYCLE_R {
        SLOT_CYCLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&self) -> DEFMSTR_TYPE_R {
        DEFMSTR_TYPE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&self) -> FIXED_DEFMSTR_R {
        FIXED_DEFMSTR_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - Arbitration Type"]
    #[inline(always)]
    pub fn arbt(&self) -> ARBT_R {
        ARBT_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    pub fn slot_cycle(&mut self) -> SLOT_CYCLE_W {
        SLOT_CYCLE_W { w: self }
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&mut self) -> DEFMSTR_TYPE_W {
        DEFMSTR_TYPE_W { w: self }
    }
    #[doc = "Bits 18:20 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&mut self) -> FIXED_DEFMSTR_W {
        FIXED_DEFMSTR_W { w: self }
    }
    #[doc = "Bits 24:25 - Arbitration Type"]
    #[inline(always)]
    pub fn arbt(&mut self) -> ARBT_W {
        ARBT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfg](index.html) module"]
pub struct SCFG_SPEC;
impl crate::RegisterSpec for SCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scfg::R](R) reader structure"]
impl crate::Readable for SCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scfg::W](W) writer structure"]
impl crate::Writable for SCFG_SPEC {
    type Writer = W;
}
