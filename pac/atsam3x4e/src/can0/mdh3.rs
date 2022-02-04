#[doc = "Register `MDH3` reader"]
pub struct R(crate::R<MDH3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDH3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDH3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDH3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDH3` writer"]
pub struct W(crate::W<MDH3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDH3_SPEC>;
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
impl From<crate::W<MDH3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDH3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDH` reader - Message Data High Value"]
pub struct MDH_R(crate::FieldReader<u32, u32>);
impl MDH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MDH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDH` writer - Message Data High Value"]
pub struct MDH_W<'a> {
    w: &'a mut W,
}
impl<'a> MDH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Message Data High Value"]
    #[inline(always)]
    pub fn mdh(&self) -> MDH_R {
        MDH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Data High Value"]
    #[inline(always)]
    pub fn mdh(&mut self) -> MDH_W {
        MDH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Data High Register (MB = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdh3](index.html) module"]
pub struct MDH3_SPEC;
impl crate::RegisterSpec for MDH3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdh3::R](R) reader structure"]
impl crate::Readable for MDH3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdh3::W](W) writer structure"]
impl crate::Writable for MDH3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDH3 to value 0"]
impl crate::Resettable for MDH3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
