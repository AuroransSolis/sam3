#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - CRC Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - CRC Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `COMPARE` reader - CRC Compare"]
pub type COMPARE_R = crate::BitReader<bool>;
#[doc = "Field `COMPARE` writer - CRC Compare"]
pub type COMPARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `PTYPE` reader - Primitive Polynomial"]
pub type PTYPE_R = crate::FieldReader<u8, PTYPE_A>;
#[doc = "Primitive Polynomial\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PTYPE_A {
    #[doc = "0: Polynom 0x04C11DB7"]
    CCITT8023 = 0,
    #[doc = "1: Polynom 0x1EDC6F41"]
    CASTAGNOLI = 1,
    #[doc = "2: Polynom 0x1021"]
    CCITT16 = 2,
}
impl From<PTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: PTYPE_A) -> Self {
        variant as _
    }
}
impl PTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PTYPE_A> {
        match self.bits {
            0 => Some(PTYPE_A::CCITT8023),
            1 => Some(PTYPE_A::CASTAGNOLI),
            2 => Some(PTYPE_A::CCITT16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CCITT8023`"]
    #[inline(always)]
    pub fn is_ccitt8023(&self) -> bool {
        *self == PTYPE_A::CCITT8023
    }
    #[doc = "Checks if the value of the field is `CASTAGNOLI`"]
    #[inline(always)]
    pub fn is_castagnoli(&self) -> bool {
        *self == PTYPE_A::CASTAGNOLI
    }
    #[doc = "Checks if the value of the field is `CCITT16`"]
    #[inline(always)]
    pub fn is_ccitt16(&self) -> bool {
        *self == PTYPE_A::CCITT16
    }
}
#[doc = "Field `PTYPE` writer - Primitive Polynomial"]
pub type PTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, PTYPE_A, 2, O>;
impl<'a, const O: u8> PTYPE_W<'a, O> {
    #[doc = "Polynom 0x04C11DB7"]
    #[inline(always)]
    pub fn ccitt8023(self) -> &'a mut W {
        self.variant(PTYPE_A::CCITT8023)
    }
    #[doc = "Polynom 0x1EDC6F41"]
    #[inline(always)]
    pub fn castagnoli(self) -> &'a mut W {
        self.variant(PTYPE_A::CASTAGNOLI)
    }
    #[doc = "Polynom 0x1021"]
    #[inline(always)]
    pub fn ccitt16(self) -> &'a mut W {
        self.variant(PTYPE_A::CCITT16)
    }
}
#[doc = "Field `DIVIDER` reader - Request Divider"]
pub type DIVIDER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVIDER` writer - Request Divider"]
pub type DIVIDER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC Compare"]
    #[inline(always)]
    pub fn compare(&self) -> COMPARE_R {
        COMPARE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Primitive Polynomial"]
    #[inline(always)]
    pub fn ptype(&self) -> PTYPE_R {
        PTYPE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Request Divider"]
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - CRC Compare"]
    #[inline(always)]
    pub fn compare(&mut self) -> COMPARE_W<1> {
        COMPARE_W::new(self)
    }
    #[doc = "Bits 2:3 - Primitive Polynomial"]
    #[inline(always)]
    pub fn ptype(&mut self) -> PTYPE_W<2> {
        PTYPE_W::new(self)
    }
    #[doc = "Bits 4:7 - Request Divider"]
    #[inline(always)]
    pub fn divider(&mut self) -> DIVIDER_W<4> {
        DIVIDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRCCU Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
