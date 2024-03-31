#[doc = "Register `BMR` reader"]
pub type R = crate::R<BmrSpec>;
#[doc = "Register `BMR` writer"]
pub type W = crate::W<BmrSpec>;
#[doc = "External Clock Signal 0 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tc0xc0s {
    #[doc = "0: Signal connected to XC0: TCLK0"]
    Tclk0 = 0,
    #[doc = "2: Signal connected to XC0: TIOA1"]
    Tioa1 = 2,
    #[doc = "3: Signal connected to XC0: TIOA2"]
    Tioa2 = 3,
}
impl From<Tc0xc0s> for u8 {
    #[inline(always)]
    fn from(variant: Tc0xc0s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tc0xc0s {
    type Ux = u8;
}
impl crate::IsEnum for Tc0xc0s {}
#[doc = "Field `TC0XC0S` reader - External Clock Signal 0 Selection"]
pub type Tc0xc0sR = crate::FieldReader<Tc0xc0s>;
impl Tc0xc0sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tc0xc0s> {
        match self.bits {
            0 => Some(Tc0xc0s::Tclk0),
            2 => Some(Tc0xc0s::Tioa1),
            3 => Some(Tc0xc0s::Tioa2),
            _ => None,
        }
    }
    #[doc = "Signal connected to XC0: TCLK0"]
    #[inline(always)]
    pub fn is_tclk0(&self) -> bool {
        *self == Tc0xc0s::Tclk0
    }
    #[doc = "Signal connected to XC0: TIOA1"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        *self == Tc0xc0s::Tioa1
    }
    #[doc = "Signal connected to XC0: TIOA2"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == Tc0xc0s::Tioa2
    }
}
#[doc = "Field `TC0XC0S` writer - External Clock Signal 0 Selection"]
pub type Tc0xc0sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tc0xc0s>;
impl<'a, REG> Tc0xc0sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal connected to XC0: TCLK0"]
    #[inline(always)]
    pub fn tclk0(self) -> &'a mut crate::W<REG> {
        self.variant(Tc0xc0s::Tclk0)
    }
    #[doc = "Signal connected to XC0: TIOA1"]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut crate::W<REG> {
        self.variant(Tc0xc0s::Tioa1)
    }
    #[doc = "Signal connected to XC0: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut crate::W<REG> {
        self.variant(Tc0xc0s::Tioa2)
    }
}
#[doc = "External Clock Signal 1 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tc1xc1s {
    #[doc = "0: Signal connected to XC1: TCLK1"]
    Tclk1 = 0,
    #[doc = "2: Signal connected to XC1: TIOA0"]
    Tioa0 = 2,
    #[doc = "3: Signal connected to XC1: TIOA2"]
    Tioa2 = 3,
}
impl From<Tc1xc1s> for u8 {
    #[inline(always)]
    fn from(variant: Tc1xc1s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tc1xc1s {
    type Ux = u8;
}
impl crate::IsEnum for Tc1xc1s {}
#[doc = "Field `TC1XC1S` reader - External Clock Signal 1 Selection"]
pub type Tc1xc1sR = crate::FieldReader<Tc1xc1s>;
impl Tc1xc1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tc1xc1s> {
        match self.bits {
            0 => Some(Tc1xc1s::Tclk1),
            2 => Some(Tc1xc1s::Tioa0),
            3 => Some(Tc1xc1s::Tioa2),
            _ => None,
        }
    }
    #[doc = "Signal connected to XC1: TCLK1"]
    #[inline(always)]
    pub fn is_tclk1(&self) -> bool {
        *self == Tc1xc1s::Tclk1
    }
    #[doc = "Signal connected to XC1: TIOA0"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        *self == Tc1xc1s::Tioa0
    }
    #[doc = "Signal connected to XC1: TIOA2"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == Tc1xc1s::Tioa2
    }
}
#[doc = "Field `TC1XC1S` writer - External Clock Signal 1 Selection"]
pub type Tc1xc1sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tc1xc1s>;
impl<'a, REG> Tc1xc1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal connected to XC1: TCLK1"]
    #[inline(always)]
    pub fn tclk1(self) -> &'a mut crate::W<REG> {
        self.variant(Tc1xc1s::Tclk1)
    }
    #[doc = "Signal connected to XC1: TIOA0"]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut crate::W<REG> {
        self.variant(Tc1xc1s::Tioa0)
    }
    #[doc = "Signal connected to XC1: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut crate::W<REG> {
        self.variant(Tc1xc1s::Tioa2)
    }
}
#[doc = "External Clock Signal 2 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tc2xc2s {
    #[doc = "0: Signal connected to XC2: TCLK2"]
    Tclk2 = 0,
    #[doc = "2: Signal connected to XC2: TIOA0"]
    Tioa0 = 2,
    #[doc = "3: Signal connected to XC2: TIOA1"]
    Tioa1 = 3,
}
impl From<Tc2xc2s> for u8 {
    #[inline(always)]
    fn from(variant: Tc2xc2s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tc2xc2s {
    type Ux = u8;
}
impl crate::IsEnum for Tc2xc2s {}
#[doc = "Field `TC2XC2S` reader - External Clock Signal 2 Selection"]
pub type Tc2xc2sR = crate::FieldReader<Tc2xc2s>;
impl Tc2xc2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tc2xc2s> {
        match self.bits {
            0 => Some(Tc2xc2s::Tclk2),
            2 => Some(Tc2xc2s::Tioa0),
            3 => Some(Tc2xc2s::Tioa1),
            _ => None,
        }
    }
    #[doc = "Signal connected to XC2: TCLK2"]
    #[inline(always)]
    pub fn is_tclk2(&self) -> bool {
        *self == Tc2xc2s::Tclk2
    }
    #[doc = "Signal connected to XC2: TIOA0"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        *self == Tc2xc2s::Tioa0
    }
    #[doc = "Signal connected to XC2: TIOA1"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        *self == Tc2xc2s::Tioa1
    }
}
#[doc = "Field `TC2XC2S` writer - External Clock Signal 2 Selection"]
pub type Tc2xc2sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tc2xc2s>;
impl<'a, REG> Tc2xc2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal connected to XC2: TCLK2"]
    #[inline(always)]
    pub fn tclk2(self) -> &'a mut crate::W<REG> {
        self.variant(Tc2xc2s::Tclk2)
    }
    #[doc = "Signal connected to XC2: TIOA0"]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut crate::W<REG> {
        self.variant(Tc2xc2s::Tioa0)
    }
    #[doc = "Signal connected to XC2: TIOA1"]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut crate::W<REG> {
        self.variant(Tc2xc2s::Tioa1)
    }
}
#[doc = "Field `QDEN` reader - Quadrature Decoder Enabled"]
pub type QdenR = crate::BitReader;
#[doc = "Field `QDEN` writer - Quadrature Decoder Enabled"]
pub type QdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSEN` reader - Position Enabled"]
pub type PosenR = crate::BitReader;
#[doc = "Field `POSEN` writer - Position Enabled"]
pub type PosenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEEDEN` reader - Speed Enabled"]
pub type SpeedenR = crate::BitReader;
#[doc = "Field `SPEEDEN` writer - Speed Enabled"]
pub type SpeedenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDTRANS` reader - Quadrature Decoding Transparent"]
pub type QdtransR = crate::BitReader;
#[doc = "Field `QDTRANS` writer - Quadrature Decoding Transparent"]
pub type QdtransW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGPHA` reader - Edge on PHA Count Mode"]
pub type EdgphaR = crate::BitReader;
#[doc = "Field `EDGPHA` writer - Edge on PHA Count Mode"]
pub type EdgphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVA` reader - Inverted PHA"]
pub type InvaR = crate::BitReader;
#[doc = "Field `INVA` writer - Inverted PHA"]
pub type InvaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVB` reader - Inverted PHB"]
pub type InvbR = crate::BitReader;
#[doc = "Field `INVB` writer - Inverted PHB"]
pub type InvbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVIDX` reader - Inverted Index"]
pub type InvidxR = crate::BitReader;
#[doc = "Field `INVIDX` writer - Inverted Index"]
pub type InvidxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP` reader - Swap PHA and PHB"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - Swap PHA and PHB"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDXPHB` reader - Index Pin is PHB Pin"]
pub type IdxphbR = crate::BitReader;
#[doc = "Field `IDXPHB` writer - Index Pin is PHB Pin"]
pub type IdxphbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAXFILT` reader - Maximum Filter"]
pub type MaxfiltR = crate::FieldReader;
#[doc = "Field `MAXFILT` writer - Maximum Filter"]
pub type MaxfiltW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    pub fn tc0xc0s(&self) -> Tc0xc0sR {
        Tc0xc0sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    pub fn tc1xc1s(&self) -> Tc1xc1sR {
        Tc1xc1sR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    pub fn tc2xc2s(&self) -> Tc2xc2sR {
        Tc2xc2sR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Quadrature Decoder Enabled"]
    #[inline(always)]
    pub fn qden(&self) -> QdenR {
        QdenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Position Enabled"]
    #[inline(always)]
    pub fn posen(&self) -> PosenR {
        PosenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Speed Enabled"]
    #[inline(always)]
    pub fn speeden(&self) -> SpeedenR {
        SpeedenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Quadrature Decoding Transparent"]
    #[inline(always)]
    pub fn qdtrans(&self) -> QdtransR {
        QdtransR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Edge on PHA Count Mode"]
    #[inline(always)]
    pub fn edgpha(&self) -> EdgphaR {
        EdgphaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Inverted PHA"]
    #[inline(always)]
    pub fn inva(&self) -> InvaR {
        InvaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Inverted PHB"]
    #[inline(always)]
    pub fn invb(&self) -> InvbR {
        InvbR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Inverted Index"]
    #[inline(always)]
    pub fn invidx(&self) -> InvidxR {
        InvidxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Swap PHA and PHB"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Index Pin is PHB Pin"]
    #[inline(always)]
    pub fn idxphb(&self) -> IdxphbR {
        IdxphbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:25 - Maximum Filter"]
    #[inline(always)]
    pub fn maxfilt(&self) -> MaxfiltR {
        MaxfiltR::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tc0xc0s(&mut self) -> Tc0xc0sW<BmrSpec> {
        Tc0xc0sW::new(self, 0)
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tc1xc1s(&mut self) -> Tc1xc1sW<BmrSpec> {
        Tc1xc1sW::new(self, 2)
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tc2xc2s(&mut self) -> Tc2xc2sW<BmrSpec> {
        Tc2xc2sW::new(self, 4)
    }
    #[doc = "Bit 8 - Quadrature Decoder Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn qden(&mut self) -> QdenW<BmrSpec> {
        QdenW::new(self, 8)
    }
    #[doc = "Bit 9 - Position Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn posen(&mut self) -> PosenW<BmrSpec> {
        PosenW::new(self, 9)
    }
    #[doc = "Bit 10 - Speed Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn speeden(&mut self) -> SpeedenW<BmrSpec> {
        SpeedenW::new(self, 10)
    }
    #[doc = "Bit 11 - Quadrature Decoding Transparent"]
    #[inline(always)]
    #[must_use]
    pub fn qdtrans(&mut self) -> QdtransW<BmrSpec> {
        QdtransW::new(self, 11)
    }
    #[doc = "Bit 12 - Edge on PHA Count Mode"]
    #[inline(always)]
    #[must_use]
    pub fn edgpha(&mut self) -> EdgphaW<BmrSpec> {
        EdgphaW::new(self, 12)
    }
    #[doc = "Bit 13 - Inverted PHA"]
    #[inline(always)]
    #[must_use]
    pub fn inva(&mut self) -> InvaW<BmrSpec> {
        InvaW::new(self, 13)
    }
    #[doc = "Bit 14 - Inverted PHB"]
    #[inline(always)]
    #[must_use]
    pub fn invb(&mut self) -> InvbW<BmrSpec> {
        InvbW::new(self, 14)
    }
    #[doc = "Bit 15 - Inverted Index"]
    #[inline(always)]
    #[must_use]
    pub fn invidx(&mut self) -> InvidxW<BmrSpec> {
        InvidxW::new(self, 15)
    }
    #[doc = "Bit 16 - Swap PHA and PHB"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SwapW<BmrSpec> {
        SwapW::new(self, 16)
    }
    #[doc = "Bit 17 - Index Pin is PHB Pin"]
    #[inline(always)]
    #[must_use]
    pub fn idxphb(&mut self) -> IdxphbW<BmrSpec> {
        IdxphbW::new(self, 17)
    }
    #[doc = "Bits 20:25 - Maximum Filter"]
    #[inline(always)]
    #[must_use]
    pub fn maxfilt(&mut self) -> MaxfiltW<BmrSpec> {
        MaxfiltW::new(self, 20)
    }
}
#[doc = "Block Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmrSpec;
impl crate::RegisterSpec for BmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmr::R`](R) reader structure"]
impl crate::Readable for BmrSpec {}
#[doc = "`write(|w| ..)` method takes [`bmr::W`](W) writer structure"]
impl crate::Writable for BmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BMR to value 0"]
impl crate::Resettable for BmrSpec {
    const RESET_VALUE: u32 = 0;
}
