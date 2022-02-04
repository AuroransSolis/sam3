#[doc = "Register `ECC_MD` reader"]
pub struct R(crate::R<ECC_MD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_MD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_MD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_MD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECC_MD` writer"]
pub struct W(crate::W<ECC_MD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECC_MD_SPEC>;
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
impl From<crate::W<ECC_MD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECC_MD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ECC Page Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ECC_PAGESIZE_A {
    #[doc = "0: Main area 512 Bytes + Spare area 16 Bytes = 528 Bytes"]
    PS512_16 = 0,
    #[doc = "1: Main area 1024 Bytes + Spare area 32 Bytes = 1056 Bytes"]
    PS1024_32 = 1,
    #[doc = "2: Main area 2048 Bytes + Spare area 64 Bytes = 2112 Bytes"]
    PS2048_64 = 2,
    #[doc = "3: Main area 4096 Bytes + Spare area 128 Bytes = 4224 Bytes"]
    PS4096_128 = 3,
}
impl From<ECC_PAGESIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: ECC_PAGESIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ECC_PAGESIZE` reader - ECC Page Size"]
pub struct ECC_PAGESIZE_R(crate::FieldReader<u8, ECC_PAGESIZE_A>);
impl ECC_PAGESIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ECC_PAGESIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECC_PAGESIZE_A {
        match self.bits {
            0 => ECC_PAGESIZE_A::PS512_16,
            1 => ECC_PAGESIZE_A::PS1024_32,
            2 => ECC_PAGESIZE_A::PS2048_64,
            3 => ECC_PAGESIZE_A::PS4096_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PS512_16`"]
    #[inline(always)]
    pub fn is_ps512_16(&self) -> bool {
        **self == ECC_PAGESIZE_A::PS512_16
    }
    #[doc = "Checks if the value of the field is `PS1024_32`"]
    #[inline(always)]
    pub fn is_ps1024_32(&self) -> bool {
        **self == ECC_PAGESIZE_A::PS1024_32
    }
    #[doc = "Checks if the value of the field is `PS2048_64`"]
    #[inline(always)]
    pub fn is_ps2048_64(&self) -> bool {
        **self == ECC_PAGESIZE_A::PS2048_64
    }
    #[doc = "Checks if the value of the field is `PS4096_128`"]
    #[inline(always)]
    pub fn is_ps4096_128(&self) -> bool {
        **self == ECC_PAGESIZE_A::PS4096_128
    }
}
impl core::ops::Deref for ECC_PAGESIZE_R {
    type Target = crate::FieldReader<u8, ECC_PAGESIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECC_PAGESIZE` writer - ECC Page Size"]
pub struct ECC_PAGESIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_PAGESIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECC_PAGESIZE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Main area 512 Bytes + Spare area 16 Bytes = 528 Bytes"]
    #[inline(always)]
    pub fn ps512_16(self) -> &'a mut W {
        self.variant(ECC_PAGESIZE_A::PS512_16)
    }
    #[doc = "Main area 1024 Bytes + Spare area 32 Bytes = 1056 Bytes"]
    #[inline(always)]
    pub fn ps1024_32(self) -> &'a mut W {
        self.variant(ECC_PAGESIZE_A::PS1024_32)
    }
    #[doc = "Main area 2048 Bytes + Spare area 64 Bytes = 2112 Bytes"]
    #[inline(always)]
    pub fn ps2048_64(self) -> &'a mut W {
        self.variant(ECC_PAGESIZE_A::PS2048_64)
    }
    #[doc = "Main area 4096 Bytes + Spare area 128 Bytes = 4224 Bytes"]
    #[inline(always)]
    pub fn ps4096_128(self) -> &'a mut W {
        self.variant(ECC_PAGESIZE_A::PS4096_128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Type of Correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPCORREC_A {
    #[doc = "0: 1 bit correction for a page of 512/1024/2048/4096 Bytes (for 8 or 16-bit NAND Flash)"]
    CPAGE = 0,
    #[doc = "1: 1 bit correction for 256 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    C256B = 1,
    #[doc = "2: 1 bit correction for 512 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    C512B = 2,
}
impl From<TYPCORREC_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPCORREC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TYPCORREC` reader - Type of Correction"]
pub struct TYPCORREC_R(crate::FieldReader<u8, TYPCORREC_A>);
impl TYPCORREC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TYPCORREC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TYPCORREC_A> {
        match self.bits {
            0 => Some(TYPCORREC_A::CPAGE),
            1 => Some(TYPCORREC_A::C256B),
            2 => Some(TYPCORREC_A::C512B),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CPAGE`"]
    #[inline(always)]
    pub fn is_cpage(&self) -> bool {
        **self == TYPCORREC_A::CPAGE
    }
    #[doc = "Checks if the value of the field is `C256B`"]
    #[inline(always)]
    pub fn is_c256b(&self) -> bool {
        **self == TYPCORREC_A::C256B
    }
    #[doc = "Checks if the value of the field is `C512B`"]
    #[inline(always)]
    pub fn is_c512b(&self) -> bool {
        **self == TYPCORREC_A::C512B
    }
}
impl core::ops::Deref for TYPCORREC_R {
    type Target = crate::FieldReader<u8, TYPCORREC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TYPCORREC` writer - Type of Correction"]
pub struct TYPCORREC_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPCORREC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPCORREC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 bit correction for a page of 512/1024/2048/4096 Bytes (for 8 or 16-bit NAND Flash)"]
    #[inline(always)]
    pub fn cpage(self) -> &'a mut W {
        self.variant(TYPCORREC_A::CPAGE)
    }
    #[doc = "1 bit correction for 256 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    #[inline(always)]
    pub fn c256b(self) -> &'a mut W {
        self.variant(TYPCORREC_A::C256B)
    }
    #[doc = "1 bit correction for 512 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    #[inline(always)]
    pub fn c512b(self) -> &'a mut W {
        self.variant(TYPCORREC_A::C512B)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ECC Page Size"]
    #[inline(always)]
    pub fn ecc_pagesize(&self) -> ECC_PAGESIZE_R {
        ECC_PAGESIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Type of Correction"]
    #[inline(always)]
    pub fn typcorrec(&self) -> TYPCORREC_R {
        TYPCORREC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ECC Page Size"]
    #[inline(always)]
    pub fn ecc_pagesize(&mut self) -> ECC_PAGESIZE_W {
        ECC_PAGESIZE_W { w: self }
    }
    #[doc = "Bits 4:5 - Type of Correction"]
    #[inline(always)]
    pub fn typcorrec(&mut self) -> TYPCORREC_W {
        TYPCORREC_W { w: self }
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
}
#[doc = "`reset()` method sets ECC_MD to value 0"]
impl crate::Resettable for ECC_MD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
