#[doc = "Register `CTRLB2` reader"]
pub struct R(crate::R<CTRLB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB2` writer"]
pub struct W(crate::W<CTRLB2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB2_SPEC>;
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
impl From<crate::W<CTRLB2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Source Address Descriptor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_DSCR_A {
    #[doc = "0: Source address is updated when the descriptor is fetched from the memory."]
    FETCH_FROM_MEM = 0,
    #[doc = "1: Buffer Descriptor Fetch operation is disabled for the source."]
    FETCH_DISABLE = 1,
}
impl From<SRC_DSCR_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_DSCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC_DSCR` reader - Source Address Descriptor"]
pub struct SRC_DSCR_R(crate::FieldReader<bool, SRC_DSCR_A>);
impl SRC_DSCR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRC_DSCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_DSCR_A {
        match self.bits {
            false => SRC_DSCR_A::FETCH_FROM_MEM,
            true => SRC_DSCR_A::FETCH_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FETCH_FROM_MEM`"]
    #[inline(always)]
    pub fn is_fetch_from_mem(&self) -> bool {
        **self == SRC_DSCR_A::FETCH_FROM_MEM
    }
    #[doc = "Checks if the value of the field is `FETCH_DISABLE`"]
    #[inline(always)]
    pub fn is_fetch_disable(&self) -> bool {
        **self == SRC_DSCR_A::FETCH_DISABLE
    }
}
impl core::ops::Deref for SRC_DSCR_R {
    type Target = crate::FieldReader<bool, SRC_DSCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC_DSCR` writer - Source Address Descriptor"]
pub struct SRC_DSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_DSCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_DSCR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Source address is updated when the descriptor is fetched from the memory."]
    #[inline(always)]
    pub fn fetch_from_mem(self) -> &'a mut W {
        self.variant(SRC_DSCR_A::FETCH_FROM_MEM)
    }
    #[doc = "Buffer Descriptor Fetch operation is disabled for the source."]
    #[inline(always)]
    pub fn fetch_disable(self) -> &'a mut W {
        self.variant(SRC_DSCR_A::FETCH_DISABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Destination Address Descriptor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_DSCR_A {
    #[doc = "0: Destination address is updated when the descriptor is fetched from the memory."]
    FETCH_FROM_MEM = 0,
    #[doc = "1: Buffer Descriptor Fetch operation is disabled for the destination."]
    FETCH_DISABLE = 1,
}
impl From<DST_DSCR_A> for bool {
    #[inline(always)]
    fn from(variant: DST_DSCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DST_DSCR` reader - Destination Address Descriptor"]
pub struct DST_DSCR_R(crate::FieldReader<bool, DST_DSCR_A>);
impl DST_DSCR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DST_DSCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_DSCR_A {
        match self.bits {
            false => DST_DSCR_A::FETCH_FROM_MEM,
            true => DST_DSCR_A::FETCH_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FETCH_FROM_MEM`"]
    #[inline(always)]
    pub fn is_fetch_from_mem(&self) -> bool {
        **self == DST_DSCR_A::FETCH_FROM_MEM
    }
    #[doc = "Checks if the value of the field is `FETCH_DISABLE`"]
    #[inline(always)]
    pub fn is_fetch_disable(&self) -> bool {
        **self == DST_DSCR_A::FETCH_DISABLE
    }
}
impl core::ops::Deref for DST_DSCR_R {
    type Target = crate::FieldReader<bool, DST_DSCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DST_DSCR` writer - Destination Address Descriptor"]
pub struct DST_DSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_DSCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_DSCR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Destination address is updated when the descriptor is fetched from the memory."]
    #[inline(always)]
    pub fn fetch_from_mem(self) -> &'a mut W {
        self.variant(DST_DSCR_A::FETCH_FROM_MEM)
    }
    #[doc = "Buffer Descriptor Fetch operation is disabled for the destination."]
    #[inline(always)]
    pub fn fetch_disable(self) -> &'a mut W {
        self.variant(DST_DSCR_A::FETCH_DISABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Flow Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FC_A {
    #[doc = "0: Memory-to-Memory Transfer DMAC is flow controller"]
    MEM2MEM_DMA_FC = 0,
    #[doc = "1: Memory-to-Peripheral Transfer DMAC is flow controller"]
    MEM2PER_DMA_FC = 1,
    #[doc = "2: Peripheral-to-Memory Transfer DMAC is flow controller"]
    PER2MEM_DMA_FC = 2,
    #[doc = "3: Peripheral-to-Peripheral Transfer DMAC is flow controller"]
    PER2PER_DMA_FC = 3,
}
impl From<FC_A> for u8 {
    #[inline(always)]
    fn from(variant: FC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FC` reader - Flow Control"]
pub struct FC_R(crate::FieldReader<u8, FC_A>);
impl FC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC_A {
        match self.bits {
            0 => FC_A::MEM2MEM_DMA_FC,
            1 => FC_A::MEM2PER_DMA_FC,
            2 => FC_A::PER2MEM_DMA_FC,
            3 => FC_A::PER2PER_DMA_FC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MEM2MEM_DMA_FC`"]
    #[inline(always)]
    pub fn is_mem2mem_dma_fc(&self) -> bool {
        **self == FC_A::MEM2MEM_DMA_FC
    }
    #[doc = "Checks if the value of the field is `MEM2PER_DMA_FC`"]
    #[inline(always)]
    pub fn is_mem2per_dma_fc(&self) -> bool {
        **self == FC_A::MEM2PER_DMA_FC
    }
    #[doc = "Checks if the value of the field is `PER2MEM_DMA_FC`"]
    #[inline(always)]
    pub fn is_per2mem_dma_fc(&self) -> bool {
        **self == FC_A::PER2MEM_DMA_FC
    }
    #[doc = "Checks if the value of the field is `PER2PER_DMA_FC`"]
    #[inline(always)]
    pub fn is_per2per_dma_fc(&self) -> bool {
        **self == FC_A::PER2PER_DMA_FC
    }
}
impl core::ops::Deref for FC_R {
    type Target = crate::FieldReader<u8, FC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC` writer - Flow Control"]
pub struct FC_W<'a> {
    w: &'a mut W,
}
impl<'a> FC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Memory-to-Memory Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn mem2mem_dma_fc(self) -> &'a mut W {
        self.variant(FC_A::MEM2MEM_DMA_FC)
    }
    #[doc = "Memory-to-Peripheral Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn mem2per_dma_fc(self) -> &'a mut W {
        self.variant(FC_A::MEM2PER_DMA_FC)
    }
    #[doc = "Peripheral-to-Memory Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn per2mem_dma_fc(self) -> &'a mut W {
        self.variant(FC_A::PER2MEM_DMA_FC)
    }
    #[doc = "Peripheral-to-Peripheral Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn per2per_dma_fc(self) -> &'a mut W {
        self.variant(FC_A::PER2PER_DMA_FC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Incrementing, Decrementing or Fixed Address for the Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_INCR_A {
    #[doc = "0: The source address is incremented"]
    INCREMENTING = 0,
    #[doc = "1: The source address is decremented"]
    DECREMENTING = 1,
    #[doc = "2: The source address remains unchanged"]
    FIXED = 2,
}
impl From<SRC_INCR_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_INCR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRC_INCR` reader - Incrementing, Decrementing or Fixed Address for the Source"]
pub struct SRC_INCR_R(crate::FieldReader<u8, SRC_INCR_A>);
impl SRC_INCR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRC_INCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRC_INCR_A> {
        match self.bits {
            0 => Some(SRC_INCR_A::INCREMENTING),
            1 => Some(SRC_INCR_A::DECREMENTING),
            2 => Some(SRC_INCR_A::FIXED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INCREMENTING`"]
    #[inline(always)]
    pub fn is_incrementing(&self) -> bool {
        **self == SRC_INCR_A::INCREMENTING
    }
    #[doc = "Checks if the value of the field is `DECREMENTING`"]
    #[inline(always)]
    pub fn is_decrementing(&self) -> bool {
        **self == SRC_INCR_A::DECREMENTING
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        **self == SRC_INCR_A::FIXED
    }
}
impl core::ops::Deref for SRC_INCR_R {
    type Target = crate::FieldReader<u8, SRC_INCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC_INCR` writer - Incrementing, Decrementing or Fixed Address for the Source"]
pub struct SRC_INCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_INCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_INCR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The source address is incremented"]
    #[inline(always)]
    pub fn incrementing(self) -> &'a mut W {
        self.variant(SRC_INCR_A::INCREMENTING)
    }
    #[doc = "The source address is decremented"]
    #[inline(always)]
    pub fn decrementing(self) -> &'a mut W {
        self.variant(SRC_INCR_A::DECREMENTING)
    }
    #[doc = "The source address remains unchanged"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(SRC_INCR_A::FIXED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Incrementing, Decrementing or Fixed Address for the Destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DST_INCR_A {
    #[doc = "0: The destination address is incremented"]
    INCREMENTING = 0,
    #[doc = "1: The destination address is decremented"]
    DECREMENTING = 1,
    #[doc = "2: The destination address remains unchanged"]
    FIXED = 2,
}
impl From<DST_INCR_A> for u8 {
    #[inline(always)]
    fn from(variant: DST_INCR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DST_INCR` reader - Incrementing, Decrementing or Fixed Address for the Destination"]
pub struct DST_INCR_R(crate::FieldReader<u8, DST_INCR_A>);
impl DST_INCR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DST_INCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DST_INCR_A> {
        match self.bits {
            0 => Some(DST_INCR_A::INCREMENTING),
            1 => Some(DST_INCR_A::DECREMENTING),
            2 => Some(DST_INCR_A::FIXED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INCREMENTING`"]
    #[inline(always)]
    pub fn is_incrementing(&self) -> bool {
        **self == DST_INCR_A::INCREMENTING
    }
    #[doc = "Checks if the value of the field is `DECREMENTING`"]
    #[inline(always)]
    pub fn is_decrementing(&self) -> bool {
        **self == DST_INCR_A::DECREMENTING
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        **self == DST_INCR_A::FIXED
    }
}
impl core::ops::Deref for DST_INCR_R {
    type Target = crate::FieldReader<u8, DST_INCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DST_INCR` writer - Incrementing, Decrementing or Fixed Address for the Destination"]
pub struct DST_INCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_INCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_INCR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The destination address is incremented"]
    #[inline(always)]
    pub fn incrementing(self) -> &'a mut W {
        self.variant(DST_INCR_A::INCREMENTING)
    }
    #[doc = "The destination address is decremented"]
    #[inline(always)]
    pub fn decrementing(self) -> &'a mut W {
        self.variant(DST_INCR_A::DECREMENTING)
    }
    #[doc = "The destination address remains unchanged"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(DST_INCR_A::FIXED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `IEN` reader - Interrupt Enable Not"]
pub struct IEN_R(crate::FieldReader<bool, bool>);
impl IEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEN` writer - Interrupt Enable Not"]
pub struct IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Source Address Descriptor"]
    #[inline(always)]
    pub fn src_dscr(&self) -> SRC_DSCR_R {
        SRC_DSCR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Destination Address Descriptor"]
    #[inline(always)]
    pub fn dst_dscr(&self) -> DST_DSCR_R {
        DST_DSCR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Flow Control"]
    #[inline(always)]
    pub fn fc(&self) -> FC_R {
        FC_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Incrementing, Decrementing or Fixed Address for the Source"]
    #[inline(always)]
    pub fn src_incr(&self) -> SRC_INCR_R {
        SRC_INCR_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Incrementing, Decrementing or Fixed Address for the Destination"]
    #[inline(always)]
    pub fn dst_incr(&self) -> DST_INCR_R {
        DST_INCR_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Interrupt Enable Not"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Source Address Descriptor"]
    #[inline(always)]
    pub fn src_dscr(&mut self) -> SRC_DSCR_W {
        SRC_DSCR_W { w: self }
    }
    #[doc = "Bit 20 - Destination Address Descriptor"]
    #[inline(always)]
    pub fn dst_dscr(&mut self) -> DST_DSCR_W {
        DST_DSCR_W { w: self }
    }
    #[doc = "Bits 21:22 - Flow Control"]
    #[inline(always)]
    pub fn fc(&mut self) -> FC_W {
        FC_W { w: self }
    }
    #[doc = "Bits 24:25 - Incrementing, Decrementing or Fixed Address for the Source"]
    #[inline(always)]
    pub fn src_incr(&mut self) -> SRC_INCR_W {
        SRC_INCR_W { w: self }
    }
    #[doc = "Bits 28:29 - Incrementing, Decrementing or Fixed Address for the Destination"]
    #[inline(always)]
    pub fn dst_incr(&mut self) -> DST_INCR_W {
        DST_INCR_W { w: self }
    }
    #[doc = "Bit 30 - Interrupt Enable Not"]
    #[inline(always)]
    pub fn ien(&mut self) -> IEN_W {
        IEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Control B Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb2](index.html) module"]
pub struct CTRLB2_SPEC;
impl crate::RegisterSpec for CTRLB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlb2::R](R) reader structure"]
impl crate::Readable for CTRLB2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb2::W](W) writer structure"]
impl crate::Writable for CTRLB2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLB2 to value 0"]
impl crate::Resettable for CTRLB2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
