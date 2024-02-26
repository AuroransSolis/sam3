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
#[doc = "Field `OVRE0` writer - Overrun Error Interrupt Disable 0"]
pub type Ovre0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE1` writer - Overrun Error Interrupt Disable 1"]
pub type Ovre1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE2` writer - Overrun Error Interrupt Disable 2"]
pub type Ovre2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE3` writer - Overrun Error Interrupt Disable 3"]
pub type Ovre3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE4` writer - Overrun Error Interrupt Disable 4"]
pub type Ovre4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE5` writer - Overrun Error Interrupt Disable 5"]
pub type Ovre5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE6` writer - Overrun Error Interrupt Disable 6"]
pub type Ovre6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE7` writer - Overrun Error Interrupt Disable 7"]
pub type Ovre7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRDY` writer - Data Ready Interrupt Disable"]
pub type DrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOVRE` writer - General Overrun Error Interrupt Disable"]
pub type GovreW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 8 - Overrun Error Interrupt Disable 0"]
    #[inline(always)]
    #[must_use]
    pub fn ovre0(&mut self) -> Ovre0W<IdrSpec> {
        Ovre0W::new(self, 8)
    }
    #[doc = "Bit 9 - Overrun Error Interrupt Disable 1"]
    #[inline(always)]
    #[must_use]
    pub fn ovre1(&mut self) -> Ovre1W<IdrSpec> {
        Ovre1W::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun Error Interrupt Disable 2"]
    #[inline(always)]
    #[must_use]
    pub fn ovre2(&mut self) -> Ovre2W<IdrSpec> {
        Ovre2W::new(self, 10)
    }
    #[doc = "Bit 11 - Overrun Error Interrupt Disable 3"]
    #[inline(always)]
    #[must_use]
    pub fn ovre3(&mut self) -> Ovre3W<IdrSpec> {
        Ovre3W::new(self, 11)
    }
    #[doc = "Bit 12 - Overrun Error Interrupt Disable 4"]
    #[inline(always)]
    #[must_use]
    pub fn ovre4(&mut self) -> Ovre4W<IdrSpec> {
        Ovre4W::new(self, 12)
    }
    #[doc = "Bit 13 - Overrun Error Interrupt Disable 5"]
    #[inline(always)]
    #[must_use]
    pub fn ovre5(&mut self) -> Ovre5W<IdrSpec> {
        Ovre5W::new(self, 13)
    }
    #[doc = "Bit 14 - Overrun Error Interrupt Disable 6"]
    #[inline(always)]
    #[must_use]
    pub fn ovre6(&mut self) -> Ovre6W<IdrSpec> {
        Ovre6W::new(self, 14)
    }
    #[doc = "Bit 15 - Overrun Error Interrupt Disable 7"]
    #[inline(always)]
    #[must_use]
    pub fn ovre7(&mut self) -> Ovre7W<IdrSpec> {
        Ovre7W::new(self, 15)
    }
    #[doc = "Bit 16 - Data Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DrdyW<IdrSpec> {
        DrdyW::new(self, 16)
    }
    #[doc = "Bit 17 - General Overrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn govre(&mut self) -> GovreW<IdrSpec> {
        GovreW::new(self, 17)
    }
    #[doc = "Bit 18 - End of Receive Buffer Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<IdrSpec> {
        EndrxW::new(self, 18)
    }
    #[doc = "Bit 19 - Receive Buffer Full Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RxbuffW<IdrSpec> {
        RxbuffW::new(self, 19)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
