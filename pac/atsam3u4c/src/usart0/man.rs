#[doc = "Register `MAN` reader"]
pub type R = crate::R<MAN_SPEC>;
#[doc = "Register `MAN` writer"]
pub type W = crate::W<MAN_SPEC>;
#[doc = "Field `TX_PL` reader - Transmitter Preamble Length"]
pub type TX_PL_R = crate::FieldReader;
#[doc = "Field `TX_PL` writer - Transmitter Preamble Length"]
pub type TX_PL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_PP` reader - Transmitter Preamble Pattern"]
pub type TX_PP_R = crate::FieldReader<TX_PP_A>;
#[doc = "Transmitter Preamble Pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_PP_A {
    #[doc = "0: The preamble is composed of '1's"]
    AllOne = 0,
    #[doc = "1: The preamble is composed of '0's"]
    AllZero = 1,
    #[doc = "2: The preamble is composed of '01's"]
    ZeroOne = 2,
    #[doc = "3: The preamble is composed of '10's"]
    OneZero = 3,
}
impl From<TX_PP_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TX_PP_A {
    type Ux = u8;
}
impl TX_PP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_PP_A {
        match self.bits {
            0 => TX_PP_A::AllOne,
            1 => TX_PP_A::AllZero,
            2 => TX_PP_A::ZeroOne,
            3 => TX_PP_A::OneZero,
            _ => unreachable!(),
        }
    }
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn is_all_one(&self) -> bool {
        *self == TX_PP_A::AllOne
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn is_all_zero(&self) -> bool {
        *self == TX_PP_A::AllZero
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn is_zero_one(&self) -> bool {
        *self == TX_PP_A::ZeroOne
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn is_one_zero(&self) -> bool {
        *self == TX_PP_A::OneZero
    }
}
#[doc = "Field `TX_PP` writer - Transmitter Preamble Pattern"]
pub type TX_PP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TX_PP_A>;
impl<'a, REG> TX_PP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn all_one(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PP_A::AllOne)
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn all_zero(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PP_A::AllZero)
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn zero_one(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PP_A::ZeroOne)
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn one_zero(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PP_A::OneZero)
    }
}
#[doc = "Field `TX_MPOL` reader - Transmitter Manchester Polarity"]
pub type TX_MPOL_R = crate::BitReader;
#[doc = "Field `TX_MPOL` writer - Transmitter Manchester Polarity"]
pub type TX_MPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PL` reader - Receiver Preamble Length"]
pub type RX_PL_R = crate::FieldReader;
#[doc = "Field `RX_PL` writer - Receiver Preamble Length"]
pub type RX_PL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_PP` reader - Receiver Preamble Pattern detected"]
pub type RX_PP_R = crate::FieldReader<RX_PP_A>;
#[doc = "Receiver Preamble Pattern detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_PP_A {
    #[doc = "0: The preamble is composed of '1's"]
    AllOne = 0,
    #[doc = "1: The preamble is composed of '0's"]
    AllZero = 1,
    #[doc = "2: The preamble is composed of '01's"]
    ZeroOne = 2,
    #[doc = "3: The preamble is composed of '10's"]
    OneZero = 3,
}
impl From<RX_PP_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RX_PP_A {
    type Ux = u8;
}
impl RX_PP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_PP_A {
        match self.bits {
            0 => RX_PP_A::AllOne,
            1 => RX_PP_A::AllZero,
            2 => RX_PP_A::ZeroOne,
            3 => RX_PP_A::OneZero,
            _ => unreachable!(),
        }
    }
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn is_all_one(&self) -> bool {
        *self == RX_PP_A::AllOne
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn is_all_zero(&self) -> bool {
        *self == RX_PP_A::AllZero
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn is_zero_one(&self) -> bool {
        *self == RX_PP_A::ZeroOne
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn is_one_zero(&self) -> bool {
        *self == RX_PP_A::OneZero
    }
}
#[doc = "Field `RX_PP` writer - Receiver Preamble Pattern detected"]
pub type RX_PP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RX_PP_A>;
impl<'a, REG> RX_PP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn all_one(self) -> &'a mut crate::W<REG> {
        self.variant(RX_PP_A::AllOne)
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn all_zero(self) -> &'a mut crate::W<REG> {
        self.variant(RX_PP_A::AllZero)
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn zero_one(self) -> &'a mut crate::W<REG> {
        self.variant(RX_PP_A::ZeroOne)
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn one_zero(self) -> &'a mut crate::W<REG> {
        self.variant(RX_PP_A::OneZero)
    }
}
#[doc = "Field `RX_MPOL` reader - Receiver Manchester Polarity"]
pub type RX_MPOL_R = crate::BitReader;
#[doc = "Field `RX_MPOL` writer - Receiver Manchester Polarity"]
pub type RX_MPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONE` reader - Must Be Set to 1"]
pub type ONE_R = crate::BitReader;
#[doc = "Field `ONE` writer - Must Be Set to 1"]
pub type ONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIFT` reader - Drift Compensation"]
pub type DRIFT_R = crate::BitReader;
#[doc = "Field `DRIFT` writer - Drift Compensation"]
pub type DRIFT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    pub fn tx_pl(&self) -> TX_PL_R {
        TX_PL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    pub fn tx_pp(&self) -> TX_PP_R {
        TX_PP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    pub fn tx_mpol(&self) -> TX_MPOL_R {
        TX_MPOL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    pub fn rx_pl(&self) -> RX_PL_R {
        RX_PL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    pub fn rx_pp(&self) -> RX_PP_R {
        RX_PP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    pub fn rx_mpol(&self) -> RX_MPOL_R {
        RX_MPOL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Drift Compensation"]
    #[inline(always)]
    pub fn drift(&self) -> DRIFT_R {
        DRIFT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pl(&mut self) -> TX_PL_W<MAN_SPEC> {
        TX_PL_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pp(&mut self) -> TX_PP_W<MAN_SPEC> {
        TX_PP_W::new(self, 8)
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn tx_mpol(&mut self) -> TX_MPOL_W<MAN_SPEC> {
        TX_MPOL_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pl(&mut self) -> RX_PL_W<MAN_SPEC> {
        RX_PL_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pp(&mut self) -> RX_PP_W<MAN_SPEC> {
        RX_PP_W::new(self, 24)
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn rx_mpol(&mut self) -> RX_MPOL_W<MAN_SPEC> {
        RX_MPOL_W::new(self, 28)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn one(&mut self) -> ONE_W<MAN_SPEC> {
        ONE_W::new(self, 29)
    }
    #[doc = "Bit 30 - Drift Compensation"]
    #[inline(always)]
    #[must_use]
    pub fn drift(&mut self) -> DRIFT_W<MAN_SPEC> {
        DRIFT_W::new(self, 30)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Manchester Encoder Decoder Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`man::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`man::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAN_SPEC;
impl crate::RegisterSpec for MAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`man::R`](R) reader structure"]
impl crate::Readable for MAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`man::W`](W) writer structure"]
impl crate::Writable for MAN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAN to value 0xb001_1004"]
impl crate::Resettable for MAN_SPEC {
    const RESET_VALUE: Self::Ux = 0xb001_1004;
}
