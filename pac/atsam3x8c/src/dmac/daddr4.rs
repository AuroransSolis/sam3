#[doc = "Register `DADDR4` reader"]
pub struct R(crate::R<DADDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DADDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DADDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DADDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DADDR4` writer"]
pub struct W(crate::W<DADDR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DADDR4_SPEC>;
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
impl From<crate::W<DADDR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DADDR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DADDR` reader - Channel x Destination Address"]
pub type DADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DADDR` writer - Channel x Destination Address"]
pub type DADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DADDR4_SPEC, u32, u32, 32, O>;
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
#[doc = "DMAC Channel Destination Address Register (ch_num = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daddr4](index.html) module"]
pub struct DADDR4_SPEC;
impl crate::RegisterSpec for DADDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daddr4::R](R) reader structure"]
impl crate::Readable for DADDR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daddr4::W](W) writer structure"]
impl crate::Writable for DADDR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DADDR4 to value 0"]
impl crate::Resettable for DADDR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
