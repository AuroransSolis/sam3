#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `URSTS` reader - User Reset Status"]
pub type URSTS_R = crate::BitReader;
#[doc = "Field `RSTTYP` reader - Reset Type"]
pub type RSTTYP_R = crate::FieldReader<RSTTYP_A>;
#[doc = "Reset Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSTTYP_A {
    #[doc = "0: First power-up Reset"]
    GeneralReset = 0,
    #[doc = "1: Return from Backup Mode"]
    BackupReset = 1,
    #[doc = "2: Watchdog fault occurred"]
    WatchdogReset = 2,
    #[doc = "3: Processor reset required by the software"]
    SoftwareReset = 3,
    #[doc = "4: NRST pin detected low"]
    UserReset = 4,
}
impl From<RSTTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTTYP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RSTTYP_A {
    type Ux = u8;
}
impl RSTTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RSTTYP_A> {
        match self.bits {
            0 => Some(RSTTYP_A::GeneralReset),
            1 => Some(RSTTYP_A::BackupReset),
            2 => Some(RSTTYP_A::WatchdogReset),
            3 => Some(RSTTYP_A::SoftwareReset),
            4 => Some(RSTTYP_A::UserReset),
            _ => None,
        }
    }
    #[doc = "First power-up Reset"]
    #[inline(always)]
    pub fn is_general_reset(&self) -> bool {
        *self == RSTTYP_A::GeneralReset
    }
    #[doc = "Return from Backup Mode"]
    #[inline(always)]
    pub fn is_backup_reset(&self) -> bool {
        *self == RSTTYP_A::BackupReset
    }
    #[doc = "Watchdog fault occurred"]
    #[inline(always)]
    pub fn is_watchdog_reset(&self) -> bool {
        *self == RSTTYP_A::WatchdogReset
    }
    #[doc = "Processor reset required by the software"]
    #[inline(always)]
    pub fn is_software_reset(&self) -> bool {
        *self == RSTTYP_A::SoftwareReset
    }
    #[doc = "NRST pin detected low"]
    #[inline(always)]
    pub fn is_user_reset(&self) -> bool {
        *self == RSTTYP_A::UserReset
    }
}
#[doc = "Field `NRSTL` reader - NRST Pin Level"]
pub type NRSTL_R = crate::BitReader;
#[doc = "Field `SRCMP` reader - Software Reset Command in Progress"]
pub type SRCMP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - User Reset Status"]
    #[inline(always)]
    pub fn ursts(&self) -> URSTS_R {
        URSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Reset Type"]
    #[inline(always)]
    pub fn rsttyp(&self) -> RSTTYP_R {
        RSTTYP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - NRST Pin Level"]
    #[inline(always)]
    pub fn nrstl(&self) -> NRSTL_R {
        NRSTL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software Reset Command in Progress"]
    #[inline(always)]
    pub fn srcmp(&self) -> SRCMP_R {
        SRCMP_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
