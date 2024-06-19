#[doc = "Register `WPSR` reader"]
pub type R = crate::R<WpsrSpec>;
#[doc = "Field `WPSWS0` reader - Write Protect SW Status"]
pub type Wpsws0R = crate::BitReader;
#[doc = "Field `WPSWS1` reader - Write Protect SW Status"]
pub type Wpsws1R = crate::BitReader;
#[doc = "Field `WPSWS2` reader - Write Protect SW Status"]
pub type Wpsws2R = crate::BitReader;
#[doc = "Field `WPSWS3` reader - Write Protect SW Status"]
pub type Wpsws3R = crate::BitReader;
#[doc = "Field `WPSWS4` reader - Write Protect SW Status"]
pub type Wpsws4R = crate::BitReader;
#[doc = "Field `WPSWS5` reader - Write Protect SW Status"]
pub type Wpsws5R = crate::BitReader;
#[doc = "Field `WPVS` reader - Write Protect Violation Status"]
pub type WpvsR = crate::BitReader;
#[doc = "Field `WPHWS0` reader - Write Protect HW Status"]
pub type Wphws0R = crate::BitReader;
#[doc = "Field `WPHWS1` reader - Write Protect HW Status"]
pub type Wphws1R = crate::BitReader;
#[doc = "Field `WPHWS2` reader - Write Protect HW Status"]
pub type Wphws2R = crate::BitReader;
#[doc = "Field `WPHWS3` reader - Write Protect HW Status"]
pub type Wphws3R = crate::BitReader;
#[doc = "Field `WPHWS4` reader - Write Protect HW Status"]
pub type Wphws4R = crate::BitReader;
#[doc = "Field `WPHWS5` reader - Write Protect HW Status"]
pub type Wphws5R = crate::BitReader;
#[doc = "Field `WPVSRC` reader - Write Protect Violation Source"]
pub type WpvsrcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws0(&self) -> Wpsws0R {
        Wpsws0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws1(&self) -> Wpsws1R {
        Wpsws1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws2(&self) -> Wpsws2R {
        Wpsws2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws3(&self) -> Wpsws3R {
        Wpsws3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws4(&self) -> Wpsws4R {
        Wpsws4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws5(&self) -> Wpsws5R {
        Wpsws5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Protect Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WpvsR {
        WpvsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws0(&self) -> Wphws0R {
        Wphws0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws1(&self) -> Wphws1R {
        Wphws1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws2(&self) -> Wphws2R {
        Wphws2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws3(&self) -> Wphws3R {
        Wphws3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws4(&self) -> Wphws4R {
        Wphws4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws5(&self) -> Wphws5R {
        Wphws5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Write Protect Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WpvsrcR {
        WpvsrcR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "PWM Write Protect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpsrSpec;
impl crate::RegisterSpec for WpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsr::R`](R) reader structure"]
impl crate::Readable for WpsrSpec {}
#[doc = "`reset()` method sets WPSR to value 0"]
impl crate::Resettable for WpsrSpec {
    const RESET_VALUE: u32 = 0;
}
