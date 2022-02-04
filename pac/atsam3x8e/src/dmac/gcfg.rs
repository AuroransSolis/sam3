#[doc = "Register `GCFG` reader"]
pub struct R(crate::R<GCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCFG` writer"]
pub struct W(crate::W<GCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCFG_SPEC>;
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
impl From<crate::W<GCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Arbiter Configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARB_CFG_A {
    #[doc = "0: Fixed priority arbiter (see \"Basic Definitions\" )"]
    FIXED = 0,
    #[doc = "1: Modified round robin arbiter."]
    ROUND_ROBIN = 1,
}
impl From<ARB_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: ARB_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARB_CFG` reader - Arbiter Configuration"]
pub struct ARB_CFG_R(crate::FieldReader<bool, ARB_CFG_A>);
impl ARB_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARB_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARB_CFG_A {
        match self.bits {
            false => ARB_CFG_A::FIXED,
            true => ARB_CFG_A::ROUND_ROBIN,
        }
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        **self == ARB_CFG_A::FIXED
    }
    #[doc = "Checks if the value of the field is `ROUND_ROBIN`"]
    #[inline(always)]
    pub fn is_round_robin(&self) -> bool {
        **self == ARB_CFG_A::ROUND_ROBIN
    }
}
impl core::ops::Deref for ARB_CFG_R {
    type Target = crate::FieldReader<bool, ARB_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB_CFG` writer - Arbiter Configuration"]
pub struct ARB_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARB_CFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fixed priority arbiter (see \"Basic Definitions\" )"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(ARB_CFG_A::FIXED)
    }
    #[doc = "Modified round robin arbiter."]
    #[inline(always)]
    pub fn round_robin(self) -> &'a mut W {
        self.variant(ARB_CFG_A::ROUND_ROBIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Arbiter Configuration"]
    #[inline(always)]
    pub fn arb_cfg(&self) -> ARB_CFG_R {
        ARB_CFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Arbiter Configuration"]
    #[inline(always)]
    pub fn arb_cfg(&mut self) -> ARB_CFG_W {
        ARB_CFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcfg](index.html) module"]
pub struct GCFG_SPEC;
impl crate::RegisterSpec for GCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcfg::R](R) reader structure"]
impl crate::Readable for GCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcfg::W](W) writer structure"]
impl crate::Writable for GCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCFG to value 0x10"]
impl crate::Resettable for GCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
