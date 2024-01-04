#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACR_SPEC>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACR_SPEC>;
#[doc = "Field `IBCTLCH0` reader - Analog Output Current Control"]
pub type IBCTLCH0_R = crate::FieldReader;
#[doc = "Field `IBCTLCH0` writer - Analog Output Current Control"]
pub type IBCTLCH0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IBCTLCH1` reader - Analog Output Current Control"]
pub type IBCTLCH1_R = crate::FieldReader;
#[doc = "Field `IBCTLCH1` writer - Analog Output Current Control"]
pub type IBCTLCH1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IBCTLDACCORE` reader - Bias Current Control for DAC Core"]
pub type IBCTLDACCORE_R = crate::FieldReader;
#[doc = "Field `IBCTLDACCORE` writer - Bias Current Control for DAC Core"]
pub type IBCTLDACCORE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Analog Output Current Control"]
    #[inline(always)]
    pub fn ibctlch0(&self) -> IBCTLCH0_R {
        IBCTLCH0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Analog Output Current Control"]
    #[inline(always)]
    pub fn ibctlch1(&self) -> IBCTLCH1_R {
        IBCTLCH1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Bias Current Control for DAC Core"]
    #[inline(always)]
    pub fn ibctldaccore(&self) -> IBCTLDACCORE_R {
        IBCTLDACCORE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Output Current Control"]
    #[inline(always)]
    #[must_use]
    pub fn ibctlch0(&mut self) -> IBCTLCH0_W<ACR_SPEC> {
        IBCTLCH0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Analog Output Current Control"]
    #[inline(always)]
    #[must_use]
    pub fn ibctlch1(&mut self) -> IBCTLCH1_W<ACR_SPEC> {
        IBCTLCH1_W::new(self, 2)
    }
    #[doc = "Bits 8:9 - Bias Current Control for DAC Core"]
    #[inline(always)]
    #[must_use]
    pub fn ibctldaccore(&mut self) -> IBCTLDACCORE_W<ACR_SPEC> {
        IBCTLDACCORE_W::new(self, 8)
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
#[doc = "Analog Current Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
