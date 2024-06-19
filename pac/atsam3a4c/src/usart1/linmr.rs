#[doc = "Register `LINMR` reader"]
pub type R = crate::R<LinmrSpec>;
#[doc = "Register `LINMR` writer"]
pub type W = crate::W<LinmrSpec>;
#[doc = "LIN Node Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nact {
    #[doc = "0: The USART transmits the response."]
    Publish = 0,
    #[doc = "1: The USART receives the response."]
    Subscribe = 1,
    #[doc = "2: The USART does not transmit and does not receive the response."]
    Ignore = 2,
}
impl From<Nact> for u8 {
    #[inline(always)]
    fn from(variant: Nact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nact {
    type Ux = u8;
}
impl crate::IsEnum for Nact {}
#[doc = "Field `NACT` reader - LIN Node Action"]
pub type NactR = crate::FieldReader<Nact>;
impl NactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nact> {
        match self.bits {
            0 => Some(Nact::Publish),
            1 => Some(Nact::Subscribe),
            2 => Some(Nact::Ignore),
            _ => None,
        }
    }
    #[doc = "The USART transmits the response."]
    #[inline(always)]
    pub fn is_publish(&self) -> bool {
        *self == Nact::Publish
    }
    #[doc = "The USART receives the response."]
    #[inline(always)]
    pub fn is_subscribe(&self) -> bool {
        *self == Nact::Subscribe
    }
    #[doc = "The USART does not transmit and does not receive the response."]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == Nact::Ignore
    }
}
#[doc = "Field `NACT` writer - LIN Node Action"]
pub type NactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nact>;
impl<'a, REG> NactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The USART transmits the response."]
    #[inline(always)]
    pub fn publish(self) -> &'a mut crate::W<REG> {
        self.variant(Nact::Publish)
    }
    #[doc = "The USART receives the response."]
    #[inline(always)]
    pub fn subscribe(self) -> &'a mut crate::W<REG> {
        self.variant(Nact::Subscribe)
    }
    #[doc = "The USART does not transmit and does not receive the response."]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut crate::W<REG> {
        self.variant(Nact::Ignore)
    }
}
#[doc = "Field `PARDIS` reader - Parity Disable"]
pub type PardisR = crate::BitReader;
#[doc = "Field `PARDIS` writer - Parity Disable"]
pub type PardisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHKDIS` reader - Checksum Disable"]
pub type ChkdisR = crate::BitReader;
#[doc = "Field `CHKDIS` writer - Checksum Disable"]
pub type ChkdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHKTYP` reader - Checksum Type"]
pub type ChktypR = crate::BitReader;
#[doc = "Field `CHKTYP` writer - Checksum Type"]
pub type ChktypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLM` reader - Data Length Mode"]
pub type DlmR = crate::BitReader;
#[doc = "Field `DLM` writer - Data Length Mode"]
pub type DlmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSDIS` reader - Frame Slot Mode Disable"]
pub type FsdisR = crate::BitReader;
#[doc = "Field `FSDIS` writer - Frame Slot Mode Disable"]
pub type FsdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPTYP` reader - Wakeup Signal Type"]
pub type WkuptypR = crate::BitReader;
#[doc = "Field `WKUPTYP` writer - Wakeup Signal Type"]
pub type WkuptypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLC` reader - Data Length Control"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - Data Length Control"]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PDCM` reader - PDC Mode"]
pub type PdcmR = crate::BitReader;
#[doc = "Field `PDCM` writer - PDC Mode"]
pub type PdcmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - LIN Node Action"]
    #[inline(always)]
    pub fn nact(&self) -> NactR {
        NactR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Parity Disable"]
    #[inline(always)]
    pub fn pardis(&self) -> PardisR {
        PardisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Checksum Disable"]
    #[inline(always)]
    pub fn chkdis(&self) -> ChkdisR {
        ChkdisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Checksum Type"]
    #[inline(always)]
    pub fn chktyp(&self) -> ChktypR {
        ChktypR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Length Mode"]
    #[inline(always)]
    pub fn dlm(&self) -> DlmR {
        DlmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Frame Slot Mode Disable"]
    #[inline(always)]
    pub fn fsdis(&self) -> FsdisR {
        FsdisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup Signal Type"]
    #[inline(always)]
    pub fn wkuptyp(&self) -> WkuptypR {
        WkuptypR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Data Length Control"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - PDC Mode"]
    #[inline(always)]
    pub fn pdcm(&self) -> PdcmR {
        PdcmR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - LIN Node Action"]
    #[inline(always)]
    #[must_use]
    pub fn nact(&mut self) -> NactW<LinmrSpec> {
        NactW::new(self, 0)
    }
    #[doc = "Bit 2 - Parity Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pardis(&mut self) -> PardisW<LinmrSpec> {
        PardisW::new(self, 2)
    }
    #[doc = "Bit 3 - Checksum Disable"]
    #[inline(always)]
    #[must_use]
    pub fn chkdis(&mut self) -> ChkdisW<LinmrSpec> {
        ChkdisW::new(self, 3)
    }
    #[doc = "Bit 4 - Checksum Type"]
    #[inline(always)]
    #[must_use]
    pub fn chktyp(&mut self) -> ChktypW<LinmrSpec> {
        ChktypW::new(self, 4)
    }
    #[doc = "Bit 5 - Data Length Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dlm(&mut self) -> DlmW<LinmrSpec> {
        DlmW::new(self, 5)
    }
    #[doc = "Bit 6 - Frame Slot Mode Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fsdis(&mut self) -> FsdisW<LinmrSpec> {
        FsdisW::new(self, 6)
    }
    #[doc = "Bit 7 - Wakeup Signal Type"]
    #[inline(always)]
    #[must_use]
    pub fn wkuptyp(&mut self) -> WkuptypW<LinmrSpec> {
        WkuptypW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Data Length Control"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DlcW<LinmrSpec> {
        DlcW::new(self, 8)
    }
    #[doc = "Bit 16 - PDC Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pdcm(&mut self) -> PdcmW<LinmrSpec> {
        PdcmW::new(self, 16)
    }
}
#[doc = "LIN Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`linmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinmrSpec;
impl crate::RegisterSpec for LinmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linmr::R`](R) reader structure"]
impl crate::Readable for LinmrSpec {}
#[doc = "`write(|w| ..)` method takes [`linmr::W`](W) writer structure"]
impl crate::Writable for LinmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINMR to value 0"]
impl crate::Resettable for LinmrSpec {
    const RESET_VALUE: u32 = 0;
}
