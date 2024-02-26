#[doc = "Register `DMA` reader"]
pub type R = crate::R<DmaSpec>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DmaSpec>;
#[doc = "Field `OFFSET` reader - DMA Write Buffer Offset"]
pub type OffsetR = crate::FieldReader;
#[doc = "Field `OFFSET` writer - DMA Write Buffer Offset"]
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "DMA Channel Read and Write Chunk Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chksize {
    #[doc = "0: 1 data available"]
    _1 = 0,
    #[doc = "1: 4 data available"]
    _4 = 1,
}
impl From<Chksize> for bool {
    #[inline(always)]
    fn from(variant: Chksize) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHKSIZE` reader - DMA Channel Read and Write Chunk Size"]
pub type ChksizeR = crate::BitReader<Chksize>;
impl ChksizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chksize {
        match self.bits {
            false => Chksize::_1,
            true => Chksize::_4,
        }
    }
    #[doc = "1 data available"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Chksize::_1
    }
    #[doc = "4 data available"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Chksize::_4
    }
}
#[doc = "Field `CHKSIZE` writer - DMA Channel Read and Write Chunk Size"]
pub type ChksizeW<'a, REG> = crate::BitWriter<'a, REG, Chksize>;
impl<'a, REG> ChksizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 data available"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Chksize::_1)
    }
    #[doc = "4 data available"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Chksize::_4)
    }
}
#[doc = "Field `DMAEN` reader - DMA Hardware Handshaking Enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA Hardware Handshaking Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROPT` reader - Read Optimization with padding"]
pub type RoptR = crate::BitReader;
#[doc = "Field `ROPT` writer - Read Optimization with padding"]
pub type RoptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - DMA Write Buffer Offset"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    pub fn chksize(&self) -> ChksizeR {
        ChksizeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Read Optimization with padding"]
    #[inline(always)]
    pub fn ropt(&self) -> RoptR {
        RoptR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DMA Write Buffer Offset"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OffsetW<DmaSpec> {
        OffsetW::new(self, 0)
    }
    #[doc = "Bit 4 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    #[must_use]
    pub fn chksize(&mut self) -> ChksizeW<DmaSpec> {
        ChksizeW::new(self, 4)
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<DmaSpec> {
        DmaenW::new(self, 8)
    }
    #[doc = "Bit 12 - Read Optimization with padding"]
    #[inline(always)]
    #[must_use]
    pub fn ropt(&mut self) -> RoptW<DmaSpec> {
        RoptW::new(self, 12)
    }
}
#[doc = "DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSpec;
impl crate::RegisterSpec for DmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DmaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DmaSpec {
    const RESET_VALUE: u32 = 0;
}
