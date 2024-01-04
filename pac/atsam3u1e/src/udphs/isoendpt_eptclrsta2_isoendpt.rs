#[doc = "Register `EPTCLRSTA2_ISOENDPT` writer"]
pub type W = crate::W<ISOENDPT_EPTCLRSTA2_ISOENDPT_SPEC>;
#[doc = "Field `TOGGLESQ` writer - Data Toggle Clear"]
pub type TOGGLESQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY_TXKL` writer - Received OUT Data Clear"]
pub type RXRDY_TXKL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_COMPLT` writer - Transmitted IN Data Complete Clear"]
pub type TX_COMPLT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_FL_ISO` writer - Error Flow Clear"]
pub type ERR_FL_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_CRC_NTR` writer - Number of Transaction Error Clear"]
pub type ERR_CRC_NTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_FLUSH` writer - Bank Flush Error Clear"]
pub type ERR_FLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 6 - Data Toggle Clear"]
    #[inline(always)]
    #[must_use]
    pub fn togglesq(&mut self) -> TOGGLESQ_W<ISOENDPT_EPTCLRSTA2_ISOENDPT_SPEC> {
        TOGGLESQ_W::new(self, 6)
    }
    #[doc = "Bit 9 - Received OUT Data Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy_txkl(&mut self) -> RXRDY_TXKL_W<ISOENDPT_EPTCLRSTA2_ISOENDPT_SPEC> {
        RXRDY_TXKL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tx_complt(&mut self) -> TX_COMPLT_W<ISOENDPT_EPTCLRSTA2_ISOENDPT_SPEC> {
        TX_COMPLT_W::new(self, 10)
    }
    #[doc = "Bit 12 - Error Flow Clear"]
    #[inline(always)]
    #[must_use]
    pub fn err_fl_iso(&mut self) -> ERR_FL_ISO_W<ISOENDPT_EPTCLRSTA2_ISOENDPT_SPEC> {
        ERR_FL_ISO_W::new(self, 12)
    }
    #[doc = "Bit 13 - Number of Transaction Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn err_crc_ntr(&mut self) -> ERR_CRC_NTR_W<ISOENDPT_EPTCLRSTA2_ISOENDPT_SPEC> {
        ERR_CRC_NTR_W::new(self, 13)
    }
    #[doc = "Bit 14 - Bank Flush Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn err_flush(&mut self) -> ERR_FLUSH_W<ISOENDPT_EPTCLRSTA2_ISOENDPT_SPEC> {
        ERR_FLUSH_W::new(self, 14)
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
#[doc = "UDPHS Endpoint Clear Status Register (endpoint = 2)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoendpt_eptclrsta2_isoendpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISOENDPT_EPTCLRSTA2_ISOENDPT_SPEC;
impl crate::RegisterSpec for ISOENDPT_EPTCLRSTA2_ISOENDPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoendpt_eptclrsta2_isoendpt::W`](W) writer structure"]
impl crate::Writable for ISOENDPT_EPTCLRSTA2_ISOENDPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
