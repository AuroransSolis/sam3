#[doc = "Register `ECC_MD` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<ECC_MD_SPEC>);
#[doc = "Register `ECC_MD` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<ECC_MD_SPEC>);
#[doc = "Field `ECC_PAGESIZE` reader - ECC Page Size"]
pub type ECC_PAGESIZE_R = crate::FieldReader<u8, ECC_PAGESIZE_A>;
#[doc = "ECC Page Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ECC_PAGESIZE_A {
    #[doc = "0: Main area 512 Words"]
    Ps512 = 0,
    #[doc = "1: Main area 1024 Words"]
    Ps1024 = 1,
    #[doc = "2: Main area 2048 Words"]
    Ps2048 = 2,
    #[doc = "3: Main area 4096 Words"]
    Ps4096 = 3,
}
impl From<ECC_PAGESIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: ECC_PAGESIZE_A) -> Self {
        variant as _
    }
}
impl ECC_PAGESIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECC_PAGESIZE_A {
        match self.bits {
            0 => ECC_PAGESIZE_A::Ps512,
            1 => ECC_PAGESIZE_A::Ps1024,
            2 => ECC_PAGESIZE_A::Ps2048,
            3 => ECC_PAGESIZE_A::Ps4096,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Ps512`"]
    #[inline(always)]
    pub fn is_ps512(&self) -> bool {
        *self == ECC_PAGESIZE_A::Ps512
    }
    #[doc = "Checks if the value of the field is `Ps1024`"]
    #[inline(always)]
    pub fn is_ps1024(&self) -> bool {
        *self == ECC_PAGESIZE_A::Ps1024
    }
    #[doc = "Checks if the value of the field is `Ps2048`"]
    #[inline(always)]
    pub fn is_ps2048(&self) -> bool {
        *self == ECC_PAGESIZE_A::Ps2048
    }
    #[doc = "Checks if the value of the field is `Ps4096`"]
    #[inline(always)]
    pub fn is_ps4096(&self) -> bool {
        *self == ECC_PAGESIZE_A::Ps4096
    }
}
#[doc = "Field `ECC_PAGESIZE` writer - ECC Page Size"]
pub type ECC_PAGESIZE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ECC_MD_SPEC, u8, ECC_PAGESIZE_A, 2, O>;
impl<'a, const O: u8> ECC_PAGESIZE_W<'a, O> {
    #[doc = "Main area 512 Words"]
    #[inline(always)]
    pub fn ps512(self) -> &'a mut W {
        self.variant(ECC_PAGESIZE_A::Ps512)
    }
    #[doc = "Main area 1024 Words"]
    #[inline(always)]
    pub fn ps1024(self) -> &'a mut W {
        self.variant(ECC_PAGESIZE_A::Ps1024)
    }
    #[doc = "Main area 2048 Words"]
    #[inline(always)]
    pub fn ps2048(self) -> &'a mut W {
        self.variant(ECC_PAGESIZE_A::Ps2048)
    }
    #[doc = "Main area 4096 Words"]
    #[inline(always)]
    pub fn ps4096(self) -> &'a mut W {
        self.variant(ECC_PAGESIZE_A::Ps4096)
    }
}
#[doc = "Field `TYPCORREC` reader - Type of Correction"]
pub type TYPCORREC_R = crate::FieldReader<u8, TYPCORREC_A>;
#[doc = "Type of Correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TYPCORREC_A {
    #[doc = "0: 1 bit correction for a page of 512/1024/2048/4096 Bytes (for 8 or 16-bit NAND Flash)"]
    Cpage = 0,
    #[doc = "1: 1 bit correction for 256 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    C256b = 1,
    #[doc = "2: 1 bit correction for 512 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    C512b = 2,
}
impl From<TYPCORREC_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPCORREC_A) -> Self {
        variant as _
    }
}
impl TYPCORREC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TYPCORREC_A> {
        match self.bits {
            0 => Some(TYPCORREC_A::Cpage),
            1 => Some(TYPCORREC_A::C256b),
            2 => Some(TYPCORREC_A::C512b),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Cpage`"]
    #[inline(always)]
    pub fn is_cpage(&self) -> bool {
        *self == TYPCORREC_A::Cpage
    }
    #[doc = "Checks if the value of the field is `C256b`"]
    #[inline(always)]
    pub fn is_c256b(&self) -> bool {
        *self == TYPCORREC_A::C256b
    }
    #[doc = "Checks if the value of the field is `C512b`"]
    #[inline(always)]
    pub fn is_c512b(&self) -> bool {
        *self == TYPCORREC_A::C512b
    }
}
#[doc = "Field `TYPCORREC` writer - Type of Correction"]
pub type TYPCORREC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ECC_MD_SPEC, u8, TYPCORREC_A, 2, O>;
impl<'a, const O: u8> TYPCORREC_W<'a, O> {
    #[doc = "1 bit correction for a page of 512/1024/2048/4096 Bytes (for 8 or 16-bit NAND Flash)"]
    #[inline(always)]
    pub fn cpage(self) -> &'a mut W {
        self.variant(TYPCORREC_A::Cpage)
    }
    #[doc = "1 bit correction for 256 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    #[inline(always)]
    pub fn c256b(self) -> &'a mut W {
        self.variant(TYPCORREC_A::C256b)
    }
    #[doc = "1 bit correction for 512 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    #[inline(always)]
    pub fn c512b(self) -> &'a mut W {
        self.variant(TYPCORREC_A::C512b)
    }
}
impl R {
    #[doc = "Bits 0:1 - ECC Page Size"]
    #[inline(always)]
    pub fn ecc_pagesize(&self) -> ECC_PAGESIZE_R {
        ECC_PAGESIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Type of Correction"]
    #[inline(always)]
    pub fn typcorrec(&self) -> TYPCORREC_R {
        TYPCORREC_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ECC Page Size"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_pagesize(&mut self) -> ECC_PAGESIZE_W<0> {
        ECC_PAGESIZE_W::new(self)
    }
    #[doc = "Bits 4:5 - Type of Correction"]
    #[inline(always)]
    #[must_use]
    pub fn typcorrec(&mut self) -> TYPCORREC_W<4> {
        TYPCORREC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC ECC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_md](index.html) module"]
pub struct ECC_MD_SPEC;
impl crate::RegisterSpec for ECC_MD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_md::R](R) reader structure"]
impl crate::Readable for ECC_MD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecc_md::W](W) writer structure"]
impl crate::Writable for ECC_MD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECC_MD to value 0"]
impl crate::Resettable for ECC_MD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
