#[doc = "Register `SFR` writer"]
pub struct W(crate::W<SFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDTIS` writer - ID Transition Interrupt Set"]
pub type IDTIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, bool, O>;
#[doc = "Field `VBUSTIS` writer - VBus Transition Interrupt Set"]
pub type VBUSTIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, bool, O>;
#[doc = "Field `SRPIS` writer - SRP Interrupt Set"]
pub type SRPIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, bool, O>;
#[doc = "Field `VBERRIS` writer - VBus Error Interrupt Set"]
pub type VBERRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, bool, O>;
#[doc = "Field `BCERRIS` writer - B-Connection Error Interrupt Set"]
pub type BCERRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, bool, O>;
#[doc = "Field `ROLEEXIS` writer - Role Exchange Interrupt Set"]
pub type ROLEEXIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, bool, O>;
#[doc = "Field `HNPERRIS` writer - HNP Error Interrupt Set"]
pub type HNPERRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, bool, O>;
#[doc = "Field `STOIS` writer - Suspend Time-Out Interrupt Set"]
pub type STOIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, bool, O>;
#[doc = "Field `VBUSRQS` writer - VBus Request Set"]
pub type VBUSRQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - ID Transition Interrupt Set"]
    #[inline(always)]
    pub fn idtis(&mut self) -> IDTIS_W<0> {
        IDTIS_W::new(self)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Set"]
    #[inline(always)]
    pub fn vbustis(&mut self) -> VBUSTIS_W<1> {
        VBUSTIS_W::new(self)
    }
    #[doc = "Bit 2 - SRP Interrupt Set"]
    #[inline(always)]
    pub fn srpis(&mut self) -> SRPIS_W<2> {
        SRPIS_W::new(self)
    }
    #[doc = "Bit 3 - VBus Error Interrupt Set"]
    #[inline(always)]
    pub fn vberris(&mut self) -> VBERRIS_W<3> {
        VBERRIS_W::new(self)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Set"]
    #[inline(always)]
    pub fn bcerris(&mut self) -> BCERRIS_W<4> {
        BCERRIS_W::new(self)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Set"]
    #[inline(always)]
    pub fn roleexis(&mut self) -> ROLEEXIS_W<5> {
        ROLEEXIS_W::new(self)
    }
    #[doc = "Bit 6 - HNP Error Interrupt Set"]
    #[inline(always)]
    pub fn hnperris(&mut self) -> HNPERRIS_W<6> {
        HNPERRIS_W::new(self)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Set"]
    #[inline(always)]
    pub fn stois(&mut self) -> STOIS_W<7> {
        STOIS_W::new(self)
    }
    #[doc = "Bit 9 - VBus Request Set"]
    #[inline(always)]
    pub fn vbusrqs(&mut self) -> VBUSRQS_W<9> {
        VBUSRQS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Status Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfr](index.html) module"]
pub struct SFR_SPEC;
impl crate::RegisterSpec for SFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sfr::W](W) writer structure"]
impl crate::Writable for SFR_SPEC {
    type Writer = W;
}
