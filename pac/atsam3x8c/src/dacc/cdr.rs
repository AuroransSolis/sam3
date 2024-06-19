#[doc = "Register `CDR` writer"]
pub type W = crate::W<CdrSpec>;
#[doc = "Field `CDR_HW0_DATA` writer - Data field of the lower CDR half-word"]
pub type CdrHw0DataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Channel select field of the lower CDR half-word\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CdrHw0Chsel {
    #[doc = "0: Channel 0"]
    Channel0 = 0,
    #[doc = "1: Channel 1"]
    Channel1 = 1,
}
impl From<CdrHw0Chsel> for u8 {
    #[inline(always)]
    fn from(variant: CdrHw0Chsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CdrHw0Chsel {
    type Ux = u8;
}
impl crate::IsEnum for CdrHw0Chsel {}
#[doc = "Field `CDR_HW0_CHSEL` writer - Channel select field of the lower CDR half-word"]
pub type CdrHw0ChselW<'a, REG> = crate::FieldWriter<'a, REG, 2, CdrHw0Chsel>;
impl<'a, REG> CdrHw0ChselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(CdrHw0Chsel::Channel0)
    }
    #[doc = "Channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(CdrHw0Chsel::Channel1)
    }
}
#[doc = "Field `CDR_HW1_DATA` writer - Data field of the upper CDR half-word"]
pub type CdrHw1DataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Channel select field of the upper CDR half-word\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CdrHw1Chsel {
    #[doc = "0: Channel 0"]
    Channel0 = 0,
    #[doc = "1: Channel 1"]
    Channel1 = 1,
}
impl From<CdrHw1Chsel> for u8 {
    #[inline(always)]
    fn from(variant: CdrHw1Chsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CdrHw1Chsel {
    type Ux = u8;
}
impl crate::IsEnum for CdrHw1Chsel {}
#[doc = "Field `CDR_HW1_CHSEL` writer - Channel select field of the upper CDR half-word"]
pub type CdrHw1ChselW<'a, REG> = crate::FieldWriter<'a, REG, 2, CdrHw1Chsel>;
impl<'a, REG> CdrHw1ChselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(CdrHw1Chsel::Channel0)
    }
    #[doc = "Channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(CdrHw1Chsel::Channel1)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data field of the lower CDR half-word"]
    #[inline(always)]
    #[must_use]
    pub fn cdr_hw0_data(&mut self) -> CdrHw0DataW<CdrSpec> {
        CdrHw0DataW::new(self, 0)
    }
    #[doc = "Bits 12:13 - Channel select field of the lower CDR half-word"]
    #[inline(always)]
    #[must_use]
    pub fn cdr_hw0_chsel(&mut self) -> CdrHw0ChselW<CdrSpec> {
        CdrHw0ChselW::new(self, 12)
    }
    #[doc = "Bits 16:27 - Data field of the upper CDR half-word"]
    #[inline(always)]
    #[must_use]
    pub fn cdr_hw1_data(&mut self) -> CdrHw1DataW<CdrSpec> {
        CdrHw1DataW::new(self, 16)
    }
    #[doc = "Bits 28:29 - Channel select field of the upper CDR half-word"]
    #[inline(always)]
    #[must_use]
    pub fn cdr_hw1_chsel(&mut self) -> CdrHw1ChselW<CdrSpec> {
        CdrHw1ChselW::new(self, 28)
    }
}
#[doc = "Conversion Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdrSpec;
impl crate::RegisterSpec for CdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cdr::W`](W) writer structure"]
impl crate::Writable for CdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CdrSpec {
    const RESET_VALUE: u32 = 0;
}
