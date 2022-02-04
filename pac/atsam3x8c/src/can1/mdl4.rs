#[doc = "Register `MDL4` reader"]
pub struct R(crate::R<MDL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDL4` writer"]
pub struct W(crate::W<MDL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDL4_SPEC>;
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
impl From<crate::W<MDL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDL` reader - Message Data Low Value"]
pub struct MDL_R(crate::FieldReader<u32, u32>);
impl MDL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDL` writer - Message Data Low Value"]
pub struct MDL_W<'a> {
    w: &'a mut W,
}
impl<'a> MDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Message Data Low Value"]
    #[inline(always)]
    pub fn mdl(&self) -> MDL_R {
        MDL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Data Low Value"]
    #[inline(always)]
    pub fn mdl(&mut self) -> MDL_W {
        MDL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Data Low Register (MB = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdl4](index.html) module"]
pub struct MDL4_SPEC;
impl crate::RegisterSpec for MDL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdl4::R](R) reader structure"]
impl crate::Readable for MDL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdl4::W](W) writer structure"]
impl crate::Writable for MDL4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDL4 to value 0"]
impl crate::Resettable for MDL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
