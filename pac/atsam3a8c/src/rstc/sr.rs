#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `URSTS` reader - User Reset Status"]
pub struct URSTS_R(crate::FieldReader<bool, bool>);
impl URSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        URSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Reset Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSTTYP_A {
    #[doc = "0: First power-up Reset"]
    GENERALRESET = 0,
    #[doc = "1: Return from Backup Mode"]
    BACKUPRESET = 1,
    #[doc = "2: Watchdog fault occurred"]
    WATCHDOGRESET = 2,
    #[doc = "3: Processor reset required by the software"]
    SOFTWARERESET = 3,
    #[doc = "4: NRST pin detected low"]
    USERRESET = 4,
}
impl From<RSTTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTTYP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RSTTYP` reader - Reset Type"]
pub struct RSTTYP_R(crate::FieldReader<u8, RSTTYP_A>);
impl RSTTYP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RSTTYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSTTYP_A> {
        match self.bits {
            0 => Some(RSTTYP_A::GENERALRESET),
            1 => Some(RSTTYP_A::BACKUPRESET),
            2 => Some(RSTTYP_A::WATCHDOGRESET),
            3 => Some(RSTTYP_A::SOFTWARERESET),
            4 => Some(RSTTYP_A::USERRESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GENERALRESET`"]
    #[inline(always)]
    pub fn is_general_reset(&self) -> bool {
        **self == RSTTYP_A::GENERALRESET
    }
    #[doc = "Checks if the value of the field is `BACKUPRESET`"]
    #[inline(always)]
    pub fn is_backup_reset(&self) -> bool {
        **self == RSTTYP_A::BACKUPRESET
    }
    #[doc = "Checks if the value of the field is `WATCHDOGRESET`"]
    #[inline(always)]
    pub fn is_watchdog_reset(&self) -> bool {
        **self == RSTTYP_A::WATCHDOGRESET
    }
    #[doc = "Checks if the value of the field is `SOFTWARERESET`"]
    #[inline(always)]
    pub fn is_software_reset(&self) -> bool {
        **self == RSTTYP_A::SOFTWARERESET
    }
    #[doc = "Checks if the value of the field is `USERRESET`"]
    #[inline(always)]
    pub fn is_user_reset(&self) -> bool {
        **self == RSTTYP_A::USERRESET
    }
}
impl core::ops::Deref for RSTTYP_R {
    type Target = crate::FieldReader<u8, RSTTYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRSTL` reader - NRST Pin Level"]
pub struct NRSTL_R(crate::FieldReader<bool, bool>);
impl NRSTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRSTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRSTL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRCMP` reader - Software Reset Command in Progress"]
pub struct SRCMP_R(crate::FieldReader<bool, bool>);
impl SRCMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - User Reset Status"]
    #[inline(always)]
    pub fn ursts(&self) -> URSTS_R {
        URSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Reset Type"]
    #[inline(always)]
    pub fn rsttyp(&self) -> RSTTYP_R {
        RSTTYP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 16 - NRST Pin Level"]
    #[inline(always)]
    pub fn nrstl(&self) -> NRSTL_R {
        NRSTL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Software Reset Command in Progress"]
    #[inline(always)]
    pub fn srcmp(&self) -> SRCMP_R {
        SRCMP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
