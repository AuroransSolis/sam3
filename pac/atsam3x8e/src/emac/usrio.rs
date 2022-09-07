#[doc = "Register `USRIO` reader"]
pub struct R(crate::R<USRIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USRIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USRIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USRIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USRIO` writer"]
pub struct W(crate::W<USRIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USRIO_SPEC>;
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
impl From<crate::W<USRIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USRIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMII` reader - Reduce MII"]
pub type RMII_R = crate::BitReader<bool>;
#[doc = "Field `RMII` writer - Reduce MII"]
pub type RMII_W<'a, const O: u8> = crate::BitWriter<'a, u32, USRIO_SPEC, bool, O>;
#[doc = "Field `CLKEN` reader - Clock Enable"]
pub type CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKEN` writer - Clock Enable"]
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USRIO_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Reduce MII"]
    #[inline(always)]
    pub fn rmii(&self) -> RMII_R {
        RMII_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reduce MII"]
    #[inline(always)]
    pub fn rmii(&mut self) -> RMII_W<0> {
        RMII_W::new(self)
    }
    #[doc = "Bit 1 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<1> {
        CLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User Input/Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usrio](index.html) module"]
pub struct USRIO_SPEC;
impl crate::RegisterSpec for USRIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usrio::R](R) reader structure"]
impl crate::Readable for USRIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usrio::W](W) writer structure"]
impl crate::Writable for USRIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USRIO to value 0"]
impl crate::Resettable for USRIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
