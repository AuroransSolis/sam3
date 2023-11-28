#[doc = "Register `DEVEPTIER0_ISOENPT` writer"]
pub type W = crate::W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC>;
#[doc = "Field `TXINES` writer - Transmitted IN Data Interrupt Enable"]
pub type TXINES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTES` writer - Received OUT Data Interrupt Enable"]
pub type RXOUTES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFES` writer - Underflow Interrupt Enable"]
pub type UNDERFES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOINERRES` writer - High Bandwidth Isochronous IN Error Interrupt Enable"]
pub type HBISOINERRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOFLUSHES` writer - High Bandwidth Isochronous IN Flush Interrupt Enable"]
pub type HBISOFLUSHES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFES` writer - Overflow Interrupt Enable"]
pub type OVERFES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRES` writer - CRC Error Interrupt Enable"]
pub type CRCERRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETES` writer - Short Packet Interrupt Enable"]
pub type SHORTPACKETES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDATAES` writer - MData Interrupt Enable"]
pub type MDATAES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAXES` writer - DataX Interrupt Enable"]
pub type DATAXES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORTRANSES` writer - Transaction Error Interrupt Enable"]
pub type ERRORTRANSES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Interrupt Enable"]
pub type NBUSYBKES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KILLBKS` writer - Kill IN Bank"]
pub type KILLBKS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONS` writer - FIFO Control"]
pub type FIFOCONS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISHDMAS` writer - Endpoint Interrupts Disable HDMA Request Enable"]
pub type EPDISHDMAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTDTS` writer - Reset Data Toggle Enable"]
pub type RSTDTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLRQS` writer - STALL Request Enable"]
pub type STALLRQS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txines(&mut self) -> TXINES_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        TXINES_W::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutes(&mut self) -> RXOUTES_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        RXOUTES_W::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn underfes(&mut self) -> UNDERFES_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        UNDERFES_W::new(self, 2)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoinerres(&mut self) -> HBISOINERRES_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        HBISOINERRES_W::new(self, 3)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoflushes(&mut self) -> HBISOFLUSHES_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        HBISOFLUSHES_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overfes(&mut self) -> OVERFES_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        OVERFES_W::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcerres(&mut self) -> CRCERRES_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        CRCERRES_W::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketes(&mut self) -> SHORTPACKETES_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        SHORTPACKETES_W::new(self, 7)
    }
    #[doc = "Bit 8 - MData Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdataes(&mut self) -> MDATAES_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        MDATAES_W::new(self, 8)
    }
    #[doc = "Bit 9 - DataX Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dataxes(&mut self) -> DATAXES_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        DATAXES_W::new(self, 9)
    }
    #[doc = "Bit 10 - Transaction Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn errortranses(&mut self) -> ERRORTRANSES_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        ERRORTRANSES_W::new(self, 10)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        NBUSYBKES_W::new(self, 12)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    #[must_use]
    pub fn killbks(&mut self) -> KILLBKS_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        KILLBKS_W::new(self, 13)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    #[must_use]
    pub fn fifocons(&mut self) -> FIFOCONS_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        FIFOCONS_W::new(self, 14)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epdishdmas(&mut self) -> EPDISHDMAS_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        EPDISHDMAS_W::new(self, 16)
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstdts(&mut self) -> RSTDTS_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        RSTDTS_W::new(self, 18)
    }
    #[doc = "Bit 19 - STALL Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stallrqs(&mut self) -> STALLRQS_W<ISOENPT_DEVEPTIER0_ISOENPT_SPEC> {
        STALLRQS_W::new(self, 19)
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
#[doc = "Device Endpoint Enable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoenpt_deveptier0_isoenpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISOENPT_DEVEPTIER0_ISOENPT_SPEC;
impl crate::RegisterSpec for ISOENPT_DEVEPTIER0_ISOENPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoenpt_deveptier0_isoenpt::W`](W) writer structure"]
impl crate::Writable for ISOENPT_DEVEPTIER0_ISOENPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
