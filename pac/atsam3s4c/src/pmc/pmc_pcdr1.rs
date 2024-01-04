#[doc = "Register `PMC_PCDR1` writer"]
pub type W = crate::W<PMC_PCDR1_SPEC>;
#[doc = "Field `PID32` writer - Peripheral Clock 32 Disable"]
pub type PID32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID33` writer - Peripheral Clock 33 Disable"]
pub type PID33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID34` writer - Peripheral Clock 34 Disable"]
pub type PID34_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Peripheral Clock 32 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid32(&mut self) -> PID32_W<PMC_PCDR1_SPEC> {
        PID32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid33(&mut self) -> PID33_W<PMC_PCDR1_SPEC> {
        PID33_W::new(self, 1)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid34(&mut self) -> PID34_W<PMC_PCDR1_SPEC> {
        PID34_W::new(self, 2)
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
#[doc = "Peripheral Clock Disable Register 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_pcdr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMC_PCDR1_SPEC;
impl crate::RegisterSpec for PMC_PCDR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pmc_pcdr1::W`](W) writer structure"]
impl crate::Writable for PMC_PCDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
