#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `WDV` reader - Watchdog Counter Value"]
pub type WdvR = crate::FieldReader<u16>;
#[doc = "Field `WDV` writer - Watchdog Counter Value"]
pub type WdvW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WDFIEN` reader - Watchdog Fault Interrupt Enable"]
pub type WdfienR = crate::BitReader;
#[doc = "Field `WDFIEN` writer - Watchdog Fault Interrupt Enable"]
pub type WdfienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDRSTEN` reader - Watchdog Reset Enable"]
pub type WdrstenR = crate::BitReader;
#[doc = "Field `WDRSTEN` writer - Watchdog Reset Enable"]
pub type WdrstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDRPROC` reader - Watchdog Reset Processor"]
pub type WdrprocR = crate::BitReader;
#[doc = "Field `WDRPROC` writer - Watchdog Reset Processor"]
pub type WdrprocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDDIS` reader - Watchdog Disable"]
pub type WddisR = crate::BitReader;
#[doc = "Field `WDDIS` writer - Watchdog Disable"]
pub type WddisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDD` reader - Watchdog Delta Value"]
pub type WddR = crate::FieldReader<u16>;
#[doc = "Field `WDD` writer - Watchdog Delta Value"]
pub type WddW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WDDBGHLT` reader - Watchdog Debug Halt"]
pub type WddbghltR = crate::BitReader;
#[doc = "Field `WDDBGHLT` writer - Watchdog Debug Halt"]
pub type WddbghltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDIDLEHLT` reader - Watchdog Idle Halt"]
pub type WdidlehltR = crate::BitReader;
#[doc = "Field `WDIDLEHLT` writer - Watchdog Idle Halt"]
pub type WdidlehltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Watchdog Counter Value"]
    #[inline(always)]
    pub fn wdv(&self) -> WdvR {
        WdvR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Watchdog Fault Interrupt Enable"]
    #[inline(always)]
    pub fn wdfien(&self) -> WdfienR {
        WdfienR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Watchdog Reset Enable"]
    #[inline(always)]
    pub fn wdrsten(&self) -> WdrstenR {
        WdrstenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Watchdog Reset Processor"]
    #[inline(always)]
    pub fn wdrproc(&self) -> WdrprocR {
        WdrprocR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Watchdog Disable"]
    #[inline(always)]
    pub fn wddis(&self) -> WddisR {
        WddisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Watchdog Delta Value"]
    #[inline(always)]
    pub fn wdd(&self) -> WddR {
        WddR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Watchdog Debug Halt"]
    #[inline(always)]
    pub fn wddbghlt(&self) -> WddbghltR {
        WddbghltR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Watchdog Idle Halt"]
    #[inline(always)]
    pub fn wdidlehlt(&self) -> WdidlehltR {
        WdidlehltR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdv(&mut self) -> WdvW<MrSpec> {
        WdvW::new(self, 0)
    }
    #[doc = "Bit 12 - Watchdog Fault Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdfien(&mut self) -> WdfienW<MrSpec> {
        WdfienW::new(self, 12)
    }
    #[doc = "Bit 13 - Watchdog Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdrsten(&mut self) -> WdrstenW<MrSpec> {
        WdrstenW::new(self, 13)
    }
    #[doc = "Bit 14 - Watchdog Reset Processor"]
    #[inline(always)]
    #[must_use]
    pub fn wdrproc(&mut self) -> WdrprocW<MrSpec> {
        WdrprocW::new(self, 14)
    }
    #[doc = "Bit 15 - Watchdog Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wddis(&mut self) -> WddisW<MrSpec> {
        WddisW::new(self, 15)
    }
    #[doc = "Bits 16:27 - Watchdog Delta Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdd(&mut self) -> WddW<MrSpec> {
        WddW::new(self, 16)
    }
    #[doc = "Bit 28 - Watchdog Debug Halt"]
    #[inline(always)]
    #[must_use]
    pub fn wddbghlt(&mut self) -> WddbghltW<MrSpec> {
        WddbghltW::new(self, 28)
    }
    #[doc = "Bit 29 - Watchdog Idle Halt"]
    #[inline(always)]
    #[must_use]
    pub fn wdidlehlt(&mut self) -> WdidlehltW<MrSpec> {
        WdidlehltW::new(self, 29)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0x3fff_2fff"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0x3fff_2fff;
}
