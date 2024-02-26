#[doc = "Register `CTRLB0` reader"]
pub type R = crate::R<Ctrlb0Spec>;
#[doc = "Register `CTRLB0` writer"]
pub type W = crate::W<Ctrlb0Spec>;
#[doc = "Source Address Descriptor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SrcDscr {
    #[doc = "0: Source address is updated when the descriptor is fetched from the memory."]
    FetchFromMem = 0,
    #[doc = "1: Buffer Descriptor Fetch operation is disabled for the source."]
    FetchDisable = 1,
}
impl From<SrcDscr> for bool {
    #[inline(always)]
    fn from(variant: SrcDscr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC_DSCR` reader - Source Address Descriptor"]
pub type SrcDscrR = crate::BitReader<SrcDscr>;
impl SrcDscrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SrcDscr {
        match self.bits {
            false => SrcDscr::FetchFromMem,
            true => SrcDscr::FetchDisable,
        }
    }
    #[doc = "Source address is updated when the descriptor is fetched from the memory."]
    #[inline(always)]
    pub fn is_fetch_from_mem(&self) -> bool {
        *self == SrcDscr::FetchFromMem
    }
    #[doc = "Buffer Descriptor Fetch operation is disabled for the source."]
    #[inline(always)]
    pub fn is_fetch_disable(&self) -> bool {
        *self == SrcDscr::FetchDisable
    }
}
#[doc = "Field `SRC_DSCR` writer - Source Address Descriptor"]
pub type SrcDscrW<'a, REG> = crate::BitWriter<'a, REG, SrcDscr>;
impl<'a, REG> SrcDscrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Source address is updated when the descriptor is fetched from the memory."]
    #[inline(always)]
    pub fn fetch_from_mem(self) -> &'a mut crate::W<REG> {
        self.variant(SrcDscr::FetchFromMem)
    }
    #[doc = "Buffer Descriptor Fetch operation is disabled for the source."]
    #[inline(always)]
    pub fn fetch_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SrcDscr::FetchDisable)
    }
}
#[doc = "Destination Address Descriptor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DstDscr {
    #[doc = "0: Destination address is updated when the descriptor is fetched from the memory."]
    FetchFromMem = 0,
    #[doc = "1: Buffer Descriptor Fetch operation is disabled for the destination."]
    FetchDisable = 1,
}
impl From<DstDscr> for bool {
    #[inline(always)]
    fn from(variant: DstDscr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DST_DSCR` reader - Destination Address Descriptor"]
pub type DstDscrR = crate::BitReader<DstDscr>;
impl DstDscrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DstDscr {
        match self.bits {
            false => DstDscr::FetchFromMem,
            true => DstDscr::FetchDisable,
        }
    }
    #[doc = "Destination address is updated when the descriptor is fetched from the memory."]
    #[inline(always)]
    pub fn is_fetch_from_mem(&self) -> bool {
        *self == DstDscr::FetchFromMem
    }
    #[doc = "Buffer Descriptor Fetch operation is disabled for the destination."]
    #[inline(always)]
    pub fn is_fetch_disable(&self) -> bool {
        *self == DstDscr::FetchDisable
    }
}
#[doc = "Field `DST_DSCR` writer - Destination Address Descriptor"]
pub type DstDscrW<'a, REG> = crate::BitWriter<'a, REG, DstDscr>;
impl<'a, REG> DstDscrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Destination address is updated when the descriptor is fetched from the memory."]
    #[inline(always)]
    pub fn fetch_from_mem(self) -> &'a mut crate::W<REG> {
        self.variant(DstDscr::FetchFromMem)
    }
    #[doc = "Buffer Descriptor Fetch operation is disabled for the destination."]
    #[inline(always)]
    pub fn fetch_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DstDscr::FetchDisable)
    }
}
#[doc = "Flow Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fc {
    #[doc = "0: Memory-to-Memory Transfer DMAC is flow controller"]
    Mem2memDmaFc = 0,
    #[doc = "1: Memory-to-Peripheral Transfer DMAC is flow controller"]
    Mem2perDmaFc = 1,
    #[doc = "2: Peripheral-to-Memory Transfer DMAC is flow controller"]
    Per2memDmaFc = 2,
    #[doc = "3: Peripheral-to-Peripheral Transfer DMAC is flow controller"]
    Per2perDmaFc = 3,
}
impl From<Fc> for u8 {
    #[inline(always)]
    fn from(variant: Fc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fc {
    type Ux = u8;
}
#[doc = "Field `FC` reader - Flow Control"]
pub type FcR = crate::FieldReader<Fc>;
impl FcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fc {
        match self.bits {
            0 => Fc::Mem2memDmaFc,
            1 => Fc::Mem2perDmaFc,
            2 => Fc::Per2memDmaFc,
            3 => Fc::Per2perDmaFc,
            _ => unreachable!(),
        }
    }
    #[doc = "Memory-to-Memory Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn is_mem2mem_dma_fc(&self) -> bool {
        *self == Fc::Mem2memDmaFc
    }
    #[doc = "Memory-to-Peripheral Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn is_mem2per_dma_fc(&self) -> bool {
        *self == Fc::Mem2perDmaFc
    }
    #[doc = "Peripheral-to-Memory Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn is_per2mem_dma_fc(&self) -> bool {
        *self == Fc::Per2memDmaFc
    }
    #[doc = "Peripheral-to-Peripheral Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn is_per2per_dma_fc(&self) -> bool {
        *self == Fc::Per2perDmaFc
    }
}
#[doc = "Field `FC` writer - Flow Control"]
pub type FcW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Fc>;
impl<'a, REG> FcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Memory-to-Memory Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn mem2mem_dma_fc(self) -> &'a mut crate::W<REG> {
        self.variant(Fc::Mem2memDmaFc)
    }
    #[doc = "Memory-to-Peripheral Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn mem2per_dma_fc(self) -> &'a mut crate::W<REG> {
        self.variant(Fc::Mem2perDmaFc)
    }
    #[doc = "Peripheral-to-Memory Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn per2mem_dma_fc(self) -> &'a mut crate::W<REG> {
        self.variant(Fc::Per2memDmaFc)
    }
    #[doc = "Peripheral-to-Peripheral Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn per2per_dma_fc(self) -> &'a mut crate::W<REG> {
        self.variant(Fc::Per2perDmaFc)
    }
}
#[doc = "Incrementing, Decrementing or Fixed Address for the Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SrcIncr {
    #[doc = "0: The source address is incremented"]
    Incrementing = 0,
    #[doc = "1: The source address is decremented"]
    Decrementing = 1,
    #[doc = "2: The source address remains unchanged"]
    Fixed = 2,
}
impl From<SrcIncr> for u8 {
    #[inline(always)]
    fn from(variant: SrcIncr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SrcIncr {
    type Ux = u8;
}
#[doc = "Field `SRC_INCR` reader - Incrementing, Decrementing or Fixed Address for the Source"]
pub type SrcIncrR = crate::FieldReader<SrcIncr>;
impl SrcIncrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SrcIncr> {
        match self.bits {
            0 => Some(SrcIncr::Incrementing),
            1 => Some(SrcIncr::Decrementing),
            2 => Some(SrcIncr::Fixed),
            _ => None,
        }
    }
    #[doc = "The source address is incremented"]
    #[inline(always)]
    pub fn is_incrementing(&self) -> bool {
        *self == SrcIncr::Incrementing
    }
    #[doc = "The source address is decremented"]
    #[inline(always)]
    pub fn is_decrementing(&self) -> bool {
        *self == SrcIncr::Decrementing
    }
    #[doc = "The source address remains unchanged"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == SrcIncr::Fixed
    }
}
#[doc = "Field `SRC_INCR` writer - Incrementing, Decrementing or Fixed Address for the Source"]
pub type SrcIncrW<'a, REG> = crate::FieldWriter<'a, REG, 2, SrcIncr>;
impl<'a, REG> SrcIncrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The source address is incremented"]
    #[inline(always)]
    pub fn incrementing(self) -> &'a mut crate::W<REG> {
        self.variant(SrcIncr::Incrementing)
    }
    #[doc = "The source address is decremented"]
    #[inline(always)]
    pub fn decrementing(self) -> &'a mut crate::W<REG> {
        self.variant(SrcIncr::Decrementing)
    }
    #[doc = "The source address remains unchanged"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(SrcIncr::Fixed)
    }
}
#[doc = "Incrementing, Decrementing or Fixed Address for the Destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DstIncr {
    #[doc = "0: The destination address is incremented"]
    Incrementing = 0,
    #[doc = "1: The destination address is decremented"]
    Decrementing = 1,
    #[doc = "2: The destination address remains unchanged"]
    Fixed = 2,
}
impl From<DstIncr> for u8 {
    #[inline(always)]
    fn from(variant: DstIncr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DstIncr {
    type Ux = u8;
}
#[doc = "Field `DST_INCR` reader - Incrementing, Decrementing or Fixed Address for the Destination"]
pub type DstIncrR = crate::FieldReader<DstIncr>;
impl DstIncrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DstIncr> {
        match self.bits {
            0 => Some(DstIncr::Incrementing),
            1 => Some(DstIncr::Decrementing),
            2 => Some(DstIncr::Fixed),
            _ => None,
        }
    }
    #[doc = "The destination address is incremented"]
    #[inline(always)]
    pub fn is_incrementing(&self) -> bool {
        *self == DstIncr::Incrementing
    }
    #[doc = "The destination address is decremented"]
    #[inline(always)]
    pub fn is_decrementing(&self) -> bool {
        *self == DstIncr::Decrementing
    }
    #[doc = "The destination address remains unchanged"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == DstIncr::Fixed
    }
}
#[doc = "Field `DST_INCR` writer - Incrementing, Decrementing or Fixed Address for the Destination"]
pub type DstIncrW<'a, REG> = crate::FieldWriter<'a, REG, 2, DstIncr>;
impl<'a, REG> DstIncrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The destination address is incremented"]
    #[inline(always)]
    pub fn incrementing(self) -> &'a mut crate::W<REG> {
        self.variant(DstIncr::Incrementing)
    }
    #[doc = "The destination address is decremented"]
    #[inline(always)]
    pub fn decrementing(self) -> &'a mut crate::W<REG> {
        self.variant(DstIncr::Decrementing)
    }
    #[doc = "The destination address remains unchanged"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(DstIncr::Fixed)
    }
}
#[doc = "Field `IEN` reader - Interrupt Enable Not"]
pub type IenR = crate::BitReader;
#[doc = "Field `IEN` writer - Interrupt Enable Not"]
pub type IenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Source Address Descriptor"]
    #[inline(always)]
    pub fn src_dscr(&self) -> SrcDscrR {
        SrcDscrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Destination Address Descriptor"]
    #[inline(always)]
    pub fn dst_dscr(&self) -> DstDscrR {
        DstDscrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Flow Control"]
    #[inline(always)]
    pub fn fc(&self) -> FcR {
        FcR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Incrementing, Decrementing or Fixed Address for the Source"]
    #[inline(always)]
    pub fn src_incr(&self) -> SrcIncrR {
        SrcIncrR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Incrementing, Decrementing or Fixed Address for the Destination"]
    #[inline(always)]
    pub fn dst_incr(&self) -> DstIncrR {
        DstIncrR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Interrupt Enable Not"]
    #[inline(always)]
    pub fn ien(&self) -> IenR {
        IenR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Source Address Descriptor"]
    #[inline(always)]
    #[must_use]
    pub fn src_dscr(&mut self) -> SrcDscrW<Ctrlb0Spec> {
        SrcDscrW::new(self, 16)
    }
    #[doc = "Bit 20 - Destination Address Descriptor"]
    #[inline(always)]
    #[must_use]
    pub fn dst_dscr(&mut self) -> DstDscrW<Ctrlb0Spec> {
        DstDscrW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Flow Control"]
    #[inline(always)]
    #[must_use]
    pub fn fc(&mut self) -> FcW<Ctrlb0Spec> {
        FcW::new(self, 21)
    }
    #[doc = "Bits 24:25 - Incrementing, Decrementing or Fixed Address for the Source"]
    #[inline(always)]
    #[must_use]
    pub fn src_incr(&mut self) -> SrcIncrW<Ctrlb0Spec> {
        SrcIncrW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Incrementing, Decrementing or Fixed Address for the Destination"]
    #[inline(always)]
    #[must_use]
    pub fn dst_incr(&mut self) -> DstIncrW<Ctrlb0Spec> {
        DstIncrW::new(self, 28)
    }
    #[doc = "Bit 30 - Interrupt Enable Not"]
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IenW<Ctrlb0Spec> {
        IenW::new(self, 30)
    }
}
#[doc = "DMAC Channel Control B Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrlb0Spec;
impl crate::RegisterSpec for Ctrlb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb0::R`](R) reader structure"]
impl crate::Readable for Ctrlb0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb0::W`](W) writer structure"]
impl crate::Writable for Ctrlb0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLB0 to value 0"]
impl crate::Resettable for Ctrlb0Spec {
    const RESET_VALUE: u32 = 0;
}
