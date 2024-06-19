#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Field `IBCTLCH0` reader - Analog Output Current Control"]
pub type Ibctlch0R = crate::FieldReader;
#[doc = "Field `IBCTLCH0` writer - Analog Output Current Control"]
pub type Ibctlch0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IBCTLCH1` reader - Analog Output Current Control"]
pub type Ibctlch1R = crate::FieldReader;
#[doc = "Field `IBCTLCH1` writer - Analog Output Current Control"]
pub type Ibctlch1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IBCTLDACCORE` reader - Bias Current Control for DAC Core"]
pub type IbctldaccoreR = crate::FieldReader;
#[doc = "Field `IBCTLDACCORE` writer - Bias Current Control for DAC Core"]
pub type IbctldaccoreW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Analog Output Current Control"]
    #[inline(always)]
    pub fn ibctlch0(&self) -> Ibctlch0R {
        Ibctlch0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Analog Output Current Control"]
    #[inline(always)]
    pub fn ibctlch1(&self) -> Ibctlch1R {
        Ibctlch1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Bias Current Control for DAC Core"]
    #[inline(always)]
    pub fn ibctldaccore(&self) -> IbctldaccoreR {
        IbctldaccoreR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Output Current Control"]
    #[inline(always)]
    #[must_use]
    pub fn ibctlch0(&mut self) -> Ibctlch0W<AcrSpec> {
        Ibctlch0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Analog Output Current Control"]
    #[inline(always)]
    #[must_use]
    pub fn ibctlch1(&mut self) -> Ibctlch1W<AcrSpec> {
        Ibctlch1W::new(self, 2)
    }
    #[doc = "Bits 8:9 - Bias Current Control for DAC Core"]
    #[inline(always)]
    #[must_use]
    pub fn ibctldaccore(&mut self) -> IbctldaccoreW<AcrSpec> {
        IbctldaccoreW::new(self, 8)
    }
}
#[doc = "Analog Current Register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcrSpec;
impl crate::RegisterSpec for AcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for AcrSpec {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for AcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for AcrSpec {
    const RESET_VALUE: u32 = 0;
}
