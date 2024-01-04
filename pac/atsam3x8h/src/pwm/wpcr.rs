#[doc = "Register `WPCR` writer"]
pub type W = crate::W<WPCR_SPEC>;
#[doc = "Field `WPCMD` writer - Write Protect Command"]
pub type WPCMD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WPRG0` writer - Write Protect Register Group 0"]
pub type WPRG0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG1` writer - Write Protect Register Group 1"]
pub type WPRG1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG2` writer - Write Protect Register Group 2"]
pub type WPRG2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG3` writer - Write Protect Register Group 3"]
pub type WPRG3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG4` writer - Write Protect Register Group 4"]
pub type WPRG4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG5` writer - Write Protect Register Group 5"]
pub type WPRG5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPKEY` writer - Write Protect Key"]
pub type WPKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bits 0:1 - Write Protect Command"]
    #[inline(always)]
    #[must_use]
    pub fn wpcmd(&mut self) -> WPCMD_W<WPCR_SPEC> {
        WPCMD_W::new(self, 0)
    }
    #[doc = "Bit 2 - Write Protect Register Group 0"]
    #[inline(always)]
    #[must_use]
    pub fn wprg0(&mut self) -> WPRG0_W<WPCR_SPEC> {
        WPRG0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write Protect Register Group 1"]
    #[inline(always)]
    #[must_use]
    pub fn wprg1(&mut self) -> WPRG1_W<WPCR_SPEC> {
        WPRG1_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write Protect Register Group 2"]
    #[inline(always)]
    #[must_use]
    pub fn wprg2(&mut self) -> WPRG2_W<WPCR_SPEC> {
        WPRG2_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write Protect Register Group 3"]
    #[inline(always)]
    #[must_use]
    pub fn wprg3(&mut self) -> WPRG3_W<WPCR_SPEC> {
        WPRG3_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write Protect Register Group 4"]
    #[inline(always)]
    #[must_use]
    pub fn wprg4(&mut self) -> WPRG4_W<WPCR_SPEC> {
        WPRG4_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write Protect Register Group 5"]
    #[inline(always)]
    #[must_use]
    pub fn wprg5(&mut self) -> WPRG5_W<WPCR_SPEC> {
        WPRG5_W::new(self, 7)
    }
    #[doc = "Bits 8:31 - Write Protect Key"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WPKEY_W<WPCR_SPEC> {
        WPKEY_W::new(self, 8)
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
#[doc = "PWM Write Protect Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPCR_SPEC;
impl crate::RegisterSpec for WPCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wpcr::W`](W) writer structure"]
impl crate::Writable for WPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
