#[doc = "Register `MAN` reader"]
pub type R = crate::R<ManSpec>;
#[doc = "Register `MAN` writer"]
pub type W = crate::W<ManSpec>;
#[doc = "Field `TX_PL` reader - Transmitter Preamble Length"]
pub type TxPlR = crate::FieldReader;
#[doc = "Field `TX_PL` writer - Transmitter Preamble Length"]
pub type TxPlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Transmitter Preamble Pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxPp {
    #[doc = "0: The preamble is composed of '1's"]
    AllOne = 0,
    #[doc = "1: The preamble is composed of '0's"]
    AllZero = 1,
    #[doc = "2: The preamble is composed of '01's"]
    ZeroOne = 2,
    #[doc = "3: The preamble is composed of '10's"]
    OneZero = 3,
}
impl From<TxPp> for u8 {
    #[inline(always)]
    fn from(variant: TxPp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TxPp {
    type Ux = u8;
}
impl crate::IsEnum for TxPp {}
#[doc = "Field `TX_PP` reader - Transmitter Preamble Pattern"]
pub type TxPpR = crate::FieldReader<TxPp>;
impl TxPpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxPp {
        match self.bits {
            0 => TxPp::AllOne,
            1 => TxPp::AllZero,
            2 => TxPp::ZeroOne,
            3 => TxPp::OneZero,
            _ => unreachable!(),
        }
    }
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn is_all_one(&self) -> bool {
        *self == TxPp::AllOne
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn is_all_zero(&self) -> bool {
        *self == TxPp::AllZero
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn is_zero_one(&self) -> bool {
        *self == TxPp::ZeroOne
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn is_one_zero(&self) -> bool {
        *self == TxPp::OneZero
    }
}
#[doc = "Field `TX_PP` writer - Transmitter Preamble Pattern"]
pub type TxPpW<'a, REG> = crate::FieldWriter<'a, REG, 2, TxPp, crate::Safe>;
impl<'a, REG> TxPpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn all_one(self) -> &'a mut crate::W<REG> {
        self.variant(TxPp::AllOne)
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn all_zero(self) -> &'a mut crate::W<REG> {
        self.variant(TxPp::AllZero)
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn zero_one(self) -> &'a mut crate::W<REG> {
        self.variant(TxPp::ZeroOne)
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn one_zero(self) -> &'a mut crate::W<REG> {
        self.variant(TxPp::OneZero)
    }
}
#[doc = "Field `TX_MPOL` reader - Transmitter Manchester Polarity"]
pub type TxMpolR = crate::BitReader;
#[doc = "Field `TX_MPOL` writer - Transmitter Manchester Polarity"]
pub type TxMpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PL` reader - Receiver Preamble Length"]
pub type RxPlR = crate::FieldReader;
#[doc = "Field `RX_PL` writer - Receiver Preamble Length"]
pub type RxPlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Receiver Preamble Pattern detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxPp {
    #[doc = "0: The preamble is composed of '1's"]
    AllOne = 0,
    #[doc = "1: The preamble is composed of '0's"]
    AllZero = 1,
    #[doc = "2: The preamble is composed of '01's"]
    ZeroOne = 2,
    #[doc = "3: The preamble is composed of '10's"]
    OneZero = 3,
}
impl From<RxPp> for u8 {
    #[inline(always)]
    fn from(variant: RxPp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RxPp {
    type Ux = u8;
}
impl crate::IsEnum for RxPp {}
#[doc = "Field `RX_PP` reader - Receiver Preamble Pattern detected"]
pub type RxPpR = crate::FieldReader<RxPp>;
impl RxPpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxPp {
        match self.bits {
            0 => RxPp::AllOne,
            1 => RxPp::AllZero,
            2 => RxPp::ZeroOne,
            3 => RxPp::OneZero,
            _ => unreachable!(),
        }
    }
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn is_all_one(&self) -> bool {
        *self == RxPp::AllOne
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn is_all_zero(&self) -> bool {
        *self == RxPp::AllZero
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn is_zero_one(&self) -> bool {
        *self == RxPp::ZeroOne
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn is_one_zero(&self) -> bool {
        *self == RxPp::OneZero
    }
}
#[doc = "Field `RX_PP` writer - Receiver Preamble Pattern detected"]
pub type RxPpW<'a, REG> = crate::FieldWriter<'a, REG, 2, RxPp, crate::Safe>;
impl<'a, REG> RxPpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn all_one(self) -> &'a mut crate::W<REG> {
        self.variant(RxPp::AllOne)
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn all_zero(self) -> &'a mut crate::W<REG> {
        self.variant(RxPp::AllZero)
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn zero_one(self) -> &'a mut crate::W<REG> {
        self.variant(RxPp::ZeroOne)
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn one_zero(self) -> &'a mut crate::W<REG> {
        self.variant(RxPp::OneZero)
    }
}
#[doc = "Field `RX_MPOL` reader - Receiver Manchester Polarity"]
pub type RxMpolR = crate::BitReader;
#[doc = "Field `RX_MPOL` writer - Receiver Manchester Polarity"]
pub type RxMpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONE` reader - Must Be Set to 1"]
pub type OneR = crate::BitReader;
#[doc = "Field `ONE` writer - Must Be Set to 1"]
pub type OneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIFT` reader - Drift Compensation"]
pub type DriftR = crate::BitReader;
#[doc = "Field `DRIFT` writer - Drift Compensation"]
pub type DriftW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    pub fn tx_pl(&self) -> TxPlR {
        TxPlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    pub fn tx_pp(&self) -> TxPpR {
        TxPpR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    pub fn tx_mpol(&self) -> TxMpolR {
        TxMpolR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    pub fn rx_pl(&self) -> RxPlR {
        RxPlR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    pub fn rx_pp(&self) -> RxPpR {
        RxPpR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    pub fn rx_mpol(&self) -> RxMpolR {
        RxMpolR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&self) -> OneR {
        OneR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Drift Compensation"]
    #[inline(always)]
    pub fn drift(&self) -> DriftR {
        DriftR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pl(&mut self) -> TxPlW<ManSpec> {
        TxPlW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pp(&mut self) -> TxPpW<ManSpec> {
        TxPpW::new(self, 8)
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn tx_mpol(&mut self) -> TxMpolW<ManSpec> {
        TxMpolW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pl(&mut self) -> RxPlW<ManSpec> {
        RxPlW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pp(&mut self) -> RxPpW<ManSpec> {
        RxPpW::new(self, 24)
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn rx_mpol(&mut self) -> RxMpolW<ManSpec> {
        RxMpolW::new(self, 28)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn one(&mut self) -> OneW<ManSpec> {
        OneW::new(self, 29)
    }
    #[doc = "Bit 30 - Drift Compensation"]
    #[inline(always)]
    #[must_use]
    pub fn drift(&mut self) -> DriftW<ManSpec> {
        DriftW::new(self, 30)
    }
}
#[doc = "Manchester Encoder Decoder Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`man::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`man::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ManSpec;
impl crate::RegisterSpec for ManSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`man::R`](R) reader structure"]
impl crate::Readable for ManSpec {}
#[doc = "`write(|w| ..)` method takes [`man::W`](W) writer structure"]
impl crate::Writable for ManSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAN to value 0xb001_1004"]
impl crate::Resettable for ManSpec {
    const RESET_VALUE: u32 = 0xb001_1004;
}
