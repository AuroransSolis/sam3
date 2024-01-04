#[doc = "Register `HSTPIPIDR0_ISOPIPES` writer"]
pub type W = crate::W<ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC>;
#[doc = "Field `RXINEC` writer - Received IN Data Interrupt Disable"]
pub type RXINEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTEC` writer - Transmitted OUT Data Interrupt Disable"]
pub type TXOUTEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIEC` writer - Underflow Interrupt Disable"]
pub type UNDERFIEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERREC` writer - Pipe Error Interrupt Disable"]
pub type PERREC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDEC` writer - NAKed Interrupt Disable"]
pub type NAKEDEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIEC` writer - Overflow Interrupt Disable"]
pub type OVERFIEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERREC` writer - CRC Error Interrupt Disable"]
pub type CRCERREC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIEC` writer - Short Packet Interrupt Disable"]
pub type SHORTPACKETIEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Disable"]
pub type NBUSYBKEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Disable"]
pub type FIFOCONC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDISHDMAC` writer - Pipe Interrupts Disable HDMA Request Disable"]
pub type PDISHDMAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFREEZEC` writer - Pipe Freeze Disable"]
pub type PFREEZEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxinec(&mut self) -> RXINEC_W<ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC> {
        RXINEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txoutec(&mut self) -> TXOUTEC_W<ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC> {
        TXOUTEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn underfiec(&mut self) -> UNDERFIEC_W<ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC> {
        UNDERFIEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn perrec(&mut self) -> PERREC_W<ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC> {
        PERREC_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nakedec(&mut self) -> NAKEDEC_W<ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC> {
        NAKEDEC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn overfiec(&mut self) -> OVERFIEC_W<ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC> {
        OVERFIEC_W::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn crcerrec(&mut self) -> CRCERREC_W<ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC> {
        CRCERREC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketiec(&mut self) -> SHORTPACKETIEC_W<ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC> {
        SHORTPACKETIEC_W::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkec(&mut self) -> NBUSYBKEC_W<ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC> {
        NBUSYBKEC_W::new(self, 12)
    }
    #[doc = "Bit 14 - FIFO Control Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoconc(&mut self) -> FIFOCONC_W<ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC> {
        FIFOCONC_W::new(self, 14)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pdishdmac(&mut self) -> PDISHDMAC_W<ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC> {
        PDISHDMAC_W::new(self, 16)
    }
    #[doc = "Bit 17 - Pipe Freeze Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pfreezec(&mut self) -> PFREEZEC_W<ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC> {
        PFREEZEC_W::new(self, 17)
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
#[doc = "Host Pipe Disable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isopipes_hstpipidr0_isopipes::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC;
impl crate::RegisterSpec for ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isopipes_hstpipidr0_isopipes::W`](W) writer structure"]
impl crate::Writable for ISOPIPES_HSTPIPIDR0_ISOPIPES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
