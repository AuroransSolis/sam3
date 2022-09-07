#[doc = "Register `MATRIX_MCFG[%s]` reader"]
pub struct R(crate::R<MATRIX_MCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATRIX_MCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATRIX_MCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATRIX_MCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATRIX_MCFG[%s]` writer"]
pub struct W(crate::W<MATRIX_MCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATRIX_MCFG_SPEC>;
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
impl From<crate::W<MATRIX_MCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATRIX_MCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub type ULBT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub type ULBT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MATRIX_MCFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> ULBT_R {
        ULBT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&mut self) -> ULBT_W<0> {
        ULBT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_mcfg](index.html) module"]
pub struct MATRIX_MCFG_SPEC;
impl crate::RegisterSpec for MATRIX_MCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrix_mcfg::R](R) reader structure"]
impl crate::Readable for MATRIX_MCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [matrix_mcfg::W](W) writer structure"]
impl crate::Writable for MATRIX_MCFG_SPEC {
    type Writer = W;
}
