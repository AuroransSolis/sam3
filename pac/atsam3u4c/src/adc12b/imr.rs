#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `EOC0` reader - End of Conversion Interrupt Mask 0"]
pub type Eoc0R = crate::BitReader;
#[doc = "Field `EOC1` reader - End of Conversion Interrupt Mask 1"]
pub type Eoc1R = crate::BitReader;
#[doc = "Field `EOC2` reader - End of Conversion Interrupt Mask 2"]
pub type Eoc2R = crate::BitReader;
#[doc = "Field `EOC3` reader - End of Conversion Interrupt Mask 3"]
pub type Eoc3R = crate::BitReader;
#[doc = "Field `EOC4` reader - End of Conversion Interrupt Mask 4"]
pub type Eoc4R = crate::BitReader;
#[doc = "Field `EOC5` reader - End of Conversion Interrupt Mask 5"]
pub type Eoc5R = crate::BitReader;
#[doc = "Field `EOC6` reader - End of Conversion Interrupt Mask 6"]
pub type Eoc6R = crate::BitReader;
#[doc = "Field `EOC7` reader - End of Conversion Interrupt Mask 7"]
pub type Eoc7R = crate::BitReader;
#[doc = "Field `OVRE0` reader - Overrun Error Interrupt Mask 0"]
pub type Ovre0R = crate::BitReader;
#[doc = "Field `OVRE1` reader - Overrun Error Interrupt Mask 1"]
pub type Ovre1R = crate::BitReader;
#[doc = "Field `OVRE2` reader - Overrun Error Interrupt Mask 2"]
pub type Ovre2R = crate::BitReader;
#[doc = "Field `OVRE3` reader - Overrun Error Interrupt Mask 3"]
pub type Ovre3R = crate::BitReader;
#[doc = "Field `OVRE4` reader - Overrun Error Interrupt Mask 4"]
pub type Ovre4R = crate::BitReader;
#[doc = "Field `OVRE5` reader - Overrun Error Interrupt Mask 5"]
pub type Ovre5R = crate::BitReader;
#[doc = "Field `OVRE6` reader - Overrun Error Interrupt Mask 6"]
pub type Ovre6R = crate::BitReader;
#[doc = "Field `OVRE7` reader - Overrun Error Interrupt Mask 7"]
pub type Ovre7R = crate::BitReader;
#[doc = "Field `DRDY` reader - Data Ready Interrupt Mask"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `GOVRE` reader - General Overrun Error Interrupt Mask"]
pub type GovreR = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of Receive Buffer Interrupt Mask"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - Receive Buffer Full Interrupt Mask"]
pub type RxbuffR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of Conversion Interrupt Mask 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> Eoc0R {
        Eoc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Mask 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> Eoc1R {
        Eoc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Conversion Interrupt Mask 2"]
    #[inline(always)]
    pub fn eoc2(&self) -> Eoc2R {
        Eoc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Conversion Interrupt Mask 3"]
    #[inline(always)]
    pub fn eoc3(&self) -> Eoc3R {
        Eoc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Mask 4"]
    #[inline(always)]
    pub fn eoc4(&self) -> Eoc4R {
        Eoc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Mask 5"]
    #[inline(always)]
    pub fn eoc5(&self) -> Eoc5R {
        Eoc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of Conversion Interrupt Mask 6"]
    #[inline(always)]
    pub fn eoc6(&self) -> Eoc6R {
        Eoc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End of Conversion Interrupt Mask 7"]
    #[inline(always)]
    pub fn eoc7(&self) -> Eoc7R {
        Eoc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Overrun Error Interrupt Mask 0"]
    #[inline(always)]
    pub fn ovre0(&self) -> Ovre0R {
        Ovre0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Overrun Error Interrupt Mask 1"]
    #[inline(always)]
    pub fn ovre1(&self) -> Ovre1R {
        Ovre1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun Error Interrupt Mask 2"]
    #[inline(always)]
    pub fn ovre2(&self) -> Ovre2R {
        Ovre2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun Error Interrupt Mask 3"]
    #[inline(always)]
    pub fn ovre3(&self) -> Ovre3R {
        Ovre3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun Error Interrupt Mask 4"]
    #[inline(always)]
    pub fn ovre4(&self) -> Ovre4R {
        Ovre4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Overrun Error Interrupt Mask 5"]
    #[inline(always)]
    pub fn ovre5(&self) -> Ovre5R {
        Ovre5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Overrun Error Interrupt Mask 6"]
    #[inline(always)]
    pub fn ovre6(&self) -> Ovre6R {
        Ovre6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Overrun Error Interrupt Mask 7"]
    #[inline(always)]
    pub fn ovre7(&self) -> Ovre7R {
        Ovre7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - General Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn govre(&self) -> GovreR {
        GovreR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - End of Receive Buffer Interrupt Mask"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive Buffer Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
