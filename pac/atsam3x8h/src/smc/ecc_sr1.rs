#[doc = "Register `ECC_SR1` reader"]
pub type R = crate::R<EccSr1Spec>;
#[doc = "Field `RECERR0` reader - Recoverable Error"]
pub type Recerr0R = crate::BitReader;
#[doc = "Field `ECCERR0` reader - ECC Error"]
pub type Eccerr0R = crate::BitReader;
#[doc = "Field `MULERR0` reader - Multiple Error"]
pub type Mulerr0R = crate::BitReader;
#[doc = "Field `RECERR1` reader - Recoverable Error in the page between the 256th and the 511th bytes or the 512nd and the 1023rd bytes"]
pub type Recerr1R = crate::BitReader;
#[doc = "Field `ECCERR1` reader - ECC Error in the page between the 256th and the 511th bytes or between the 512nd and the 1023rd bytes"]
pub type Eccerr1R = crate::BitReader;
#[doc = "Field `MULERR1` reader - Multiple Error in the page between the 256th and the 511th bytes or between the 512nd and the 1023rd bytes"]
pub type Mulerr1R = crate::BitReader;
#[doc = "Field `RECERR2` reader - Recoverable Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
pub type Recerr2R = crate::BitReader;
#[doc = "Field `ECCERR2` reader - ECC Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
pub type Eccerr2R = crate::BitReader;
#[doc = "Field `MULERR2` reader - Multiple Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
pub type Mulerr2R = crate::BitReader;
#[doc = "Field `RECERR3` reader - Recoverable Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
pub type Recerr3R = crate::BitReader;
#[doc = "Field `ECCERR3` reader - ECC Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
pub type Eccerr3R = crate::BitReader;
#[doc = "Field `MULERR3` reader - Multiple Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
pub type Mulerr3R = crate::BitReader;
#[doc = "Field `RECERR4` reader - Recoverable Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
pub type Recerr4R = crate::BitReader;
#[doc = "Field `ECCERR4` reader - ECC Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
pub type Eccerr4R = crate::BitReader;
#[doc = "Field `MULERR4` reader - Multiple Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
pub type Mulerr4R = crate::BitReader;
#[doc = "Field `RECERR5` reader - Recoverable Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
pub type Recerr5R = crate::BitReader;
#[doc = "Field `ECCERR5` reader - ECC Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
pub type Eccerr5R = crate::BitReader;
#[doc = "Field `MULERR5` reader - Multiple Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
pub type Mulerr5R = crate::BitReader;
#[doc = "Field `RECERR6` reader - Recoverable Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
pub type Recerr6R = crate::BitReader;
#[doc = "Field `ECCERR6` reader - ECC Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
pub type Eccerr6R = crate::BitReader;
#[doc = "Field `MULERR6` reader - Multiple Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
pub type Mulerr6R = crate::BitReader;
#[doc = "Field `RECERR7` reader - Recoverable Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
pub type Recerr7R = crate::BitReader;
#[doc = "Field `ECCERR7` reader - ECC Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
pub type Eccerr7R = crate::BitReader;
#[doc = "Field `MULERR7` reader - Multiple Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
pub type Mulerr7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Recoverable Error"]
    #[inline(always)]
    pub fn recerr0(&self) -> Recerr0R {
        Recerr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC Error"]
    #[inline(always)]
    pub fn eccerr0(&self) -> Eccerr0R {
        Eccerr0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Multiple Error"]
    #[inline(always)]
    pub fn mulerr0(&self) -> Mulerr0R {
        Mulerr0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Recoverable Error in the page between the 256th and the 511th bytes or the 512nd and the 1023rd bytes"]
    #[inline(always)]
    pub fn recerr1(&self) -> Recerr1R {
        Recerr1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ECC Error in the page between the 256th and the 511th bytes or between the 512nd and the 1023rd bytes"]
    #[inline(always)]
    pub fn eccerr1(&self) -> Eccerr1R {
        Eccerr1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multiple Error in the page between the 256th and the 511th bytes or between the 512nd and the 1023rd bytes"]
    #[inline(always)]
    pub fn mulerr1(&self) -> Mulerr1R {
        Mulerr1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Recoverable Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
    #[inline(always)]
    pub fn recerr2(&self) -> Recerr2R {
        Recerr2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ECC Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
    #[inline(always)]
    pub fn eccerr2(&self) -> Eccerr2R {
        Eccerr2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multiple Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
    #[inline(always)]
    pub fn mulerr2(&self) -> Mulerr2R {
        Mulerr2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Recoverable Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
    #[inline(always)]
    pub fn recerr3(&self) -> Recerr3R {
        Recerr3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ECC Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
    #[inline(always)]
    pub fn eccerr3(&self) -> Eccerr3R {
        Eccerr3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Multiple Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
    #[inline(always)]
    pub fn mulerr3(&self) -> Mulerr3R {
        Mulerr3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Recoverable Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
    #[inline(always)]
    pub fn recerr4(&self) -> Recerr4R {
        Recerr4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ECC Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
    #[inline(always)]
    pub fn eccerr4(&self) -> Eccerr4R {
        Eccerr4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Multiple Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
    #[inline(always)]
    pub fn mulerr4(&self) -> Mulerr4R {
        Mulerr4R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Recoverable Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
    #[inline(always)]
    pub fn recerr5(&self) -> Recerr5R {
        Recerr5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ECC Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
    #[inline(always)]
    pub fn eccerr5(&self) -> Eccerr5R {
        Eccerr5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Multiple Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
    #[inline(always)]
    pub fn mulerr5(&self) -> Mulerr5R {
        Mulerr5R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Recoverable Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
    #[inline(always)]
    pub fn recerr6(&self) -> Recerr6R {
        Recerr6R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ECC Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
    #[inline(always)]
    pub fn eccerr6(&self) -> Eccerr6R {
        Eccerr6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Multiple Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
    #[inline(always)]
    pub fn mulerr6(&self) -> Mulerr6R {
        Mulerr6R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Recoverable Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
    #[inline(always)]
    pub fn recerr7(&self) -> Recerr7R {
        Recerr7R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ECC Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
    #[inline(always)]
    pub fn eccerr7(&self) -> Eccerr7R {
        Eccerr7R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Multiple Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
    #[inline(always)]
    pub fn mulerr7(&self) -> Mulerr7R {
        Mulerr7R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "SMC ECC Status 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccSr1Spec;
impl crate::RegisterSpec for EccSr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_sr1::R`](R) reader structure"]
impl crate::Readable for EccSr1Spec {}
#[doc = "`reset()` method sets ECC_SR1 to value 0"]
impl crate::Resettable for EccSr1Spec {
    const RESET_VALUE: u32 = 0;
}
