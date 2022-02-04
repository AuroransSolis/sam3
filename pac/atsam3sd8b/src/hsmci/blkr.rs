#[doc = "Register `BLKR` reader"]
pub struct R(crate::R<BLKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLKR` writer"]
pub struct W(crate::W<BLKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLKR_SPEC>;
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
impl From<crate::W<BLKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MMC/SDIO Block Count - SDIO Byte Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum BCNT_A {
    #[doc = "0: MMC/SDCARD Multiple BlockFrom 1 to 65635: Value 0 corresponds to an infinite block transfer."]
    MULTIPLE = 0,
    #[doc = "4: SDIO ByteFrom 1 to 512 bytes: Value 0 corresponds to a 512-byte transfer.Values from 0x200 to 0xFFFF are forbidden."]
    BYTE = 4,
    #[doc = "5: SDIO BlockFrom 1 to 511 blocks: Value 0 corresponds to an infinite block transfer.Values from 0x200 to 0xFFFF are forbidden."]
    BLOCK = 5,
}
impl From<BCNT_A> for u16 {
    #[inline(always)]
    fn from(variant: BCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BCNT` reader - MMC/SDIO Block Count - SDIO Byte Count"]
pub struct BCNT_R(crate::FieldReader<u16, BCNT_A>);
impl BCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BCNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BCNT_A> {
        match self.bits {
            0 => Some(BCNT_A::MULTIPLE),
            4 => Some(BCNT_A::BYTE),
            5 => Some(BCNT_A::BLOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        **self == BCNT_A::MULTIPLE
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        **self == BCNT_A::BYTE
    }
    #[doc = "Checks if the value of the field is `BLOCK`"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        **self == BCNT_A::BLOCK
    }
}
impl core::ops::Deref for BCNT_R {
    type Target = crate::FieldReader<u16, BCNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCNT` writer - MMC/SDIO Block Count - SDIO Byte Count"]
pub struct BCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MMC/SDCARD Multiple BlockFrom 1 to 65635: Value 0 corresponds to an infinite block transfer."]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut W {
        self.variant(BCNT_A::MULTIPLE)
    }
    #[doc = "SDIO ByteFrom 1 to 512 bytes: Value 0 corresponds to a 512-byte transfer.Values from 0x200 to 0xFFFF are forbidden."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(BCNT_A::BYTE)
    }
    #[doc = "SDIO BlockFrom 1 to 511 blocks: Value 0 corresponds to an infinite block transfer.Values from 0x200 to 0xFFFF are forbidden."]
    #[inline(always)]
    pub fn block(self) -> &'a mut W {
        self.variant(BCNT_A::BLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `BLKLEN` reader - Data Block Length"]
pub struct BLKLEN_R(crate::FieldReader<u16, u16>);
impl BLKLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BLKLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLKLEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLKLEN` writer - Data Block Length"]
pub struct BLKLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MMC/SDIO Block Count - SDIO Byte Count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data Block Length"]
    #[inline(always)]
    pub fn blklen(&self) -> BLKLEN_R {
        BLKLEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MMC/SDIO Block Count - SDIO Byte Count"]
    #[inline(always)]
    pub fn bcnt(&mut self) -> BCNT_W {
        BCNT_W { w: self }
    }
    #[doc = "Bits 16:31 - Data Block Length"]
    #[inline(always)]
    pub fn blklen(&mut self) -> BLKLEN_W {
        BLKLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blkr](index.html) module"]
pub struct BLKR_SPEC;
impl crate::RegisterSpec for BLKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blkr::R](R) reader structure"]
impl crate::Readable for BLKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blkr::W](W) writer structure"]
impl crate::Writable for BLKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLKR to value 0"]
impl crate::Resettable for BLKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
