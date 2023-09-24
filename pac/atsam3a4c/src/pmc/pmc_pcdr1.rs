#[doc = "Register `PMC_PCDR1` writer"]
pub type W = crate::W<PMC_PCDR1_SPEC>;
#[doc = "Field `PID32` writer - Peripheral Clock 32 Disable"]
pub type PID32_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID33` writer - Peripheral Clock 33 Disable"]
pub type PID33_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID34` writer - Peripheral Clock 34 Disable"]
pub type PID34_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID35` writer - Peripheral Clock 35 Disable"]
pub type PID35_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID36` writer - Peripheral Clock 36 Disable"]
pub type PID36_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID37` writer - Peripheral Clock 37 Disable"]
pub type PID37_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID38` writer - Peripheral Clock 38 Disable"]
pub type PID38_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID39` writer - Peripheral Clock 39 Disable"]
pub type PID39_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID40` writer - Peripheral Clock 40 Disable"]
pub type PID40_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID41` writer - Peripheral Clock 41 Disable"]
pub type PID41_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID42` writer - Peripheral Clock 42 Disable"]
pub type PID42_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID43` writer - Peripheral Clock 43 Disable"]
pub type PID43_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID44` writer - Peripheral Clock 44 Disable"]
pub type PID44_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Peripheral Clock 32 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid32(&mut self) -> PID32_W<PMC_PCDR1_SPEC, 0> {
        PID32_W::new(self)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid33(&mut self) -> PID33_W<PMC_PCDR1_SPEC, 1> {
        PID33_W::new(self)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid34(&mut self) -> PID34_W<PMC_PCDR1_SPEC, 2> {
        PID34_W::new(self)
    }
    #[doc = "Bit 3 - Peripheral Clock 35 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid35(&mut self) -> PID35_W<PMC_PCDR1_SPEC, 3> {
        PID35_W::new(self)
    }
    #[doc = "Bit 4 - Peripheral Clock 36 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid36(&mut self) -> PID36_W<PMC_PCDR1_SPEC, 4> {
        PID36_W::new(self)
    }
    #[doc = "Bit 5 - Peripheral Clock 37 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid37(&mut self) -> PID37_W<PMC_PCDR1_SPEC, 5> {
        PID37_W::new(self)
    }
    #[doc = "Bit 6 - Peripheral Clock 38 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid38(&mut self) -> PID38_W<PMC_PCDR1_SPEC, 6> {
        PID38_W::new(self)
    }
    #[doc = "Bit 7 - Peripheral Clock 39 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid39(&mut self) -> PID39_W<PMC_PCDR1_SPEC, 7> {
        PID39_W::new(self)
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid40(&mut self) -> PID40_W<PMC_PCDR1_SPEC, 8> {
        PID40_W::new(self)
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid41(&mut self) -> PID41_W<PMC_PCDR1_SPEC, 9> {
        PID41_W::new(self)
    }
    #[doc = "Bit 10 - Peripheral Clock 42 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid42(&mut self) -> PID42_W<PMC_PCDR1_SPEC, 10> {
        PID42_W::new(self)
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid43(&mut self) -> PID43_W<PMC_PCDR1_SPEC, 11> {
        PID43_W::new(self)
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid44(&mut self) -> PID44_W<PMC_PCDR1_SPEC, 12> {
        PID44_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral Clock Disable Register 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_pcdr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMC_PCDR1_SPEC;
impl crate::RegisterSpec for PMC_PCDR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmc_pcdr1::W`](W) writer structure"]
impl crate::Writable for PMC_PCDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
