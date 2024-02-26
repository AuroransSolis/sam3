#[doc = "Register `SMMR1` reader"]
pub type R = crate::R<Smmr1Spec>;
#[doc = "Register `SMMR1` writer"]
pub type W = crate::W<Smmr1Spec>;
#[doc = "Field `GCEN` reader - Gray Count Enable"]
pub type GcenR = crate::BitReader;
#[doc = "Field `GCEN` writer - Gray Count Enable"]
pub type GcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWN` reader - Down Count"]
pub type DownR = crate::BitReader;
#[doc = "Field `DOWN` writer - Down Count"]
pub type DownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Gray Count Enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GcenR {
        GcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Down Count"]
    #[inline(always)]
    pub fn down(&self) -> DownR {
        DownR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gray Count Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GcenW<Smmr1Spec> {
        GcenW::new(self, 0)
    }
    #[doc = "Bit 1 - Down Count"]
    #[inline(always)]
    #[must_use]
    pub fn down(&mut self) -> DownW<Smmr1Spec> {
        DownW::new(self, 1)
    }
}
#[doc = "Stepper Motor Mode Register (channel = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smmr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smmr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Smmr1Spec;
impl crate::RegisterSpec for Smmr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smmr1::R`](R) reader structure"]
impl crate::Readable for Smmr1Spec {}
#[doc = "`write(|w| ..)` method takes [`smmr1::W`](W) writer structure"]
impl crate::Writable for Smmr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMMR1 to value 0"]
impl crate::Resettable for Smmr1Spec {
    const RESET_VALUE: u32 = 0;
}
