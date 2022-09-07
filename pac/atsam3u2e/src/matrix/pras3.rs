#[doc = "Register `PRAS3` reader"]
pub struct R(crate::R<PRAS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRAS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRAS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRAS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRAS3` writer"]
pub struct W(crate::W<PRAS3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRAS3_SPEC>;
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
impl From<crate::W<PRAS3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRAS3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M0PR` reader - Master 0 Priority"]
pub type M0PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M0PR` writer - Master 0 Priority"]
pub type M0PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRAS3_SPEC, u8, u8, 2, O>;
#[doc = "Field `M1PR` reader - Master 1 Priority"]
pub type M1PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M1PR` writer - Master 1 Priority"]
pub type M1PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRAS3_SPEC, u8, u8, 2, O>;
#[doc = "Field `M2PR` reader - Master 2 Priority"]
pub type M2PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M2PR` writer - Master 2 Priority"]
pub type M2PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRAS3_SPEC, u8, u8, 2, O>;
#[doc = "Field `M3PR` reader - Master 3 Priority"]
pub type M3PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M3PR` writer - Master 3 Priority"]
pub type M3PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRAS3_SPEC, u8, u8, 2, O>;
#[doc = "Field `M4PR` reader - Master 4 Priority"]
pub type M4PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M4PR` writer - Master 4 Priority"]
pub type M4PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRAS3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline(always)]
    pub fn m0pr(&self) -> M0PR_R {
        M0PR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Master 1 Priority"]
    #[inline(always)]
    pub fn m1pr(&self) -> M1PR_R {
        M1PR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Master 2 Priority"]
    #[inline(always)]
    pub fn m2pr(&self) -> M2PR_R {
        M2PR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Master 3 Priority"]
    #[inline(always)]
    pub fn m3pr(&self) -> M3PR_R {
        M3PR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Master 4 Priority"]
    #[inline(always)]
    pub fn m4pr(&self) -> M4PR_R {
        M4PR_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline(always)]
    pub fn m0pr(&mut self) -> M0PR_W<0> {
        M0PR_W::new(self)
    }
    #[doc = "Bits 4:5 - Master 1 Priority"]
    #[inline(always)]
    pub fn m1pr(&mut self) -> M1PR_W<4> {
        M1PR_W::new(self)
    }
    #[doc = "Bits 8:9 - Master 2 Priority"]
    #[inline(always)]
    pub fn m2pr(&mut self) -> M2PR_W<8> {
        M2PR_W::new(self)
    }
    #[doc = "Bits 12:13 - Master 3 Priority"]
    #[inline(always)]
    pub fn m3pr(&mut self) -> M3PR_W<12> {
        M3PR_W::new(self)
    }
    #[doc = "Bits 16:17 - Master 4 Priority"]
    #[inline(always)]
    pub fn m4pr(&mut self) -> M4PR_W<16> {
        M4PR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Priority Register A for Slave 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pras3](index.html) module"]
pub struct PRAS3_SPEC;
impl crate::RegisterSpec for PRAS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pras3::R](R) reader structure"]
impl crate::Readable for PRAS3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pras3::W](W) writer structure"]
impl crate::Writable for PRAS3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRAS3 to value 0"]
impl crate::Resettable for PRAS3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
