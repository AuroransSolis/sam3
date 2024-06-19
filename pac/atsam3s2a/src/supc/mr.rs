#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Brownout Detector Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bodrsten {
    #[doc = "0: the core reset signal \"vddcore_nreset\" is not affected when a brownout detection occurs."]
    NotEnable = 0,
    #[doc = "1: the core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    Enable = 1,
}
impl From<Bodrsten> for bool {
    #[inline(always)]
    fn from(variant: Bodrsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODRSTEN` reader - Brownout Detector Reset Enable"]
pub type BodrstenR = crate::BitReader<Bodrsten>;
impl BodrstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodrsten {
        match self.bits {
            false => Bodrsten::NotEnable,
            true => Bodrsten::Enable,
        }
    }
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a brownout detection occurs."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Bodrsten::NotEnable
    }
    #[doc = "the core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bodrsten::Enable
    }
}
#[doc = "Field `BODRSTEN` writer - Brownout Detector Reset Enable"]
pub type BodrstenW<'a, REG> = crate::BitWriter<'a, REG, Bodrsten>;
impl<'a, REG> BodrstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a brownout detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bodrsten::NotEnable)
    }
    #[doc = "the core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bodrsten::Enable)
    }
}
#[doc = "Brownout Detector Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boddis {
    #[doc = "0: the core brownout detector is enabled."]
    Enable = 0,
    #[doc = "1: the core brownout detector is disabled."]
    Disable = 1,
}
impl From<Boddis> for bool {
    #[inline(always)]
    fn from(variant: Boddis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODDIS` reader - Brownout Detector Disable"]
pub type BoddisR = crate::BitReader<Boddis>;
impl BoddisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boddis {
        match self.bits {
            false => Boddis::Enable,
            true => Boddis::Disable,
        }
    }
    #[doc = "the core brownout detector is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Boddis::Enable
    }
    #[doc = "the core brownout detector is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Boddis::Disable
    }
}
#[doc = "Field `BODDIS` writer - Brownout Detector Disable"]
pub type BoddisW<'a, REG> = crate::BitWriter<'a, REG, Boddis>;
impl<'a, REG> BoddisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the core brownout detector is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Boddis::Enable)
    }
    #[doc = "the core brownout detector is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Boddis::Disable)
    }
}
#[doc = "Voltage Regulator enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Onreg {
    #[doc = "0: Internal voltage regulator is not used (external power supply is used)"]
    OnregUnused = 0,
    #[doc = "1: internal voltage regulator is used"]
    OnregUsed = 1,
}
impl From<Onreg> for bool {
    #[inline(always)]
    fn from(variant: Onreg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONREG` reader - Voltage Regulator enable"]
pub type OnregR = crate::BitReader<Onreg>;
impl OnregR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Onreg {
        match self.bits {
            false => Onreg::OnregUnused,
            true => Onreg::OnregUsed,
        }
    }
    #[doc = "Internal voltage regulator is not used (external power supply is used)"]
    #[inline(always)]
    pub fn is_onreg_unused(&self) -> bool {
        *self == Onreg::OnregUnused
    }
    #[doc = "internal voltage regulator is used"]
    #[inline(always)]
    pub fn is_onreg_used(&self) -> bool {
        *self == Onreg::OnregUsed
    }
}
#[doc = "Field `ONREG` writer - Voltage Regulator enable"]
pub type OnregW<'a, REG> = crate::BitWriter<'a, REG, Onreg>;
impl<'a, REG> OnregW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal voltage regulator is not used (external power supply is used)"]
    #[inline(always)]
    pub fn onreg_unused(self) -> &'a mut crate::W<REG> {
        self.variant(Onreg::OnregUnused)
    }
    #[doc = "internal voltage regulator is used"]
    #[inline(always)]
    pub fn onreg_used(self) -> &'a mut crate::W<REG> {
        self.variant(Onreg::OnregUsed)
    }
}
#[doc = "Oscillator Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscbypass {
    #[doc = "0: no effect. Clock selection depends on XTALSEL value."]
    NoEffect = 0,
    #[doc = "1: the 32-KHz XTAL oscillator is selected and is put in bypass mode."]
    Bypass = 1,
}
impl From<Oscbypass> for bool {
    #[inline(always)]
    fn from(variant: Oscbypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCBYPASS` reader - Oscillator Bypass"]
pub type OscbypassR = crate::BitReader<Oscbypass>;
impl OscbypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscbypass {
        match self.bits {
            false => Oscbypass::NoEffect,
            true => Oscbypass::Bypass,
        }
    }
    #[doc = "no effect. Clock selection depends on XTALSEL value."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Oscbypass::NoEffect
    }
    #[doc = "the 32-KHz XTAL oscillator is selected and is put in bypass mode."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Oscbypass::Bypass
    }
}
#[doc = "Field `OSCBYPASS` writer - Oscillator Bypass"]
pub type OscbypassW<'a, REG> = crate::BitWriter<'a, REG, Oscbypass>;
impl<'a, REG> OscbypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect. Clock selection depends on XTALSEL value."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Oscbypass::NoEffect)
    }
    #[doc = "the 32-KHz XTAL oscillator is selected and is put in bypass mode."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Oscbypass::Bypass)
    }
}
#[doc = "Password Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "165: Writing any other value in this field aborts the write operation."]
    Passwd = 165,
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u8;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` reader - Password Key"]
pub type KeyR = crate::FieldReader<Key>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Key> {
        match self.bits {
            165 => Some(Key::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == Key::Passwd
    }
}
#[doc = "Field `KEY` writer - Password Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Passwd)
    }
}
impl R {
    #[doc = "Bit 12 - Brownout Detector Reset Enable"]
    #[inline(always)]
    pub fn bodrsten(&self) -> BodrstenR {
        BodrstenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Brownout Detector Disable"]
    #[inline(always)]
    pub fn boddis(&self) -> BoddisR {
        BoddisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Voltage Regulator enable"]
    #[inline(always)]
    pub fn onreg(&self) -> OnregR {
        OnregR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    pub fn oscbypass(&self) -> OscbypassR {
        OscbypassR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - Brownout Detector Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bodrsten(&mut self) -> BodrstenW<MrSpec> {
        BodrstenW::new(self, 12)
    }
    #[doc = "Bit 13 - Brownout Detector Disable"]
    #[inline(always)]
    #[must_use]
    pub fn boddis(&mut self) -> BoddisW<MrSpec> {
        BoddisW::new(self, 13)
    }
    #[doc = "Bit 14 - Voltage Regulator enable"]
    #[inline(always)]
    #[must_use]
    pub fn onreg(&mut self) -> OnregW<MrSpec> {
        OnregW::new(self, 14)
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn oscbypass(&mut self) -> OscbypassW<MrSpec> {
        OscbypassW::new(self, 20)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<MrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Supply Controller Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0x5a00"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0x5a00;
}
