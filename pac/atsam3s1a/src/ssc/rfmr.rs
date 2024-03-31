#[doc = "Register `RFMR` reader"]
pub type R = crate::R<RfmrSpec>;
#[doc = "Register `RFMR` writer"]
pub type W = crate::W<RfmrSpec>;
#[doc = "Field `DATLEN` reader - Data Length"]
pub type DatlenR = crate::FieldReader;
#[doc = "Field `DATLEN` writer - Data Length"]
pub type DatlenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LOOP` reader - Loop Mode"]
pub type LoopR = crate::BitReader;
#[doc = "Field `LOOP` writer - Loop Mode"]
pub type LoopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MsbfR = crate::BitReader;
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MsbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATNB` reader - Data Number per Frame"]
pub type DatnbR = crate::FieldReader;
#[doc = "Field `DATNB` writer - Data Number per Frame"]
pub type DatnbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FSLEN` reader - Receive Frame Sync Length"]
pub type FslenR = crate::FieldReader;
#[doc = "Field `FSLEN` writer - Receive Frame Sync Length"]
pub type FslenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Receive Frame Sync Output Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsos {
    #[doc = "0: None, RF pin is an input"]
    None = 0,
    #[doc = "1: Negative Pulse, RF pin is an output"]
    Negative = 1,
    #[doc = "2: Positive Pulse, RF pin is an output"]
    Positive = 2,
    #[doc = "3: Driven Low during data transfer, RF pin is an output"]
    Low = 3,
    #[doc = "4: Driven High during data transfer, RF pin is an output"]
    High = 4,
    #[doc = "5: Toggling at each start of data transfer, RF pin is an output"]
    Toggling = 5,
}
impl From<Fsos> for u8 {
    #[inline(always)]
    fn from(variant: Fsos) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsos {
    type Ux = u8;
}
impl crate::IsEnum for Fsos {}
#[doc = "Field `FSOS` reader - Receive Frame Sync Output Selection"]
pub type FsosR = crate::FieldReader<Fsos>;
impl FsosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fsos> {
        match self.bits {
            0 => Some(Fsos::None),
            1 => Some(Fsos::Negative),
            2 => Some(Fsos::Positive),
            3 => Some(Fsos::Low),
            4 => Some(Fsos::High),
            5 => Some(Fsos::Toggling),
            _ => None,
        }
    }
    #[doc = "None, RF pin is an input"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Fsos::None
    }
    #[doc = "Negative Pulse, RF pin is an output"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == Fsos::Negative
    }
    #[doc = "Positive Pulse, RF pin is an output"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == Fsos::Positive
    }
    #[doc = "Driven Low during data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Fsos::Low
    }
    #[doc = "Driven High during data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Fsos::High
    }
    #[doc = "Toggling at each start of data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn is_toggling(&self) -> bool {
        *self == Fsos::Toggling
    }
}
#[doc = "Field `FSOS` writer - Receive Frame Sync Output Selection"]
pub type FsosW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fsos>;
impl<'a, REG> FsosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None, RF pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Fsos::None)
    }
    #[doc = "Negative Pulse, RF pin is an output"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(Fsos::Negative)
    }
    #[doc = "Positive Pulse, RF pin is an output"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(Fsos::Positive)
    }
    #[doc = "Driven Low during data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Fsos::Low)
    }
    #[doc = "Driven High during data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Fsos::High)
    }
    #[doc = "Toggling at each start of data transfer, RF pin is an output"]
    #[inline(always)]
    pub fn toggling(self) -> &'a mut crate::W<REG> {
        self.variant(Fsos::Toggling)
    }
}
#[doc = "Frame Sync Edge Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fsedge {
    #[doc = "0: Positive Edge Detection"]
    Positive = 0,
    #[doc = "1: Negative Edge Detection"]
    Negative = 1,
}
impl From<Fsedge> for bool {
    #[inline(always)]
    fn from(variant: Fsedge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSEDGE` reader - Frame Sync Edge Detection"]
pub type FsedgeR = crate::BitReader<Fsedge>;
impl FsedgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsedge {
        match self.bits {
            false => Fsedge::Positive,
            true => Fsedge::Negative,
        }
    }
    #[doc = "Positive Edge Detection"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == Fsedge::Positive
    }
    #[doc = "Negative Edge Detection"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == Fsedge::Negative
    }
}
#[doc = "Field `FSEDGE` writer - Frame Sync Edge Detection"]
pub type FsedgeW<'a, REG> = crate::BitWriter<'a, REG, Fsedge>;
impl<'a, REG> FsedgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Positive Edge Detection"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(Fsedge::Positive)
    }
    #[doc = "Negative Edge Detection"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(Fsedge::Negative)
    }
}
#[doc = "Field `FSLEN_EXT` reader - FSLEN Field Extension"]
pub type FslenExtR = crate::FieldReader;
#[doc = "Field `FSLEN_EXT` writer - FSLEN Field Extension"]
pub type FslenExtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - Data Length"]
    #[inline(always)]
    pub fn datlen(&self) -> DatlenR {
        DatlenR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Loop Mode"]
    #[inline(always)]
    pub fn loop_(&self) -> LoopR {
        LoopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MsbfR {
        MsbfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline(always)]
    pub fn datnb(&self) -> DatnbR {
        DatnbR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Receive Frame Sync Length"]
    #[inline(always)]
    pub fn fslen(&self) -> FslenR {
        FslenR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Receive Frame Sync Output Selection"]
    #[inline(always)]
    pub fn fsos(&self) -> FsosR {
        FsosR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline(always)]
    pub fn fsedge(&self) -> FsedgeR {
        FsedgeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline(always)]
    pub fn fslen_ext(&self) -> FslenExtR {
        FslenExtR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DatlenW<RfmrSpec> {
        DatlenW::new(self, 0)
    }
    #[doc = "Bit 5 - Loop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn loop_(&mut self) -> LoopW<RfmrSpec> {
        LoopW::new(self, 5)
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MsbfW<RfmrSpec> {
        MsbfW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline(always)]
    #[must_use]
    pub fn datnb(&mut self) -> DatnbW<RfmrSpec> {
        DatnbW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Receive Frame Sync Length"]
    #[inline(always)]
    #[must_use]
    pub fn fslen(&mut self) -> FslenW<RfmrSpec> {
        FslenW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Receive Frame Sync Output Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fsos(&mut self) -> FsosW<RfmrSpec> {
        FsosW::new(self, 20)
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline(always)]
    #[must_use]
    pub fn fsedge(&mut self) -> FsedgeW<RfmrSpec> {
        FsedgeW::new(self, 24)
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline(always)]
    #[must_use]
    pub fn fslen_ext(&mut self) -> FslenExtW<RfmrSpec> {
        FslenExtW::new(self, 28)
    }
}
#[doc = "Receive Frame Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfmrSpec;
impl crate::RegisterSpec for RfmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfmr::R`](R) reader structure"]
impl crate::Readable for RfmrSpec {}
#[doc = "`write(|w| ..)` method takes [`rfmr::W`](W) writer structure"]
impl crate::Writable for RfmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFMR to value 0"]
impl crate::Resettable for RfmrSpec {
    const RESET_VALUE: u32 = 0;
}
