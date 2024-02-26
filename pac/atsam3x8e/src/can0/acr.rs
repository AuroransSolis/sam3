#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Field `MB0` writer - Abort Request for Mailbox 0"]
pub type Mb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB1` writer - Abort Request for Mailbox 1"]
pub type Mb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB2` writer - Abort Request for Mailbox 2"]
pub type Mb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB3` writer - Abort Request for Mailbox 3"]
pub type Mb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB4` writer - Abort Request for Mailbox 4"]
pub type Mb4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB5` writer - Abort Request for Mailbox 5"]
pub type Mb5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB6` writer - Abort Request for Mailbox 6"]
pub type Mb6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB7` writer - Abort Request for Mailbox 7"]
pub type Mb7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Abort Request for Mailbox 0"]
    #[inline(always)]
    #[must_use]
    pub fn mb0(&mut self) -> Mb0W<AcrSpec> {
        Mb0W::new(self, 0)
    }
    #[doc = "Bit 1 - Abort Request for Mailbox 1"]
    #[inline(always)]
    #[must_use]
    pub fn mb1(&mut self) -> Mb1W<AcrSpec> {
        Mb1W::new(self, 1)
    }
    #[doc = "Bit 2 - Abort Request for Mailbox 2"]
    #[inline(always)]
    #[must_use]
    pub fn mb2(&mut self) -> Mb2W<AcrSpec> {
        Mb2W::new(self, 2)
    }
    #[doc = "Bit 3 - Abort Request for Mailbox 3"]
    #[inline(always)]
    #[must_use]
    pub fn mb3(&mut self) -> Mb3W<AcrSpec> {
        Mb3W::new(self, 3)
    }
    #[doc = "Bit 4 - Abort Request for Mailbox 4"]
    #[inline(always)]
    #[must_use]
    pub fn mb4(&mut self) -> Mb4W<AcrSpec> {
        Mb4W::new(self, 4)
    }
    #[doc = "Bit 5 - Abort Request for Mailbox 5"]
    #[inline(always)]
    #[must_use]
    pub fn mb5(&mut self) -> Mb5W<AcrSpec> {
        Mb5W::new(self, 5)
    }
    #[doc = "Bit 6 - Abort Request for Mailbox 6"]
    #[inline(always)]
    #[must_use]
    pub fn mb6(&mut self) -> Mb6W<AcrSpec> {
        Mb6W::new(self, 6)
    }
    #[doc = "Bit 7 - Abort Request for Mailbox 7"]
    #[inline(always)]
    #[must_use]
    pub fn mb7(&mut self) -> Mb7W<AcrSpec> {
        Mb7W::new(self, 7)
    }
}
#[doc = "Abort Command Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcrSpec;
impl crate::RegisterSpec for AcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for AcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
