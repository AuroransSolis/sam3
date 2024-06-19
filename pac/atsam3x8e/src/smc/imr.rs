#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `RB_RISE` reader - Ready Busy Rising Edge Detection Interrupt Mask"]
pub type RbRiseR = crate::BitReader;
#[doc = "Field `RB_FALL` reader - Ready Busy Falling Edge Detection Interrupt Mask"]
pub type RbFallR = crate::BitReader;
#[doc = "Field `XFRDONE` reader - Transfer Done Interrupt Mask"]
pub type XfrdoneR = crate::BitReader;
#[doc = "Field `CMDDONE` reader - Command Done Interrupt Mask"]
pub type CmddoneR = crate::BitReader;
#[doc = "Field `DTOE` reader - Data Timeout Error Interrupt Mask"]
pub type DtoeR = crate::BitReader;
#[doc = "Field `UNDEF` reader - Undefined Area Access Interrupt Mask5"]
pub type UndefR = crate::BitReader;
#[doc = "Field `AWB` reader - Accessing While Busy Interrupt Mask"]
pub type AwbR = crate::BitReader;
#[doc = "Field `NFCASE` reader - NFC Access Size Error Interrupt Mask"]
pub type NfcaseR = crate::BitReader;
#[doc = "Field `RB_EDGE0` reader - Ready/Busy Line 0 Interrupt Mask"]
pub type RbEdge0R = crate::BitReader;
impl R {
    #[doc = "Bit 4 - Ready Busy Rising Edge Detection Interrupt Mask"]
    #[inline(always)]
    pub fn rb_rise(&self) -> RbRiseR {
        RbRiseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Ready Busy Falling Edge Detection Interrupt Mask"]
    #[inline(always)]
    pub fn rb_fall(&self) -> RbFallR {
        RbFallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Transfer Done Interrupt Mask"]
    #[inline(always)]
    pub fn xfrdone(&self) -> XfrdoneR {
        XfrdoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command Done Interrupt Mask"]
    #[inline(always)]
    pub fn cmddone(&self) -> CmddoneR {
        CmddoneR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Mask"]
    #[inline(always)]
    pub fn dtoe(&self) -> DtoeR {
        DtoeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Undefined Area Access Interrupt Mask5"]
    #[inline(always)]
    pub fn undef(&self) -> UndefR {
        UndefR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Accessing While Busy Interrupt Mask"]
    #[inline(always)]
    pub fn awb(&self) -> AwbR {
        AwbR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - NFC Access Size Error Interrupt Mask"]
    #[inline(always)]
    pub fn nfcase(&self) -> NfcaseR {
        NfcaseR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Ready/Busy Line 0 Interrupt Mask"]
    #[inline(always)]
    pub fn rb_edge0(&self) -> RbEdge0R {
        RbEdge0R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "SMC NFC Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
