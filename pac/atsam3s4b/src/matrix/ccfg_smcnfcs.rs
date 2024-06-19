#[doc = "Register `CCFG_SMCNFCS` reader"]
pub type R = crate::R<CcfgSmcnfcsSpec>;
#[doc = "Register `CCFG_SMCNFCS` writer"]
pub type W = crate::W<CcfgSmcnfcsSpec>;
#[doc = "Field `SMC_NFCS0` reader - SMC NAND Flash Chip Select 0 Assignment"]
pub type SmcNfcs0R = crate::BitReader;
#[doc = "Field `SMC_NFCS0` writer - SMC NAND Flash Chip Select 0 Assignment"]
pub type SmcNfcs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMC_NFCS1` reader - SMC NAND Flash Chip Select 1 Assignment"]
pub type SmcNfcs1R = crate::BitReader;
#[doc = "Field `SMC_NFCS1` writer - SMC NAND Flash Chip Select 1 Assignment"]
pub type SmcNfcs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMC_NFCS2` reader - SMC NAND Flash Chip Select 2 Assignment"]
pub type SmcNfcs2R = crate::BitReader;
#[doc = "Field `SMC_NFCS2` writer - SMC NAND Flash Chip Select 2 Assignment"]
pub type SmcNfcs2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMC_NFCS3` reader - SMC NAND Flash Chip Select 3 Assignment"]
pub type SmcNfcs3R = crate::BitReader;
#[doc = "Field `SMC_NFCS3` writer - SMC NAND Flash Chip Select 3 Assignment"]
pub type SmcNfcs3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SMC NAND Flash Chip Select 0 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs0(&self) -> SmcNfcs0R {
        SmcNfcs0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMC NAND Flash Chip Select 1 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs1(&self) -> SmcNfcs1R {
        SmcNfcs1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMC NAND Flash Chip Select 2 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs2(&self) -> SmcNfcs2R {
        SmcNfcs2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMC NAND Flash Chip Select 3 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs3(&self) -> SmcNfcs3R {
        SmcNfcs3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SMC NAND Flash Chip Select 0 Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn smc_nfcs0(&mut self) -> SmcNfcs0W<CcfgSmcnfcsSpec> {
        SmcNfcs0W::new(self, 0)
    }
    #[doc = "Bit 1 - SMC NAND Flash Chip Select 1 Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn smc_nfcs1(&mut self) -> SmcNfcs1W<CcfgSmcnfcsSpec> {
        SmcNfcs1W::new(self, 1)
    }
    #[doc = "Bit 2 - SMC NAND Flash Chip Select 2 Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn smc_nfcs2(&mut self) -> SmcNfcs2W<CcfgSmcnfcsSpec> {
        SmcNfcs2W::new(self, 2)
    }
    #[doc = "Bit 3 - SMC NAND Flash Chip Select 3 Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn smc_nfcs3(&mut self) -> SmcNfcs3W<CcfgSmcnfcsSpec> {
        SmcNfcs3W::new(self, 3)
    }
}
#[doc = "SMC Chip Select NAND Flash Assignment Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_smcnfcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_smcnfcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcfgSmcnfcsSpec;
impl crate::RegisterSpec for CcfgSmcnfcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_smcnfcs::R`](R) reader structure"]
impl crate::Readable for CcfgSmcnfcsSpec {}
#[doc = "`write(|w| ..)` method takes [`ccfg_smcnfcs::W`](W) writer structure"]
impl crate::Writable for CcfgSmcnfcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCFG_SMCNFCS to value 0"]
impl crate::Resettable for CcfgSmcnfcsSpec {
    const RESET_VALUE: u32 = 0;
}
