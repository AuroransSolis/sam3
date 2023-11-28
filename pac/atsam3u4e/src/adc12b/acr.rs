#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACR_SPEC>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACR_SPEC>;
#[doc = "Field `GAIN` reader - Input Gain"]
pub type GAIN_R = crate::FieldReader;
#[doc = "Field `GAIN` writer - Input Gain"]
pub type GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IBCTL` reader - Bias Current Control"]
pub type IBCTL_R = crate::FieldReader;
#[doc = "Field `IBCTL` writer - Bias Current Control"]
pub type IBCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIFF` reader - Differential Mode"]
pub type DIFF_R = crate::BitReader;
#[doc = "Field `DIFF` writer - Differential Mode"]
pub type DIFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSET` reader - Input OFFSET"]
pub type OFFSET_R = crate::BitReader;
#[doc = "Field `OFFSET` writer - Input OFFSET"]
pub type OFFSET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Input Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Bias Current Control"]
    #[inline(always)]
    pub fn ibctl(&self) -> IBCTL_R {
        IBCTL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input OFFSET"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Gain"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GAIN_W<ACR_SPEC> {
        GAIN_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Bias Current Control"]
    #[inline(always)]
    #[must_use]
    pub fn ibctl(&mut self) -> IBCTL_W<ACR_SPEC> {
        IBCTL_W::new(self, 8)
    }
    #[doc = "Bit 16 - Differential Mode"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DIFF_W<ACR_SPEC> {
        DIFF_W::new(self, 16)
    }
    #[doc = "Bit 17 - Input OFFSET"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<ACR_SPEC> {
        OFFSET_W::new(self, 17)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
