#[doc = "Register `SCF` reader"]
pub struct R(crate::R<SCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCF` writer"]
pub struct W(crate::W<SCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCF_SPEC>;
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
impl From<crate::W<SCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCF` reader - Single Collision Frames"]
pub type SCF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SCF` writer - Single Collision Frames"]
pub type SCF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCF_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Single Collision Frames"]
    #[inline(always)]
    pub fn scf(&self) -> SCF_R {
        SCF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Single Collision Frames"]
    #[inline(always)]
    pub fn scf(&mut self) -> SCF_W<0> {
        SCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Single Collision Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scf](index.html) module"]
pub struct SCF_SPEC;
impl crate::RegisterSpec for SCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scf::R](R) reader structure"]
impl crate::Readable for SCF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scf::W](W) writer structure"]
impl crate::Writable for SCF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCF to value 0"]
impl crate::Resettable for SCF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
