#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `EOC0` writer - End of Conversion Interrupt Enable 0"]
pub type Eoc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC1` writer - End of Conversion Interrupt Enable 1"]
pub type Eoc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC2` writer - End of Conversion Interrupt Enable 2"]
pub type Eoc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC3` writer - End of Conversion Interrupt Enable 3"]
pub type Eoc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC4` writer - End of Conversion Interrupt Enable 4"]
pub type Eoc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC5` writer - End of Conversion Interrupt Enable 5"]
pub type Eoc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC6` writer - End of Conversion Interrupt Enable 6"]
pub type Eoc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC7` writer - End of Conversion Interrupt Enable 7"]
pub type Eoc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC8` writer - End of Conversion Interrupt Enable 8"]
pub type Eoc8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC9` writer - End of Conversion Interrupt Enable 9"]
pub type Eoc9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC10` writer - End of Conversion Interrupt Enable 10"]
pub type Eoc10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC11` writer - End of Conversion Interrupt Enable 11"]
pub type Eoc11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC12` writer - End of Conversion Interrupt Enable 12"]
pub type Eoc12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC13` writer - End of Conversion Interrupt Enable 13"]
pub type Eoc13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC14` writer - End of Conversion Interrupt Enable 14"]
pub type Eoc14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC15` writer - End of Conversion Interrupt Enable 15"]
pub type Eoc15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCAL` writer - End of Calibration Sequence"]
pub type EocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRDY` writer - Data Ready Interrupt Enable"]
pub type DrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOVRE` writer - General Overrun Error Interrupt Enable"]
pub type GovreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPE` writer - Comparison Event Interrupt Enable"]
pub type CompeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Enable"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Enable"]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - End of Conversion Interrupt Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn eoc0(&mut self) -> Eoc0W<IerSpec> {
        Eoc0W::new(self, 0)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn eoc1(&mut self) -> Eoc1W<IerSpec> {
        Eoc1W::new(self, 1)
    }
    #[doc = "Bit 2 - End of Conversion Interrupt Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn eoc2(&mut self) -> Eoc2W<IerSpec> {
        Eoc2W::new(self, 2)
    }
    #[doc = "Bit 3 - End of Conversion Interrupt Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn eoc3(&mut self) -> Eoc3W<IerSpec> {
        Eoc3W::new(self, 3)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn eoc4(&mut self) -> Eoc4W<IerSpec> {
        Eoc4W::new(self, 4)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn eoc5(&mut self) -> Eoc5W<IerSpec> {
        Eoc5W::new(self, 5)
    }
    #[doc = "Bit 6 - End of Conversion Interrupt Enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn eoc6(&mut self) -> Eoc6W<IerSpec> {
        Eoc6W::new(self, 6)
    }
    #[doc = "Bit 7 - End of Conversion Interrupt Enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn eoc7(&mut self) -> Eoc7W<IerSpec> {
        Eoc7W::new(self, 7)
    }
    #[doc = "Bit 8 - End of Conversion Interrupt Enable 8"]
    #[inline(always)]
    #[must_use]
    pub fn eoc8(&mut self) -> Eoc8W<IerSpec> {
        Eoc8W::new(self, 8)
    }
    #[doc = "Bit 9 - End of Conversion Interrupt Enable 9"]
    #[inline(always)]
    #[must_use]
    pub fn eoc9(&mut self) -> Eoc9W<IerSpec> {
        Eoc9W::new(self, 9)
    }
    #[doc = "Bit 10 - End of Conversion Interrupt Enable 10"]
    #[inline(always)]
    #[must_use]
    pub fn eoc10(&mut self) -> Eoc10W<IerSpec> {
        Eoc10W::new(self, 10)
    }
    #[doc = "Bit 11 - End of Conversion Interrupt Enable 11"]
    #[inline(always)]
    #[must_use]
    pub fn eoc11(&mut self) -> Eoc11W<IerSpec> {
        Eoc11W::new(self, 11)
    }
    #[doc = "Bit 12 - End of Conversion Interrupt Enable 12"]
    #[inline(always)]
    #[must_use]
    pub fn eoc12(&mut self) -> Eoc12W<IerSpec> {
        Eoc12W::new(self, 12)
    }
    #[doc = "Bit 13 - End of Conversion Interrupt Enable 13"]
    #[inline(always)]
    #[must_use]
    pub fn eoc13(&mut self) -> Eoc13W<IerSpec> {
        Eoc13W::new(self, 13)
    }
    #[doc = "Bit 14 - End of Conversion Interrupt Enable 14"]
    #[inline(always)]
    #[must_use]
    pub fn eoc14(&mut self) -> Eoc14W<IerSpec> {
        Eoc14W::new(self, 14)
    }
    #[doc = "Bit 15 - End of Conversion Interrupt Enable 15"]
    #[inline(always)]
    #[must_use]
    pub fn eoc15(&mut self) -> Eoc15W<IerSpec> {
        Eoc15W::new(self, 15)
    }
    #[doc = "Bit 23 - End of Calibration Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn eocal(&mut self) -> EocalW<IerSpec> {
        EocalW::new(self, 23)
    }
    #[doc = "Bit 24 - Data Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DrdyW<IerSpec> {
        DrdyW::new(self, 24)
    }
    #[doc = "Bit 25 - General Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn govre(&mut self) -> GovreW<IerSpec> {
        GovreW::new(self, 25)
    }
    #[doc = "Bit 26 - Comparison Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compe(&mut self) -> CompeW<IerSpec> {
        CompeW::new(self, 26)
    }
    #[doc = "Bit 27 - End of Receive Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<IerSpec> {
        EndrxW::new(self, 27)
    }
    #[doc = "Bit 28 - Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RxbuffW<IerSpec> {
        RxbuffW::new(self, 28)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
