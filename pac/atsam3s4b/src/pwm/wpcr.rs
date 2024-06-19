#[doc = "Register `WPCR` writer"]
pub type W = crate::W<WpcrSpec>;
#[doc = "Field `WPCMD` writer - Write Protect Command"]
pub type WpcmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WPRG0` writer - Write Protect Register Group 0"]
pub type Wprg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG1` writer - Write Protect Register Group 1"]
pub type Wprg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG2` writer - Write Protect Register Group 2"]
pub type Wprg2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG3` writer - Write Protect Register Group 3"]
pub type Wprg3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG4` writer - Write Protect Register Group 4"]
pub type Wprg4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRG5` writer - Write Protect Register Group 5"]
pub type Wprg5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPKEY` writer - Write Protect Key"]
pub type WpkeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bits 0:1 - Write Protect Command"]
    #[inline(always)]
    #[must_use]
    pub fn wpcmd(&mut self) -> WpcmdW<WpcrSpec> {
        WpcmdW::new(self, 0)
    }
    #[doc = "Bit 2 - Write Protect Register Group 0"]
    #[inline(always)]
    #[must_use]
    pub fn wprg0(&mut self) -> Wprg0W<WpcrSpec> {
        Wprg0W::new(self, 2)
    }
    #[doc = "Bit 3 - Write Protect Register Group 1"]
    #[inline(always)]
    #[must_use]
    pub fn wprg1(&mut self) -> Wprg1W<WpcrSpec> {
        Wprg1W::new(self, 3)
    }
    #[doc = "Bit 4 - Write Protect Register Group 2"]
    #[inline(always)]
    #[must_use]
    pub fn wprg2(&mut self) -> Wprg2W<WpcrSpec> {
        Wprg2W::new(self, 4)
    }
    #[doc = "Bit 5 - Write Protect Register Group 3"]
    #[inline(always)]
    #[must_use]
    pub fn wprg3(&mut self) -> Wprg3W<WpcrSpec> {
        Wprg3W::new(self, 5)
    }
    #[doc = "Bit 6 - Write Protect Register Group 4"]
    #[inline(always)]
    #[must_use]
    pub fn wprg4(&mut self) -> Wprg4W<WpcrSpec> {
        Wprg4W::new(self, 6)
    }
    #[doc = "Bit 7 - Write Protect Register Group 5"]
    #[inline(always)]
    #[must_use]
    pub fn wprg5(&mut self) -> Wprg5W<WpcrSpec> {
        Wprg5W::new(self, 7)
    }
    #[doc = "Bits 8:31 - Write Protect Key"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WpkeyW<WpcrSpec> {
        WpkeyW::new(self, 8)
    }
}
#[doc = "PWM Write Protect Control Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpcrSpec;
impl crate::RegisterSpec for WpcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wpcr::W`](W) writer structure"]
impl crate::Writable for WpcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
