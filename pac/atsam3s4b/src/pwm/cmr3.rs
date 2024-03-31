#[doc = "Register `CMR3` reader"]
pub type R = crate::R<Cmr3Spec>;
#[doc = "Register `CMR3` writer"]
pub type W = crate::W<Cmr3Spec>;
#[doc = "Channel Pre-scaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpre {
    #[doc = "0: Master clock"]
    Mck = 0,
    #[doc = "1: Master clock/2"]
    MckDiv2 = 1,
    #[doc = "2: Master clock/4"]
    MckDiv4 = 2,
    #[doc = "3: Master clock/8"]
    MckDiv8 = 3,
    #[doc = "4: Master clock/16"]
    MckDiv16 = 4,
    #[doc = "5: Master clock/32"]
    MckDiv32 = 5,
    #[doc = "6: Master clock/64"]
    MckDiv64 = 6,
    #[doc = "7: Master clock/128"]
    MckDiv128 = 7,
    #[doc = "8: Master clock/256"]
    MckDiv256 = 8,
    #[doc = "9: Master clock/512"]
    MckDiv512 = 9,
    #[doc = "10: Master clock/1024"]
    MckDiv1024 = 10,
    #[doc = "11: Clock A"]
    Clka = 11,
    #[doc = "12: Clock B"]
    Clkb = 12,
}
impl From<Cpre> for u8 {
    #[inline(always)]
    fn from(variant: Cpre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpre {
    type Ux = u8;
}
impl crate::IsEnum for Cpre {}
#[doc = "Field `CPRE` reader - Channel Pre-scaler"]
pub type CpreR = crate::FieldReader<Cpre>;
impl CpreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpre> {
        match self.bits {
            0 => Some(Cpre::Mck),
            1 => Some(Cpre::MckDiv2),
            2 => Some(Cpre::MckDiv4),
            3 => Some(Cpre::MckDiv8),
            4 => Some(Cpre::MckDiv16),
            5 => Some(Cpre::MckDiv32),
            6 => Some(Cpre::MckDiv64),
            7 => Some(Cpre::MckDiv128),
            8 => Some(Cpre::MckDiv256),
            9 => Some(Cpre::MckDiv512),
            10 => Some(Cpre::MckDiv1024),
            11 => Some(Cpre::Clka),
            12 => Some(Cpre::Clkb),
            _ => None,
        }
    }
    #[doc = "Master clock"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Cpre::Mck
    }
    #[doc = "Master clock/2"]
    #[inline(always)]
    pub fn is_mck_div_2(&self) -> bool {
        *self == Cpre::MckDiv2
    }
    #[doc = "Master clock/4"]
    #[inline(always)]
    pub fn is_mck_div_4(&self) -> bool {
        *self == Cpre::MckDiv4
    }
    #[doc = "Master clock/8"]
    #[inline(always)]
    pub fn is_mck_div_8(&self) -> bool {
        *self == Cpre::MckDiv8
    }
    #[doc = "Master clock/16"]
    #[inline(always)]
    pub fn is_mck_div_16(&self) -> bool {
        *self == Cpre::MckDiv16
    }
    #[doc = "Master clock/32"]
    #[inline(always)]
    pub fn is_mck_div_32(&self) -> bool {
        *self == Cpre::MckDiv32
    }
    #[doc = "Master clock/64"]
    #[inline(always)]
    pub fn is_mck_div_64(&self) -> bool {
        *self == Cpre::MckDiv64
    }
    #[doc = "Master clock/128"]
    #[inline(always)]
    pub fn is_mck_div_128(&self) -> bool {
        *self == Cpre::MckDiv128
    }
    #[doc = "Master clock/256"]
    #[inline(always)]
    pub fn is_mck_div_256(&self) -> bool {
        *self == Cpre::MckDiv256
    }
    #[doc = "Master clock/512"]
    #[inline(always)]
    pub fn is_mck_div_512(&self) -> bool {
        *self == Cpre::MckDiv512
    }
    #[doc = "Master clock/1024"]
    #[inline(always)]
    pub fn is_mck_div_1024(&self) -> bool {
        *self == Cpre::MckDiv1024
    }
    #[doc = "Clock A"]
    #[inline(always)]
    pub fn is_clka(&self) -> bool {
        *self == Cpre::Clka
    }
    #[doc = "Clock B"]
    #[inline(always)]
    pub fn is_clkb(&self) -> bool {
        *self == Cpre::Clkb
    }
}
#[doc = "Field `CPRE` writer - Channel Pre-scaler"]
pub type CpreW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cpre>;
impl<'a, REG> CpreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Master clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Mck)
    }
    #[doc = "Master clock/2"]
    #[inline(always)]
    pub fn mck_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::MckDiv2)
    }
    #[doc = "Master clock/4"]
    #[inline(always)]
    pub fn mck_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::MckDiv4)
    }
    #[doc = "Master clock/8"]
    #[inline(always)]
    pub fn mck_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::MckDiv8)
    }
    #[doc = "Master clock/16"]
    #[inline(always)]
    pub fn mck_div_16(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::MckDiv16)
    }
    #[doc = "Master clock/32"]
    #[inline(always)]
    pub fn mck_div_32(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::MckDiv32)
    }
    #[doc = "Master clock/64"]
    #[inline(always)]
    pub fn mck_div_64(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::MckDiv64)
    }
    #[doc = "Master clock/128"]
    #[inline(always)]
    pub fn mck_div_128(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::MckDiv128)
    }
    #[doc = "Master clock/256"]
    #[inline(always)]
    pub fn mck_div_256(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::MckDiv256)
    }
    #[doc = "Master clock/512"]
    #[inline(always)]
    pub fn mck_div_512(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::MckDiv512)
    }
    #[doc = "Master clock/1024"]
    #[inline(always)]
    pub fn mck_div_1024(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::MckDiv1024)
    }
    #[doc = "Clock A"]
    #[inline(always)]
    pub fn clka(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Clka)
    }
    #[doc = "Clock B"]
    #[inline(always)]
    pub fn clkb(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Clkb)
    }
}
#[doc = "Field `CALG` reader - Channel Alignment"]
pub type CalgR = crate::BitReader;
#[doc = "Field `CALG` writer - Channel Alignment"]
pub type CalgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Channel Polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Channel Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CES` reader - Counter Event Selection"]
pub type CesR = crate::BitReader;
#[doc = "Field `CES` writer - Counter Event Selection"]
pub type CesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTE` reader - Dead-Time Generator Enable"]
pub type DteR = crate::BitReader;
#[doc = "Field `DTE` writer - Dead-Time Generator Enable"]
pub type DteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTHI` reader - Dead-Time PWMHx Output Inverted"]
pub type DthiR = crate::BitReader;
#[doc = "Field `DTHI` writer - Dead-Time PWMHx Output Inverted"]
pub type DthiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTLI` reader - Dead-Time PWMLx Output Inverted"]
pub type DtliR = crate::BitReader;
#[doc = "Field `DTLI` writer - Dead-Time PWMLx Output Inverted"]
pub type DtliW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&self) -> CpreR {
        CpreR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&self) -> CalgR {
        CalgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    pub fn ces(&self) -> CesR {
        CesR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    pub fn dte(&self) -> DteR {
        DteR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    pub fn dthi(&self) -> DthiR {
        DthiR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    pub fn dtli(&self) -> DtliR {
        DtliR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    #[must_use]
    pub fn cpre(&mut self) -> CpreW<Cmr3Spec> {
        CpreW::new(self, 0)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    #[must_use]
    pub fn calg(&mut self) -> CalgW<Cmr3Spec> {
        CalgW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CpolW<Cmr3Spec> {
        CpolW::new(self, 9)
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ces(&mut self) -> CesW<Cmr3Spec> {
        CesW::new(self, 10)
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dte(&mut self) -> DteW<Cmr3Spec> {
        DteW::new(self, 16)
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    #[must_use]
    pub fn dthi(&mut self) -> DthiW<Cmr3Spec> {
        DthiW::new(self, 17)
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    #[must_use]
    pub fn dtli(&mut self) -> DtliW<Cmr3Spec> {
        DtliW::new(self, 18)
    }
}
#[doc = "PWM Channel Mode Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmr3Spec;
impl crate::RegisterSpec for Cmr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmr3::R`](R) reader structure"]
impl crate::Readable for Cmr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cmr3::W`](W) writer structure"]
impl crate::Writable for Cmr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMR3 to value 0"]
impl crate::Resettable for Cmr3Spec {
    const RESET_VALUE: u32 = 0;
}
