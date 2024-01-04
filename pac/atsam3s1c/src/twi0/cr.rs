#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `START` writer - Send a START Condition"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` writer - Send a STOP Condition"]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSEN` writer - TWI Master Mode Enabled"]
pub type MSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSDIS` writer - TWI Master Mode Disabled"]
pub type MSDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVEN` writer - TWI Slave Mode Enabled"]
pub type SVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVDIS` writer - TWI Slave Mode Disabled"]
pub type SVDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUICK` writer - SMBUS Quick Command"]
pub type QUICK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Send a START Condition"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CR_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Send a STOP Condition"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR_SPEC> {
        STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - TWI Master Mode Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn msen(&mut self) -> MSEN_W<CR_SPEC> {
        MSEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - TWI Master Mode Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn msdis(&mut self) -> MSDIS_W<CR_SPEC> {
        MSDIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - TWI Slave Mode Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sven(&mut self) -> SVEN_W<CR_SPEC> {
        SVEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TWI Slave Mode Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn svdis(&mut self) -> SVDIS_W<CR_SPEC> {
        SVDIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - SMBUS Quick Command"]
    #[inline(always)]
    #[must_use]
    pub fn quick(&mut self) -> QUICK_W<CR_SPEC> {
        QUICK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CR_SPEC> {
        SWRST_W::new(self, 7)
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
#[doc = "Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
