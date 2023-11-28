#[doc = "Register `EBCIER` writer"]
pub type W = crate::W<EBCIER_SPEC>;
#[doc = "Field `BTC0` writer - Buffer Transfer Completed \\[5:0\\]"]
pub type BTC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTC1` writer - Buffer Transfer Completed \\[5:0\\]"]
pub type BTC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTC2` writer - Buffer Transfer Completed \\[5:0\\]"]
pub type BTC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTC3` writer - Buffer Transfer Completed \\[5:0\\]"]
pub type BTC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTC4` writer - Buffer Transfer Completed \\[5:0\\]"]
pub type BTC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTC5` writer - Buffer Transfer Completed \\[5:0\\]"]
pub type BTC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTC0` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub type CBTC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTC1` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub type CBTC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTC2` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub type CBTC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTC3` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub type CBTC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTC4` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub type CBTC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTC5` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub type CBTC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR0` writer - Access Error \\[5:0\\]"]
pub type ERR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR1` writer - Access Error \\[5:0\\]"]
pub type ERR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR2` writer - Access Error \\[5:0\\]"]
pub type ERR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR3` writer - Access Error \\[5:0\\]"]
pub type ERR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR4` writer - Access Error \\[5:0\\]"]
pub type ERR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR5` writer - Access Error \\[5:0\\]"]
pub type ERR5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn btc0(&mut self) -> BTC0_W<EBCIER_SPEC> {
        BTC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn btc1(&mut self) -> BTC1_W<EBCIER_SPEC> {
        BTC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn btc2(&mut self) -> BTC2_W<EBCIER_SPEC> {
        BTC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn btc3(&mut self) -> BTC3_W<EBCIER_SPEC> {
        BTC3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn btc4(&mut self) -> BTC4_W<EBCIER_SPEC> {
        BTC4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn btc5(&mut self) -> BTC5_W<EBCIER_SPEC> {
        BTC5_W::new(self, 5)
    }
    #[doc = "Bit 8 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cbtc0(&mut self) -> CBTC0_W<EBCIER_SPEC> {
        CBTC0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cbtc1(&mut self) -> CBTC1_W<EBCIER_SPEC> {
        CBTC1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cbtc2(&mut self) -> CBTC2_W<EBCIER_SPEC> {
        CBTC2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cbtc3(&mut self) -> CBTC3_W<EBCIER_SPEC> {
        CBTC3_W::new(self, 11)
    }
    #[doc = "Bit 12 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cbtc4(&mut self) -> CBTC4_W<EBCIER_SPEC> {
        CBTC4_W::new(self, 12)
    }
    #[doc = "Bit 13 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cbtc5(&mut self) -> CBTC5_W<EBCIER_SPEC> {
        CBTC5_W::new(self, 13)
    }
    #[doc = "Bit 16 - Access Error \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn err0(&mut self) -> ERR0_W<EBCIER_SPEC> {
        ERR0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Access Error \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn err1(&mut self) -> ERR1_W<EBCIER_SPEC> {
        ERR1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Access Error \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn err2(&mut self) -> ERR2_W<EBCIER_SPEC> {
        ERR2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Access Error \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn err3(&mut self) -> ERR3_W<EBCIER_SPEC> {
        ERR3_W::new(self, 19)
    }
    #[doc = "Bit 20 - Access Error \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn err4(&mut self) -> ERR4_W<EBCIER_SPEC> {
        ERR4_W::new(self, 20)
    }
    #[doc = "Bit 21 - Access Error \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn err5(&mut self) -> ERR5_W<EBCIER_SPEC> {
        ERR5_W::new(self, 21)
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
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register.\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ebcier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EBCIER_SPEC;
impl crate::RegisterSpec for EBCIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ebcier::W`](W) writer structure"]
impl crate::Writable for EBCIER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
