#[doc = "Register `DEVCTRL` reader"]
pub type R = crate::R<DevctrlSpec>;
#[doc = "Register `DEVCTRL` writer"]
pub type W = crate::W<DevctrlSpec>;
#[doc = "Field `UADD` reader - USB Address"]
pub type UaddR = crate::FieldReader;
#[doc = "Field `UADD` writer - USB Address"]
pub type UaddW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADDEN` reader - Address Enable"]
pub type AddenR = crate::BitReader;
#[doc = "Field `ADDEN` writer - Address Enable"]
pub type AddenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DETACH` reader - Detach"]
pub type DetachR = crate::BitReader;
#[doc = "Field `DETACH` writer - Detach"]
pub type DetachW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMWKUP` reader - Remote Wake-Up"]
pub type RmwkupR = crate::BitReader;
#[doc = "Field `RMWKUP` writer - Remote Wake-Up"]
pub type RmwkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spdconf {
    #[doc = "0: The peripheral starts in full-speed mode and performs a high-speed reset to switch to the high-speed mode if the host is high-speed capable."]
    Normal = 0,
    #[doc = "1: For a better consumption, if high-speed is not needed."]
    LowPower = 1,
    #[doc = "2: Forced high speed."]
    HighSpeed = 2,
    #[doc = "3: The peripheral remains in full-speed mode whatever the host speed capability."]
    ForcedFs = 3,
}
impl From<Spdconf> for u8 {
    #[inline(always)]
    fn from(variant: Spdconf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spdconf {
    type Ux = u8;
}
impl crate::IsEnum for Spdconf {}
#[doc = "Field `SPDCONF` reader - Mode Configuration"]
pub type SpdconfR = crate::FieldReader<Spdconf>;
impl SpdconfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spdconf {
        match self.bits {
            0 => Spdconf::Normal,
            1 => Spdconf::LowPower,
            2 => Spdconf::HighSpeed,
            3 => Spdconf::ForcedFs,
            _ => unreachable!(),
        }
    }
    #[doc = "The peripheral starts in full-speed mode and performs a high-speed reset to switch to the high-speed mode if the host is high-speed capable."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Spdconf::Normal
    }
    #[doc = "For a better consumption, if high-speed is not needed."]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == Spdconf::LowPower
    }
    #[doc = "Forced high speed."]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == Spdconf::HighSpeed
    }
    #[doc = "The peripheral remains in full-speed mode whatever the host speed capability."]
    #[inline(always)]
    pub fn is_forced_fs(&self) -> bool {
        *self == Spdconf::ForcedFs
    }
}
#[doc = "Field `SPDCONF` writer - Mode Configuration"]
pub type SpdconfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Spdconf, crate::Safe>;
impl<'a, REG> SpdconfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The peripheral starts in full-speed mode and performs a high-speed reset to switch to the high-speed mode if the host is high-speed capable."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconf::Normal)
    }
    #[doc = "For a better consumption, if high-speed is not needed."]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconf::LowPower)
    }
    #[doc = "Forced high speed."]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconf::HighSpeed)
    }
    #[doc = "The peripheral remains in full-speed mode whatever the host speed capability."]
    #[inline(always)]
    pub fn forced_fs(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconf::ForcedFs)
    }
}
#[doc = "Field `LS` reader - Low-Speed Mode Force"]
pub type LsR = crate::BitReader;
#[doc = "Field `LS` writer - Low-Speed Mode Force"]
pub type LsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTJ` reader - Test mode J"]
pub type TstjR = crate::BitReader;
#[doc = "Field `TSTJ` writer - Test mode J"]
pub type TstjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTK` reader - Test mode K"]
pub type TstkR = crate::BitReader;
#[doc = "Field `TSTK` writer - Test mode K"]
pub type TstkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTPCKT` reader - Test packet mode"]
pub type TstpcktR = crate::BitReader;
#[doc = "Field `TSTPCKT` writer - Test packet mode"]
pub type TstpcktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPMODE2` reader - Specific Operational mode"]
pub type Opmode2R = crate::BitReader;
#[doc = "Field `OPMODE2` writer - Specific Operational mode"]
pub type Opmode2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - USB Address"]
    #[inline(always)]
    pub fn uadd(&self) -> UaddR {
        UaddR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Address Enable"]
    #[inline(always)]
    pub fn adden(&self) -> AddenR {
        AddenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Detach"]
    #[inline(always)]
    pub fn detach(&self) -> DetachR {
        DetachR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Remote Wake-Up"]
    #[inline(always)]
    pub fn rmwkup(&self) -> RmwkupR {
        RmwkupR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Mode Configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SpdconfR {
        SpdconfR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Low-Speed Mode Force"]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&self) -> TstjR {
        TstjR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&self) -> TstkR {
        TstkR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Test packet mode"]
    #[inline(always)]
    pub fn tstpckt(&self) -> TstpcktR {
        TstpcktR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Specific Operational mode"]
    #[inline(always)]
    pub fn opmode2(&self) -> Opmode2R {
        Opmode2R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Address"]
    #[inline(always)]
    #[must_use]
    pub fn uadd(&mut self) -> UaddW<DevctrlSpec> {
        UaddW::new(self, 0)
    }
    #[doc = "Bit 7 - Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adden(&mut self) -> AddenW<DevctrlSpec> {
        AddenW::new(self, 7)
    }
    #[doc = "Bit 8 - Detach"]
    #[inline(always)]
    #[must_use]
    pub fn detach(&mut self) -> DetachW<DevctrlSpec> {
        DetachW::new(self, 8)
    }
    #[doc = "Bit 9 - Remote Wake-Up"]
    #[inline(always)]
    #[must_use]
    pub fn rmwkup(&mut self) -> RmwkupW<DevctrlSpec> {
        RmwkupW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn spdconf(&mut self) -> SpdconfW<DevctrlSpec> {
        SpdconfW::new(self, 10)
    }
    #[doc = "Bit 12 - Low-Speed Mode Force"]
    #[inline(always)]
    #[must_use]
    pub fn ls(&mut self) -> LsW<DevctrlSpec> {
        LsW::new(self, 12)
    }
    #[doc = "Bit 13 - Test mode J"]
    #[inline(always)]
    #[must_use]
    pub fn tstj(&mut self) -> TstjW<DevctrlSpec> {
        TstjW::new(self, 13)
    }
    #[doc = "Bit 14 - Test mode K"]
    #[inline(always)]
    #[must_use]
    pub fn tstk(&mut self) -> TstkW<DevctrlSpec> {
        TstkW::new(self, 14)
    }
    #[doc = "Bit 15 - Test packet mode"]
    #[inline(always)]
    #[must_use]
    pub fn tstpckt(&mut self) -> TstpcktW<DevctrlSpec> {
        TstpcktW::new(self, 15)
    }
    #[doc = "Bit 16 - Specific Operational mode"]
    #[inline(always)]
    #[must_use]
    pub fn opmode2(&mut self) -> Opmode2W<DevctrlSpec> {
        Opmode2W::new(self, 16)
    }
}
#[doc = "Device General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevctrlSpec;
impl crate::RegisterSpec for DevctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devctrl::R`](R) reader structure"]
impl crate::Readable for DevctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`devctrl::W`](W) writer structure"]
impl crate::Writable for DevctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVCTRL to value 0x0100"]
impl crate::Resettable for DevctrlSpec {
    const RESET_VALUE: u32 = 0x0100;
}
