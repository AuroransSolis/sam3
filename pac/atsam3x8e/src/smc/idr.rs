#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `RB_RISE` writer - Ready Busy Rising Edge Detection Interrupt Disable"]
pub type RB_RISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_FALL` writer - Ready Busy Falling Edge Detection Interrupt Disable"]
pub type RB_FALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFRDONE` writer - Transfer Done Interrupt Disable"]
pub type XFRDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDDONE` writer - Command Done Interrupt Disable"]
pub type CMDDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOE` writer - Data Timeout Error Interrupt Disable"]
pub type DTOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDEF` writer - Undefined Area Access Interrupt Disable"]
pub type UNDEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWB` writer - Accessing While Busy Interrupt Disable"]
pub type AWB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFCASE` writer - NFC Access Size Error Interrupt Disable"]
pub type NFCASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EDGE0` writer - Ready/Busy Line 0 Interrupt Disable"]
pub type RB_EDGE0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 4 - Ready Busy Rising Edge Detection Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_rise(&mut self) -> RB_RISE_W<IDR_SPEC> {
        RB_RISE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Ready Busy Falling Edge Detection Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_fall(&mut self) -> RB_FALL_W<IDR_SPEC> {
        RB_FALL_W::new(self, 5)
    }
    #[doc = "Bit 16 - Transfer Done Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn xfrdone(&mut self) -> XFRDONE_W<IDR_SPEC> {
        XFRDONE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Command Done Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmddone(&mut self) -> CMDDONE_W<IDR_SPEC> {
        CMDDONE_W::new(self, 17)
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dtoe(&mut self) -> DTOE_W<IDR_SPEC> {
        DTOE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Undefined Area Access Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn undef(&mut self) -> UNDEF_W<IDR_SPEC> {
        UNDEF_W::new(self, 21)
    }
    #[doc = "Bit 22 - Accessing While Busy Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn awb(&mut self) -> AWB_W<IDR_SPEC> {
        AWB_W::new(self, 22)
    }
    #[doc = "Bit 23 - NFC Access Size Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nfcase(&mut self) -> NFCASE_W<IDR_SPEC> {
        NFCASE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Ready/Busy Line 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_edge0(&mut self) -> RB_EDGE0_W<IDR_SPEC> {
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
#[doc = "SMC NFC Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
