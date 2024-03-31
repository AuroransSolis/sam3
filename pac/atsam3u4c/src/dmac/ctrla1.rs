#[doc = "Register `CTRLA1` reader"]
pub type R = crate::R<Ctrla1Spec>;
#[doc = "Register `CTRLA1` writer"]
pub type W = crate::W<Ctrla1Spec>;
#[doc = "Field `BTSIZE` reader - Buffer Transfer Size"]
pub type BtsizeR = crate::FieldReader<u16>;
#[doc = "Field `BTSIZE` writer - Buffer Transfer Size"]
pub type BtsizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Transfer Width for the Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SrcWidth {
    #[doc = "0: the transfer size is set to 8-bit width"]
    Byte = 0,
    #[doc = "1: the transfer size is set to 16-bit width"]
    HalfWord = 1,
    #[doc = "2: the transfer size is set to 32-bit width"]
    Word = 2,
}
impl From<SrcWidth> for u8 {
    #[inline(always)]
    fn from(variant: SrcWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SrcWidth {
    type Ux = u8;
}
impl crate::IsEnum for SrcWidth {}
#[doc = "Field `SRC_WIDTH` reader - Transfer Width for the Source"]
pub type SrcWidthR = crate::FieldReader<SrcWidth>;
impl SrcWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SrcWidth> {
        match self.bits {
            0 => Some(SrcWidth::Byte),
            1 => Some(SrcWidth::HalfWord),
            2 => Some(SrcWidth::Word),
            _ => None,
        }
    }
    #[doc = "the transfer size is set to 8-bit width"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SrcWidth::Byte
    }
    #[doc = "the transfer size is set to 16-bit width"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == SrcWidth::HalfWord
    }
    #[doc = "the transfer size is set to 32-bit width"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SrcWidth::Word
    }
}
#[doc = "Field `SRC_WIDTH` writer - Transfer Width for the Source"]
pub type SrcWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, SrcWidth>;
impl<'a, REG> SrcWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "the transfer size is set to 8-bit width"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(SrcWidth::Byte)
    }
    #[doc = "the transfer size is set to 16-bit width"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(SrcWidth::HalfWord)
    }
    #[doc = "the transfer size is set to 32-bit width"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(SrcWidth::Word)
    }
}
#[doc = "Transfer Width for the Destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DstWidth {
    #[doc = "0: the transfer size is set to 8-bit width"]
    Byte = 0,
    #[doc = "1: the transfer size is set to 16-bit width"]
    HalfWord = 1,
    #[doc = "2: the transfer size is set to 32-bit width"]
    Word = 2,
}
impl From<DstWidth> for u8 {
    #[inline(always)]
    fn from(variant: DstWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DstWidth {
    type Ux = u8;
}
impl crate::IsEnum for DstWidth {}
#[doc = "Field `DST_WIDTH` reader - Transfer Width for the Destination"]
pub type DstWidthR = crate::FieldReader<DstWidth>;
impl DstWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DstWidth> {
        match self.bits {
            0 => Some(DstWidth::Byte),
            1 => Some(DstWidth::HalfWord),
            2 => Some(DstWidth::Word),
            _ => None,
        }
    }
    #[doc = "the transfer size is set to 8-bit width"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DstWidth::Byte
    }
    #[doc = "the transfer size is set to 16-bit width"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == DstWidth::HalfWord
    }
    #[doc = "the transfer size is set to 32-bit width"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DstWidth::Word
    }
}
#[doc = "Field `DST_WIDTH` writer - Transfer Width for the Destination"]
pub type DstWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, DstWidth>;
impl<'a, REG> DstWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "the transfer size is set to 8-bit width"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(DstWidth::Byte)
    }
    #[doc = "the transfer size is set to 16-bit width"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(DstWidth::HalfWord)
    }
    #[doc = "the transfer size is set to 32-bit width"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(DstWidth::Word)
    }
}
#[doc = "Field `DONE` reader - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Buffer Transfer Size"]
    #[inline(always)]
    pub fn btsize(&self) -> BtsizeR {
        BtsizeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:25 - Transfer Width for the Source"]
    #[inline(always)]
    pub fn src_width(&self) -> SrcWidthR {
        SrcWidthR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Transfer Width for the Destination"]
    #[inline(always)]
    pub fn dst_width(&self) -> DstWidthR {
        DstWidthR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Buffer Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn btsize(&mut self) -> BtsizeW<Ctrla1Spec> {
        BtsizeW::new(self, 0)
    }
    #[doc = "Bits 24:25 - Transfer Width for the Source"]
    #[inline(always)]
    #[must_use]
    pub fn src_width(&mut self) -> SrcWidthW<Ctrla1Spec> {
        SrcWidthW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Transfer Width for the Destination"]
    #[inline(always)]
    #[must_use]
    pub fn dst_width(&mut self) -> DstWidthW<Ctrla1Spec> {
        DstWidthW::new(self, 28)
    }
    #[doc = "Bit 31 - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<Ctrla1Spec> {
        DoneW::new(self, 31)
    }
}
#[doc = "DMAC Channel Control A Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrla1Spec;
impl crate::RegisterSpec for Ctrla1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla1::R`](R) reader structure"]
impl crate::Readable for Ctrla1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrla1::W`](W) writer structure"]
impl crate::Writable for Ctrla1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLA1 to value 0"]
impl crate::Resettable for Ctrla1Spec {
    const RESET_VALUE: u32 = 0;
}
