#[doc = "Register `HSTPIPIER3` writer"]
pub type W = crate::W<HSTPIPIER3_SPEC>;
#[doc = "Field `RXINES` writer - Received IN Data Interrupt Enable"]
pub type RXINES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUTES` writer - Transmitted OUT Data Interrupt Enable"]
pub type TXOUTES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTPES` writer - Transmitted SETUP Interrupt Enable"]
pub type TXSTPES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRES` writer - Pipe Error Interrupt Enable"]
pub type PERRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKEDES` writer - NAKed Interrupt Enable"]
pub type NAKEDES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIES` writer - Overflow Interrupt Enable"]
pub type OVERFIES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALLDES` writer - Received STALLed Interrupt Enable"]
pub type RXSTALLDES_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn rxines(&mut self) -> RXINES_W<HSTPIPIER3_SPEC> {
        RXINES_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txoutes(&mut self) -> TXOUTES_W<HSTPIPIER3_SPEC> {
        TXOUTES_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txstpes(&mut self) -> TXSTPES_W<HSTPIPIER3_SPEC> {
        TXSTPES_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn perres(&mut self) -> PERRES_W<HSTPIPIER3_SPEC> {
        PERRES_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakedes(&mut self) -> NAKEDES_W<HSTPIPIER3_SPEC> {
        NAKEDES_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overfies(&mut self) -> OVERFIES_W<HSTPIPIER3_SPEC> {
        OVERFIES_W::new(self, 5)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldes(&mut self) -> RXSTALLDES_W<HSTPIPIER3_SPEC> {
        RXSTALLDES_W::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketies(&mut self) -> SHORTPACKETIES_W<HSTPIPIER3_SPEC> {
        SHORTPACKETIES_W::new(self, 7)
    }
    #[doc = "Bit 12 - Number of Busy Banks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W<HSTPIPIER3_SPEC> {
        NBUSYBKES_W::new(self, 12)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdishdmas(&mut self) -> PDISHDMAS_W<HSTPIPIER3_SPEC> {
        PDISHDMAS_W::new(self, 16)
    }
    #[doc = "Bit 17 - Pipe Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfreezes(&mut self) -> PFREEZES_W<HSTPIPIER3_SPEC> {
        PFREEZES_W::new(self, 17)
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstdts(&mut self) -> RSTDTS_W<HSTPIPIER3_SPEC> {
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
#[doc = "Host Pipe Enable Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTPIPIER3_SPEC;
impl crate::RegisterSpec for HSTPIPIER3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstpipier3::W`](W) writer structure"]
impl crate::Writable for HSTPIPIER3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
