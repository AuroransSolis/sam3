#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Voltage Regulator Off"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VROFF_AW {
    #[doc = "0: no effect."]
    NoEffect = 0,
    #[doc = "1: if KEY is correct, asserts the vddcore_nreset and stops the voltage regulator."]
    StopVreg = 1,
}
impl From<VROFF_AW> for bool {
    #[inline(always)]
    fn from(variant: VROFF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VROFF` writer - Voltage Regulator Off"]
pub type VROFF_W<'a, REG> = crate::BitWriter<'a, REG, VROFF_AW>;
impl<'a, REG> VROFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(VROFF_AW::NoEffect)
    }
    #[doc = "if KEY is correct, asserts the vddcore_nreset and stops the voltage regulator."]
    #[inline(always)]
    pub fn stop_vreg(self) -> &'a mut crate::W<REG> {
        self.variant(VROFF_AW::StopVreg)
    }
}
#[doc = "Crystal Oscillator Select"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XTALSEL_AW {
    #[doc = "0: no effect."]
    NoEffect = 0,
    #[doc = "1: if KEY is correct, switches the slow clock on the crystal oscillator output."]
    CrystalSel = 1,
}
impl From<XTALSEL_AW> for bool {
    #[inline(always)]
    fn from(variant: XTALSEL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTALSEL` writer - Crystal Oscillator Select"]
pub type XTALSEL_W<'a, REG> = crate::BitWriter<'a, REG, XTALSEL_AW>;
impl<'a, REG> XTALSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(XTALSEL_AW::NoEffect)
    }
    #[doc = "if KEY is correct, switches the slow clock on the crystal oscillator output."]
    #[inline(always)]
    pub fn crystal_sel(self) -> &'a mut crate::W<REG> {
        self.variant(XTALSEL_AW::CrystalSel)
    }
}
#[doc = "Password"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_AW {
    #[doc = "165: Writing any other value in this field aborts the write operation."]
    Passwd = 165,
}
impl From<KEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEY_AW {
    type Ux = u8;
}
#[doc = "Field `KEY` writer - Password"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, KEY_AW>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_AW::Passwd)
    }
}
impl W {
    #[doc = "Bit 2 - Voltage Regulator Off"]
    #[inline(always)]
    #[must_use]
    pub fn vroff(&mut self) -> VROFF_W<CR_SPEC> {
        VROFF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Crystal Oscillator Select"]
    #[inline(always)]
    #[must_use]
    pub fn xtalsel(&mut self) -> XTALSEL_W<CR_SPEC> {
        XTALSEL_W::new(self, 3)
    }
    #[doc = "Bits 24:31 - Password"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<CR_SPEC> {
        KEY_W::new(self, 24)
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
#[doc = "Supply Controller Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
