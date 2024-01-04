#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACR_SPEC>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACR_SPEC>;
#[doc = "Field `ISEL` reader - Current SELection"]
pub type ISEL_R = crate::BitReader<ISEL_A>;
#[doc = "Current SELection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISEL_A {
    #[doc = "0: low power option."]
    Lopw = 0,
    #[doc = "1: high speed option."]
    Hisp = 1,
}
impl From<ISEL_A> for bool {
    #[inline(always)]
    fn from(variant: ISEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ISEL_A {
        match self.bits {
            false => ISEL_A::Lopw,
            true => ISEL_A::Hisp,
        }
    }
    #[doc = "low power option."]
    #[inline(always)]
    pub fn is_lopw(&self) -> bool {
        *self == ISEL_A::Lopw
    }
    #[doc = "high speed option."]
    #[inline(always)]
    pub fn is_hisp(&self) -> bool {
        *self == ISEL_A::Hisp
    }
}
#[doc = "Field `ISEL` writer - Current SELection"]
pub type ISEL_W<'a, REG> = crate::BitWriter<'a, REG, ISEL_A>;
impl<'a, REG> ISEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "low power option."]
    #[inline(always)]
    pub fn lopw(self) -> &'a mut crate::W<REG> {
        self.variant(ISEL_A::Lopw)
    }
    #[doc = "high speed option."]
    #[inline(always)]
    pub fn hisp(self) -> &'a mut crate::W<REG> {
        self.variant(ISEL_A::Hisp)
    }
}
#[doc = "Field `HYST` reader - HYSTeresis selection"]
pub type HYST_R = crate::FieldReader;
#[doc = "Field `HYST` writer - HYSTeresis selection"]
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Current SELection"]
    #[inline(always)]
    pub fn isel(&self) -> ISEL_R {
        ISEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - HYSTeresis selection"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Current SELection"]
    #[inline(always)]
    #[must_use]
    pub fn isel(&mut self) -> ISEL_W<ACR_SPEC> {
        ISEL_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - HYSTeresis selection"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<ACR_SPEC> {
        HYST_W::new(self, 1)
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
#[doc = "Analog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for ACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: u32 = 0;
}
