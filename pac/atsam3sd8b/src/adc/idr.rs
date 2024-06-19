#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `EOC0` writer - End of Conversion Interrupt Disable 0"]
pub type Eoc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC1` writer - End of Conversion Interrupt Disable 1"]
pub type Eoc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC2` writer - End of Conversion Interrupt Disable 2"]
pub type Eoc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC3` writer - End of Conversion Interrupt Disable 3"]
pub type Eoc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC4` writer - End of Conversion Interrupt Disable 4"]
pub type Eoc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC5` writer - End of Conversion Interrupt Disable 5"]
pub type Eoc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC6` writer - End of Conversion Interrupt Disable 6"]
pub type Eoc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC7` writer - End of Conversion Interrupt Disable 7"]
pub type Eoc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC8` writer - End of Conversion Interrupt Disable 8"]
pub type Eoc8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC9` writer - End of Conversion Interrupt Disable 9"]
pub type Eoc9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC10` writer - End of Conversion Interrupt Disable 10"]
pub type Eoc10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC11` writer - End of Conversion Interrupt Disable 11"]
pub type Eoc11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC12` writer - End of Conversion Interrupt Disable 12"]
pub type Eoc12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC13` writer - End of Conversion Interrupt Disable 13"]
pub type Eoc13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC14` writer - End of Conversion Interrupt Disable 14"]
pub type Eoc14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC15` writer - End of Conversion Interrupt Disable 15"]
pub type Eoc15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCAL` writer - End of Calibration Sequence"]
pub type EocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRDY` writer - Data Ready Interrupt Disable"]
pub type DrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOVRE` writer - General Overrun Error Interrupt Disable"]
pub type GovreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPE` writer - Comparison Event Interrupt Disable"]
pub type CompeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Disable"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Disable"]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - End of Conversion Interrupt Disable 0"]
    #[inline(always)]
    #[must_use]
    pub fn eoc0(&mut self) -> Eoc0W<IdrSpec> {
        Eoc0W::new(self, 0)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Disable 1"]
    #[inline(always)]
    #[must_use]
    pub fn eoc1(&mut self) -> Eoc1W<IdrSpec> {
        Eoc1W::new(self, 1)
    }
    #[doc = "Bit 2 - End of Conversion Interrupt Disable 2"]
    #[inline(always)]
    #[must_use]
    pub fn eoc2(&mut self) -> Eoc2W<IdrSpec> {
        Eoc2W::new(self, 2)
    }
    #[doc = "Bit 3 - End of Conversion Interrupt Disable 3"]
    #[inline(always)]
    #[must_use]
    pub fn eoc3(&mut self) -> Eoc3W<IdrSpec> {
        Eoc3W::new(self, 3)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Disable 4"]
    #[inline(always)]
    #[must_use]
    pub fn eoc4(&mut self) -> Eoc4W<IdrSpec> {
        Eoc4W::new(self, 4)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Disable 5"]
    #[inline(always)]
    #[must_use]
    pub fn eoc5(&mut self) -> Eoc5W<IdrSpec> {
        Eoc5W::new(self, 5)
    }
    #[doc = "Bit 6 - End of Conversion Interrupt Disable 6"]
    #[inline(always)]
    #[must_use]
    pub fn eoc6(&mut self) -> Eoc6W<IdrSpec> {
        Eoc6W::new(self, 6)
    }
    #[doc = "Bit 7 - End of Conversion Interrupt Disable 7"]
    #[inline(always)]
    #[must_use]
    pub fn eoc7(&mut self) -> Eoc7W<IdrSpec> {
        Eoc7W::new(self, 7)
    }
    #[doc = "Bit 8 - End of Conversion Interrupt Disable 8"]
    #[inline(always)]
    #[must_use]
    pub fn eoc8(&mut self) -> Eoc8W<IdrSpec> {
        Eoc8W::new(self, 8)
    }
    #[doc = "Bit 9 - End of Conversion Interrupt Disable 9"]
    #[inline(always)]
    #[must_use]
    pub fn eoc9(&mut self) -> Eoc9W<IdrSpec> {
        Eoc9W::new(self, 9)
    }
    #[doc = "Bit 10 - End of Conversion Interrupt Disable 10"]
    #[inline(always)]
    #[must_use]
    pub fn eoc10(&mut self) -> Eoc10W<IdrSpec> {
        Eoc10W::new(self, 10)
    }
    #[doc = "Bit 11 - End of Conversion Interrupt Disable 11"]
    #[inline(always)]
    #[must_use]
    pub fn eoc11(&mut self) -> Eoc11W<IdrSpec> {
        Eoc11W::new(self, 11)
    }
    #[doc = "Bit 12 - End of Conversion Interrupt Disable 12"]
    #[inline(always)]
    #[must_use]
    pub fn eoc12(&mut self) -> Eoc12W<IdrSpec> {
        Eoc12W::new(self, 12)
    }
    #[doc = "Bit 13 - End of Conversion Interrupt Disable 13"]
    #[inline(always)]
    #[must_use]
    pub fn eoc13(&mut self) -> Eoc13W<IdrSpec> {
        Eoc13W::new(self, 13)
    }
    #[doc = "Bit 14 - End of Conversion Interrupt Disable 14"]
    #[inline(always)]
    #[must_use]
    pub fn eoc14(&mut self) -> Eoc14W<IdrSpec> {
        Eoc14W::new(self, 14)
    }
    #[doc = "Bit 15 - End of Conversion Interrupt Disable 15"]
    #[inline(always)]
    #[must_use]
    pub fn eoc15(&mut self) -> Eoc15W<IdrSpec> {
        Eoc15W::new(self, 15)
    }
    #[doc = "Bit 23 - End of Calibration Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn eocal(&mut self) -> EocalW<IdrSpec> {
        EocalW::new(self, 23)
    }
    #[doc = "Bit 24 - Data Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DrdyW<IdrSpec> {
        DrdyW::new(self, 24)
    }
    #[doc = "Bit 25 - General Overrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn govre(&mut self) -> GovreW<IdrSpec> {
        GovreW::new(self, 25)
    }
    #[doc = "Bit 26 - Comparison Event Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn compe(&mut self) -> CompeW<IdrSpec> {
        CompeW::new(self, 26)
    }
    #[doc = "Bit 27 - End of Receive Buffer Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<IdrSpec> {
        EndrxW::new(self, 27)
    }
    #[doc = "Bit 28 - Receive Buffer Full Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RxbuffW<IdrSpec> {
        RxbuffW::new(self, 28)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
