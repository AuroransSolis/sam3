#[doc = "Register `CCFG_SYSIO` reader"]
pub type R = crate::R<CcfgSysioSpec>;
#[doc = "Register `CCFG_SYSIO` writer"]
pub type W = crate::W<CcfgSysioSpec>;
#[doc = "Field `SYSIO12` reader - PC0 or ERASE Assignment"]
pub type Sysio12R = crate::BitReader;
#[doc = "Field `SYSIO12` writer - PC0 or ERASE Assignment"]
pub type Sysio12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - PC0 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&self) -> Sysio12R {
        Sysio12R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - PC0 or ERASE Assignment"]
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
