#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `SMCSTS` reader - NAND Flash Controller status (this field cannot be reset)"]
pub type SmcstsR = crate::BitReader;
#[doc = "Field `RB_RISE` reader - Selected Ready Busy Rising Edge Detected"]
pub type RbRiseR = crate::BitReader;
#[doc = "Field `RB_FALL` reader - Selected Ready Busy Falling Edge Detected"]
pub type RbFallR = crate::BitReader;
#[doc = "Field `NFCBUSY` reader - NFC Busy (this field cannot be reset)"]
pub type NfcbusyR = crate::BitReader;
#[doc = "Field `NFCWR` reader - NFC Write/Read Operation (this field cannot be reset)"]
pub type NfcwrR = crate::BitReader;
#[doc = "Field `NFCSID` reader - NFC Chip Select ID (this field cannot be reset)"]
pub type NfcsidR = crate::FieldReader;
#[doc = "Field `XFRDONE` reader - NFC Data Transfer Terminated"]
pub type XfrdoneR = crate::BitReader;
#[doc = "Field `CMDDONE` reader - Command Done"]
pub type CmddoneR = crate::BitReader;
#[doc = "Field `DTOE` reader - Data Timeout Error"]
pub type DtoeR = crate::BitReader;
#[doc = "Field `UNDEF` reader - Undefined Area Error"]
pub type UndefR = crate::BitReader;
#[doc = "Field `AWB` reader - Accessing While Busy"]
pub type AwbR = crate::BitReader;
#[doc = "Field `NFCASE` reader - NFC Access Size Error"]
pub type NfcaseR = crate::BitReader;
#[doc = "Field `RB_EDGE0` reader - Ready/Busy Line 0 Edge Detected"]
pub type RbEdge0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NAND Flash Controller status (this field cannot be reset)"]
    #[inline(always)]
    pub fn smcsts(&self) -> SmcstsR {
        SmcstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Selected Ready Busy Rising Edge Detected"]
    #[inline(always)]
    pub fn rb_rise(&self) -> RbRiseR {
        RbRiseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selected Ready Busy Falling Edge Detected"]
    #[inline(always)]
    pub fn rb_fall(&self) -> RbFallR {
        RbFallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - NFC Busy (this field cannot be reset)"]
    #[inline(always)]
    pub fn nfcbusy(&self) -> NfcbusyR {
        NfcbusyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - NFC Write/Read Operation (this field cannot be reset)"]
    #[inline(always)]
    pub fn nfcwr(&self) -> NfcwrR {
        NfcwrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - NFC Chip Select ID (this field cannot be reset)"]
    #[inline(always)]
    pub fn nfcsid(&self) -> NfcsidR {
        NfcsidR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - NFC Data Transfer Terminated"]
    #[inline(always)]
    pub fn xfrdone(&self) -> XfrdoneR {
        XfrdoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command Done"]
    #[inline(always)]
    pub fn cmddone(&self) -> CmddoneR {
        CmddoneR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error"]
    #[inline(always)]
    pub fn dtoe(&self) -> DtoeR {
        DtoeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Undefined Area Error"]
    #[inline(always)]
    pub fn undef(&self) -> UndefR {
        UndefR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Accessing While Busy"]
    #[inline(always)]
    pub fn awb(&self) -> AwbR {
        AwbR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - NFC Access Size Error"]
    #[inline(always)]
    pub fn nfcase(&self) -> NfcaseR {
        NfcaseR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Ready/Busy Line 0 Edge Detected"]
    #[inline(always)]
    pub fn rb_edge0(&self) -> RbEdge0R {
        RbEdge0R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "SMC NFC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
