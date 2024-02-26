#[doc = "Register `CCR0` writer"]
pub type W = crate::W<Ccr0Spec>;
#[doc = "Field `CLKEN` writer - Counter Clock Enable Command"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIS` writer - Counter Clock Disable Command"]
pub type ClkdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRG` writer - Software Trigger Command"]
pub type SwtrgW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Counter Clock Enable Command"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> ClkenW<Ccr0Spec> {
        ClkenW::new(self, 0)
    }
    #[doc = "Bit 1 - Counter Clock Disable Command"]
    #[inline(always)]
    #[must_use]
    pub fn clkdis(&mut self) -> ClkdisW<Ccr0Spec> {
        ClkdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Software Trigger Command"]
    #[inline(always)]
    #[must_use]
    pub fn swtrg(&mut self) -> SwtrgW<Ccr0Spec> {
        SwtrgW::new(self, 2)
    }
}
#[doc = "Channel Control Register (channel = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr0Spec;
impl crate::RegisterSpec for Ccr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ccr0::W`](W) writer structure"]
impl crate::Writable for Ccr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
