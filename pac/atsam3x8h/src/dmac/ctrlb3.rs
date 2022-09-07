#[doc = "Register `CTRLB3` reader"]
pub struct R(crate::R<CTRLB3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB3` writer"]
pub struct W(crate::W<CTRLB3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB3_SPEC>;
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
impl From<crate::W<CTRLB3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC_DSCR` reader - Source Address Descriptor"]
pub type SRC_DSCR_R = crate::BitReader<SRC_DSCR_A>;
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
impl SRC_DSCR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SRC_DSCR_A::FETCH_FROM_MEM
    }
    #[doc = "Checks if the value of the field is `FETCH_DISABLE`"]
    #[inline(always)]
    pub fn is_fetch_disable(&self) -> bool {
        *self == SRC_DSCR_A::FETCH_DISABLE
    }
}
#[doc = "Field `SRC_DSCR` writer - Source Address Descriptor"]
pub type SRC_DSCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB3_SPEC, SRC_DSCR_A, O>;
impl<'a, const O: u8> SRC_DSCR_W<'a, O> {
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
}
#[doc = "Field `DST_DSCR` reader - Destination Address Descriptor"]
pub type DST_DSCR_R = crate::BitReader<DST_DSCR_A>;
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
impl DST_DSCR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DST_DSCR_A::FETCH_FROM_MEM
    }
    #[doc = "Checks if the value of the field is `FETCH_DISABLE`"]
    #[inline(always)]
    pub fn is_fetch_disable(&self) -> bool {
        *self == DST_DSCR_A::FETCH_DISABLE
    }
}
#[doc = "Field `DST_DSCR` writer - Destination Address Descriptor"]
pub type DST_DSCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB3_SPEC, DST_DSCR_A, O>;
impl<'a, const O: u8> DST_DSCR_W<'a, O> {
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
}
#[doc = "Field `FC` reader - Flow Control"]
pub type FC_R = crate::FieldReader<u8, FC_A>;
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
impl FC_R {
    #[doc = "Get enumerated values variant"]
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
        *self == FC_A::MEM2MEM_DMA_FC
    }
    #[doc = "Checks if the value of the field is `MEM2PER_DMA_FC`"]
    #[inline(always)]
    pub fn is_mem2per_dma_fc(&self) -> bool {
        *self == FC_A::MEM2PER_DMA_FC
    }
    #[doc = "Checks if the value of the field is `PER2MEM_DMA_FC`"]
    #[inline(always)]
    pub fn is_per2mem_dma_fc(&self) -> bool {
        *self == FC_A::PER2MEM_DMA_FC
    }
    #[doc = "Checks if the value of the field is `PER2PER_DMA_FC`"]
    #[inline(always)]
    pub fn is_per2per_dma_fc(&self) -> bool {
        *self == FC_A::PER2PER_DMA_FC
    }
}
#[doc = "Field `FC` writer - Flow Control"]
pub type FC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRLB3_SPEC, u8, FC_A, 2, O>;
impl<'a, const O: u8> FC_W<'a, O> {
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
}
#[doc = "Field `SRC_INCR` reader - Incrementing, Decrementing or Fixed Address for the Source"]
pub type SRC_INCR_R = crate::FieldReader<u8, SRC_INCR_A>;
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
impl SRC_INCR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SRC_INCR_A::INCREMENTING
    }
    #[doc = "Checks if the value of the field is `DECREMENTING`"]
    #[inline(always)]
    pub fn is_decrementing(&self) -> bool {
        *self == SRC_INCR_A::DECREMENTING
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == SRC_INCR_A::FIXED
    }
}
#[doc = "Field `SRC_INCR` writer - Incrementing, Decrementing or Fixed Address for the Source"]
pub type SRC_INCR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRLB3_SPEC, u8, SRC_INCR_A, 2, O>;
impl<'a, const O: u8> SRC_INCR_W<'a, O> {
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
}
#[doc = "Field `DST_INCR` reader - Incrementing, Decrementing or Fixed Address for the Destination"]
pub type DST_INCR_R = crate::FieldReader<u8, DST_INCR_A>;
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
impl DST_INCR_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DST_INCR_A::INCREMENTING
    }
    #[doc = "Checks if the value of the field is `DECREMENTING`"]
    #[inline(always)]
    pub fn is_decrementing(&self) -> bool {
        *self == DST_INCR_A::DECREMENTING
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == DST_INCR_A::FIXED
    }
}
#[doc = "Field `DST_INCR` writer - Incrementing, Decrementing or Fixed Address for the Destination"]
pub type DST_INCR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRLB3_SPEC, u8, DST_INCR_A, 2, O>;
impl<'a, const O: u8> DST_INCR_W<'a, O> {
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
}
#[doc = "Field `IEN` reader - Interrupt Enable Not"]
pub type IEN_R = crate::BitReader<bool>;
#[doc = "Field `IEN` writer - Interrupt Enable Not"]
pub type IEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - Source Address Descriptor"]
    #[inline(always)]
    pub fn src_dscr(&self) -> SRC_DSCR_R {
        SRC_DSCR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Destination Address Descriptor"]
    #[inline(always)]
    pub fn dst_dscr(&self) -> DST_DSCR_R {
        DST_DSCR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Flow Control"]
    #[inline(always)]
    pub fn fc(&self) -> FC_R {
        FC_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Incrementing, Decrementing or Fixed Address for the Source"]
    #[inline(always)]
    pub fn src_incr(&self) -> SRC_INCR_R {
        SRC_INCR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Incrementing, Decrementing or Fixed Address for the Destination"]
    #[inline(always)]
    pub fn dst_incr(&self) -> DST_INCR_R {
        DST_INCR_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Interrupt Enable Not"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Source Address Descriptor"]
    #[inline(always)]
    pub fn src_dscr(&mut self) -> SRC_DSCR_W<16> {
        SRC_DSCR_W::new(self)
    }
    #[doc = "Bit 20 - Destination Address Descriptor"]
    #[inline(always)]
    pub fn dst_dscr(&mut self) -> DST_DSCR_W<20> {
        DST_DSCR_W::new(self)
    }
    #[doc = "Bits 21:22 - Flow Control"]
    #[inline(always)]
    pub fn fc(&mut self) -> FC_W<21> {
        FC_W::new(self)
    }
    #[doc = "Bits 24:25 - Incrementing, Decrementing or Fixed Address for the Source"]
    #[inline(always)]
    pub fn src_incr(&mut self) -> SRC_INCR_W<24> {
        SRC_INCR_W::new(self)
    }
    #[doc = "Bits 28:29 - Incrementing, Decrementing or Fixed Address for the Destination"]
    #[inline(always)]
    pub fn dst_incr(&mut self) -> DST_INCR_W<28> {
        DST_INCR_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt Enable Not"]
    #[inline(always)]
    pub fn ien(&mut self) -> IEN_W<30> {
        IEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Control B Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb3](index.html) module"]
pub struct CTRLB3_SPEC;
impl crate::RegisterSpec for CTRLB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlb3::R](R) reader structure"]
impl crate::Readable for CTRLB3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb3::W](W) writer structure"]
impl crate::Writable for CTRLB3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLB3 to value 0"]
impl crate::Resettable for CTRLB3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
