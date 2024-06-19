#[doc = "Register `TCR` writer"]
pub type W = crate::W<TcrSpec>;
#[doc = "Field `MB0` writer - Transfer Request for Mailbox 0"]
pub type Mb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB1` writer - Transfer Request for Mailbox 1"]
pub type Mb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB2` writer - Transfer Request for Mailbox 2"]
pub type Mb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB3` writer - Transfer Request for Mailbox 3"]
pub type Mb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB4` writer - Transfer Request for Mailbox 4"]
pub type Mb4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB5` writer - Transfer Request for Mailbox 5"]
pub type Mb5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB6` writer - Transfer Request for Mailbox 6"]
pub type Mb6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB7` writer - Transfer Request for Mailbox 7"]
pub type Mb7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMRST` writer - Timer Reset"]
pub type TimrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transfer Request for Mailbox 0"]
    #[inline(always)]
    #[must_use]
    pub fn mb0(&mut self) -> Mb0W<TcrSpec> {
        Mb0W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Request for Mailbox 1"]
    #[inline(always)]
    #[must_use]
    pub fn mb1(&mut self) -> Mb1W<TcrSpec> {
        Mb1W::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer Request for Mailbox 2"]
    #[inline(always)]
    #[must_use]
    pub fn mb2(&mut self) -> Mb2W<TcrSpec> {
        Mb2W::new(self, 2)
    }
    #[doc = "Bit 3 - Transfer Request for Mailbox 3"]
    #[inline(always)]
    #[must_use]
    pub fn mb3(&mut self) -> Mb3W<TcrSpec> {
        Mb3W::new(self, 3)
    }
    #[doc = "Bit 4 - Transfer Request for Mailbox 4"]
    #[inline(always)]
    #[must_use]
    pub fn mb4(&mut self) -> Mb4W<TcrSpec> {
        Mb4W::new(self, 4)
    }
    #[doc = "Bit 5 - Transfer Request for Mailbox 5"]
    #[inline(always)]
    #[must_use]
    pub fn mb5(&mut self) -> Mb5W<TcrSpec> {
        Mb5W::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer Request for Mailbox 6"]
    #[inline(always)]
    #[must_use]
    pub fn mb6(&mut self) -> Mb6W<TcrSpec> {
        Mb6W::new(self, 6)
    }
    #[doc = "Bit 7 - Transfer Request for Mailbox 7"]
    #[inline(always)]
    #[must_use]
    pub fn mb7(&mut self) -> Mb7W<TcrSpec> {
        Mb7W::new(self, 7)
    }
    #[doc = "Bit 31 - Timer Reset"]
    #[inline(always)]
    #[must_use]
    pub fn timrst(&mut self) -> TimrstW<TcrSpec> {
        TimrstW::new(self, 31)
    }
}
#[doc = "Transfer Command Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrSpec;
impl crate::RegisterSpec for TcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
