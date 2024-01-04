#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `EOC0` writer - End of Conversion Interrupt Enable 0"]
pub type EOC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC1` writer - End of Conversion Interrupt Enable 1"]
pub type EOC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC2` writer - End of Conversion Interrupt Enable 2"]
pub type EOC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC3` writer - End of Conversion Interrupt Enable 3"]
pub type EOC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC4` writer - End of Conversion Interrupt Enable 4"]
pub type EOC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC5` writer - End of Conversion Interrupt Enable 5"]
pub type EOC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC6` writer - End of Conversion Interrupt Enable 6"]
pub type EOC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC7` writer - End of Conversion Interrupt Enable 7"]
pub type EOC7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE0` writer - Overrun Error Interrupt Enable 0"]
pub type OVRE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE1` writer - Overrun Error Interrupt Enable 1"]
pub type OVRE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE2` writer - Overrun Error Interrupt Enable 2"]
pub type OVRE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE3` writer - Overrun Error Interrupt Enable 3"]
pub type OVRE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE4` writer - Overrun Error Interrupt Enable 4"]
pub type OVRE4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE5` writer - Overrun Error Interrupt Enable 5"]
pub type OVRE5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE6` writer - Overrun Error Interrupt Enable 6"]
pub type OVRE6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE7` writer - Overrun Error Interrupt Enable 7"]
pub type OVRE7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRDY` writer - Data Ready Interrupt Enable"]
pub type DRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOVRE` writer - General Overrun Error Interrupt Enable"]
pub type GOVRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Enable"]
pub type ENDRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Enable"]
pub type RXBUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - End of Conversion Interrupt Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn eoc0(&mut self) -> EOC0_W<IER_SPEC> {
        EOC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn eoc1(&mut self) -> EOC1_W<IER_SPEC> {
        EOC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - End of Conversion Interrupt Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn eoc2(&mut self) -> EOC2_W<IER_SPEC> {
        EOC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - End of Conversion Interrupt Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn eoc3(&mut self) -> EOC3_W<IER_SPEC> {
        EOC3_W::new(self, 3)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn eoc4(&mut self) -> EOC4_W<IER_SPEC> {
        EOC4_W::new(self, 4)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn eoc5(&mut self) -> EOC5_W<IER_SPEC> {
        EOC5_W::new(self, 5)
    }
    #[doc = "Bit 6 - End of Conversion Interrupt Enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn eoc6(&mut self) -> EOC6_W<IER_SPEC> {
        EOC6_W::new(self, 6)
    }
    #[doc = "Bit 7 - End of Conversion Interrupt Enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn eoc7(&mut self) -> EOC7_W<IER_SPEC> {
        EOC7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Overrun Error Interrupt Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn ovre0(&mut self) -> OVRE0_W<IER_SPEC> {
        OVRE0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Overrun Error Interrupt Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn ovre1(&mut self) -> OVRE1_W<IER_SPEC> {
        OVRE1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun Error Interrupt Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn ovre2(&mut self) -> OVRE2_W<IER_SPEC> {
        OVRE2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Overrun Error Interrupt Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn ovre3(&mut self) -> OVRE3_W<IER_SPEC> {
        OVRE3_W::new(self, 11)
    }
    #[doc = "Bit 12 - Overrun Error Interrupt Enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn ovre4(&mut self) -> OVRE4_W<IER_SPEC> {
        OVRE4_W::new(self, 12)
    }
    #[doc = "Bit 13 - Overrun Error Interrupt Enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn ovre5(&mut self) -> OVRE5_W<IER_SPEC> {
        OVRE5_W::new(self, 13)
    }
    #[doc = "Bit 14 - Overrun Error Interrupt Enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn ovre6(&mut self) -> OVRE6_W<IER_SPEC> {
        OVRE6_W::new(self, 14)
    }
    #[doc = "Bit 15 - Overrun Error Interrupt Enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn ovre7(&mut self) -> OVRE7_W<IER_SPEC> {
        OVRE7_W::new(self, 15)
    }
    #[doc = "Bit 16 - Data Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DRDY_W<IER_SPEC> {
        DRDY_W::new(self, 16)
    }
    #[doc = "Bit 17 - General Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn govre(&mut self) -> GOVRE_W<IER_SPEC> {
        GOVRE_W::new(self, 17)
    }
    #[doc = "Bit 18 - End of Receive Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<IER_SPEC> {
        ENDRX_W::new(self, 18)
    }
    #[doc = "Bit 19 - Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<IER_SPEC> {
        RXBUFF_W::new(self, 19)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
