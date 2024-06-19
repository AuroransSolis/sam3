#[doc = "Register `FIDI` reader"]
pub type R = crate::R<FidiSpec>;
#[doc = "Register `FIDI` writer"]
pub type W = crate::W<FidiSpec>;
#[doc = "Field `FI_DI_RATIO` reader - FI Over DI Ratio Value"]
pub type FiDiRatioR = crate::FieldReader<u16>;
#[doc = "Field `FI_DI_RATIO` writer - FI Over DI Ratio Value"]
pub type FiDiRatioW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - FI Over DI Ratio Value"]
    #[inline(always)]
    pub fn fi_di_ratio(&self) -> FiDiRatioR {
        FiDiRatioR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - FI Over DI Ratio Value"]
    #[inline(always)]
    #[must_use]
    pub fn fi_di_ratio(&mut self) -> FiDiRatioW<FidiSpec> {
        FiDiRatioW::new(self, 0)
    }
}
#[doc = "FI DI Ratio Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fidi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fidi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FidiSpec;
impl crate::RegisterSpec for FidiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fidi::R`](R) reader structure"]
impl crate::Readable for FidiSpec {}
#[doc = "`write(|w| ..)` method takes [`fidi::W`](W) writer structure"]
impl crate::Writable for FidiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIDI to value 0x0174"]
impl crate::Resettable for FidiSpec {
    const RESET_VALUE: u32 = 0x0174;
}
