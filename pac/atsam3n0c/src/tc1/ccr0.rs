#[doc = "Register `CCR0` writer"]
pub type W = crate::W<CCR0_SPEC>;
#[doc = "Field `CLKEN` writer - Counter Clock Enable Command"]
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIS` writer - Counter Clock Disable Command"]
pub type CLKDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRG` writer - Software Trigger Command"]
pub type SWTRG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Counter Clock Enable Command"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CCR0_SPEC> {
        CLKEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Counter Clock Disable Command"]
    #[inline(always)]
    #[must_use]
    pub fn clkdis(&mut self) -> CLKDIS_W<CCR0_SPEC> {
        CLKDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Software Trigger Command"]
    #[inline(always)]
    #[must_use]
    pub fn swtrg(&mut self) -> SWTRG_W<CCR0_SPEC> {
        SWTRG_W::new(self, 2)
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
#[doc = "Channel Control Register (channel = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR0_SPEC;
impl crate::RegisterSpec for CCR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ccr0::W`](W) writer structure"]
impl crate::Writable for CCR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
