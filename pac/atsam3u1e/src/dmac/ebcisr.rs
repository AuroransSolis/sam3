#[doc = "Register `EBCISR` reader"]
pub type R = crate::R<EbcisrSpec>;
#[doc = "Field `BTC0` reader - Buffer Transfer Completed \\[3:0\\]"]
pub type Btc0R = crate::BitReader;
#[doc = "Field `BTC1` reader - Buffer Transfer Completed \\[3:0\\]"]
pub type Btc1R = crate::BitReader;
#[doc = "Field `BTC2` reader - Buffer Transfer Completed \\[3:0\\]"]
pub type Btc2R = crate::BitReader;
#[doc = "Field `BTC3` reader - Buffer Transfer Completed \\[3:0\\]"]
pub type Btc3R = crate::BitReader;
#[doc = "Field `CBTC0` reader - Chained Buffer Transfer Completed \\[3:0\\]"]
pub type Cbtc0R = crate::BitReader;
#[doc = "Field `CBTC1` reader - Chained Buffer Transfer Completed \\[3:0\\]"]
pub type Cbtc1R = crate::BitReader;
#[doc = "Field `CBTC2` reader - Chained Buffer Transfer Completed \\[3:0\\]"]
pub type Cbtc2R = crate::BitReader;
#[doc = "Field `CBTC3` reader - Chained Buffer Transfer Completed \\[3:0\\]"]
pub type Cbtc3R = crate::BitReader;
#[doc = "Field `ERR0` reader - Access Error \\[3:0\\]"]
pub type Err0R = crate::BitReader;
#[doc = "Field `ERR1` reader - Access Error \\[3:0\\]"]
pub type Err1R = crate::BitReader;
#[doc = "Field `ERR2` reader - Access Error \\[3:0\\]"]
pub type Err2R = crate::BitReader;
#[doc = "Field `ERR3` reader - Access Error \\[3:0\\]"]
pub type Err3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    pub fn btc0(&self) -> Btc0R {
        Btc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    pub fn btc1(&self) -> Btc1R {
        Btc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    pub fn btc2(&self) -> Btc2R {
        Btc2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    pub fn btc3(&self) -> Btc3R {
        Btc3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Chained Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    pub fn cbtc0(&self) -> Cbtc0R {
        Cbtc0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Chained Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    pub fn cbtc1(&self) -> Cbtc1R {
        Cbtc1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Chained Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    pub fn cbtc2(&self) -> Cbtc2R {
        Cbtc2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Chained Buffer Transfer Completed \\[3:0\\]"]
    #[inline(always)]
    pub fn cbtc3(&self) -> Cbtc3R {
        Cbtc3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Access Error \\[3:0\\]"]
    #[inline(always)]
    pub fn err0(&self) -> Err0R {
        Err0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Access Error \\[3:0\\]"]
    #[inline(always)]
    pub fn err1(&self) -> Err1R {
        Err1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Access Error \\[3:0\\]"]
    #[inline(always)]
    pub fn err2(&self) -> Err2R {
        Err2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Access Error \\[3:0\\]"]
    #[inline(always)]
    pub fn err3(&self) -> Err3R {
        Err3R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ebcisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EbcisrSpec;
impl crate::RegisterSpec for EbcisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ebcisr::R`](R) reader structure"]
impl crate::Readable for EbcisrSpec {}
#[doc = "`reset()` method sets EBCISR to value 0"]
impl crate::Resettable for EbcisrSpec {
    const RESET_VALUE: u32 = 0;
}
