#[doc = "Register `TCR` writer"]
pub type W = crate::W<TCR_SPEC>;
#[doc = "Field `MB0` writer - Transfer Request for Mailbox 0"]
pub type MB0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB1` writer - Transfer Request for Mailbox 1"]
pub type MB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB2` writer - Transfer Request for Mailbox 2"]
pub type MB2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB3` writer - Transfer Request for Mailbox 3"]
pub type MB3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB4` writer - Transfer Request for Mailbox 4"]
pub type MB4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB5` writer - Transfer Request for Mailbox 5"]
pub type MB5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB6` writer - Transfer Request for Mailbox 6"]
pub type MB6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB7` writer - Transfer Request for Mailbox 7"]
pub type MB7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMRST` writer - Timer Reset"]
pub type TIMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transfer Request for Mailbox 0"]
    #[inline(always)]
    #[must_use]
    pub fn mb0(&mut self) -> MB0_W<TCR_SPEC> {
        MB0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Request for Mailbox 1"]
    #[inline(always)]
    #[must_use]
    pub fn mb1(&mut self) -> MB1_W<TCR_SPEC> {
        MB1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer Request for Mailbox 2"]
    #[inline(always)]
    #[must_use]
    pub fn mb2(&mut self) -> MB2_W<TCR_SPEC> {
        MB2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transfer Request for Mailbox 3"]
    #[inline(always)]
    #[must_use]
    pub fn mb3(&mut self) -> MB3_W<TCR_SPEC> {
        MB3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Transfer Request for Mailbox 4"]
    #[inline(always)]
    #[must_use]
    pub fn mb4(&mut self) -> MB4_W<TCR_SPEC> {
        MB4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Transfer Request for Mailbox 5"]
    #[inline(always)]
    #[must_use]
    pub fn mb5(&mut self) -> MB5_W<TCR_SPEC> {
        MB5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer Request for Mailbox 6"]
    #[inline(always)]
    #[must_use]
    pub fn mb6(&mut self) -> MB6_W<TCR_SPEC> {
        MB6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transfer Request for Mailbox 7"]
    #[inline(always)]
    #[must_use]
    pub fn mb7(&mut self) -> MB7_W<TCR_SPEC> {
        MB7_W::new(self, 7)
    }
    #[doc = "Bit 31 - Timer Reset"]
    #[inline(always)]
    #[must_use]
    pub fn timrst(&mut self) -> TIMRST_W<TCR_SPEC> {
        TIMRST_W::new(self, 31)
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
#[doc = "Transfer Command Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
