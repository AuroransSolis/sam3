#[doc = "Register `CDR` writer"]
pub type W = crate::W<CDR_SPEC>;
#[doc = "Field `CDR_HW0_DATA` writer - Data field of the lower CDR half-word"]
pub type CDR_HW0_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Channel select field of the lower CDR half-word\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDR_HW0_CHSEL_AW {
    #[doc = "0: Channel 0"]
    Channel0 = 0,
    #[doc = "1: Channel 1"]
    Channel1 = 1,
}
impl From<CDR_HW0_CHSEL_AW> for u8 {
    #[inline(always)]
    fn from(variant: CDR_HW0_CHSEL_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CDR_HW0_CHSEL_AW {
    type Ux = u8;
}
#[doc = "Field `CDR_HW0_CHSEL` writer - Channel select field of the lower CDR half-word"]
pub type CDR_HW0_CHSEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, CDR_HW0_CHSEL_AW>;
impl<'a, REG, const O: u8> CDR_HW0_CHSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(CDR_HW0_CHSEL_AW::Channel0)
    }
    #[doc = "Channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(CDR_HW0_CHSEL_AW::Channel1)
    }
}
#[doc = "Field `CDR_HW1_DATA` writer - Data field of the upper CDR half-word"]
pub type CDR_HW1_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Channel select field of the upper CDR half-word\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDR_HW1_CHSEL_AW {
    #[doc = "0: Channel 0"]
    Channel0 = 0,
    #[doc = "1: Channel 1"]
    Channel1 = 1,
}
impl From<CDR_HW1_CHSEL_AW> for u8 {
    #[inline(always)]
    fn from(variant: CDR_HW1_CHSEL_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CDR_HW1_CHSEL_AW {
    type Ux = u8;
}
#[doc = "Field `CDR_HW1_CHSEL` writer - Channel select field of the upper CDR half-word"]
pub type CDR_HW1_CHSEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, CDR_HW1_CHSEL_AW>;
impl<'a, REG, const O: u8> CDR_HW1_CHSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(CDR_HW1_CHSEL_AW::Channel0)
    }
    #[doc = "Channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(CDR_HW1_CHSEL_AW::Channel1)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data field of the lower CDR half-word"]
    #[inline(always)]
    #[must_use]
    pub fn cdr_hw0_data(&mut self) -> CDR_HW0_DATA_W<CDR_SPEC, 0> {
        CDR_HW0_DATA_W::new(self)
    }
    #[doc = "Bits 12:13 - Channel select field of the lower CDR half-word"]
    #[inline(always)]
    #[must_use]
    pub fn cdr_hw0_chsel(&mut self) -> CDR_HW0_CHSEL_W<CDR_SPEC, 12> {
        CDR_HW0_CHSEL_W::new(self)
    }
    #[doc = "Bits 16:27 - Data field of the upper CDR half-word"]
    #[inline(always)]
    #[must_use]
    pub fn cdr_hw1_data(&mut self) -> CDR_HW1_DATA_W<CDR_SPEC, 16> {
        CDR_HW1_DATA_W::new(self)
    }
    #[doc = "Bits 28:29 - Channel select field of the upper CDR half-word"]
    #[inline(always)]
    #[must_use]
    pub fn cdr_hw1_chsel(&mut self) -> CDR_HW1_CHSEL_W<CDR_SPEC, 28> {
        CDR_HW1_CHSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Conversion Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cdr::W`](W) writer structure"]
impl crate::Writable for CDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
