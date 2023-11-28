#[doc = "Register `SFR` writer"]
pub type W = crate::W<SFR_SPEC>;
#[doc = "Field `IDTIS` writer - ID Transition Interrupt Set"]
pub type IDTIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSTIS` writer - VBus Transition Interrupt Set"]
pub type VBUSTIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRPIS` writer - SRP Interrupt Set"]
pub type SRPIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBERRIS` writer - VBus Error Interrupt Set"]
pub type VBERRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCERRIS` writer - B-Connection Error Interrupt Set"]
pub type BCERRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROLEEXIS` writer - Role Exchange Interrupt Set"]
pub type ROLEEXIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPERRIS` writer - HNP Error Interrupt Set"]
pub type HNPERRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOIS` writer - Suspend Time-Out Interrupt Set"]
pub type STOIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSRQS` writer - VBus Request Set"]
pub type VBUSRQS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - ID Transition Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn idtis(&mut self) -> IDTIS_W<SFR_SPEC> {
        IDTIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn vbustis(&mut self) -> VBUSTIS_W<SFR_SPEC> {
        VBUSTIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - SRP Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn srpis(&mut self) -> SRPIS_W<SFR_SPEC> {
        SRPIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - VBus Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn vberris(&mut self) -> VBERRIS_W<SFR_SPEC> {
        VBERRIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn bcerris(&mut self) -> BCERRIS_W<SFR_SPEC> {
        BCERRIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn roleexis(&mut self) -> ROLEEXIS_W<SFR_SPEC> {
        ROLEEXIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - HNP Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn hnperris(&mut self) -> HNPERRIS_W<SFR_SPEC> {
        HNPERRIS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn stois(&mut self) -> STOIS_W<SFR_SPEC> {
        STOIS_W::new(self, 7)
    }
    #[doc = "Bit 9 - VBus Request Set"]
    #[inline(always)]
    #[must_use]
    pub fn vbusrqs(&mut self) -> VBUSRQS_W<SFR_SPEC> {
        VBUSRQS_W::new(self, 9)
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
#[doc = "General Status Set Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFR_SPEC;
impl crate::RegisterSpec for SFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sfr::W`](W) writer structure"]
impl crate::Writable for SFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
