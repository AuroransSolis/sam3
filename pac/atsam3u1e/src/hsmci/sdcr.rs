#[doc = "Register `SDCR` reader"]
pub type R = crate::R<SdcrSpec>;
#[doc = "Register `SDCR` writer"]
pub type W = crate::W<SdcrSpec>;
#[doc = "SDCard/SDIO Slot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdcsel {
    #[doc = "0: Slot A is selected."]
    Slota = 0,
    #[doc = "1: -"]
    Slotb = 1,
    #[doc = "2: -"]
    Slotc = 2,
    #[doc = "3: -"]
    Slotd = 3,
}
impl From<Sdcsel> for u8 {
    #[inline(always)]
    fn from(variant: Sdcsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdcsel {
    type Ux = u8;
}
#[doc = "Field `SDCSEL` reader - SDCard/SDIO Slot"]
pub type SdcselR = crate::FieldReader<Sdcsel>;
impl SdcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdcsel {
        match self.bits {
            0 => Sdcsel::Slota,
            1 => Sdcsel::Slotb,
            2 => Sdcsel::Slotc,
            3 => Sdcsel::Slotd,
            _ => unreachable!(),
        }
    }
    #[doc = "Slot A is selected."]
    #[inline(always)]
    pub fn is_slota(&self) -> bool {
        *self == Sdcsel::Slota
    }
    #[doc = "-"]
    #[inline(always)]
    pub fn is_slotb(&self) -> bool {
        *self == Sdcsel::Slotb
    }
    #[doc = "-"]
    #[inline(always)]
    pub fn is_slotc(&self) -> bool {
        *self == Sdcsel::Slotc
    }
    #[doc = "-"]
    #[inline(always)]
    pub fn is_slotd(&self) -> bool {
        *self == Sdcsel::Slotd
    }
}
#[doc = "Field `SDCSEL` writer - SDCard/SDIO Slot"]
pub type SdcselW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Sdcsel>;
impl<'a, REG> SdcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slot A is selected."]
    #[inline(always)]
    pub fn slota(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcsel::Slota)
    }
    #[doc = "-"]
    #[inline(always)]
    pub fn slotb(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcsel::Slotb)
    }
    #[doc = "-"]
    #[inline(always)]
    pub fn slotc(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcsel::Slotc)
    }
    #[doc = "-"]
    #[inline(always)]
    pub fn slotd(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcsel::Slotd)
    }
}
#[doc = "SDCard/SDIO Bus Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdcbus {
    #[doc = "0: 1 bit"]
    _1 = 0,
    #[doc = "2: 4 bit"]
    _4 = 2,
    #[doc = "3: 8 bit"]
    _8 = 3,
}
impl From<Sdcbus> for u8 {
    #[inline(always)]
    fn from(variant: Sdcbus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdcbus {
    type Ux = u8;
}
#[doc = "Field `SDCBUS` reader - SDCard/SDIO Bus Width"]
pub type SdcbusR = crate::FieldReader<Sdcbus>;
impl SdcbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sdcbus> {
        match self.bits {
            0 => Some(Sdcbus::_1),
            2 => Some(Sdcbus::_4),
            3 => Some(Sdcbus::_8),
            _ => None,
        }
    }
    #[doc = "1 bit"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdcbus::_1
    }
    #[doc = "4 bit"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Sdcbus::_4
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Sdcbus::_8
    }
}
#[doc = "Field `SDCBUS` writer - SDCard/SDIO Bus Width"]
pub type SdcbusW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdcbus>;
impl<'a, REG> SdcbusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcbus::_1)
    }
    #[doc = "4 bit"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcbus::_4)
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Sdcbus::_8)
    }
}
impl R {
    #[doc = "Bits 0:1 - SDCard/SDIO Slot"]
    #[inline(always)]
    pub fn sdcsel(&self) -> SdcselR {
        SdcselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 6:7 - SDCard/SDIO Bus Width"]
    #[inline(always)]
    pub fn sdcbus(&self) -> SdcbusR {
        SdcbusR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDCard/SDIO Slot"]
    #[inline(always)]
    #[must_use]
    pub fn sdcsel(&mut self) -> SdcselW<SdcrSpec> {
        SdcselW::new(self, 0)
    }
    #[doc = "Bits 6:7 - SDCard/SDIO Bus Width"]
    #[inline(always)]
    #[must_use]
    pub fn sdcbus(&mut self) -> SdcbusW<SdcrSpec> {
        SdcbusW::new(self, 6)
    }
}
#[doc = "SD/SDIO Card Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdcrSpec;
impl crate::RegisterSpec for SdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdcr::R`](R) reader structure"]
impl crate::Readable for SdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdcr::W`](W) writer structure"]
impl crate::Writable for SdcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDCR to value 0"]
impl crate::Resettable for SdcrSpec {
    const RESET_VALUE: u32 = 0;
}
