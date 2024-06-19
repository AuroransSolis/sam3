#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `IDTI` reader - ID Transition Interrupt"]
pub type IdtiR = crate::BitReader;
#[doc = "Field `VBUSTI` reader - VBus Transition Interrupt"]
pub type VbustiR = crate::BitReader;
#[doc = "Field `SRPI` reader - SRP Interrupt"]
pub type SrpiR = crate::BitReader;
#[doc = "Field `VBERRI` reader - VBus Error Interrupt"]
pub type VberriR = crate::BitReader;
#[doc = "Field `BCERRI` reader - B-Connection Error Interrupt"]
pub type BcerriR = crate::BitReader;
#[doc = "Field `ROLEEXI` reader - Role Exchange Interrupt"]
pub type RoleexiR = crate::BitReader;
#[doc = "Field `HNPERRI` reader - HNP Error Interrupt"]
pub type HnperriR = crate::BitReader;
#[doc = "Field `STOI` reader - Suspend Time-Out Interrupt"]
pub type StoiR = crate::BitReader;
#[doc = "Field `VBUSRQ` reader - VBus Request"]
pub type VbusrqR = crate::BitReader;
#[doc = "Field `ID` reader - UOTGID Pin State"]
pub type IdR = crate::BitReader;
#[doc = "Field `VBUS` reader - VBus Level"]
pub type VbusR = crate::BitReader;
#[doc = "Speed Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Speed {
    #[doc = "0: Full-Speed mode"]
    FullSpeed = 0,
    #[doc = "1: High-Speed mode"]
    HighSpeed = 1,
    #[doc = "2: Low-Speed mode"]
    LowSpeed = 2,
}
impl From<Speed> for u8 {
    #[inline(always)]
    fn from(variant: Speed) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Speed {
    type Ux = u8;
}
impl crate::IsEnum for Speed {}
#[doc = "Field `SPEED` reader - Speed Status"]
pub type SpeedR = crate::FieldReader<Speed>;
impl SpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Speed> {
        match self.bits {
            0 => Some(Speed::FullSpeed),
            1 => Some(Speed::HighSpeed),
            2 => Some(Speed::LowSpeed),
            _ => None,
        }
    }
    #[doc = "Full-Speed mode"]
    #[inline(always)]
    pub fn is_full_speed(&self) -> bool {
        *self == Speed::FullSpeed
    }
    #[doc = "High-Speed mode"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == Speed::HighSpeed
    }
    #[doc = "Low-Speed mode"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == Speed::LowSpeed
    }
}
#[doc = "Field `CLKUSABLE` reader - UTMI Clock Usable"]
pub type ClkusableR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ID Transition Interrupt"]
    #[inline(always)]
    pub fn idti(&self) -> IdtiR {
        IdtiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt"]
    #[inline(always)]
    pub fn vbusti(&self) -> VbustiR {
        VbustiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRP Interrupt"]
    #[inline(always)]
    pub fn srpi(&self) -> SrpiR {
        SrpiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBus Error Interrupt"]
    #[inline(always)]
    pub fn vberri(&self) -> VberriR {
        VberriR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt"]
    #[inline(always)]
    pub fn bcerri(&self) -> BcerriR {
        BcerriR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt"]
    #[inline(always)]
    pub fn roleexi(&self) -> RoleexiR {
        RoleexiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HNP Error Interrupt"]
    #[inline(always)]
    pub fn hnperri(&self) -> HnperriR {
        HnperriR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt"]
    #[inline(always)]
    pub fn stoi(&self) -> StoiR {
        StoiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - VBus Request"]
    #[inline(always)]
    pub fn vbusrq(&self) -> VbusrqR {
        VbusrqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UOTGID Pin State"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - VBus Level"]
    #[inline(always)]
    pub fn vbus(&self) -> VbusR {
        VbusR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Speed Status"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - UTMI Clock Usable"]
    #[inline(always)]
    pub fn clkusable(&self) -> ClkusableR {
        ClkusableR::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "General Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0x0400"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x0400;
}
