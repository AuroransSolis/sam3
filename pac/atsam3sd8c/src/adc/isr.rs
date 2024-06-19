#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `EOC0` reader - End of Conversion 0"]
pub type Eoc0R = crate::BitReader;
#[doc = "Field `EOC1` reader - End of Conversion 1"]
pub type Eoc1R = crate::BitReader;
#[doc = "Field `EOC2` reader - End of Conversion 2"]
pub type Eoc2R = crate::BitReader;
#[doc = "Field `EOC3` reader - End of Conversion 3"]
pub type Eoc3R = crate::BitReader;
#[doc = "Field `EOC4` reader - End of Conversion 4"]
pub type Eoc4R = crate::BitReader;
#[doc = "Field `EOC5` reader - End of Conversion 5"]
pub type Eoc5R = crate::BitReader;
#[doc = "Field `EOC6` reader - End of Conversion 6"]
pub type Eoc6R = crate::BitReader;
#[doc = "Field `EOC7` reader - End of Conversion 7"]
pub type Eoc7R = crate::BitReader;
#[doc = "Field `EOC8` reader - End of Conversion 8"]
pub type Eoc8R = crate::BitReader;
#[doc = "Field `EOC9` reader - End of Conversion 9"]
pub type Eoc9R = crate::BitReader;
#[doc = "Field `EOC10` reader - End of Conversion 10"]
pub type Eoc10R = crate::BitReader;
#[doc = "Field `EOC11` reader - End of Conversion 11"]
pub type Eoc11R = crate::BitReader;
#[doc = "Field `EOC12` reader - End of Conversion 12"]
pub type Eoc12R = crate::BitReader;
#[doc = "Field `EOC13` reader - End of Conversion 13"]
pub type Eoc13R = crate::BitReader;
#[doc = "Field `EOC14` reader - End of Conversion 14"]
pub type Eoc14R = crate::BitReader;
#[doc = "Field `EOC15` reader - End of Conversion 15"]
pub type Eoc15R = crate::BitReader;
#[doc = "Field `EOCAL` reader - End of Calibration Sequence"]
pub type EocalR = crate::BitReader;
#[doc = "Field `DRDY` reader - Data Ready"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `GOVRE` reader - General Overrun Error"]
pub type GovreR = crate::BitReader;
#[doc = "Field `COMPE` reader - Comparison Error"]
pub type CompeR = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of RX Buffer"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - RX Buffer Full"]
pub type RxbuffR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of Conversion 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> Eoc0R {
        Eoc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Conversion 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> Eoc1R {
        Eoc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Conversion 2"]
    #[inline(always)]
    pub fn eoc2(&self) -> Eoc2R {
        Eoc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Conversion 3"]
    #[inline(always)]
    pub fn eoc3(&self) -> Eoc3R {
        Eoc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Conversion 4"]
    #[inline(always)]
    pub fn eoc4(&self) -> Eoc4R {
        Eoc4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion 5"]
    #[inline(always)]
    pub fn eoc5(&self) -> Eoc5R {
        Eoc5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of Conversion 6"]
    #[inline(always)]
    pub fn eoc6(&self) -> Eoc6R {
        Eoc6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End of Conversion 7"]
    #[inline(always)]
    pub fn eoc7(&self) -> Eoc7R {
        Eoc7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - End of Conversion 8"]
    #[inline(always)]
    pub fn eoc8(&self) -> Eoc8R {
        Eoc8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - End of Conversion 9"]
    #[inline(always)]
    pub fn eoc9(&self) -> Eoc9R {
        Eoc9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - End of Conversion 10"]
    #[inline(always)]
    pub fn eoc10(&self) -> Eoc10R {
        Eoc10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End of Conversion 11"]
    #[inline(always)]
    pub fn eoc11(&self) -> Eoc11R {
        Eoc11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - End of Conversion 12"]
    #[inline(always)]
    pub fn eoc12(&self) -> Eoc12R {
        Eoc12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - End of Conversion 13"]
    #[inline(always)]
    pub fn eoc13(&self) -> Eoc13R {
        Eoc13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - End of Conversion 14"]
    #[inline(always)]
    pub fn eoc14(&self) -> Eoc14R {
        Eoc14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Conversion 15"]
    #[inline(always)]
    pub fn eoc15(&self) -> Eoc15R {
        Eoc15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 23 - End of Calibration Sequence"]
    #[inline(always)]
    pub fn eocal(&self) -> EocalR {
        EocalR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Data Ready"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - General Overrun Error"]
    #[inline(always)]
    pub fn govre(&self) -> GovreR {
        GovreR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Comparison Error"]
    #[inline(always)]
    pub fn compe(&self) -> CompeR {
        CompeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - End of RX Buffer"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RX Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
