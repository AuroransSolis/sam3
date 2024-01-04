#[doc = "Register `SMMR0` reader"]
pub type R = crate::R<SMMR0_SPEC>;
#[doc = "Register `SMMR0` writer"]
pub type W = crate::W<SMMR0_SPEC>;
#[doc = "Field `GCEN` reader - Gray Count Enable"]
pub type GCEN_R = crate::BitReader;
#[doc = "Field `GCEN` writer - Gray Count Enable"]
pub type GCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWN` reader - DOWN Count"]
pub type DOWN_R = crate::BitReader;
#[doc = "Field `DOWN` writer - DOWN Count"]
pub type DOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Gray Count Enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DOWN Count"]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gray Count Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<SMMR0_SPEC> {
        GCEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DOWN Count"]
    #[inline(always)]
    #[must_use]
    pub fn down(&mut self) -> DOWN_W<SMMR0_SPEC> {
        DOWN_W::new(self, 1)
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
#[doc = "Stepper Motor Mode Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smmr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smmr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMMR0_SPEC;
impl crate::RegisterSpec for SMMR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smmr0::R`](R) reader structure"]
impl crate::Readable for SMMR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smmr0::W`](W) writer structure"]
impl crate::Writable for SMMR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMMR0 to value 0"]
impl crate::Resettable for SMMR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
