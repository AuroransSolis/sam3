#[doc = "Register `CMPV5` reader"]
pub type R = crate::R<Cmpv5Spec>;
#[doc = "Register `CMPV5` writer"]
pub type W = crate::W<Cmpv5Spec>;
#[doc = "Field `CV` reader - Comparison x Value"]
pub type CvR = crate::FieldReader<u32>;
#[doc = "Field `CV` writer - Comparison x Value"]
pub type CvW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `CVM` reader - Comparison x Value Mode"]
pub type CvmR = crate::BitReader;
#[doc = "Field `CVM` writer - Comparison x Value Mode"]
pub type CvmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&self) -> CvR {
        CvR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&self) -> CvmR {
        CvmR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    #[must_use]
    pub fn cv(&mut self) -> CvW<Cmpv5Spec> {
        CvW::new(self, 0)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cvm(&mut self) -> CvmW<Cmpv5Spec> {
        CvmW::new(self, 24)
    }
}
#[doc = "PWM Comparison 5 Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpv5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpv5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmpv5Spec;
impl crate::RegisterSpec for Cmpv5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpv5::R`](R) reader structure"]
impl crate::Readable for Cmpv5Spec {}
#[doc = "`write(|w| ..)` method takes [`cmpv5::W`](W) writer structure"]
impl crate::Writable for Cmpv5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPV5 to value 0"]
impl crate::Resettable for Cmpv5Spec {
    const RESET_VALUE: u32 = 0;
}
