#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCR_SPEC>;
#[doc = "Flash Command"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCMD_AW {
    #[doc = "0: Get Flash Descriptor"]
    Getd = 0,
    #[doc = "1: Write page"]
    Wp = 1,
    #[doc = "2: Write page and lock"]
    Wpl = 2,
    #[doc = "3: Erase page and write page"]
    Ewp = 3,
    #[doc = "4: Erase page and write page then lock"]
    Ewpl = 4,
    #[doc = "5: Erase all"]
    Ea = 5,
    #[doc = "8: Set Lock Bit"]
    Slb = 8,
    #[doc = "9: Clear Lock Bit"]
    Clb = 9,
    #[doc = "10: Get Lock Bit"]
    Glb = 10,
    #[doc = "11: Set GPNVM Bit"]
    Sgpb = 11,
    #[doc = "12: Clear GPNVM Bit"]
    Cgpb = 12,
    #[doc = "13: Get GPNVM Bit"]
    Ggpb = 13,
    #[doc = "14: Start Read Unique Identifier"]
    Stui = 14,
    #[doc = "15: Stop Read Unique Identifier"]
    Spui = 15,
    #[doc = "16: Get CALIB Bit"]
    Gcalb = 16,
}
impl From<FCMD_AW> for u8 {
    #[inline(always)]
    fn from(variant: FCMD_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FCMD_AW {
    type Ux = u8;
}
#[doc = "Field `FCMD` writer - Flash Command"]
pub type FCMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, FCMD_AW>;
impl<'a, REG, const O: u8> FCMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Get Flash Descriptor"]
    #[inline(always)]
    pub fn getd(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Getd)
    }
    #[doc = "Write page"]
    #[inline(always)]
    pub fn wp(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Wp)
    }
    #[doc = "Write page and lock"]
    #[inline(always)]
    pub fn wpl(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Wpl)
    }
    #[doc = "Erase page and write page"]
    #[inline(always)]
    pub fn ewp(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Ewp)
    }
    #[doc = "Erase page and write page then lock"]
    #[inline(always)]
    pub fn ewpl(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Ewpl)
    }
    #[doc = "Erase all"]
    #[inline(always)]
    pub fn ea(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Ea)
    }
    #[doc = "Set Lock Bit"]
    #[inline(always)]
    pub fn slb(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Slb)
    }
    #[doc = "Clear Lock Bit"]
    #[inline(always)]
    pub fn clb(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Clb)
    }
    #[doc = "Get Lock Bit"]
    #[inline(always)]
    pub fn glb(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Glb)
    }
    #[doc = "Set GPNVM Bit"]
    #[inline(always)]
    pub fn sgpb(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Sgpb)
    }
    #[doc = "Clear GPNVM Bit"]
    #[inline(always)]
    pub fn cgpb(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Cgpb)
    }
    #[doc = "Get GPNVM Bit"]
    #[inline(always)]
    pub fn ggpb(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Ggpb)
    }
    #[doc = "Start Read Unique Identifier"]
    #[inline(always)]
    pub fn stui(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Stui)
    }
    #[doc = "Stop Read Unique Identifier"]
    #[inline(always)]
    pub fn spui(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Spui)
    }
    #[doc = "Get CALIB Bit"]
    #[inline(always)]
    pub fn gcalb(self) -> &'a mut crate::W<REG> {
        self.variant(FCMD_AW::Gcalb)
    }
}
#[doc = "Field `FARG` writer - Flash Command Argument"]
pub type FARG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Flash Writing Protection Key"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FKEY_AW {
    #[doc = "90: The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    Passwd = 90,
}
impl From<FKEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: FKEY_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FKEY_AW {
    type Ux = u8;
}
#[doc = "Field `FKEY` writer - Flash Writing Protection Key"]
pub type FKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, FKEY_AW>;
impl<'a, REG, const O: u8> FKEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(FKEY_AW::Passwd)
    }
}
impl W {
    #[doc = "Bits 0:7 - Flash Command"]
    #[inline(always)]
    #[must_use]
    pub fn fcmd(&mut self) -> FCMD_W<FCR_SPEC, 0> {
        FCMD_W::new(self)
    }
    #[doc = "Bits 8:23 - Flash Command Argument"]
    #[inline(always)]
    #[must_use]
    pub fn farg(&mut self) -> FARG_W<FCR_SPEC, 8> {
        FARG_W::new(self)
    }
    #[doc = "Bits 24:31 - Flash Writing Protection Key"]
    #[inline(always)]
    #[must_use]
    pub fn fkey(&mut self) -> FKEY_W<FCR_SPEC, 24> {
        FKEY_W::new(self)
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
#[doc = "EEFC Flash Command Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
