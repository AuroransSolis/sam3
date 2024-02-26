#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Voltage Regulator Off"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vroff {
    #[doc = "0: no effect."]
    NoEffect = 0,
    #[doc = "1: if KEY is correct, asserts the vddcore_nreset and stops the voltage regulator."]
    StopVreg = 1,
}
impl From<Vroff> for bool {
    #[inline(always)]
    fn from(variant: Vroff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VROFF` writer - Voltage Regulator Off"]
pub type VroffW<'a, REG> = crate::BitWriter<'a, REG, Vroff>;
impl<'a, REG> VroffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Vroff::NoEffect)
    }
    #[doc = "if KEY is correct, asserts the vddcore_nreset and stops the voltage regulator."]
    #[inline(always)]
    pub fn stop_vreg(self) -> &'a mut crate::W<REG> {
        self.variant(Vroff::StopVreg)
    }
}
#[doc = "Crystal Oscillator Select"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xtalsel {
    #[doc = "0: no effect."]
    NoEffect = 0,
    #[doc = "1: if KEY is correct, switches the slow clock on the crystal oscillator output."]
    CrystalSel = 1,
}
impl From<Xtalsel> for bool {
    #[inline(always)]
    fn from(variant: Xtalsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTALSEL` writer - Crystal Oscillator Select"]
pub type XtalselW<'a, REG> = crate::BitWriter<'a, REG, Xtalsel>;
impl<'a, REG> XtalselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalsel::NoEffect)
    }
    #[doc = "if KEY is correct, switches the slow clock on the crystal oscillator output."]
    #[inline(always)]
    pub fn crystal_sel(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalsel::CrystalSel)
    }
}
#[doc = "Password"]
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
#[doc = "Field `KEY` writer - Password"]
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
impl W {
    #[doc = "Bit 2 - Voltage Regulator Off"]
    #[inline(always)]
    #[must_use]
    pub fn vroff(&mut self) -> VroffW<CrSpec> {
        VroffW::new(self, 2)
    }
    #[doc = "Bit 3 - Crystal Oscillator Select"]
    #[inline(always)]
    #[must_use]
    pub fn xtalsel(&mut self) -> XtalselW<CrSpec> {
        XtalselW::new(self, 3)
    }
    #[doc = "Bits 24:31 - Password"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Supply Controller Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
