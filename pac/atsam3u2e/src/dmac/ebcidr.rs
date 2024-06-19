#[doc = "Register `EBCIDR` writer"]
pub type W = crate::W<EbcidrSpec>;
#[doc = "Field `BTC0` writer - Buffer Transfer Completed \\[3:0\\]"]
pub type Btc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTC1` writer - Buffer Transfer Completed \\[3:0\\]"]
pub type Btc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTC2` writer - Buffer Transfer Completed \\[3:0\\]"]
pub type Btc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTC3` writer - Buffer Transfer Completed \\[3:0\\]"]
pub type Btc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTC0` writer - Chained Buffer Transfer Completed \\[3:0\\]"]
pub type Cbtc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTC1` writer - Chained Buffer Transfer Completed \\[3:0\\]"]
pub type Cbtc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTC2` writer - Chained Buffer Transfer Completed \\[3:0\\]"]
pub type Cbtc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTC3` writer - Chained Buffer Transfer Completed \\[3:0\\]"]
pub type Cbtc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR0` writer - Access Error \\[3:0\\]"]
pub type Err0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR1` writer - Access Error \\[3:0\\]"]
pub type Err1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR2` writer - Access Error \\[3:0\\]"]
pub type Err2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR3` writer - Access Error \\[3:0\\]"]
pub type Err3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn btc0(&mut self) -> Btc0W<EbcidrSpec> {
        Btc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn btc1(&mut self) -> Btc1W<EbcidrSpec> {
        Btc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn btc2(&mut self) -> Btc2W<EbcidrSpec> {
        Btc2W::new(self, 2)
    }
    #[doc = "Bit 3 - Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn btc3(&mut self) -> Btc3W<EbcidrSpec> {
        Btc3W::new(self, 3)
    }
    #[doc = "Bit 8 - Chained Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cbtc0(&mut self) -> Cbtc0W<EbcidrSpec> {
        Cbtc0W::new(self, 8)
    }
    #[doc = "Bit 9 - Chained Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cbtc1(&mut self) -> Cbtc1W<EbcidrSpec> {
        Cbtc1W::new(self, 9)
    }
    #[doc = "Bit 10 - Chained Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cbtc2(&mut self) -> Cbtc2W<EbcidrSpec> {
        Cbtc2W::new(self, 10)
    }
    #[doc = "Bit 11 - Chained Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cbtc3(&mut self) -> Cbtc3W<EbcidrSpec> {
        Cbtc3W::new(self, 11)
    }
    #[doc = "Bit 16 - Access Error \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn err0(&mut self) -> Err0W<EbcidrSpec> {
        Err0W::new(self, 16)
    }
    #[doc = "Bit 17 - Access Error \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn err1(&mut self) -> Err1W<EbcidrSpec> {
        Err1W::new(self, 17)
    }
    #[doc = "Bit 18 - Access Error \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn err2(&mut self) -> Err2W<EbcidrSpec> {
        Err2W::new(self, 18)
    }
    #[doc = "Bit 19 - Access Error \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn err3(&mut self) -> Err3W<EbcidrSpec> {
        Err3W::new(self, 19)
    }
}
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register.\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ebcidr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EbcidrSpec;
impl crate::RegisterSpec for EbcidrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ebcidr::W`](W) writer structure"]
impl crate::Writable for EbcidrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
