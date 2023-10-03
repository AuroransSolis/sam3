#[doc = "Register `DEVEPTIFR0_ISOENPT` writer"]
pub type W = crate::W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC>;
#[doc = "Field `TXINIS` writer - Transmitted IN Data Interrupt Set"]
pub type TXINIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOUTIS` writer - Received OUT Data Interrupt Set"]
pub type RXOUTIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNDERFIS` writer - Underflow Interrupt Set"]
pub type UNDERFIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HBISOINERRIS` writer - High bandwidth isochronous IN Underflow Error Interrupt Set"]
pub type HBISOINERRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HBISOFLUSHIS` writer - High Bandwidth Isochronous IN Flush Interrupt Set"]
pub type HBISOFLUSHIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OVERFIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCERRIS` writer - CRC Error Interrupt Set"]
pub type CRCERRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHORTPACKETS` writer - Short Packet Interrupt Set"]
pub type SHORTPACKETS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Interrupt Set"]
pub type NBUSYBKS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txinis(&mut self) -> TXINIS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC, 0> {
        TXINIS_W::new(self)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutis(&mut self) -> RXOUTIS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC, 1> {
        RXOUTIS_W::new(self)
    }
    #[doc = "Bit 2 - Underflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn underfis(&mut self) -> UNDERFIS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC, 2> {
        UNDERFIS_W::new(self)
    }
    #[doc = "Bit 3 - High bandwidth isochronous IN Underflow Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoinerris(&mut self) -> HBISOINERRIS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC, 3> {
        HBISOINERRIS_W::new(self)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoflushis(&mut self) -> HBISOFLUSHIS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC, 4> {
        HBISOFLUSHIS_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn overfis(&mut self) -> OVERFIS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC, 5> {
        OVERFIS_W::new(self)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn crcerris(&mut self) -> CRCERRIS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC, 6> {
        CRCERRIS_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn shortpackets(&mut self) -> SHORTPACKETS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC, 7> {
        SHORTPACKETS_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybks(&mut self) -> NBUSYBKS_W<ISOENPT_DEVEPTIFR0_ISOENPT_SPEC, 12> {
        NBUSYBKS_W::new(self)
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
