#[doc = "Register `IER` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<IER_SPEC>);
#[doc = "Field `RB_RISE` writer - Ready Busy Rising Edge Detection Interrupt Enable"]
pub type RB_RISE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `RB_FALL` writer - Ready Busy Falling Edge Detection Interrupt Enable"]
pub type RB_FALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `XFRDONE` writer - Transfer Done Interrupt Enable"]
pub type XFRDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `CMDDONE` writer - Command Done Interrupt Enable"]
pub type CMDDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `DTOE` writer - Data Timeout Error Interrupt Enable"]
pub type DTOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `UNDEF` writer - Undefined Area Access Interrupt Enable"]
pub type UNDEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `AWB` writer - Accessing While Busy Interrupt Enable"]
pub type AWB_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `NFCASE` writer - NFC Access Size Error Interrupt Enable"]
pub type NFCASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `RB_EDGE0` writer - Ready/Busy Line 0 Interrupt Enable"]
pub type RB_EDGE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 4 - Ready Busy Rising Edge Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_rise(&mut self) -> RB_RISE_W<4> {
        RB_RISE_W::new(self)
    }
    #[doc = "Bit 5 - Ready Busy Falling Edge Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_fall(&mut self) -> RB_FALL_W<5> {
        RB_FALL_W::new(self)
    }
    #[doc = "Bit 16 - Transfer Done Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xfrdone(&mut self) -> XFRDONE_W<16> {
        XFRDONE_W::new(self)
    }
    #[doc = "Bit 17 - Command Done Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmddone(&mut self) -> CMDDONE_W<17> {
        CMDDONE_W::new(self)
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtoe(&mut self) -> DTOE_W<20> {
        DTOE_W::new(self)
    }
    #[doc = "Bit 21 - Undefined Area Access Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn undef(&mut self) -> UNDEF_W<21> {
        UNDEF_W::new(self)
    }
    #[doc = "Bit 22 - Accessing While Busy Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn awb(&mut self) -> AWB_W<22> {
        AWB_W::new(self)
    }
    #[doc = "Bit 23 - NFC Access Size Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfcase(&mut self) -> NFCASE_W<23> {
        NFCASE_W::new(self)
    }
    #[doc = "Bit 24 - Ready/Busy Line 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_edge0(&mut self) -> RB_EDGE0_W<24> {
        RB_EDGE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC NFC Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
