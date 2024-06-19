#[doc = "Register `CKGR_PLLBR` reader"]
pub type R = crate::R<CkgrPllbrSpec>;
#[doc = "Register `CKGR_PLLBR` writer"]
pub type W = crate::W<CkgrPllbrSpec>;
#[doc = "Field `DIVB` reader - Divider"]
pub type DivbR = crate::FieldReader;
#[doc = "Field `DIVB` writer - Divider"]
pub type DivbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PLLBCOUNT` reader - PLLB Counter"]
pub type PllbcountR = crate::FieldReader;
#[doc = "Field `PLLBCOUNT` writer - PLLB Counter"]
pub type PllbcountW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MULB` reader - PLLB Multiplier"]
pub type MulbR = crate::FieldReader<u16>;
#[doc = "Field `MULB` writer - PLLB Multiplier"]
pub type MulbW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:7 - Divider"]
    #[inline(always)]
    pub fn divb(&self) -> DivbR {
        DivbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - PLLB Counter"]
    #[inline(always)]
    pub fn pllbcount(&self) -> PllbcountR {
        PllbcountR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:26 - PLLB Multiplier"]
    #[inline(always)]
    pub fn mulb(&self) -> MulbR {
        MulbR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Divider"]
    #[inline(always)]
    #[must_use]
    pub fn divb(&mut self) -> DivbW<CkgrPllbrSpec> {
        DivbW::new(self, 0)
    }
    #[doc = "Bits 8:13 - PLLB Counter"]
    #[inline(always)]
    #[must_use]
    pub fn pllbcount(&mut self) -> PllbcountW<CkgrPllbrSpec> {
        PllbcountW::new(self, 8)
    }
    #[doc = "Bits 16:26 - PLLB Multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn mulb(&mut self) -> MulbW<CkgrPllbrSpec> {
        MulbW::new(self, 16)
    }
}
#[doc = "PLLB Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_pllbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_pllbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkgrPllbrSpec;
impl crate::RegisterSpec for CkgrPllbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgr_pllbr::R`](R) reader structure"]
impl crate::Readable for CkgrPllbrSpec {}
#[doc = "`write(|w| ..)` method takes [`ckgr_pllbr::W`](W) writer structure"]
impl crate::Writable for CkgrPllbrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CKGR_PLLBR to value 0x3f00"]
impl crate::Resettable for CkgrPllbrSpec {
    const RESET_VALUE: u32 = 0x3f00;
}
