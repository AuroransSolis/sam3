#[doc = "Register `ECC_SR2` reader"]
pub type R = crate::R<EccSr2Spec>;
#[doc = "Field `RECERR8` reader - Recoverable Error in the page between the 2048th and the 2303rd bytes"]
pub type Recerr8R = crate::BitReader;
#[doc = "Field `ECCERR8` reader - ECC Error in the page between the 2048th and the 2303rd bytes"]
pub type Eccerr8R = crate::BitReader;
#[doc = "Field `MULERR8` reader - Multiple Error in the page between the 2048th and the 2303rd bytes"]
pub type Mulerr8R = crate::BitReader;
#[doc = "Field `RECERR9` reader - Recoverable Error in the page between the 2304th and the 2559th bytes"]
pub type Recerr9R = crate::BitReader;
#[doc = "Field `ECCERR9` reader - ECC Error in the page between the 2304th and the 2559th bytes"]
pub type Eccerr9R = crate::BitReader;
#[doc = "Field `MULERR9` reader - Multiple Error in the page between the 2304th and the 2559th bytes"]
pub type Mulerr9R = crate::BitReader;
#[doc = "Field `RECERR10` reader - Recoverable Error in the page between the 2560th and the 2815th bytes"]
pub type Recerr10R = crate::BitReader;
#[doc = "Field `ECCERR10` reader - ECC Error in the page between the 2560th and the 2815th bytes"]
pub type Eccerr10R = crate::BitReader;
#[doc = "Field `MULERR10` reader - Multiple Error in the page between the 2560th and the 2815th bytes"]
pub type Mulerr10R = crate::BitReader;
#[doc = "Field `RECERR11` reader - Recoverable Error in the page between the 2816th and the 3071st bytes"]
pub type Recerr11R = crate::BitReader;
#[doc = "Field `ECCERR11` reader - ECC Error in the page between the 2816th and the 3071st bytes"]
pub type Eccerr11R = crate::BitReader;
#[doc = "Field `MULERR11` reader - Multiple Error in the page between the 2816th and the 3071st bytes"]
pub type Mulerr11R = crate::BitReader;
#[doc = "Field `RECERR12` reader - Recoverable Error in the page between the 3072nd and the 3327th bytes"]
pub type Recerr12R = crate::BitReader;
#[doc = "Field `ECCERR12` reader - ECC Error in the page between the 3072nd and the 3327th bytes"]
pub type Eccerr12R = crate::BitReader;
#[doc = "Field `MULERR12` reader - Multiple Error in the page between the 3072nd and the 3327th bytes"]
pub type Mulerr12R = crate::BitReader;
#[doc = "Field `RECERR13` reader - Recoverable Error in the page between the 3328th and the 3583rd bytes"]
pub type Recerr13R = crate::BitReader;
#[doc = "Field `ECCERR13` reader - ECC Error in the page between the 3328th and the 3583rd bytes"]
pub type Eccerr13R = crate::BitReader;
#[doc = "Field `MULERR13` reader - Multiple Error in the page between the 3328th and the 3583rd bytes"]
pub type Mulerr13R = crate::BitReader;
#[doc = "Field `RECERR14` reader - Recoverable Error in the page between the 3584th and the 3839th bytes"]
pub type Recerr14R = crate::BitReader;
#[doc = "Field `ECCERR14` reader - ECC Error in the page between the 3584th and the 3839th bytes"]
pub type Eccerr14R = crate::BitReader;
#[doc = "Field `MULERR14` reader - Multiple Error in the page between the 3584th and the 3839th bytes"]
pub type Mulerr14R = crate::BitReader;
#[doc = "Field `RECERR15` reader - Recoverable Error in the page between the 3840th and the 4095th bytes"]
pub type Recerr15R = crate::BitReader;
#[doc = "Field `ECCERR15` reader - ECC Error in the page between the 3840th and the 4095th bytes"]
pub type Eccerr15R = crate::BitReader;
#[doc = "Field `MULERR15` reader - Multiple Error in the page between the 3840th and the 4095th bytes"]
pub type Mulerr15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Recoverable Error in the page between the 2048th and the 2303rd bytes"]
    #[inline(always)]
    pub fn recerr8(&self) -> Recerr8R {
        Recerr8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC Error in the page between the 2048th and the 2303rd bytes"]
    #[inline(always)]
    pub fn eccerr8(&self) -> Eccerr8R {
        Eccerr8R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Multiple Error in the page between the 2048th and the 2303rd bytes"]
    #[inline(always)]
    pub fn mulerr8(&self) -> Mulerr8R {
        Mulerr8R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Recoverable Error in the page between the 2304th and the 2559th bytes"]
    #[inline(always)]
    pub fn recerr9(&self) -> Recerr9R {
        Recerr9R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ECC Error in the page between the 2304th and the 2559th bytes"]
    #[inline(always)]
    pub fn eccerr9(&self) -> Eccerr9R {
        Eccerr9R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multiple Error in the page between the 2304th and the 2559th bytes"]
    #[inline(always)]
    pub fn mulerr9(&self) -> Mulerr9R {
        Mulerr9R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Recoverable Error in the page between the 2560th and the 2815th bytes"]
    #[inline(always)]
    pub fn recerr10(&self) -> Recerr10R {
        Recerr10R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ECC Error in the page between the 2560th and the 2815th bytes"]
    #[inline(always)]
    pub fn eccerr10(&self) -> Eccerr10R {
        Eccerr10R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multiple Error in the page between the 2560th and the 2815th bytes"]
    #[inline(always)]
    pub fn mulerr10(&self) -> Mulerr10R {
        Mulerr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Recoverable Error in the page between the 2816th and the 3071st bytes"]
    #[inline(always)]
    pub fn recerr11(&self) -> Recerr11R {
        Recerr11R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ECC Error in the page between the 2816th and the 3071st bytes"]
    #[inline(always)]
    pub fn eccerr11(&self) -> Eccerr11R {
        Eccerr11R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Multiple Error in the page between the 2816th and the 3071st bytes"]
    #[inline(always)]
    pub fn mulerr11(&self) -> Mulerr11R {
        Mulerr11R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Recoverable Error in the page between the 3072nd and the 3327th bytes"]
    #[inline(always)]
    pub fn recerr12(&self) -> Recerr12R {
        Recerr12R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ECC Error in the page between the 3072nd and the 3327th bytes"]
    #[inline(always)]
    pub fn eccerr12(&self) -> Eccerr12R {
        Eccerr12R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Multiple Error in the page between the 3072nd and the 3327th bytes"]
    #[inline(always)]
    pub fn mulerr12(&self) -> Mulerr12R {
        Mulerr12R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Recoverable Error in the page between the 3328th and the 3583rd bytes"]
    #[inline(always)]
    pub fn recerr13(&self) -> Recerr13R {
        Recerr13R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ECC Error in the page between the 3328th and the 3583rd bytes"]
    #[inline(always)]
    pub fn eccerr13(&self) -> Eccerr13R {
        Eccerr13R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Multiple Error in the page between the 3328th and the 3583rd bytes"]
    #[inline(always)]
    pub fn mulerr13(&self) -> Mulerr13R {
        Mulerr13R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Recoverable Error in the page between the 3584th and the 3839th bytes"]
    #[inline(always)]
    pub fn recerr14(&self) -> Recerr14R {
        Recerr14R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ECC Error in the page between the 3584th and the 3839th bytes"]
    #[inline(always)]
    pub fn eccerr14(&self) -> Eccerr14R {
        Eccerr14R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Multiple Error in the page between the 3584th and the 3839th bytes"]
    #[inline(always)]
    pub fn mulerr14(&self) -> Mulerr14R {
        Mulerr14R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Recoverable Error in the page between the 3840th and the 4095th bytes"]
    #[inline(always)]
    pub fn recerr15(&self) -> Recerr15R {
        Recerr15R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ECC Error in the page between the 3840th and the 4095th bytes"]
    #[inline(always)]
    pub fn eccerr15(&self) -> Eccerr15R {
        Eccerr15R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Multiple Error in the page between the 3840th and the 4095th bytes"]
    #[inline(always)]
    pub fn mulerr15(&self) -> Mulerr15R {
        Mulerr15R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "SMC ECC status 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccSr2Spec;
impl crate::RegisterSpec for EccSr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_sr2::R`](R) reader structure"]
impl crate::Readable for EccSr2Spec {}
#[doc = "`reset()` method sets ECC_SR2 to value 0"]
impl crate::Resettable for EccSr2Spec {
    const RESET_VALUE: u32 = 0;
}
