#[doc = "Register `PCMR` reader"]
pub type R = crate::R<PcmrSpec>;
#[doc = "Register `PCMR` writer"]
pub type W = crate::W<PcmrSpec>;
#[doc = "Field `PCEN` reader - Parallel Capture Mode Enable"]
pub type PcenR = crate::BitReader;
#[doc = "Field `PCEN` writer - Parallel Capture Mode Enable"]
pub type PcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Parallel Capture Mode Data Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsize {
    #[doc = "0: The reception data in the PIO_PCRHR register is a BYTE (8-bit)"]
    Byte = 0,
    #[doc = "1: The reception data in the PIO_PCRHR register is a HALF-WORD (16-bit)"]
    Halfword = 1,
    #[doc = "2: The reception data in the PIO_PCRHR register is a WORD (32-bit)"]
    Word = 2,
}
impl From<Dsize> for u8 {
    #[inline(always)]
    fn from(variant: Dsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsize {
    type Ux = u8;
}
impl crate::IsEnum for Dsize {}
#[doc = "Field `DSIZE` reader - Parallel Capture Mode Data Size"]
pub type DsizeR = crate::FieldReader<Dsize>;
impl DsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dsize> {
        match self.bits {
            0 => Some(Dsize::Byte),
            1 => Some(Dsize::Halfword),
            2 => Some(Dsize::Word),
            _ => None,
        }
    }
    #[doc = "The reception data in the PIO_PCRHR register is a BYTE (8-bit)"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Dsize::Byte
    }
    #[doc = "The reception data in the PIO_PCRHR register is a HALF-WORD (16-bit)"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == Dsize::Halfword
    }
    #[doc = "The reception data in the PIO_PCRHR register is a WORD (32-bit)"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == Dsize::Word
    }
}
#[doc = "Field `DSIZE` writer - Parallel Capture Mode Data Size"]
pub type DsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dsize>;
impl<'a, REG> DsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The reception data in the PIO_PCRHR register is a BYTE (8-bit)"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Dsize::Byte)
    }
    #[doc = "The reception data in the PIO_PCRHR register is a HALF-WORD (16-bit)"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut crate::W<REG> {
        self.variant(Dsize::Halfword)
    }
    #[doc = "The reception data in the PIO_PCRHR register is a WORD (32-bit)"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(Dsize::Word)
    }
}
#[doc = "Field `ALWYS` reader - Parallel Capture Mode Always Sampling"]
pub type AlwysR = crate::BitReader;
#[doc = "Field `ALWYS` writer - Parallel Capture Mode Always Sampling"]
pub type AlwysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALFS` reader - Parallel Capture Mode Half Sampling"]
pub type HalfsR = crate::BitReader;
#[doc = "Field `HALFS` writer - Parallel Capture Mode Half Sampling"]
pub type HalfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRSTS` reader - Parallel Capture Mode First Sample"]
pub type FrstsR = crate::BitReader;
#[doc = "Field `FRSTS` writer - Parallel Capture Mode First Sample"]
pub type FrstsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parallel Capture Mode Enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PcenR {
        PcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parallel Capture Mode Data Size"]
    #[inline(always)]
    pub fn dsize(&self) -> DsizeR {
        DsizeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 9 - Parallel Capture Mode Always Sampling"]
    #[inline(always)]
    pub fn alwys(&self) -> AlwysR {
        AlwysR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parallel Capture Mode Half Sampling"]
    #[inline(always)]
    pub fn halfs(&self) -> HalfsR {
        HalfsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Parallel Capture Mode First Sample"]
    #[inline(always)]
    pub fn frsts(&self) -> FrstsR {
        FrstsR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parallel Capture Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PcenW<PcmrSpec> {
        PcenW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Parallel Capture Mode Data Size"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DsizeW<PcmrSpec> {
        DsizeW::new(self, 4)
    }
    #[doc = "Bit 9 - Parallel Capture Mode Always Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn alwys(&mut self) -> AlwysW<PcmrSpec> {
        AlwysW::new(self, 9)
    }
    #[doc = "Bit 10 - Parallel Capture Mode Half Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn halfs(&mut self) -> HalfsW<PcmrSpec> {
        HalfsW::new(self, 10)
    }
    #[doc = "Bit 11 - Parallel Capture Mode First Sample"]
    #[inline(always)]
    #[must_use]
    pub fn frsts(&mut self) -> FrstsW<PcmrSpec> {
        FrstsW::new(self, 11)
    }
}
#[doc = "Parallel Capture Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcmrSpec;
impl crate::RegisterSpec for PcmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcmr::R`](R) reader structure"]
impl crate::Readable for PcmrSpec {}
#[doc = "`write(|w| ..)` method takes [`pcmr::W`](W) writer structure"]
impl crate::Writable for PcmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCMR to value 0"]
impl crate::Resettable for PcmrSpec {
    const RESET_VALUE: u32 = 0;
}
