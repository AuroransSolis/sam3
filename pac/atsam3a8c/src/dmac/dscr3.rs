#[doc = "Register `DSCR3` reader"]
pub struct R(crate::R<DSCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSCR3` writer"]
pub struct W(crate::W<DSCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSCR3_SPEC>;
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
impl From<crate::W<DSCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSCR` reader - Buffer Transfer Descriptor Address"]
pub type DSCR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSCR` writer - Buffer Transfer Descriptor Address"]
pub type DSCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSCR3_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Buffer Transfer Descriptor Address"]
    #[inline(always)]
    pub fn dscr(&self) -> DSCR_R {
        DSCR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Buffer Transfer Descriptor Address"]
    #[inline(always)]
    pub fn dscr(&mut self) -> DSCR_W<2> {
        DSCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscr3](index.html) module"]
pub struct DSCR3_SPEC;
impl crate::RegisterSpec for DSCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dscr3::R](R) reader structure"]
impl crate::Readable for DSCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dscr3::W](W) writer structure"]
impl crate::Writable for DSCR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSCR3 to value 0"]
impl crate::Resettable for DSCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
