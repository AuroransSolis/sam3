#[doc = "Register `CCFG_SYSIO` reader"]
pub type R = crate::R<CCFG_SYSIO_SPEC>;
#[doc = "Register `CCFG_SYSIO` writer"]
pub type W = crate::W<CCFG_SYSIO_SPEC>;
#[doc = "Field `SYSIO12` reader - PC0 or ERASE Assignment"]
pub type SYSIO12_R = crate::BitReader;
#[doc = "Field `SYSIO12` writer - PC0 or ERASE Assignment"]
pub type SYSIO12_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - PC0 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&self) -> SYSIO12_R {
        SYSIO12_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - PC0 or ERASE Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn sysio12(&mut self) -> SYSIO12_W<CCFG_SYSIO_SPEC> {
        SYSIO12_W::new(self, 12)
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
#[doc = "System I/O Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_sysio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_sysio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCFG_SYSIO_SPEC;
impl crate::RegisterSpec for CCFG_SYSIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_sysio::R`](R) reader structure"]
impl crate::Readable for CCFG_SYSIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccfg_sysio::W`](W) writer structure"]
impl crate::Writable for CCFG_SYSIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCFG_SYSIO to value 0"]
impl crate::Resettable for CCFG_SYSIO_SPEC {
    const RESET_VALUE: u32 = 0;
}
