#[doc = "Register `DADDR1` reader"]
pub struct R(crate::R<DADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DADDR1` writer"]
pub struct W(crate::W<DADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DADDR1_SPEC>;
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
impl From<crate::W<DADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DADDR` reader - Channel x Destination Address"]
pub type DADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DADDR` writer - Channel x Destination Address"]
pub type DADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DADDR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    pub fn daddr(&self) -> DADDR_R {
        DADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    pub fn daddr(&mut self) -> DADDR_W<0> {
        DADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Destination Address Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daddr1](index.html) module"]
pub struct DADDR1_SPEC;
impl crate::RegisterSpec for DADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daddr1::R](R) reader structure"]
impl crate::Readable for DADDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daddr1::W](W) writer structure"]
impl crate::Writable for DADDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DADDR1 to value 0"]
impl crate::Resettable for DADDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
