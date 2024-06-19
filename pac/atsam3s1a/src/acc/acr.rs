#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Current SELection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isel {
    #[doc = "0: low power option."]
    Lopw = 0,
    #[doc = "1: high speed option."]
    Hisp = 1,
}
impl From<Isel> for bool {
    #[inline(always)]
    fn from(variant: Isel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISEL` reader - Current SELection"]
pub type IselR = crate::BitReader<Isel>;
impl IselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isel {
        match self.bits {
            false => Isel::Lopw,
            true => Isel::Hisp,
        }
    }
    #[doc = "low power option."]
    #[inline(always)]
    pub fn is_lopw(&self) -> bool {
        *self == Isel::Lopw
    }
    #[doc = "high speed option."]
    #[inline(always)]
    pub fn is_hisp(&self) -> bool {
        *self == Isel::Hisp
    }
}
#[doc = "Field `ISEL` writer - Current SELection"]
pub type IselW<'a, REG> = crate::BitWriter<'a, REG, Isel>;
impl<'a, REG> IselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "low power option."]
    #[inline(always)]
    pub fn lopw(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::Lopw)
    }
    #[doc = "high speed option."]
    #[inline(always)]
    pub fn hisp(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::Hisp)
    }
}
#[doc = "Field `HYST` reader - HYSTeresis selection"]
pub type HystR = crate::FieldReader;
#[doc = "Field `HYST` writer - HYSTeresis selection"]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Current SELection"]
    #[inline(always)]
    pub fn isel(&self) -> IselR {
        IselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - HYSTeresis selection"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Current SELection"]
    #[inline(always)]
    #[must_use]
    pub fn isel(&mut self) -> IselW<AcrSpec> {
        IselW::new(self, 0)
    }
    #[doc = "Bits 1:2 - HYSTeresis selection"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HystW<AcrSpec> {
        HystW::new(self, 1)
    }
}
#[doc = "Analog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
