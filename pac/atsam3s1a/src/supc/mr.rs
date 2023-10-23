#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `BODRSTEN` reader - Brownout Detector Reset Enable"]
pub type BODRSTEN_R = crate::BitReader<BODRSTEN_A>;
#[doc = "Brownout Detector Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BODRSTEN_A {
    #[doc = "0: the core reset signal \"vddcore_nreset\" is not affected when a brownout detection occurs."]
    NotEnable = 0,
    #[doc = "1: the core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    Enable = 1,
}
impl From<BODRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: BODRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BODRSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BODRSTEN_A {
        match self.bits {
            false => BODRSTEN_A::NotEnable,
            true => BODRSTEN_A::Enable,
        }
    }
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a brownout detection occurs."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == BODRSTEN_A::NotEnable
    }
    #[doc = "the core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODRSTEN_A::Enable
    }
}
#[doc = "Field `BODRSTEN` writer - Brownout Detector Reset Enable"]
pub type BODRSTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BODRSTEN_A>;
impl<'a, REG, const O: u8> BODRSTEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a brownout detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(BODRSTEN_A::NotEnable)
    }
    #[doc = "the core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BODRSTEN_A::Enable)
    }
}
#[doc = "Field `BODDIS` reader - Brownout Detector Disable"]
pub type BODDIS_R = crate::BitReader<BODDIS_A>;
#[doc = "Brownout Detector Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BODDIS_A {
    #[doc = "0: the core brownout detector is enabled."]
    Enable = 0,
    #[doc = "1: the core brownout detector is disabled."]
    Disable = 1,
}
impl From<BODDIS_A> for bool {
    #[inline(always)]
    fn from(variant: BODDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl BODDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BODDIS_A {
        match self.bits {
            false => BODDIS_A::Enable,
            true => BODDIS_A::Disable,
        }
    }
    #[doc = "the core brownout detector is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODDIS_A::Enable
    }
    #[doc = "the core brownout detector is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODDIS_A::Disable
    }
}
#[doc = "Field `BODDIS` writer - Brownout Detector Disable"]
pub type BODDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BODDIS_A>;
impl<'a, REG, const O: u8> BODDIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the core brownout detector is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BODDIS_A::Enable)
    }
    #[doc = "the core brownout detector is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BODDIS_A::Disable)
    }
}
#[doc = "Field `ONREG` reader - Voltage Regulator enable"]
pub type ONREG_R = crate::BitReader<ONREG_A>;
#[doc = "Voltage Regulator enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONREG_A {
    #[doc = "0: Internal voltage regulator is not used (external power supply is used)"]
    OnregUnused = 0,
    #[doc = "1: internal voltage regulator is used"]
    OnregUsed = 1,
}
impl From<ONREG_A> for bool {
    #[inline(always)]
    fn from(variant: ONREG_A) -> Self {
        variant as u8 != 0
    }
}
impl ONREG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ONREG_A {
        match self.bits {
            false => ONREG_A::OnregUnused,
            true => ONREG_A::OnregUsed,
        }
    }
    #[doc = "Internal voltage regulator is not used (external power supply is used)"]
    #[inline(always)]
    pub fn is_onreg_unused(&self) -> bool {
        *self == ONREG_A::OnregUnused
    }
    #[doc = "internal voltage regulator is used"]
    #[inline(always)]
    pub fn is_onreg_used(&self) -> bool {
        *self == ONREG_A::OnregUsed
    }
}
#[doc = "Field `ONREG` writer - Voltage Regulator enable"]
pub type ONREG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ONREG_A>;
impl<'a, REG, const O: u8> ONREG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal voltage regulator is not used (external power supply is used)"]
    #[inline(always)]
    pub fn onreg_unused(self) -> &'a mut crate::W<REG> {
        self.variant(ONREG_A::OnregUnused)
    }
    #[doc = "internal voltage regulator is used"]
    #[inline(always)]
    pub fn onreg_used(self) -> &'a mut crate::W<REG> {
        self.variant(ONREG_A::OnregUsed)
    }
}
#[doc = "Field `OSCBYPASS` reader - Oscillator Bypass"]
pub type OSCBYPASS_R = crate::BitReader<OSCBYPASS_A>;
#[doc = "Oscillator Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCBYPASS_A {
    #[doc = "0: no effect. Clock selection depends on XTALSEL value."]
    NoEffect = 0,
    #[doc = "1: the 32-KHz XTAL oscillator is selected and is put in bypass mode."]
    Bypass = 1,
}
impl From<OSCBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: OSCBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl OSCBYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSCBYPASS_A {
        match self.bits {
            false => OSCBYPASS_A::NoEffect,
            true => OSCBYPASS_A::Bypass,
        }
    }
    #[doc = "no effect. Clock selection depends on XTALSEL value."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OSCBYPASS_A::NoEffect
    }
    #[doc = "the 32-KHz XTAL oscillator is selected and is put in bypass mode."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == OSCBYPASS_A::Bypass
    }
}
#[doc = "Field `OSCBYPASS` writer - Oscillator Bypass"]
pub type OSCBYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OSCBYPASS_A>;
impl<'a, REG, const O: u8> OSCBYPASS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect. Clock selection depends on XTALSEL value."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OSCBYPASS_A::NoEffect)
    }
    #[doc = "the 32-KHz XTAL oscillator is selected and is put in bypass mode."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(OSCBYPASS_A::Bypass)
    }
}
#[doc = "Field `KEY` reader - Password Key"]
pub type KEY_R = crate::FieldReader<KEY_A>;
#[doc = "Password Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "165: Writing any other value in this field aborts the write operation."]
    Passwd = 165,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEY_A {
    type Ux = u8;
}
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            165 => Some(KEY_A::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == KEY_A::Passwd
    }
}
#[doc = "Field `KEY` writer - Password Key"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, KEY_A>;
impl<'a, REG, const O: u8> KEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::Passwd)
    }
}
impl R {
    #[doc = "Bit 12 - Brownout Detector Reset Enable"]
    #[inline(always)]
    pub fn bodrsten(&self) -> BODRSTEN_R {
        BODRSTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Brownout Detector Disable"]
    #[inline(always)]
    pub fn boddis(&self) -> BODDIS_R {
        BODDIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Voltage Regulator enable"]
    #[inline(always)]
    pub fn onreg(&self) -> ONREG_R {
        ONREG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    pub fn oscbypass(&self) -> OSCBYPASS_R {
        OSCBYPASS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - Brownout Detector Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bodrsten(&mut self) -> BODRSTEN_W<MR_SPEC, 12> {
        BODRSTEN_W::new(self)
    }
    #[doc = "Bit 13 - Brownout Detector Disable"]
    #[inline(always)]
    #[must_use]
    pub fn boddis(&mut self) -> BODDIS_W<MR_SPEC, 13> {
        BODDIS_W::new(self)
    }
    #[doc = "Bit 14 - Voltage Regulator enable"]
    #[inline(always)]
    #[must_use]
    pub fn onreg(&mut self) -> ONREG_W<MR_SPEC, 14> {
        ONREG_W::new(self)
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn oscbypass(&mut self) -> OSCBYPASS_W<MR_SPEC, 20> {
        OSCBYPASS_W::new(self)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<MR_SPEC, 24> {
        KEY_W::new(self)
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
#[doc = "Supply Controller Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0x5a00"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0x5a00;
}
