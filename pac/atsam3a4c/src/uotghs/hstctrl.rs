#[doc = "Register `HSTCTRL` reader"]
pub type R = crate::R<HstctrlSpec>;
#[doc = "Register `HSTCTRL` writer"]
pub type W = crate::W<HstctrlSpec>;
#[doc = "Field `SOFE` reader - Start of Frame Generation Enable"]
pub type SofeR = crate::BitReader;
#[doc = "Field `SOFE` writer - Start of Frame Generation Enable"]
pub type SofeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Send USB Reset"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - Send USB Reset"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME` reader - Send USB Resume"]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESUME` writer - Send USB Resume"]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spdconf {
    #[doc = "0: The host starts in full-speed mode and performs a high-speed reset to switch to the high-speed mode if the downstream peripheral is high-speed capable."]
    Normal = 0,
    #[doc = "1: For a better consumption, if high-speed is not needed."]
    LowPower = 1,
    #[doc = "2: Forced high speed."]
    HighSpeed = 2,
    #[doc = "3: The host remains to full-speed mode whatever the peripheral speed capability."]
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
    #[doc = "The host starts in full-speed mode and performs a high-speed reset to switch to the high-speed mode if the downstream peripheral is high-speed capable."]
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
    #[doc = "The host remains to full-speed mode whatever the peripheral speed capability."]
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
    #[doc = "The host starts in full-speed mode and performs a high-speed reset to switch to the high-speed mode if the downstream peripheral is high-speed capable."]
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
    #[doc = "The host remains to full-speed mode whatever the peripheral speed capability."]
    #[inline(always)]
    pub fn forced_fs(self) -> &'a mut crate::W<REG> {
        self.variant(Spdconf::ForcedFs)
    }
}
impl R {
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SofeR {
        SofeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Mode Configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SpdconfR {
        SpdconfR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofe(&mut self) -> SofeW<HstctrlSpec> {
        SofeW::new(self, 8)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<HstctrlSpec> {
        ResetW::new(self, 9)
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> ResumeW<HstctrlSpec> {
        ResumeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn spdconf(&mut self) -> SpdconfW<HstctrlSpec> {
        SpdconfW::new(self, 12)
    }
}
#[doc = "Host General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstctrlSpec;
impl crate::RegisterSpec for HstctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstctrl::R`](R) reader structure"]
impl crate::Readable for HstctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hstctrl::W`](W) writer structure"]
impl crate::Writable for HstctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSTCTRL to value 0"]
impl crate::Resettable for HstctrlSpec {
    const RESET_VALUE: u32 = 0;
}
