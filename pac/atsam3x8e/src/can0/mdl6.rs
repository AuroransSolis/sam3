#[doc = "Register `MDL6` reader"]
pub struct R(crate::R<MDL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDL6` writer"]
pub struct W(crate::W<MDL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDL6_SPEC>;
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
impl From<crate::W<MDL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDL` reader - Message Data Low Value"]
pub type MDL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MDL` writer - Message Data Low Value"]
pub type MDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDL6_SPEC, u32, u32, 32, O>;
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
    pub fn mdl(&mut self) -> MDL_W<0> {
        MDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Data Low Register (MB = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdl6](index.html) module"]
pub struct MDL6_SPEC;
impl crate::RegisterSpec for MDL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdl6::R](R) reader structure"]
impl crate::Readable for MDL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdl6::W](W) writer structure"]
impl crate::Writable for MDL6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDL6 to value 0"]
impl crate::Resettable for MDL6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
