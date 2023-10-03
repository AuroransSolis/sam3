#[doc = "Register `HSTPIPIER0_INTPIPES` writer"]
pub type W = crate::W<INTPIPES_HSTPIPIER0_INTPIPES_SPEC>;
#[doc = "Field `RXINES` writer - Received IN Data Interrupt Enable"]
pub type RXINES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXOUTES` writer - Transmitted OUT Data Interrupt Enable"]
pub type TXOUTES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNDERFIES` writer - Underflow Interrupt Enable"]
pub type UNDERFIES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERRES` writer - Pipe Error Interrupt Enable"]
pub type PERRES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKEDES` writer - NAKed Interrupt Enable"]
pub type NAKEDES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERFIES` writer - Overflow Interrupt Enable"]
pub type OVERFIES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSTALLDES` writer - Received STALLed Interrupt Enable"]
pub type RXSTALLDES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHORTPACKETIES` writer - Short Packet Interrupt Enable"]
pub type SHORTPACKETIES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Enable"]
pub type NBUSYBKES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDISHDMAS` writer - Pipe Interrupts Disable HDMA Request Enable"]
pub type PDISHDMAS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PFREEZES` writer - Pipe Freeze Enable"]
pub type PFREEZES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTDTS` writer - Reset Data Toggle Enable"]
pub type RSTDTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxines(&mut self) -> RXINES_W<INTPIPES_HSTPIPIER0_INTPIPES_SPEC, 0> {
        RXINES_W::new(self)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txoutes(&mut self) -> TXOUTES_W<INTPIPES_HSTPIPIER0_INTPIPES_SPEC, 1> {
        TXOUTES_W::new(self)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn underfies(&mut self) -> UNDERFIES_W<INTPIPES_HSTPIPIER0_INTPIPES_SPEC, 2> {
        UNDERFIES_W::new(self)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn perres(&mut self) -> PERRES_W<INTPIPES_HSTPIPIER0_INTPIPES_SPEC, 3> {
        PERRES_W::new(self)
    }
    #[doc = "Bit 4 - NAKed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakedes(&mut self) -> NAKEDES_W<INTPIPES_HSTPIPIER0_INTPIPES_SPEC, 4> {
        NAKEDES_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overfies(&mut self) -> OVERFIES_W<INTPIPES_HSTPIPIER0_INTPIPES_SPEC, 5> {
        OVERFIES_W::new(self)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldes(&mut self) -> RXSTALLDES_W<INTPIPES_HSTPIPIER0_INTPIPES_SPEC, 6> {
        RXSTALLDES_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketies(&mut self) -> SHORTPACKETIES_W<INTPIPES_HSTPIPIER0_INTPIPES_SPEC, 7> {
        SHORTPACKETIES_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W<INTPIPES_HSTPIPIER0_INTPIPES_SPEC, 12> {
        NBUSYBKES_W::new(self)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdishdmas(&mut self) -> PDISHDMAS_W<INTPIPES_HSTPIPIER0_INTPIPES_SPEC, 16> {
        PDISHDMAS_W::new(self)
    }
    #[doc = "Bit 17 - Pipe Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfreezes(&mut self) -> PFREEZES_W<INTPIPES_HSTPIPIER0_INTPIPES_SPEC, 17> {
        PFREEZES_W::new(self)
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstdts(&mut self) -> RSTDTS_W<INTPIPES_HSTPIPIER0_INTPIPES_SPEC, 18> {
        RSTDTS_W::new(self)
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
#[doc = "Host Pipe Enable Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpipes_hstpipier0_intpipes::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTPIPES_HSTPIPIER0_INTPIPES_SPEC;
impl crate::RegisterSpec for INTPIPES_HSTPIPIER0_INTPIPES_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intpipes_hstpipier0_intpipes::W`](W) writer structure"]
impl crate::Writable for INTPIPES_HSTPIPIER0_INTPIPES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
