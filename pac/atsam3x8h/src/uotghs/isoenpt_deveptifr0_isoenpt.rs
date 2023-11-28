#[doc = "Register `DEVEPTIFR0_ISOENPT` writer"]
pub type W = crate::W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC>;
#[doc = "Field `TXINIS` writer - Transmitted IN Data Interrupt Set"]
pub type TXINIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTIS` writer - Received OUT Data Interrupt Set"]
pub type RXOUTIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIS` writer - Underflow Interrupt Set"]
pub type UNDERFIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOINERRIS` writer - High bandwidth isochronous IN Underflow Error Interrupt Set"]
pub type HBISOINERRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOFLUSHIS` writer - High Bandwidth Isochronous IN Flush Interrupt Set"]
pub type HBISOFLUSHIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OVERFIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRIS` writer - CRC Error Interrupt Set"]
pub type CRCERRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETS` writer - Short Packet Interrupt Set"]
pub type SHORTPACKETS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Interrupt Set"]
pub type NBUSYBKS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txinis(&mut self) -> TXINIS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC> {
        TXINIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutis(&mut self) -> RXOUTIS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC> {
        RXOUTIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn underfis(&mut self) -> UNDERFIS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC> {
        UNDERFIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - High bandwidth isochronous IN Underflow Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoinerris(&mut self) -> HBISOINERRIS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC> {
        HBISOINERRIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoflushis(&mut self) -> HBISOFLUSHIS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC> {
        HBISOFLUSHIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn overfis(&mut self) -> OVERFIS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC> {
        OVERFIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn crcerris(&mut self) -> CRCERRIS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC> {
        CRCERRIS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn shortpackets(&mut self) -> SHORTPACKETS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC> {
        SHORTPACKETS_W::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybks(&mut self) -> NBUSYBKS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC> {
        NBUSYBKS_W::new(self, 12)
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
#[doc = "Device Endpoint Set Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoenpt_deveptifr0_isoenpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISOENPT_DEVEPTIFR0_ISOENPT_SPEC;
impl crate::RegisterSpec for ISOENPT_DEVEPTIFR0_ISOENPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoenpt_deveptifr0_isoenpt::W`](W) writer structure"]
impl crate::Writable for ISOENPT_DEVEPTIFR0_ISOENPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
