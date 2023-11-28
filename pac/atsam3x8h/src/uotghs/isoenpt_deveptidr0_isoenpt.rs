#[doc = "Register `DEVEPTIDR0_ISOENPT` writer"]
pub type W = crate::W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC>;
#[doc = "Field `TXINEC` writer - Transmitted IN Interrupt Clear"]
pub type TXINEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTEC` writer - Received OUT Data Interrupt Clear"]
pub type RXOUTEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFEC` writer - Underflow Interrupt Clear"]
pub type UNDERFEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOINERREC` writer - High Bandwidth Isochronous IN Error Interrupt Clear"]
pub type HBISOINERREC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOFLUSHEC` writer - High Bandwidth Isochronous IN Flush Interrupt Clear"]
pub type HBISOFLUSHEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFEC` writer - Overflow Interrupt Clear"]
pub type OVERFEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERREC` writer - CRC Error Interrupt Clear"]
pub type CRCERREC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETEC` writer - Shortpacket Interrupt Clear"]
pub type SHORTPACKETEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDATEC` writer - MData Interrupt Clear"]
pub type MDATEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAXEC` writer - DataX Interrupt Clear"]
pub type DATAXEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORTRANSEC` writer - Transaction Error Interrupt Clear"]
pub type ERRORTRANSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Interrupt Clear"]
pub type NBUSYBKEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Clear"]
pub type FIFOCONC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISHDMAC` writer - Endpoint Interrupts Disable HDMA Request Clear"]
pub type EPDISHDMAC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinec(&mut self) -> TXINEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        TXINEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutec(&mut self) -> RXOUTEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        RXOUTEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn underfec(&mut self) -> UNDERFEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        UNDERFEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoinerrec(&mut self) -> HBISOINERREC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        HBISOINERREC_W::new(self, 3)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoflushec(&mut self) -> HBISOFLUSHEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        HBISOFLUSHEC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfec(&mut self) -> OVERFEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        OVERFEC_W::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn crcerrec(&mut self) -> CRCERREC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        CRCERREC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Shortpacket Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketec(&mut self) -> SHORTPACKETEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        SHORTPACKETEC_W::new(self, 7)
    }
    #[doc = "Bit 8 - MData Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mdatec(&mut self) -> MDATEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        MDATEC_W::new(self, 8)
    }
    #[doc = "Bit 9 - DataX Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dataxec(&mut self) -> DATAXEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        DATAXEC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Transaction Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn errortransec(&mut self) -> ERRORTRANSEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        ERRORTRANSEC_W::new(self, 10)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkec(&mut self) -> NBUSYBKEC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        NBUSYBKEC_W::new(self, 12)
    }
    #[doc = "Bit 14 - FIFO Control Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoconc(&mut self) -> FIFOCONC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        FIFOCONC_W::new(self, 14)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn epdishdmac(&mut self) -> EPDISHDMAC_W<ISOENPT_DEVEPTIDR0_ISOENPT_SPEC> {
        EPDISHDMAC_W::new(self, 16)
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
#[doc = "Device Endpoint Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoenpt_deveptidr0_isoenpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISOENPT_DEVEPTIDR0_ISOENPT_SPEC;
impl crate::RegisterSpec for ISOENPT_DEVEPTIDR0_ISOENPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoenpt_deveptidr0_isoenpt::W`](W) writer structure"]
impl crate::Writable for ISOENPT_DEVEPTIDR0_ISOENPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
