#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAGESIZE` reader - "]
pub type PAGESIZE_R = crate::FieldReader<u8, PAGESIZE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAGESIZE_A {
    #[doc = "0: Main area 512 Bytes + Spare area 16 Bytes = 528 Bytes"]
    PS512_16 = 0,
    #[doc = "1: Main area 1024 Bytes + Spare area 32 Bytes = 1056 Bytes"]
    PS1024_32 = 1,
    #[doc = "2: Main area 2048 Bytes + Spare area 64 Bytes = 2112 Bytes"]
    PS2048_64 = 2,
    #[doc = "3: Main area 4096 Bytes + Spare area 128 Bytes = 4224 Bytes"]
    PS4096_128 = 3,
}
impl From<PAGESIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PAGESIZE_A) -> Self {
        variant as _
    }
}
impl PAGESIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAGESIZE_A {
        match self.bits {
            0 => PAGESIZE_A::PS512_16,
            1 => PAGESIZE_A::PS1024_32,
            2 => PAGESIZE_A::PS2048_64,
            3 => PAGESIZE_A::PS4096_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PS512_16`"]
    #[inline(always)]
    pub fn is_ps512_16(&self) -> bool {
        *self == PAGESIZE_A::PS512_16
    }
    #[doc = "Checks if the value of the field is `PS1024_32`"]
    #[inline(always)]
    pub fn is_ps1024_32(&self) -> bool {
        *self == PAGESIZE_A::PS1024_32
    }
    #[doc = "Checks if the value of the field is `PS2048_64`"]
    #[inline(always)]
    pub fn is_ps2048_64(&self) -> bool {
        *self == PAGESIZE_A::PS2048_64
    }
    #[doc = "Checks if the value of the field is `PS4096_128`"]
    #[inline(always)]
    pub fn is_ps4096_128(&self) -> bool {
        *self == PAGESIZE_A::PS4096_128
    }
}
#[doc = "Field `PAGESIZE` writer - "]
pub type PAGESIZE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, PAGESIZE_A, 2, O>;
impl<'a, const O: u8> PAGESIZE_W<'a, O> {
    #[doc = "Main area 512 Bytes + Spare area 16 Bytes = 528 Bytes"]
    #[inline(always)]
    pub fn ps512_16(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PS512_16)
    }
    #[doc = "Main area 1024 Bytes + Spare area 32 Bytes = 1056 Bytes"]
    #[inline(always)]
    pub fn ps1024_32(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PS1024_32)
    }
    #[doc = "Main area 2048 Bytes + Spare area 64 Bytes = 2112 Bytes"]
    #[inline(always)]
    pub fn ps2048_64(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PS2048_64)
    }
    #[doc = "Main area 4096 Bytes + Spare area 128 Bytes = 4224 Bytes"]
    #[inline(always)]
    pub fn ps4096_128(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PS4096_128)
    }
}
#[doc = "Field `WSPARE` reader - Write Spare Area"]
pub type WSPARE_R = crate::BitReader<bool>;
#[doc = "Field `WSPARE` writer - Write Spare Area"]
pub type WSPARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RSPARE` reader - Read Spare Area"]
pub type RSPARE_R = crate::BitReader<bool>;
#[doc = "Field `RSPARE` writer - Read Spare Area"]
pub type RSPARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `EDGECTRL` reader - Rising/Falling Edge Detection Control"]
pub type EDGECTRL_R = crate::BitReader<bool>;
#[doc = "Field `EDGECTRL` writer - Rising/Falling Edge Detection Control"]
pub type EDGECTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RBEDGE` reader - Ready/Busy Signal Edge Detection"]
pub type RBEDGE_R = crate::BitReader<bool>;
#[doc = "Field `RBEDGE` writer - Ready/Busy Signal Edge Detection"]
pub type RBEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DTOCYC` reader - Data Timeout Cycle Number"]
pub type DTOCYC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTOCYC` writer - Data Timeout Cycle Number"]
pub type DTOCYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DTOMUL` reader - Data Timeout Multiplier"]
pub type DTOMUL_R = crate::FieldReader<u8, DTOMUL_A>;
#[doc = "Data Timeout Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTOMUL_A {
    #[doc = "0: DTOCYC"]
    X1 = 0,
    #[doc = "1: DTOCYC x 16"]
    X16 = 1,
    #[doc = "2: DTOCYC x 128"]
    X128 = 2,
    #[doc = "3: DTOCYC x 256"]
    X256 = 3,
    #[doc = "4: DTOCYC x 1024"]
    X1024 = 4,
    #[doc = "5: DTOCYC x 4096"]
    X4096 = 5,
    #[doc = "6: DTOCYC x 65536"]
    X65536 = 6,
    #[doc = "7: DTOCYC x 1048576"]
    X1048576 = 7,
}
impl From<DTOMUL_A> for u8 {
    #[inline(always)]
    fn from(variant: DTOMUL_A) -> Self {
        variant as _
    }
}
impl DTOMUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOMUL_A {
        match self.bits {
            0 => DTOMUL_A::X1,
            1 => DTOMUL_A::X16,
            2 => DTOMUL_A::X128,
            3 => DTOMUL_A::X256,
            4 => DTOMUL_A::X1024,
            5 => DTOMUL_A::X4096,
            6 => DTOMUL_A::X65536,
            7 => DTOMUL_A::X1048576,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == DTOMUL_A::X1
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == DTOMUL_A::X16
    }
    #[doc = "Checks if the value of the field is `X128`"]
    #[inline(always)]
    pub fn is_x128(&self) -> bool {
        *self == DTOMUL_A::X128
    }
    #[doc = "Checks if the value of the field is `X256`"]
    #[inline(always)]
    pub fn is_x256(&self) -> bool {
        *self == DTOMUL_A::X256
    }
    #[doc = "Checks if the value of the field is `X1024`"]
    #[inline(always)]
    pub fn is_x1024(&self) -> bool {
        *self == DTOMUL_A::X1024
    }
    #[doc = "Checks if the value of the field is `X4096`"]
    #[inline(always)]
    pub fn is_x4096(&self) -> bool {
        *self == DTOMUL_A::X4096
    }
    #[doc = "Checks if the value of the field is `X65536`"]
    #[inline(always)]
    pub fn is_x65536(&self) -> bool {
        *self == DTOMUL_A::X65536
    }
    #[doc = "Checks if the value of the field is `X1048576`"]
    #[inline(always)]
    pub fn is_x1048576(&self) -> bool {
        *self == DTOMUL_A::X1048576
    }
}
#[doc = "Field `DTOMUL` writer - Data Timeout Multiplier"]
pub type DTOMUL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, DTOMUL_A, 3, O>;
impl<'a, const O: u8> DTOMUL_W<'a, O> {
    #[doc = "DTOCYC"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(DTOMUL_A::X1)
    }
    #[doc = "DTOCYC x 16"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut W {
        self.variant(DTOMUL_A::X16)
    }
    #[doc = "DTOCYC x 128"]
    #[inline(always)]
    pub fn x128(self) -> &'a mut W {
        self.variant(DTOMUL_A::X128)
    }
    #[doc = "DTOCYC x 256"]
    #[inline(always)]
    pub fn x256(self) -> &'a mut W {
        self.variant(DTOMUL_A::X256)
    }
    #[doc = "DTOCYC x 1024"]
    #[inline(always)]
    pub fn x1024(self) -> &'a mut W {
        self.variant(DTOMUL_A::X1024)
    }
    #[doc = "DTOCYC x 4096"]
    #[inline(always)]
    pub fn x4096(self) -> &'a mut W {
        self.variant(DTOMUL_A::X4096)
    }
    #[doc = "DTOCYC x 65536"]
    #[inline(always)]
    pub fn x65536(self) -> &'a mut W {
        self.variant(DTOMUL_A::X65536)
    }
    #[doc = "DTOCYC x 1048576"]
    #[inline(always)]
    pub fn x1048576(self) -> &'a mut W {
        self.variant(DTOMUL_A::X1048576)
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pagesize(&self) -> PAGESIZE_R {
        PAGESIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Write Spare Area"]
    #[inline(always)]
    pub fn wspare(&self) -> WSPARE_R {
        WSPARE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read Spare Area"]
    #[inline(always)]
    pub fn rspare(&self) -> RSPARE_R {
        RSPARE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising/Falling Edge Detection Control"]
    #[inline(always)]
    pub fn edgectrl(&self) -> EDGECTRL_R {
        EDGECTRL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Ready/Busy Signal Edge Detection"]
    #[inline(always)]
    pub fn rbedge(&self) -> RBEDGE_R {
        RBEDGE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Data Timeout Cycle Number"]
    #[inline(always)]
    pub fn dtocyc(&self) -> DTOCYC_R {
        DTOCYC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Data Timeout Multiplier"]
    #[inline(always)]
    pub fn dtomul(&self) -> DTOMUL_R {
        DTOMUL_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pagesize(&mut self) -> PAGESIZE_W<0> {
        PAGESIZE_W::new(self)
    }
    #[doc = "Bit 8 - Write Spare Area"]
    #[inline(always)]
    pub fn wspare(&mut self) -> WSPARE_W<8> {
        WSPARE_W::new(self)
    }
    #[doc = "Bit 9 - Read Spare Area"]
    #[inline(always)]
    pub fn rspare(&mut self) -> RSPARE_W<9> {
        RSPARE_W::new(self)
    }
    #[doc = "Bit 12 - Rising/Falling Edge Detection Control"]
    #[inline(always)]
    pub fn edgectrl(&mut self) -> EDGECTRL_W<12> {
        EDGECTRL_W::new(self)
    }
    #[doc = "Bit 13 - Ready/Busy Signal Edge Detection"]
    #[inline(always)]
    pub fn rbedge(&mut self) -> RBEDGE_W<13> {
        RBEDGE_W::new(self)
    }
    #[doc = "Bits 16:19 - Data Timeout Cycle Number"]
    #[inline(always)]
    pub fn dtocyc(&mut self) -> DTOCYC_W<16> {
        DTOCYC_W::new(self)
    }
    #[doc = "Bits 20:22 - Data Timeout Multiplier"]
    #[inline(always)]
    pub fn dtomul(&mut self) -> DTOMUL_W<20> {
        DTOMUL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC NFC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
