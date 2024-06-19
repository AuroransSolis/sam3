#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `TRGEN` reader - Trigger Enable"]
pub type TrgenR = crate::BitReader;
#[doc = "Field `TRGEN` writer - Trigger Enable"]
pub type TrgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSEL` reader - Trigger Selection"]
pub type TrgselR = crate::FieldReader;
#[doc = "Field `TRGSEL` writer - Trigger Selection"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LOWRES` reader - Resolution"]
pub type LowresR = crate::BitReader;
#[doc = "Field `LOWRES` writer - Resolution"]
pub type LowresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` reader - Sleep Mode"]
pub type SleepR = crate::BitReader;
#[doc = "Field `SLEEP` writer - Sleep Mode"]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCAL` reader - Prescaler Rate Selection"]
pub type PrescalR = crate::FieldReader;
#[doc = "Field `PRESCAL` writer - Prescaler Rate Selection"]
pub type PrescalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STARTUP` reader - Start Up Time"]
pub type StartupR = crate::FieldReader;
#[doc = "Field `STARTUP` writer - Start Up Time"]
pub type StartupW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SHTIM` reader - Sample &amp; Hold Time"]
pub type ShtimR = crate::FieldReader;
#[doc = "Field `SHTIM` writer - Sample &amp; Hold Time"]
pub type ShtimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TrgenR {
        TrgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Resolution"]
    #[inline(always)]
    pub fn lowres(&self) -> LowresR {
        LowresR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&self) -> PrescalR {
        PrescalR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - Start Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> StartupR {
        StartupR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - Sample &amp; Hold Time"]
    #[inline(always)]
    pub fn shtim(&self) -> ShtimR {
        ShtimR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgen(&mut self) -> TrgenW<MrSpec> {
        TrgenW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TrgselW<MrSpec> {
        TrgselW::new(self, 1)
    }
    #[doc = "Bit 4 - Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn lowres(&mut self) -> LowresW<MrSpec> {
        LowresW::new(self, 4)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SleepW<MrSpec> {
        SleepW::new(self, 5)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    #[must_use]
    pub fn prescal(&mut self) -> PrescalW<MrSpec> {
        PrescalW::new(self, 8)
    }
    #[doc = "Bits 16:22 - Start Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> StartupW<MrSpec> {
        StartupW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Sample &amp; Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn shtim(&mut self) -> ShtimW<MrSpec> {
        ShtimW::new(self, 24)
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
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0;
}
