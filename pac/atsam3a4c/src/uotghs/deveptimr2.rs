#[doc = "Register `DEVEPTIMR2` reader"]
pub type R = crate::R<Deveptimr2Spec>;
#[doc = "Field `TXINE` reader - Transmitted IN Data Interrupt"]
pub type TxineR = crate::BitReader;
#[doc = "Field `RXOUTE` reader - Received OUT Data Interrupt"]
pub type RxouteR = crate::BitReader;
#[doc = "Field `RXSTPE` reader - Received SETUP Interrupt"]
pub type RxstpeR = crate::BitReader;
#[doc = "Field `NAKOUTE` reader - NAKed OUT Interrupt"]
pub type NakouteR = crate::BitReader;
#[doc = "Field `NAKINE` reader - NAKed IN Interrupt"]
pub type NakineR = crate::BitReader;
#[doc = "Field `OVERFE` reader - Overflow Interrupt"]
pub type OverfeR = crate::BitReader;
#[doc = "Field `STALLEDE` reader - STALLed Interrupt"]
pub type StalledeR = crate::BitReader;
#[doc = "Field `SHORTPACKETE` reader - Short Packet Interrupt"]
pub type ShortpacketeR = crate::BitReader;
#[doc = "Field `NBUSYBKE` reader - Number of Busy Banks Interrupt"]
pub type NbusybkeR = crate::BitReader;
#[doc = "Field `KILLBK` reader - Kill IN Bank"]
pub type KillbkR = crate::BitReader;
#[doc = "Field `FIFOCON` reader - FIFO Control"]
pub type FifoconR = crate::BitReader;
#[doc = "Field `EPDISHDMA` reader - Endpoint Interrupts Disable HDMA Request"]
pub type EpdishdmaR = crate::BitReader;
#[doc = "Field `NYETDIS` reader - NYET Token Disable"]
pub type NyetdisR = crate::BitReader;
#[doc = "Field `RSTDT` reader - Reset Data Toggle"]
pub type RstdtR = crate::BitReader;
#[doc = "Field `STALLRQ` reader - STALL Request"]
pub type StallrqR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txine(&self) -> TxineR {
        TxineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxoute(&self) -> RxouteR {
        RxouteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt"]
    #[inline(always)]
    pub fn rxstpe(&self) -> RxstpeR {
        RxstpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt"]
    #[inline(always)]
    pub fn nakoute(&self) -> NakouteR {
        NakouteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt"]
    #[inline(always)]
    pub fn nakine(&self) -> NakineR {
        NakineR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfe(&self) -> OverfeR {
        OverfeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STALLed Interrupt"]
    #[inline(always)]
    pub fn stallede(&self) -> StalledeR {
        StalledeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpackete(&self) -> ShortpacketeR {
        ShortpacketeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt"]
    #[inline(always)]
    pub fn nbusybke(&self) -> NbusybkeR {
        NbusybkeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    pub fn killbk(&self) -> KillbkR {
        KillbkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocon(&self) -> FifoconR {
        FifoconR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request"]
    #[inline(always)]
    pub fn epdishdma(&self) -> EpdishdmaR {
        EpdishdmaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NYET Token Disable"]
    #[inline(always)]
    pub fn nyetdis(&self) -> NyetdisR {
        NyetdisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset Data Toggle"]
    #[inline(always)]
    pub fn rstdt(&self) -> RstdtR {
        RstdtR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - STALL Request"]
    #[inline(always)]
    pub fn stallrq(&self) -> StallrqR {
        StallrqR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Device Endpoint Mask Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Deveptimr2Spec;
impl crate::RegisterSpec for Deveptimr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deveptimr2::R`](R) reader structure"]
impl crate::Readable for Deveptimr2Spec {}
