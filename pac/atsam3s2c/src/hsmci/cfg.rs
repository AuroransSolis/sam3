#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `FIFOMODE` reader - HSMCI Internal FIFO control mode"]
pub type FIFOMODE_R = crate::BitReader;
#[doc = "Field `FIFOMODE` writer - HSMCI Internal FIFO control mode"]
pub type FIFOMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERRCTRL` reader - Flow Error flag reset control mode"]
pub type FERRCTRL_R = crate::BitReader;
#[doc = "Field `FERRCTRL` writer - Flow Error flag reset control mode"]
pub type FERRCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSMODE` reader - High Speed Mode"]
pub type HSMODE_R = crate::BitReader;
#[doc = "Field `HSMODE` writer - High Speed Mode"]
pub type HSMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSYNC` reader - Synchronize on the last block"]
pub type LSYNC_R = crate::BitReader;
#[doc = "Field `LSYNC` writer - Synchronize on the last block"]
pub type LSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HSMCI Internal FIFO control mode"]
    #[inline(always)]
    pub fn fifomode(&self) -> FIFOMODE_R {
        FIFOMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Flow Error flag reset control mode"]
    #[inline(always)]
    pub fn ferrctrl(&self) -> FERRCTRL_R {
        FERRCTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - High Speed Mode"]
    #[inline(always)]
    pub fn hsmode(&self) -> HSMODE_R {
        HSMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Synchronize on the last block"]
    #[inline(always)]
    pub fn lsync(&self) -> LSYNC_R {
        LSYNC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSMCI Internal FIFO control mode"]
    #[inline(always)]
    #[must_use]
    pub fn fifomode(&mut self) -> FIFOMODE_W<CFG_SPEC> {
        FIFOMODE_W::new(self, 0)
    }
    #[doc = "Bit 4 - Flow Error flag reset control mode"]
    #[inline(always)]
    #[must_use]
    pub fn ferrctrl(&mut self) -> FERRCTRL_W<CFG_SPEC> {
        FERRCTRL_W::new(self, 4)
    }
    #[doc = "Bit 8 - High Speed Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hsmode(&mut self) -> HSMODE_W<CFG_SPEC> {
        HSMODE_W::new(self, 8)
    }
    #[doc = "Bit 12 - Synchronize on the last block"]
    #[inline(always)]
    #[must_use]
    pub fn lsync(&mut self) -> LSYNC_W<CFG_SPEC> {
        LSYNC_W::new(self, 12)
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
#[doc = "Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
