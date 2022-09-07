#[doc = "Register `SADDR2` reader"]
pub struct R(crate::R<SADDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SADDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SADDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SADDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SADDR2` writer"]
pub struct W(crate::W<SADDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SADDR2_SPEC>;
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
impl From<crate::W<SADDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SADDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADDR` reader - Channel x Source Address"]
pub type SADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SADDR` writer - Channel x Source Address"]
pub type SADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SADDR2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn saddr(&mut self) -> SADDR_W<0> {
        SADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Source Address Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saddr2](index.html) module"]
pub struct SADDR2_SPEC;
impl crate::RegisterSpec for SADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saddr2::R](R) reader structure"]
impl crate::Readable for SADDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saddr2::W](W) writer structure"]
impl crate::Writable for SADDR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SADDR2 to value 0"]
impl crate::Resettable for SADDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
