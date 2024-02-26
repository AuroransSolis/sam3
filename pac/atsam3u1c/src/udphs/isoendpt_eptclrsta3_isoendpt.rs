#[doc = "Register `EPTCLRSTA3_ISOENDPT` writer"]
pub type W = crate::W<IsoendptEptclrsta3IsoendptSpec>;
#[doc = "Field `TOGGLESQ` writer - Data Toggle Clear"]
pub type TogglesqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Clear"]
pub type RxrdyTxklW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Clear"]
pub type TxCompltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_FL_ISO` writer - Error Flow Clear"]
pub type ErrFlIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_CRC_NTR` writer - Number of Transaction Error Clear"]
pub type ErrCrcNtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_FLUSH` writer - Bank Flush Error Clear"]
pub type ErrFlushW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 6 - Data Toggle Clear"]
    #[inline(always)]
    #[must_use]
    pub fn togglesq(&mut self) -> TogglesqW<IsoendptEptclrsta3IsoendptSpec> {
        TogglesqW::new(self, 6)
    }
    #[doc = "Bit 9 - Received OUT Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RxrdyTxklW<IsoendptEptclrsta3IsoendptSpec> {
        RxrdyTxklW::new(self, 9)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complt(&mut self) -> TxCompltW<IsoendptEptclrsta3IsoendptSpec> {
        TxCompltW::new(self, 10)
    }
    #[doc = "Bit 12 - Error Flow Clear"]
    #[inline(always)]
    #[must_use]
    pub fn err_fl_iso(&mut self) -> ErrFlIsoW<IsoendptEptclrsta3IsoendptSpec> {
        ErrFlIsoW::new(self, 12)
    }
    #[doc = "Bit 13 - Number of Transaction Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn err_crc_ntr(&mut self) -> ErrCrcNtrW<IsoendptEptclrsta3IsoendptSpec> {
        ErrCrcNtrW::new(self, 13)
    }
    #[doc = "Bit 14 - Bank Flush Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn err_flush(&mut self) -> ErrFlushW<IsoendptEptclrsta3IsoendptSpec> {
        ErrFlushW::new(self, 14)
    }
}
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 3)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptclrsta3_isoendpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoendptEptclrsta3IsoendptSpec;
impl crate::RegisterSpec for IsoendptEptclrsta3IsoendptSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoendpt_eptclrsta3_isoendpt::W`](W) writer structure"]
impl crate::Writable for IsoendptEptclrsta3IsoendptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
