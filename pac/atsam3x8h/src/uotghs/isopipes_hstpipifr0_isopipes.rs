#[doc = "Register `HSTPIPIFR0_ISOPIPES` writer"]
pub type W = crate::W<ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC>;
#[doc = "Field `RXINIS` writer - Received IN Data Interrupt Set"]
pub type RXINIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTIS` writer - Transmitted OUT Data Interrupt Set"]
pub type TXOUTIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIS` writer - Underflow Interrupt Set"]
pub type UNDERFIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRIS` writer - Pipe Error Interrupt Set"]
pub type PERRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDIS` writer - NAKed Interrupt Set"]
pub type NAKEDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OVERFIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRIS` writer - CRC Error Interrupt Set"]
pub type CRCERRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIS` writer - Short Packet Interrupt Set"]
pub type SHORTPACKETIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Set"]
pub type NBUSYBKS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxinis(&mut self) -> RXINIS_W<ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC> {
        RXINIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txoutis(&mut self) -> TXOUTIS_W<ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC> {
        TXOUTIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn underfis(&mut self) -> UNDERFIS_W<ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC> {
        UNDERFIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn perris(&mut self) -> PERRIS_W<ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC> {
        PERRIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakedis(&mut self) -> NAKEDIS_W<ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC> {
        NAKEDIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn overfis(&mut self) -> OVERFIS_W<ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC> {
        OVERFIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn crcerris(&mut self) -> CRCERRIS_W<ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC> {
        CRCERRIS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketis(&mut self) -> SHORTPACKETIS_W<ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC> {
        SHORTPACKETIS_W::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybks(&mut self) -> NBUSYBKS_W<ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC> {
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
#[doc = "Host Pipe Set Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isopipes_hstpipifr0_isopipes::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC;
impl crate::RegisterSpec for ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isopipes_hstpipifr0_isopipes::W`](W) writer structure"]
impl crate::Writable for ISOPIPES_HSTPIPIFR0_ISOPIPES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
