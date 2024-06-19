#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `RB_RISE` writer - Ready Busy Rising Edge Detection Interrupt Disable"]
pub type RbRiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_FALL` writer - Ready Busy Falling Edge Detection Interrupt Disable"]
pub type RbFallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFRDONE` writer - Transfer Done Interrupt Disable"]
pub type XfrdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDDONE` writer - Command Done Interrupt Disable"]
pub type CmddoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOE` writer - Data Timeout Error Interrupt Disable"]
pub type DtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDEF` writer - Undefined Area Access Interrupt Disable"]
pub type UndefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWB` writer - Accessing While Busy Interrupt Disable"]
pub type AwbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFCASE` writer - NFC Access Size Error Interrupt Disable"]
pub type NfcaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_EDGE0` writer - Ready/Busy Line 0 Interrupt Disable"]
pub type RbEdge0W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 4 - Ready Busy Rising Edge Detection Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_rise(&mut self) -> RbRiseW<IdrSpec> {
        RbRiseW::new(self, 4)
    }
    #[doc = "Bit 5 - Ready Busy Falling Edge Detection Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_fall(&mut self) -> RbFallW<IdrSpec> {
        RbFallW::new(self, 5)
    }
    #[doc = "Bit 16 - Transfer Done Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn xfrdone(&mut self) -> XfrdoneW<IdrSpec> {
        XfrdoneW::new(self, 16)
    }
    #[doc = "Bit 17 - Command Done Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmddone(&mut self) -> CmddoneW<IdrSpec> {
        CmddoneW::new(self, 17)
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dtoe(&mut self) -> DtoeW<IdrSpec> {
        DtoeW::new(self, 20)
    }
    #[doc = "Bit 21 - Undefined Area Access Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn undef(&mut self) -> UndefW<IdrSpec> {
        UndefW::new(self, 21)
    }
    #[doc = "Bit 22 - Accessing While Busy Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn awb(&mut self) -> AwbW<IdrSpec> {
        AwbW::new(self, 22)
    }
    #[doc = "Bit 23 - NFC Access Size Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nfcase(&mut self) -> NfcaseW<IdrSpec> {
        NfcaseW::new(self, 23)
    }
    #[doc = "Bit 24 - Ready/Busy Line 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_edge0(&mut self) -> RbEdge0W<IdrSpec> {
        RbEdge0W::new(self, 24)
    }
}
#[doc = "SMC NFC Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {
    const RESET_VALUE: u32 = 0;
}
