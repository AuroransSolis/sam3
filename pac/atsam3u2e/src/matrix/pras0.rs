#[doc = "Register `PRAS0` reader"]
pub struct R(crate::R<PRAS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRAS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRAS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRAS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRAS0` writer"]
pub struct W(crate::W<PRAS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRAS0_SPEC>;
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
impl From<crate::W<PRAS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRAS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M0PR` reader - Master 0 Priority"]
pub struct M0PR_R(crate::FieldReader<u8, u8>);
impl M0PR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        M0PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M0PR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M0PR` writer - Master 0 Priority"]
pub struct M0PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M0PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `M1PR` reader - Master 1 Priority"]
pub struct M1PR_R(crate::FieldReader<u8, u8>);
impl M1PR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        M1PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M1PR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M1PR` writer - Master 1 Priority"]
pub struct M1PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M1PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `M2PR` reader - Master 2 Priority"]
pub struct M2PR_R(crate::FieldReader<u8, u8>);
impl M2PR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        M2PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M2PR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M2PR` writer - Master 2 Priority"]
pub struct M2PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M2PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `M3PR` reader - Master 3 Priority"]
pub struct M3PR_R(crate::FieldReader<u8, u8>);
impl M3PR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        M3PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M3PR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M3PR` writer - Master 3 Priority"]
pub struct M3PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M3PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `M4PR` reader - Master 4 Priority"]
pub struct M4PR_R(crate::FieldReader<u8, u8>);
impl M4PR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        M4PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4PR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4PR` writer - Master 4 Priority"]
pub struct M4PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M4PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline(always)]
    pub fn m0pr(&self) -> M0PR_R {
        M0PR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Master 1 Priority"]
    #[inline(always)]
    pub fn m1pr(&self) -> M1PR_R {
        M1PR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Master 2 Priority"]
    #[inline(always)]
    pub fn m2pr(&self) -> M2PR_R {
        M2PR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Master 3 Priority"]
    #[inline(always)]
    pub fn m3pr(&self) -> M3PR_R {
        M3PR_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Master 4 Priority"]
    #[inline(always)]
    pub fn m4pr(&self) -> M4PR_R {
        M4PR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline(always)]
    pub fn m0pr(&mut self) -> M0PR_W {
        M0PR_W { w: self }
    }
    #[doc = "Bits 4:5 - Master 1 Priority"]
    #[inline(always)]
    pub fn m1pr(&mut self) -> M1PR_W {
        M1PR_W { w: self }
    }
    #[doc = "Bits 8:9 - Master 2 Priority"]
    #[inline(always)]
    pub fn m2pr(&mut self) -> M2PR_W {
        M2PR_W { w: self }
    }
    #[doc = "Bits 12:13 - Master 3 Priority"]
    #[inline(always)]
    pub fn m3pr(&mut self) -> M3PR_W {
        M3PR_W { w: self }
    }
    #[doc = "Bits 16:17 - Master 4 Priority"]
    #[inline(always)]
    pub fn m4pr(&mut self) -> M4PR_W {
        M4PR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Priority Register A for Slave 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras0](index.html) module"]
pub struct PRAS0_SPEC;
impl crate::RegisterSpec for PRAS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pras0::R](R) reader structure"]
impl crate::Readable for PRAS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pras0::W](W) writer structure"]
impl crate::Writable for PRAS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRAS0 to value 0"]
impl crate::Resettable for PRAS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
