#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACR_SPEC>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACR_SPEC>;
#[doc = "Field `TSON` reader - Temperature Sensor On"]
pub type TSON_R = crate::BitReader;
#[doc = "Field `TSON` writer - Temperature Sensor On"]
pub type TSON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBCTL` reader - ADC Bias Current Control"]
pub type IBCTL_R = crate::FieldReader;
#[doc = "Field `IBCTL` writer - ADC Bias Current Control"]
pub type IBCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 4 - Temperature Sensor On"]
    #[inline(always)]
    pub fn tson(&self) -> TSON_R {
        TSON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - ADC Bias Current Control"]
    #[inline(always)]
    pub fn ibctl(&self) -> IBCTL_R {
        IBCTL_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Temperature Sensor On"]
    #[inline(always)]
    #[must_use]
    pub fn tson(&mut self) -> TSON_W<ACR_SPEC> {
        TSON_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - ADC Bias Current Control"]
    #[inline(always)]
    #[must_use]
    pub fn ibctl(&mut self) -> IBCTL_W<ACR_SPEC> {
        IBCTL_W::new(self, 8)
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
#[doc = "`reset()` method sets ACR to value 0x0100"]
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
