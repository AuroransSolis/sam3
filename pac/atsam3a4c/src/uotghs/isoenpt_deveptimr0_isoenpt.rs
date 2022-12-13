#[doc = "Register `DEVEPTIMR0_ISOENPT` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<ISOENPT_DEVEPTIMR0_ISOENPT_SPEC>);
#[doc = "Field `TXINE` reader - Transmitted IN Data Interrupt"]
pub type TXINE_R = crate::BitReader<bool>;
#[doc = "Field `RXOUTE` reader - Received OUT Data Interrupt"]
pub type RXOUTE_R = crate::BitReader<bool>;
#[doc = "Field `UNDERFE` reader - Underflow Interrupt"]
pub type UNDERFE_R = crate::BitReader<bool>;
#[doc = "Field `HBISOINERRE` reader - High Bandwidth Isochronous IN Error Interrupt"]
pub type HBISOINERRE_R = crate::BitReader<bool>;
#[doc = "Field `HBISOFLUSHE` reader - High Bandwidth Isochronous IN Flush Interrupt"]
pub type HBISOFLUSHE_R = crate::BitReader<bool>;
#[doc = "Field `OVERFE` reader - Overflow Interrupt"]
pub type OVERFE_R = crate::BitReader<bool>;
#[doc = "Field `CRCERRE` reader - CRC Error Interrupt"]
pub type CRCERRE_R = crate::BitReader<bool>;
#[doc = "Field `SHORTPACKETE` reader - Short Packet Interrupt"]
pub type SHORTPACKETE_R = crate::BitReader<bool>;
#[doc = "Field `MDATAE` reader - MData Interrupt"]
pub type MDATAE_R = crate::BitReader<bool>;
#[doc = "Field `DATAXE` reader - DataX Interrupt"]
pub type DATAXE_R = crate::BitReader<bool>;
#[doc = "Field `ERRORTRANSE` reader - Transaction Error Interrupt"]
pub type ERRORTRANSE_R = crate::BitReader<bool>;
#[doc = "Field `NBUSYBKE` reader - Number of Busy Banks Interrupt"]
pub type NBUSYBKE_R = crate::BitReader<bool>;
#[doc = "Field `KILLBK` reader - Kill IN Bank"]
pub type KILLBK_R = crate::BitReader<bool>;
#[doc = "Field `FIFOCON` reader - FIFO Control"]
pub type FIFOCON_R = crate::BitReader<bool>;
#[doc = "Field `EPDISHDMA` reader - Endpoint Interrupts Disable HDMA Request"]
pub type EPDISHDMA_R = crate::BitReader<bool>;
#[doc = "Field `RSTDT` reader - Reset Data Toggle"]
pub type RSTDT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txine(&self) -> TXINE_R {
        TXINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxoute(&self) -> RXOUTE_R {
        RXOUTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Underflow Interrupt"]
    #[inline(always)]
    pub fn underfe(&self) -> UNDERFE_R {
        UNDERFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Error Interrupt"]
    #[inline(always)]
    pub fn hbisoinerre(&self) -> HBISOINERRE_R {
        HBISOINERRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt"]
    #[inline(always)]
    pub fn hbisoflushe(&self) -> HBISOFLUSHE_R {
        HBISOFLUSHE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfe(&self) -> OVERFE_R {
        OVERFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC Error Interrupt"]
    #[inline(always)]
    pub fn crcerre(&self) -> CRCERRE_R {
        CRCERRE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpackete(&self) -> SHORTPACKETE_R {
        SHORTPACKETE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MData Interrupt"]
    #[inline(always)]
    pub fn mdatae(&self) -> MDATAE_R {
        MDATAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DataX Interrupt"]
    #[inline(always)]
    pub fn dataxe(&self) -> DATAXE_R {
        DATAXE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transaction Error Interrupt"]
    #[inline(always)]
    pub fn errortranse(&self) -> ERRORTRANSE_R {
        ERRORTRANSE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt"]
    #[inline(always)]
    pub fn nbusybke(&self) -> NBUSYBKE_R {
        NBUSYBKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    pub fn killbk(&self) -> KILLBK_R {
        KILLBK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocon(&self) -> FIFOCON_R {
        FIFOCON_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request"]
    #[inline(always)]
    pub fn epdishdma(&self) -> EPDISHDMA_R {
        EPDISHDMA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset Data Toggle"]
    #[inline(always)]
    pub fn rstdt(&self) -> RSTDT_R {
        RSTDT_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Device Endpoint Mask Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoenpt_deveptimr0_isoenpt](index.html) module"]
pub struct ISOENPT_DEVEPTIMR0_ISOENPT_SPEC;
impl crate::RegisterSpec for ISOENPT_DEVEPTIMR0_ISOENPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isoenpt_deveptimr0_isoenpt::R](R) reader structure"]
impl crate::Readable for ISOENPT_DEVEPTIMR0_ISOENPT_SPEC {
    type Reader = R;
}
