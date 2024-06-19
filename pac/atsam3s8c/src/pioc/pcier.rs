#[doc = "Register `PCIER` writer"]
pub type W = crate::W<PcierSpec>;
#[doc = "Field `DRDY` writer - Parallel Capture Mode Data Ready Interrupt Enable"]
pub type DrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Parallel Capture Mode Overrun Error Interrupt Enable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Reception Transfer Interrupt Enable"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Reception Buffer Full Interrupt Enable"]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Parallel Capture Mode Data Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DrdyW<PcierSpec> {
        DrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Parallel Capture Mode Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OvreW<PcierSpec> {
        OvreW::new(self, 1)
    }
    #[doc = "Bit 2 - End of Reception Transfer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<PcierSpec> {
        EndrxW::new(self, 2)
    }
    #[doc = "Bit 3 - Reception Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RxbuffW<PcierSpec> {
        RxbuffW::new(self, 3)
    }
}
#[doc = "Parallel Capture Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcierSpec;
impl crate::RegisterSpec for PcierSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pcier::W`](W) writer structure"]
impl crate::Writable for PcierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
