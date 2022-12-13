#[doc = "Register `MMR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<MMR_SPEC>);
#[doc = "Register `MMR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<MMR_SPEC>);
#[doc = "Field `IADRSZ` reader - Internal Device Address Size"]
pub type IADRSZ_R = crate::FieldReader<u8, IADRSZ_A>;
#[doc = "Internal Device Address Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IADRSZ_A {
    #[doc = "0: No internal device address"]
    None = 0,
    #[doc = "1: One-byte internal device address"]
    _1Byte = 1,
    #[doc = "2: Two-byte internal device address"]
    _2Byte = 2,
    #[doc = "3: Three-byte internal device address"]
    _3Byte = 3,
}
impl From<IADRSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: IADRSZ_A) -> Self {
        variant as _
    }
}
impl IADRSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IADRSZ_A {
        match self.bits {
            0 => IADRSZ_A::None,
            1 => IADRSZ_A::_1Byte,
            2 => IADRSZ_A::_2Byte,
            3 => IADRSZ_A::_3Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `None`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == IADRSZ_A::None
    }
    #[doc = "Checks if the value of the field is `_1Byte`"]
    #[inline(always)]
    pub fn is_1_byte(&self) -> bool {
        *self == IADRSZ_A::_1Byte
    }
    #[doc = "Checks if the value of the field is `_2Byte`"]
    #[inline(always)]
    pub fn is_2_byte(&self) -> bool {
        *self == IADRSZ_A::_2Byte
    }
    #[doc = "Checks if the value of the field is `_3Byte`"]
    #[inline(always)]
    pub fn is_3_byte(&self) -> bool {
        *self == IADRSZ_A::_3Byte
    }
}
#[doc = "Field `IADRSZ` writer - Internal Device Address Size"]
pub type IADRSZ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MMR_SPEC, u8, IADRSZ_A, 2, O>;
impl<'a, const O: u8> IADRSZ_W<'a, O> {
    #[doc = "No internal device address"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(IADRSZ_A::None)
    }
    #[doc = "One-byte internal device address"]
    #[inline(always)]
    pub fn _1_byte(self) -> &'a mut W {
        self.variant(IADRSZ_A::_1Byte)
    }
    #[doc = "Two-byte internal device address"]
    #[inline(always)]
    pub fn _2_byte(self) -> &'a mut W {
        self.variant(IADRSZ_A::_2Byte)
    }
    #[doc = "Three-byte internal device address"]
    #[inline(always)]
    pub fn _3_byte(self) -> &'a mut W {
        self.variant(IADRSZ_A::_3Byte)
    }
}
#[doc = "Field `MREAD` reader - Master Read Direction"]
pub type MREAD_R = crate::BitReader<bool>;
#[doc = "Field `MREAD` writer - Master Read Direction"]
pub type MREAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMR_SPEC, bool, O>;
#[doc = "Field `DADR` reader - Device Address"]
pub type DADR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DADR` writer - Device Address"]
pub type DADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline(always)]
    pub fn iadrsz(&self) -> IADRSZ_R {
        IADRSZ_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline(always)]
    pub fn mread(&self) -> MREAD_R {
        MREAD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline(always)]
    pub fn dadr(&self) -> DADR_R {
        DADR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline(always)]
    #[must_use]
    pub fn iadrsz(&mut self) -> IADRSZ_W<8> {
        IADRSZ_W::new(self)
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline(always)]
    #[must_use]
    pub fn mread(&mut self) -> MREAD_W<12> {
        MREAD_W::new(self)
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn dadr(&mut self) -> DADR_W<16> {
        DADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmr](index.html) module"]
pub struct MMR_SPEC;
impl crate::RegisterSpec for MMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmr::R](R) reader structure"]
impl crate::Readable for MMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmr::W](W) writer structure"]
impl crate::Writable for MMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMR to value 0"]
impl crate::Resettable for MMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
