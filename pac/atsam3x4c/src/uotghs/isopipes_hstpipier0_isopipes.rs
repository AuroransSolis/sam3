#[doc = "Register `HSTPIPIER0_ISOPIPES` writer"]
pub type W = crate::W<ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC>;
#[doc = "Field `RXINES` writer - Received IN Data Interrupt Enable"]
pub type RXINES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTES` writer - Transmitted OUT Data Interrupt Enable"]
pub type TXOUTES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIES` writer - Underflow Interrupt Enable"]
pub type UNDERFIES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRES` writer - Pipe Error Interrupt Enable"]
pub type PERRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDES` writer - NAKed Interrupt Enable"]
pub type NAKEDES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIES` writer - Overflow Interrupt Enable"]
pub type OVERFIES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRES` writer - CRC Error Interrupt Enable"]
pub type CRCERRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETIES` writer - Short Packet Interrupt Enable"]
pub type SHORTPACKETIES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Enable"]
pub type NBUSYBKES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDISHDMAS` writer - Pipe Interrupts Disable HDMA Request Enable"]
pub type PDISHDMAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFREEZES` writer - Pipe Freeze Enable"]
pub type PFREEZES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTDTS` writer - Reset Data Toggle Enable"]
pub type RSTDTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxines(&mut self) -> RXINES_W<ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC> {
        RXINES_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txoutes(&mut self) -> TXOUTES_W<ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC> {
        TXOUTES_W::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn underfies(&mut self) -> UNDERFIES_W<ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC> {
        UNDERFIES_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn perres(&mut self) -> PERRES_W<ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC> {
        PERRES_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakedes(&mut self) -> NAKEDES_W<ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC> {
        NAKEDES_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overfies(&mut self) -> OVERFIES_W<ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC> {
        OVERFIES_W::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcerres(&mut self) -> CRCERRES_W<ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC> {
        CRCERRES_W::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketies(&mut self) -> SHORTPACKETIES_W<ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC> {
        SHORTPACKETIES_W::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W<ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC> {
        NBUSYBKES_W::new(self, 12)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdishdmas(&mut self) -> PDISHDMAS_W<ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC> {
        PDISHDMAS_W::new(self, 16)
    }
    #[doc = "Bit 17 - Pipe Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfreezes(&mut self) -> PFREEZES_W<ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC> {
        PFREEZES_W::new(self, 17)
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstdts(&mut self) -> RSTDTS_W<ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC> {
        RSTDTS_W::new(self, 18)
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
#[doc = "Host Pipe Enable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isopipes_hstpipier0_isopipes::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC;
impl crate::RegisterSpec for ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isopipes_hstpipier0_isopipes::W`](W) writer structure"]
impl crate::Writable for ISOPIPES_HSTPIPIER0_ISOPIPES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
