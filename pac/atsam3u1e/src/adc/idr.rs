#[doc = "Register `IDR` writer"]
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOC0` writer - End of Conversion Interrupt Disable 0"]
pub type EOC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `EOC1` writer - End of Conversion Interrupt Disable 1"]
pub type EOC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `EOC2` writer - End of Conversion Interrupt Disable 2"]
pub type EOC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `EOC3` writer - End of Conversion Interrupt Disable 3"]
pub type EOC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `EOC4` writer - End of Conversion Interrupt Disable 4"]
pub type EOC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `EOC5` writer - End of Conversion Interrupt Disable 5"]
pub type EOC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `EOC6` writer - End of Conversion Interrupt Disable 6"]
pub type EOC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `EOC7` writer - End of Conversion Interrupt Disable 7"]
pub type EOC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `OVRE0` writer - Overrun Error Interrupt Disable 0"]
pub type OVRE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `OVRE1` writer - Overrun Error Interrupt Disable 1"]
pub type OVRE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `OVRE2` writer - Overrun Error Interrupt Disable 2"]
pub type OVRE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `OVRE3` writer - Overrun Error Interrupt Disable 3"]
pub type OVRE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `OVRE4` writer - Overrun Error Interrupt Disable 4"]
pub type OVRE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `OVRE5` writer - Overrun Error Interrupt Disable 5"]
pub type OVRE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `OVRE6` writer - Overrun Error Interrupt Disable 6"]
pub type OVRE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `OVRE7` writer - Overrun Error Interrupt Disable 7"]
pub type OVRE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `DRDY` writer - Data Ready Interrupt Disable"]
pub type DRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `GOVRE` writer - General Overrun Error Interrupt Disable"]
pub type GOVRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Disable"]
pub type ENDRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Disable"]
pub type RXBUFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - End of Conversion Interrupt Disable 0"]
    #[inline(always)]
    pub fn eoc0(&mut self) -> EOC0_W<0> {
        EOC0_W::new(self)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Disable 1"]
    #[inline(always)]
    pub fn eoc1(&mut self) -> EOC1_W<1> {
        EOC1_W::new(self)
    }
    #[doc = "Bit 2 - End of Conversion Interrupt Disable 2"]
    #[inline(always)]
    pub fn eoc2(&mut self) -> EOC2_W<2> {
        EOC2_W::new(self)
    }
    #[doc = "Bit 3 - End of Conversion Interrupt Disable 3"]
    #[inline(always)]
    pub fn eoc3(&mut self) -> EOC3_W<3> {
        EOC3_W::new(self)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Disable 4"]
    #[inline(always)]
    pub fn eoc4(&mut self) -> EOC4_W<4> {
        EOC4_W::new(self)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Disable 5"]
    #[inline(always)]
    pub fn eoc5(&mut self) -> EOC5_W<5> {
        EOC5_W::new(self)
    }
    #[doc = "Bit 6 - End of Conversion Interrupt Disable 6"]
    #[inline(always)]
    pub fn eoc6(&mut self) -> EOC6_W<6> {
        EOC6_W::new(self)
    }
    #[doc = "Bit 7 - End of Conversion Interrupt Disable 7"]
    #[inline(always)]
    pub fn eoc7(&mut self) -> EOC7_W<7> {
        EOC7_W::new(self)
    }
    #[doc = "Bit 8 - Overrun Error Interrupt Disable 0"]
    #[inline(always)]
    pub fn ovre0(&mut self) -> OVRE0_W<8> {
        OVRE0_W::new(self)
    }
    #[doc = "Bit 9 - Overrun Error Interrupt Disable 1"]
    #[inline(always)]
    pub fn ovre1(&mut self) -> OVRE1_W<9> {
        OVRE1_W::new(self)
    }
    #[doc = "Bit 10 - Overrun Error Interrupt Disable 2"]
    #[inline(always)]
    pub fn ovre2(&mut self) -> OVRE2_W<10> {
        OVRE2_W::new(self)
    }
    #[doc = "Bit 11 - Overrun Error Interrupt Disable 3"]
    #[inline(always)]
    pub fn ovre3(&mut self) -> OVRE3_W<11> {
        OVRE3_W::new(self)
    }
    #[doc = "Bit 12 - Overrun Error Interrupt Disable 4"]
    #[inline(always)]
    pub fn ovre4(&mut self) -> OVRE4_W<12> {
        OVRE4_W::new(self)
    }
    #[doc = "Bit 13 - Overrun Error Interrupt Disable 5"]
    #[inline(always)]
    pub fn ovre5(&mut self) -> OVRE5_W<13> {
        OVRE5_W::new(self)
    }
    #[doc = "Bit 14 - Overrun Error Interrupt Disable 6"]
    #[inline(always)]
    pub fn ovre6(&mut self) -> OVRE6_W<14> {
        OVRE6_W::new(self)
    }
    #[doc = "Bit 15 - Overrun Error Interrupt Disable 7"]
    #[inline(always)]
    pub fn ovre7(&mut self) -> OVRE7_W<15> {
        OVRE7_W::new(self)
    }
    #[doc = "Bit 16 - Data Ready Interrupt Disable"]
    #[inline(always)]
    pub fn drdy(&mut self) -> DRDY_W<16> {
        DRDY_W::new(self)
    }
    #[doc = "Bit 17 - General Overrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn govre(&mut self) -> GOVRE_W<17> {
        GOVRE_W::new(self)
    }
    #[doc = "Bit 18 - End of Receive Buffer Interrupt Disable"]
    #[inline(always)]
    pub fn endrx(&mut self) -> ENDRX_W<18> {
        ENDRX_W::new(self)
    }
    #[doc = "Bit 19 - Receive Buffer Full Interrupt Disable"]
    #[inline(always)]
    pub fn rxbuff(&mut self) -> RXBUFF_W<19> {
        RXBUFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
}
