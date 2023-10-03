#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `EOC0` writer - End of Conversion Interrupt Disable 0"]
pub type EOC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC1` writer - End of Conversion Interrupt Disable 1"]
pub type EOC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC2` writer - End of Conversion Interrupt Disable 2"]
pub type EOC2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC3` writer - End of Conversion Interrupt Disable 3"]
pub type EOC3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC4` writer - End of Conversion Interrupt Disable 4"]
pub type EOC4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC5` writer - End of Conversion Interrupt Disable 5"]
pub type EOC5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC6` writer - End of Conversion Interrupt Disable 6"]
pub type EOC6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC7` writer - End of Conversion Interrupt Disable 7"]
pub type EOC7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC8` writer - End of Conversion Interrupt Disable 8"]
pub type EOC8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC9` writer - End of Conversion Interrupt Disable 9"]
pub type EOC9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC10` writer - End of Conversion Interrupt Disable 10"]
pub type EOC10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC11` writer - End of Conversion Interrupt Disable 11"]
pub type EOC11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC12` writer - End of Conversion Interrupt Disable 12"]
pub type EOC12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC13` writer - End of Conversion Interrupt Disable 13"]
pub type EOC13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC14` writer - End of Conversion Interrupt Disable 14"]
pub type EOC14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC15` writer - End of Conversion Interrupt Disable 15"]
pub type EOC15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOCAL` writer - End of Calibration Sequence"]
pub type EOCAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRDY` writer - Data Ready Interrupt Disable"]
pub type DRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GOVRE` writer - General Overrun Error Interrupt Disable"]
pub type GOVRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMPE` writer - Comparison Event Interrupt Disable"]
pub type COMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Disable"]
pub type ENDRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Disable"]
pub type RXBUFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - End of Conversion Interrupt Disable 0"]
    #[inline(always)]
    #[must_use]
    pub fn eoc0(&mut self) -> EOC0_W<IDR_SPEC, 0> {
        EOC0_W::new(self)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Disable 1"]
    #[inline(always)]
    #[must_use]
    pub fn eoc1(&mut self) -> EOC1_W<IDR_SPEC, 1> {
        EOC1_W::new(self)
    }
    #[doc = "Bit 2 - End of Conversion Interrupt Disable 2"]
    #[inline(always)]
    #[must_use]
    pub fn eoc2(&mut self) -> EOC2_W<IDR_SPEC, 2> {
        EOC2_W::new(self)
    }
    #[doc = "Bit 3 - End of Conversion Interrupt Disable 3"]
    #[inline(always)]
    #[must_use]
    pub fn eoc3(&mut self) -> EOC3_W<IDR_SPEC, 3> {
        EOC3_W::new(self)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Disable 4"]
    #[inline(always)]
    #[must_use]
    pub fn eoc4(&mut self) -> EOC4_W<IDR_SPEC, 4> {
        EOC4_W::new(self)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Disable 5"]
    #[inline(always)]
    #[must_use]
    pub fn eoc5(&mut self) -> EOC5_W<IDR_SPEC, 5> {
        EOC5_W::new(self)
    }
    #[doc = "Bit 6 - End of Conversion Interrupt Disable 6"]
    #[inline(always)]
    #[must_use]
    pub fn eoc6(&mut self) -> EOC6_W<IDR_SPEC, 6> {
        EOC6_W::new(self)
    }
    #[doc = "Bit 7 - End of Conversion Interrupt Disable 7"]
    #[inline(always)]
    #[must_use]
    pub fn eoc7(&mut self) -> EOC7_W<IDR_SPEC, 7> {
        EOC7_W::new(self)
    }
    #[doc = "Bit 8 - End of Conversion Interrupt Disable 8"]
    #[inline(always)]
    #[must_use]
    pub fn eoc8(&mut self) -> EOC8_W<IDR_SPEC, 8> {
        EOC8_W::new(self)
    }
    #[doc = "Bit 9 - End of Conversion Interrupt Disable 9"]
    #[inline(always)]
    #[must_use]
    pub fn eoc9(&mut self) -> EOC9_W<IDR_SPEC, 9> {
        EOC9_W::new(self)
    }
    #[doc = "Bit 10 - End of Conversion Interrupt Disable 10"]
    #[inline(always)]
    #[must_use]
    pub fn eoc10(&mut self) -> EOC10_W<IDR_SPEC, 10> {
        EOC10_W::new(self)
    }
    #[doc = "Bit 11 - End of Conversion Interrupt Disable 11"]
    #[inline(always)]
    #[must_use]
    pub fn eoc11(&mut self) -> EOC11_W<IDR_SPEC, 11> {
        EOC11_W::new(self)
    }
    #[doc = "Bit 12 - End of Conversion Interrupt Disable 12"]
    #[inline(always)]
    #[must_use]
    pub fn eoc12(&mut self) -> EOC12_W<IDR_SPEC, 12> {
        EOC12_W::new(self)
    }
    #[doc = "Bit 13 - End of Conversion Interrupt Disable 13"]
    #[inline(always)]
    #[must_use]
    pub fn eoc13(&mut self) -> EOC13_W<IDR_SPEC, 13> {
        EOC13_W::new(self)
    }
    #[doc = "Bit 14 - End of Conversion Interrupt Disable 14"]
    #[inline(always)]
    #[must_use]
    pub fn eoc14(&mut self) -> EOC14_W<IDR_SPEC, 14> {
        EOC14_W::new(self)
    }
    #[doc = "Bit 15 - End of Conversion Interrupt Disable 15"]
    #[inline(always)]
    #[must_use]
    pub fn eoc15(&mut self) -> EOC15_W<IDR_SPEC, 15> {
        EOC15_W::new(self)
    }
    #[doc = "Bit 23 - End of Calibration Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn eocal(&mut self) -> EOCAL_W<IDR_SPEC, 23> {
        EOCAL_W::new(self)
    }
    #[doc = "Bit 24 - Data Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DRDY_W<IDR_SPEC, 24> {
        DRDY_W::new(self)
    }
    #[doc = "Bit 25 - General Overrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn govre(&mut self) -> GOVRE_W<IDR_SPEC, 25> {
        GOVRE_W::new(self)
    }
    #[doc = "Bit 26 - Comparison Event Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn compe(&mut self) -> COMPE_W<IDR_SPEC, 26> {
        COMPE_W::new(self)
    }
    #[doc = "Bit 27 - End of Receive Buffer Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<IDR_SPEC, 27> {
        ENDRX_W::new(self)
    }
    #[doc = "Bit 28 - Receive Buffer Full Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<IDR_SPEC, 28> {
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
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
