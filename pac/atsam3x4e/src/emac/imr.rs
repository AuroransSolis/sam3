#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `MFD` reader - Management Frame sent"]
pub type MfdR = crate::BitReader;
#[doc = "Field `RCOMP` reader - Receive Complete"]
pub type RcompR = crate::BitReader;
#[doc = "Field `RXUBR` reader - Receive Used Bit Read"]
pub type RxubrR = crate::BitReader;
#[doc = "Field `TXUBR` reader - Transmit Used Bit Read"]
pub type TxubrR = crate::BitReader;
#[doc = "Field `TUND` reader - Ethernet Transmit Buffer Underrun"]
pub type TundR = crate::BitReader;
#[doc = "Field `RLE` reader - Retry Limit Exceeded"]
pub type RleR = crate::BitReader;
#[doc = "Field `TXERR` reader - "]
pub type TxerrR = crate::BitReader;
#[doc = "Field `TCOMP` reader - Transmit Complete"]
pub type TcompR = crate::BitReader;
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub type RovrR = crate::BitReader;
#[doc = "Field `HRESP` reader - Hresp not OK"]
pub type HrespR = crate::BitReader;
#[doc = "Field `PFR` reader - Pause Frame Received"]
pub type PfrR = crate::BitReader;
#[doc = "Field `PTZ` reader - Pause Time Zero"]
pub type PtzR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Management Frame sent"]
    #[inline(always)]
    pub fn mfd(&self) -> MfdR {
        MfdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&self) -> RcompR {
        RcompR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&self) -> RxubrR {
        RxubrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Used Bit Read"]
    #[inline(always)]
    pub fn txubr(&self) -> TxubrR {
        TxubrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ethernet Transmit Buffer Underrun"]
    #[inline(always)]
    pub fn tund(&self) -> TundR {
        TundR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded"]
    #[inline(always)]
    pub fn rle(&self) -> RleR {
        RleR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn txerr(&self) -> TxerrR {
        TxerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TcompR {
        TcompR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> RovrR {
        RovrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Hresp not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HrespR {
        HrespR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pause Frame Received"]
    #[inline(always)]
    pub fn pfr(&self) -> PfrR {
        PfrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    pub fn ptz(&self) -> PtzR {
        PtzR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0x3fff"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0x3fff;
}
