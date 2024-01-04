#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `IDTI` reader - ID Transition Interrupt"]
pub type IDTI_R = crate::BitReader;
#[doc = "Field `VBUSTI` reader - VBus Transition Interrupt"]
pub type VBUSTI_R = crate::BitReader;
#[doc = "Field `SRPI` reader - SRP Interrupt"]
pub type SRPI_R = crate::BitReader;
#[doc = "Field `VBERRI` reader - VBus Error Interrupt"]
pub type VBERRI_R = crate::BitReader;
#[doc = "Field `BCERRI` reader - B-Connection Error Interrupt"]
pub type BCERRI_R = crate::BitReader;
#[doc = "Field `ROLEEXI` reader - Role Exchange Interrupt"]
pub type ROLEEXI_R = crate::BitReader;
#[doc = "Field `HNPERRI` reader - HNP Error Interrupt"]
pub type HNPERRI_R = crate::BitReader;
#[doc = "Field `STOI` reader - Suspend Time-Out Interrupt"]
pub type STOI_R = crate::BitReader;
#[doc = "Field `VBUSRQ` reader - VBus Request"]
pub type VBUSRQ_R = crate::BitReader;
#[doc = "Field `ID` reader - UOTGID Pin State"]
pub type ID_R = crate::BitReader;
#[doc = "Field `VBUS` reader - VBus Level"]
pub type VBUS_R = crate::BitReader;
#[doc = "Field `SPEED` reader - Speed Status"]
pub type SPEED_R = crate::FieldReader<SPEED_A>;
#[doc = "Speed Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "0: Full-Speed mode"]
    FullSpeed = 0,
    #[doc = "1: High-Speed mode"]
    HighSpeed = 1,
    #[doc = "2: Low-Speed mode"]
    LowSpeed = 2,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPEED_A {
    type Ux = u8;
}
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPEED_A> {
        match self.bits {
            0 => Some(SPEED_A::FullSpeed),
            1 => Some(SPEED_A::HighSpeed),
            2 => Some(SPEED_A::LowSpeed),
            _ => None,
        }
    }
    #[doc = "Full-Speed mode"]
    #[inline(always)]
    pub fn is_full_speed(&self) -> bool {
        *self == SPEED_A::FullSpeed
    }
    #[doc = "High-Speed mode"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == SPEED_A::HighSpeed
    }
    #[doc = "Low-Speed mode"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == SPEED_A::LowSpeed
    }
}
#[doc = "Field `CLKUSABLE` reader - UTMI Clock Usable"]
pub type CLKUSABLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ID Transition Interrupt"]
    #[inline(always)]
    pub fn idti(&self) -> IDTI_R {
        IDTI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt"]
    #[inline(always)]
    pub fn vbusti(&self) -> VBUSTI_R {
        VBUSTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRP Interrupt"]
    #[inline(always)]
    pub fn srpi(&self) -> SRPI_R {
        SRPI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBus Error Interrupt"]
    #[inline(always)]
    pub fn vberri(&self) -> VBERRI_R {
        VBERRI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt"]
    #[inline(always)]
    pub fn bcerri(&self) -> BCERRI_R {
        BCERRI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt"]
    #[inline(always)]
    pub fn roleexi(&self) -> ROLEEXI_R {
        ROLEEXI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HNP Error Interrupt"]
    #[inline(always)]
    pub fn hnperri(&self) -> HNPERRI_R {
        HNPERRI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt"]
    #[inline(always)]
    pub fn stoi(&self) -> STOI_R {
        STOI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - VBus Request"]
    #[inline(always)]
    pub fn vbusrq(&self) -> VBUSRQ_R {
        VBUSRQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UOTGID Pin State"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - VBus Level"]
    #[inline(always)]
    pub fn vbus(&self) -> VBUS_R {
        VBUS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Speed Status"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - UTMI Clock Usable"]
    #[inline(always)]
    pub fn clkusable(&self) -> CLKUSABLE_R {
        CLKUSABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "General Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0x0400"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0x0400;
}
