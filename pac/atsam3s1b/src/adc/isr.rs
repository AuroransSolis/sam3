#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EOC0` reader - End of Conversion 0"]
pub type EOC0_R = crate::BitReader<bool>;
#[doc = "Field `EOC1` reader - End of Conversion 1"]
pub type EOC1_R = crate::BitReader<bool>;
#[doc = "Field `EOC2` reader - End of Conversion 2"]
pub type EOC2_R = crate::BitReader<bool>;
#[doc = "Field `EOC3` reader - End of Conversion 3"]
pub type EOC3_R = crate::BitReader<bool>;
#[doc = "Field `EOC4` reader - End of Conversion 4"]
pub type EOC4_R = crate::BitReader<bool>;
#[doc = "Field `EOC5` reader - End of Conversion 5"]
pub type EOC5_R = crate::BitReader<bool>;
#[doc = "Field `EOC6` reader - End of Conversion 6"]
pub type EOC6_R = crate::BitReader<bool>;
#[doc = "Field `EOC7` reader - End of Conversion 7"]
pub type EOC7_R = crate::BitReader<bool>;
#[doc = "Field `EOC8` reader - End of Conversion 8"]
pub type EOC8_R = crate::BitReader<bool>;
#[doc = "Field `EOC9` reader - End of Conversion 9"]
pub type EOC9_R = crate::BitReader<bool>;
#[doc = "Field `EOC10` reader - End of Conversion 10"]
pub type EOC10_R = crate::BitReader<bool>;
#[doc = "Field `EOC11` reader - End of Conversion 11"]
pub type EOC11_R = crate::BitReader<bool>;
#[doc = "Field `EOC12` reader - End of Conversion 12"]
pub type EOC12_R = crate::BitReader<bool>;
#[doc = "Field `EOC13` reader - End of Conversion 13"]
pub type EOC13_R = crate::BitReader<bool>;
#[doc = "Field `EOC14` reader - End of Conversion 14"]
pub type EOC14_R = crate::BitReader<bool>;
#[doc = "Field `EOC15` reader - End of Conversion 15"]
pub type EOC15_R = crate::BitReader<bool>;
#[doc = "Field `DRDY` reader - Data Ready"]
pub type DRDY_R = crate::BitReader<bool>;
#[doc = "Field `GOVRE` reader - General Overrun Error"]
pub type GOVRE_R = crate::BitReader<bool>;
#[doc = "Field `COMPE` reader - Comparison Error"]
pub type COMPE_R = crate::BitReader<bool>;
#[doc = "Field `ENDRX` reader - End of RX Buffer"]
pub type ENDRX_R = crate::BitReader<bool>;
#[doc = "Field `RXBUFF` reader - RX Buffer Full"]
pub type RXBUFF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - End of Conversion 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> EOC0_R {
        EOC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Conversion 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Conversion 2"]
    #[inline(always)]
    pub fn eoc2(&self) -> EOC2_R {
        EOC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Conversion 3"]
    #[inline(always)]
    pub fn eoc3(&self) -> EOC3_R {
        EOC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Conversion 4"]
    #[inline(always)]
    pub fn eoc4(&self) -> EOC4_R {
        EOC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion 5"]
    #[inline(always)]
    pub fn eoc5(&self) -> EOC5_R {
        EOC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of Conversion 6"]
    #[inline(always)]
    pub fn eoc6(&self) -> EOC6_R {
        EOC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End of Conversion 7"]
    #[inline(always)]
    pub fn eoc7(&self) -> EOC7_R {
        EOC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - End of Conversion 8"]
    #[inline(always)]
    pub fn eoc8(&self) -> EOC8_R {
        EOC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - End of Conversion 9"]
    #[inline(always)]
    pub fn eoc9(&self) -> EOC9_R {
        EOC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - End of Conversion 10"]
    #[inline(always)]
    pub fn eoc10(&self) -> EOC10_R {
        EOC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End of Conversion 11"]
    #[inline(always)]
    pub fn eoc11(&self) -> EOC11_R {
        EOC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - End of Conversion 12"]
    #[inline(always)]
    pub fn eoc12(&self) -> EOC12_R {
        EOC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - End of Conversion 13"]
    #[inline(always)]
    pub fn eoc13(&self) -> EOC13_R {
        EOC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - End of Conversion 14"]
    #[inline(always)]
    pub fn eoc14(&self) -> EOC14_R {
        EOC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Conversion 15"]
    #[inline(always)]
    pub fn eoc15(&self) -> EOC15_R {
        EOC15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Data Ready"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - General Overrun Error"]
    #[inline(always)]
    pub fn govre(&self) -> GOVRE_R {
        GOVRE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Comparison Error"]
    #[inline(always)]
    pub fn compe(&self) -> COMPE_R {
        COMPE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - End of RX Buffer"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RX Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
