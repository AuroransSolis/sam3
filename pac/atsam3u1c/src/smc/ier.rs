#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `RB_RISE` writer - Ready Busy Rising Edge Detection Interrupt Enable"]
pub type RbRiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_FALL` writer - Ready Busy Falling Edge Detection Interrupt Enable"]
pub type RbFallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFRDONE` writer - Transfer Done Interrupt Enable"]
pub type XfrdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDDONE` writer - Command Done Interrupt Enable"]
pub type CmddoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOE` writer - Data Timeout Error Interrupt Enable"]
pub type DtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDEF` writer - Undefined Area Access Interrupt Enable"]
pub type UndefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWB` writer - Accessing While Busy Interrupt Enable"]
pub type AwbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFCASE` writer - NFC Access Size Error Interrupt Enable"]
pub type NfcaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EDGE0` writer - Ready/Busy Line 0 Interrupt Enable"]
pub type RbEdge0W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 4 - Ready Busy Rising Edge Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_rise(&mut self) -> RbRiseW<IerSpec> {
        RbRiseW::new(self, 4)
    }
    #[doc = "Bit 5 - Ready Busy Falling Edge Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_fall(&mut self) -> RbFallW<IerSpec> {
        RbFallW::new(self, 5)
    }
    #[doc = "Bit 16 - Transfer Done Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xfrdone(&mut self) -> XfrdoneW<IerSpec> {
        XfrdoneW::new(self, 16)
    }
    #[doc = "Bit 17 - Command Done Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmddone(&mut self) -> CmddoneW<IerSpec> {
        CmddoneW::new(self, 17)
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtoe(&mut self) -> DtoeW<IerSpec> {
        DtoeW::new(self, 20)
    }
    #[doc = "Bit 21 - Undefined Area Access Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn undef(&mut self) -> UndefW<IerSpec> {
        UndefW::new(self, 21)
    }
    #[doc = "Bit 22 - Accessing While Busy Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn awb(&mut self) -> AwbW<IerSpec> {
        AwbW::new(self, 22)
    }
    #[doc = "Bit 23 - NFC Access Size Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfcase(&mut self) -> NfcaseW<IerSpec> {
        NfcaseW::new(self, 23)
    }
    #[doc = "Bit 24 - Ready/Busy Line 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_edge0(&mut self) -> RbEdge0W<IerSpec> {
        RbEdge0W::new(self, 24)
    }
}
#[doc = "SMC NFC Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {
    const RESET_VALUE: u32 = 0;
}
