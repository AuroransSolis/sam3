#[doc = "Register `CCFG_SYSIO` reader"]
pub type R = crate::R<CcfgSysioSpec>;
#[doc = "Register `CCFG_SYSIO` writer"]
pub type W = crate::W<CcfgSysioSpec>;
#[doc = "Field `SYSIO4` reader - PB4 or TDI Assignment"]
pub type Sysio4R = crate::BitReader;
#[doc = "Field `SYSIO4` writer - PB4 or TDI Assignment"]
pub type Sysio4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSIO5` reader - PB5 or TDO/TRACESWO Assignment"]
pub type Sysio5R = crate::BitReader;
#[doc = "Field `SYSIO5` writer - PB5 or TDO/TRACESWO Assignment"]
pub type Sysio5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSIO6` reader - PB6 or TMS/SWDIO Assignment"]
pub type Sysio6R = crate::BitReader;
#[doc = "Field `SYSIO6` writer - PB6 or TMS/SWDIO Assignment"]
pub type Sysio6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSIO7` reader - PB7 or TCK/SWCLK Assignment"]
pub type Sysio7R = crate::BitReader;
#[doc = "Field `SYSIO7` writer - PB7 or TCK/SWCLK Assignment"]
pub type Sysio7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSIO10` reader - PB10 or DDM Assignment"]
pub type Sysio10R = crate::BitReader;
#[doc = "Field `SYSIO10` writer - PB10 or DDM Assignment"]
pub type Sysio10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSIO11` reader - PB11 or DDP Assignment"]
pub type Sysio11R = crate::BitReader;
#[doc = "Field `SYSIO11` writer - PB11 or DDP Assignment"]
pub type Sysio11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSIO12` reader - PB12 or ERASE Assignment"]
pub type Sysio12R = crate::BitReader;
#[doc = "Field `SYSIO12` writer - PB12 or ERASE Assignment"]
pub type Sysio12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - PB4 or TDI Assignment"]
    #[inline(always)]
    pub fn sysio4(&self) -> Sysio4R {
        Sysio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PB5 or TDO/TRACESWO Assignment"]
    #[inline(always)]
    pub fn sysio5(&self) -> Sysio5R {
        Sysio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PB6 or TMS/SWDIO Assignment"]
    #[inline(always)]
    pub fn sysio6(&self) -> Sysio6R {
        Sysio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PB7 or TCK/SWCLK Assignment"]
    #[inline(always)]
    pub fn sysio7(&self) -> Sysio7R {
        Sysio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - PB10 or DDM Assignment"]
    #[inline(always)]
    pub fn sysio10(&self) -> Sysio10R {
        Sysio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PB11 or DDP Assignment"]
    #[inline(always)]
    pub fn sysio11(&self) -> Sysio11R {
        Sysio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PB12 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&self) -> Sysio12R {
        Sysio12R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - PB4 or TDI Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn sysio4(&mut self) -> Sysio4W<CcfgSysioSpec> {
        Sysio4W::new(self, 4)
    }
    #[doc = "Bit 5 - PB5 or TDO/TRACESWO Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn sysio5(&mut self) -> Sysio5W<CcfgSysioSpec> {
        Sysio5W::new(self, 5)
    }
    #[doc = "Bit 6 - PB6 or TMS/SWDIO Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn sysio6(&mut self) -> Sysio6W<CcfgSysioSpec> {
        Sysio6W::new(self, 6)
    }
    #[doc = "Bit 7 - PB7 or TCK/SWCLK Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn sysio7(&mut self) -> Sysio7W<CcfgSysioSpec> {
        Sysio7W::new(self, 7)
    }
    #[doc = "Bit 10 - PB10 or DDM Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn sysio10(&mut self) -> Sysio10W<CcfgSysioSpec> {
        Sysio10W::new(self, 10)
    }
    #[doc = "Bit 11 - PB11 or DDP Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn sysio11(&mut self) -> Sysio11W<CcfgSysioSpec> {
        Sysio11W::new(self, 11)
    }
    #[doc = "Bit 12 - PB12 or ERASE Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn sysio12(&mut self) -> Sysio12W<CcfgSysioSpec> {
        Sysio12W::new(self, 12)
    }
}
#[doc = "System I/O Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_sysio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_sysio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcfgSysioSpec;
impl crate::RegisterSpec for CcfgSysioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_sysio::R`](R) reader structure"]
impl crate::Readable for CcfgSysioSpec {}
#[doc = "`write(|w| ..)` method takes [`ccfg_sysio::W`](W) writer structure"]
impl crate::Writable for CcfgSysioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCFG_SYSIO to value 0"]
impl crate::Resettable for CcfgSysioSpec {
    const RESET_VALUE: u32 = 0;
}
