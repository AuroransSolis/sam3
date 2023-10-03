#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `EOC0` writer - End of Conversion Interrupt Enable 0"]
pub type EOC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC1` writer - End of Conversion Interrupt Enable 1"]
pub type EOC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC2` writer - End of Conversion Interrupt Enable 2"]
pub type EOC2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC3` writer - End of Conversion Interrupt Enable 3"]
pub type EOC3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC4` writer - End of Conversion Interrupt Enable 4"]
pub type EOC4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC5` writer - End of Conversion Interrupt Enable 5"]
pub type EOC5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC6` writer - End of Conversion Interrupt Enable 6"]
pub type EOC6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC7` writer - End of Conversion Interrupt Enable 7"]
pub type EOC7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC8` writer - End of Conversion Interrupt Enable 8"]
pub type EOC8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC9` writer - End of Conversion Interrupt Enable 9"]
pub type EOC9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC10` writer - End of Conversion Interrupt Enable 10"]
pub type EOC10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC11` writer - End of Conversion Interrupt Enable 11"]
pub type EOC11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC12` writer - End of Conversion Interrupt Enable 12"]
pub type EOC12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC13` writer - End of Conversion Interrupt Enable 13"]
pub type EOC13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC14` writer - End of Conversion Interrupt Enable 14"]
pub type EOC14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC15` writer - End of Conversion Interrupt Enable 15"]
pub type EOC15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOCAL` writer - End of Calibration Sequence"]
pub type EOCAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRDY` writer - Data Ready Interrupt Enable"]
pub type DRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GOVRE` writer - General Overrun Error Interrupt Enable"]
pub type GOVRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMPE` writer - Comparison Event Interrupt Enable"]
pub type COMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Enable"]
pub type ENDRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Enable"]
pub type RXBUFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - End of Conversion Interrupt Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn eoc0(&mut self) -> EOC0_W<IER_SPEC, 0> {
        EOC0_W::new(self)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn eoc1(&mut self) -> EOC1_W<IER_SPEC, 1> {
        EOC1_W::new(self)
    }
    #[doc = "Bit 2 - End of Conversion Interrupt Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn eoc2(&mut self) -> EOC2_W<IER_SPEC, 2> {
        EOC2_W::new(self)
    }
    #[doc = "Bit 3 - End of Conversion Interrupt Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn eoc3(&mut self) -> EOC3_W<IER_SPEC, 3> {
        EOC3_W::new(self)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn eoc4(&mut self) -> EOC4_W<IER_SPEC, 4> {
        EOC4_W::new(self)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn eoc5(&mut self) -> EOC5_W<IER_SPEC, 5> {
        EOC5_W::new(self)
    }
    #[doc = "Bit 6 - End of Conversion Interrupt Enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn eoc6(&mut self) -> EOC6_W<IER_SPEC, 6> {
        EOC6_W::new(self)
    }
    #[doc = "Bit 7 - End of Conversion Interrupt Enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn eoc7(&mut self) -> EOC7_W<IER_SPEC, 7> {
        EOC7_W::new(self)
    }
    #[doc = "Bit 8 - End of Conversion Interrupt Enable 8"]
    #[inline(always)]
    #[must_use]
    pub fn eoc8(&mut self) -> EOC8_W<IER_SPEC, 8> {
        EOC8_W::new(self)
    }
    #[doc = "Bit 9 - End of Conversion Interrupt Enable 9"]
    #[inline(always)]
    #[must_use]
    pub fn eoc9(&mut self) -> EOC9_W<IER_SPEC, 9> {
        EOC9_W::new(self)
    }
    #[doc = "Bit 10 - End of Conversion Interrupt Enable 10"]
    #[inline(always)]
    #[must_use]
    pub fn eoc10(&mut self) -> EOC10_W<IER_SPEC, 10> {
        EOC10_W::new(self)
    }
    #[doc = "Bit 11 - End of Conversion Interrupt Enable 11"]
    #[inline(always)]
    #[must_use]
    pub fn eoc11(&mut self) -> EOC11_W<IER_SPEC, 11> {
        EOC11_W::new(self)
    }
    #[doc = "Bit 12 - End of Conversion Interrupt Enable 12"]
    #[inline(always)]
    #[must_use]
    pub fn eoc12(&mut self) -> EOC12_W<IER_SPEC, 12> {
        EOC12_W::new(self)
    }
    #[doc = "Bit 13 - End of Conversion Interrupt Enable 13"]
    #[inline(always)]
    #[must_use]
    pub fn eoc13(&mut self) -> EOC13_W<IER_SPEC, 13> {
        EOC13_W::new(self)
    }
    #[doc = "Bit 14 - End of Conversion Interrupt Enable 14"]
    #[inline(always)]
    #[must_use]
    pub fn eoc14(&mut self) -> EOC14_W<IER_SPEC, 14> {
        EOC14_W::new(self)
    }
    #[doc = "Bit 15 - End of Conversion Interrupt Enable 15"]
    #[inline(always)]
    #[must_use]
    pub fn eoc15(&mut self) -> EOC15_W<IER_SPEC, 15> {
        EOC15_W::new(self)
    }
    #[doc = "Bit 23 - End of Calibration Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn eocal(&mut self) -> EOCAL_W<IER_SPEC, 23> {
        EOCAL_W::new(self)
    }
    #[doc = "Bit 24 - Data Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DRDY_W<IER_SPEC, 24> {
        DRDY_W::new(self)
    }
    #[doc = "Bit 25 - General Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn govre(&mut self) -> GOVRE_W<IER_SPEC, 25> {
        GOVRE_W::new(self)
    }
    #[doc = "Bit 26 - Comparison Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compe(&mut self) -> COMPE_W<IER_SPEC, 26> {
        COMPE_W::new(self)
    }
    #[doc = "Bit 27 - End of Receive Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<IER_SPEC, 27> {
        ENDRX_W::new(self)
    }
    #[doc = "Bit 28 - Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<IER_SPEC, 28> {
        RXBUFF_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
