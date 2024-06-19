#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `URSTS` reader - User Reset Status"]
pub type UrstsR = crate::BitReader;
#[doc = "Reset Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rsttyp {
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
impl From<Rsttyp> for u8 {
    #[inline(always)]
    fn from(variant: Rsttyp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rsttyp {
    type Ux = u8;
}
impl crate::IsEnum for Rsttyp {}
#[doc = "Field `RSTTYP` reader - Reset Type"]
pub type RsttypR = crate::FieldReader<Rsttyp>;
impl RsttypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rsttyp> {
        match self.bits {
            0 => Some(Rsttyp::GeneralReset),
            1 => Some(Rsttyp::BackupReset),
            2 => Some(Rsttyp::WatchdogReset),
            3 => Some(Rsttyp::SoftwareReset),
            4 => Some(Rsttyp::UserReset),
            _ => None,
        }
    }
    #[doc = "First power-up Reset"]
    #[inline(always)]
    pub fn is_general_reset(&self) -> bool {
        *self == Rsttyp::GeneralReset
    }
    #[doc = "Return from Backup Mode"]
    #[inline(always)]
    pub fn is_backup_reset(&self) -> bool {
        *self == Rsttyp::BackupReset
    }
    #[doc = "Watchdog fault occurred"]
    #[inline(always)]
    pub fn is_watchdog_reset(&self) -> bool {
        *self == Rsttyp::WatchdogReset
    }
    #[doc = "Processor reset required by the software"]
    #[inline(always)]
    pub fn is_software_reset(&self) -> bool {
        *self == Rsttyp::SoftwareReset
    }
    #[doc = "NRST pin detected low"]
    #[inline(always)]
    pub fn is_user_reset(&self) -> bool {
        *self == Rsttyp::UserReset
    }
}
#[doc = "Field `NRSTL` reader - NRST Pin Level"]
pub type NrstlR = crate::BitReader;
#[doc = "Field `SRCMP` reader - Software Reset Command in Progress"]
pub type SrcmpR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - User Reset Status"]
    #[inline(always)]
    pub fn ursts(&self) -> UrstsR {
        UrstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Reset Type"]
    #[inline(always)]
    pub fn rsttyp(&self) -> RsttypR {
        RsttypR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - NRST Pin Level"]
    #[inline(always)]
    pub fn nrstl(&self) -> NrstlR {
        NrstlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software Reset Command in Progress"]
    #[inline(always)]
    pub fn srcmp(&self) -> SrcmpR {
        SrcmpR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
