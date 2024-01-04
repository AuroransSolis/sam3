#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `RB_RISE` writer - Ready Busy Rising Edge Detection Interrupt Enable"]
pub type RB_RISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_FALL` writer - Ready Busy Falling Edge Detection Interrupt Enable"]
pub type RB_FALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFRDONE` writer - Transfer Done Interrupt Enable"]
pub type XFRDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDDONE` writer - Command Done Interrupt Enable"]
pub type CMDDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOE` writer - Data Timeout Error Interrupt Enable"]
pub type DTOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDEF` writer - Undefined Area Access Interrupt Enable"]
pub type UNDEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWB` writer - Accessing While Busy Interrupt Enable"]
pub type AWB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFCASE` writer - NFC Access Size Error Interrupt Enable"]
pub type NFCASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EDGE0` writer - Ready/Busy Line 0 Interrupt Enable"]
pub type RB_EDGE0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 4 - Ready Busy Rising Edge Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_rise(&mut self) -> RB_RISE_W<IER_SPEC> {
        RB_RISE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Ready Busy Falling Edge Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_fall(&mut self) -> RB_FALL_W<IER_SPEC> {
        RB_FALL_W::new(self, 5)
    }
    #[doc = "Bit 16 - Transfer Done Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xfrdone(&mut self) -> XFRDONE_W<IER_SPEC> {
        XFRDONE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Command Done Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmddone(&mut self) -> CMDDONE_W<IER_SPEC> {
        CMDDONE_W::new(self, 17)
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtoe(&mut self) -> DTOE_W<IER_SPEC> {
        DTOE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Undefined Area Access Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn undef(&mut self) -> UNDEF_W<IER_SPEC> {
        UNDEF_W::new(self, 21)
    }
    #[doc = "Bit 22 - Accessing While Busy Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn awb(&mut self) -> AWB_W<IER_SPEC> {
        AWB_W::new(self, 22)
    }
    #[doc = "Bit 23 - NFC Access Size Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfcase(&mut self) -> NFCASE_W<IER_SPEC> {
        NFCASE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Ready/Busy Line 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_edge0(&mut self) -> RB_EDGE0_W<IER_SPEC> {
        RB_EDGE0_W::new(self, 24)
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
#[doc = "SMC NFC Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: u32 = 0;
}
