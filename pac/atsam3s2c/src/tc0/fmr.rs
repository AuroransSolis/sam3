#[doc = "Register `FMR` reader"]
pub type R = crate::R<FmrSpec>;
#[doc = "Register `FMR` writer"]
pub type W = crate::W<FmrSpec>;
#[doc = "Field `ENCF0` reader - ENable Compare Fault Channel 0"]
pub type Encf0R = crate::BitReader;
#[doc = "Field `ENCF0` writer - ENable Compare Fault Channel 0"]
pub type Encf0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENCF1` reader - ENable Compare Fault Channel 1"]
pub type Encf1R = crate::BitReader;
#[doc = "Field `ENCF1` writer - ENable Compare Fault Channel 1"]
pub type Encf1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ENable Compare Fault Channel 0"]
    #[inline(always)]
    pub fn encf0(&self) -> Encf0R {
        Encf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ENable Compare Fault Channel 1"]
    #[inline(always)]
    pub fn encf1(&self) -> Encf1R {
        Encf1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENable Compare Fault Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn encf0(&mut self) -> Encf0W<FmrSpec> {
        Encf0W::new(self, 0)
    }
    #[doc = "Bit 1 - ENable Compare Fault Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn encf1(&mut self) -> Encf1W<FmrSpec> {
        Encf1W::new(self, 1)
    }
}
#[doc = "Fault Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmrSpec;
impl crate::RegisterSpec for FmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmr::R`](R) reader structure"]
impl crate::Readable for FmrSpec {}
#[doc = "`write(|w| ..)` method takes [`fmr::W`](W) writer structure"]
impl crate::Writable for FmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMR to value 0"]
impl crate::Resettable for FmrSpec {
    const RESET_VALUE: u32 = 0;
}
