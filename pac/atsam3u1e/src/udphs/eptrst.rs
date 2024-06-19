#[doc = "Register `EPTRST` writer"]
pub type W = crate::W<EptrstSpec>;
#[doc = "Field `EPT_0` writer - Endpoint 0 Reset"]
pub type Ept0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPT_1` writer - Endpoint 1 Reset"]
pub type Ept1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPT_2` writer - Endpoint 2 Reset"]
pub type Ept2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPT_3` writer - Endpoint 3 Reset"]
pub type Ept3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPT_4` writer - Endpoint 4 Reset"]
pub type Ept4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPT_5` writer - Endpoint 5 Reset"]
pub type Ept5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPT_6` writer - Endpoint 6 Reset"]
pub type Ept6W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Endpoint 0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ept_0(&mut self) -> Ept0W<EptrstSpec> {
        Ept0W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ept_1(&mut self) -> Ept1W<EptrstSpec> {
        Ept1W::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint 2 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ept_2(&mut self) -> Ept2W<EptrstSpec> {
        Ept2W::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint 3 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ept_3(&mut self) -> Ept3W<EptrstSpec> {
        Ept3W::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint 4 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ept_4(&mut self) -> Ept4W<EptrstSpec> {
        Ept4W::new(self, 4)
    }
    #[doc = "Bit 5 - Endpoint 5 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ept_5(&mut self) -> Ept5W<EptrstSpec> {
        Ept5W::new(self, 5)
    }
    #[doc = "Bit 6 - Endpoint 6 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ept_6(&mut self) -> Ept6W<EptrstSpec> {
        Ept6W::new(self, 6)
    }
}
#[doc = "UDPHS Endpoints Reset Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptrst::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EptrstSpec;
impl crate::RegisterSpec for EptrstSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eptrst::W`](W) writer structure"]
impl crate::Writable for EptrstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
