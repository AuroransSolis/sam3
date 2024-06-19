#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `START` writer - Send a START Condition"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` writer - Send a STOP Condition"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSEN` writer - TWI Master Mode Enabled"]
pub type MsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSDIS` writer - TWI Master Mode Disabled"]
pub type MsdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVEN` writer - TWI Slave Mode Enabled"]
pub type SvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVDIS` writer - TWI Slave Mode Disabled"]
pub type SvdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUICK` writer - SMBUS Quick Command"]
pub type QuickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Send a START Condition"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Send a STOP Condition"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<CrSpec> {
        StopW::new(self, 1)
    }
    #[doc = "Bit 2 - TWI Master Mode Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn msen(&mut self) -> MsenW<CrSpec> {
        MsenW::new(self, 2)
    }
    #[doc = "Bit 3 - TWI Master Mode Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn msdis(&mut self) -> MsdisW<CrSpec> {
        MsdisW::new(self, 3)
    }
    #[doc = "Bit 4 - TWI Slave Mode Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sven(&mut self) -> SvenW<CrSpec> {
        SvenW::new(self, 4)
    }
    #[doc = "Bit 5 - TWI Slave Mode Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn svdis(&mut self) -> SvdisW<CrSpec> {
        SvdisW::new(self, 5)
    }
    #[doc = "Bit 6 - SMBUS Quick Command"]
    #[inline(always)]
    #[must_use]
    pub fn quick(&mut self) -> QuickW<CrSpec> {
        QuickW::new(self, 6)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<CrSpec> {
        SwrstW::new(self, 7)
    }
}
#[doc = "Control Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
