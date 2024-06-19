#[doc = "Register `SCUP` reader"]
pub type R = crate::R<ScupSpec>;
#[doc = "Register `SCUP` writer"]
pub type W = crate::W<ScupSpec>;
#[doc = "Field `UPR` reader - Update Period"]
pub type UprR = crate::FieldReader;
#[doc = "Field `UPR` writer - Update Period"]
pub type UprW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UPRCNT` reader - Update Period Counter"]
pub type UprcntR = crate::FieldReader;
#[doc = "Field `UPRCNT` writer - Update Period Counter"]
pub type UprcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Update Period"]
    #[inline(always)]
    pub fn upr(&self) -> UprR {
        UprR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Update Period Counter"]
    #[inline(always)]
    pub fn uprcnt(&self) -> UprcntR {
        UprcntR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Update Period"]
    #[inline(always)]
    #[must_use]
    pub fn upr(&mut self) -> UprW<ScupSpec> {
        UprW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Update Period Counter"]
    #[inline(always)]
    #[must_use]
    pub fn uprcnt(&mut self) -> UprcntW<ScupSpec> {
        UprcntW::new(self, 4)
    }
}
#[doc = "PWM Sync Channels Update Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScupSpec;
impl crate::RegisterSpec for ScupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scup::R`](R) reader structure"]
impl crate::Readable for ScupSpec {}
#[doc = "`write(|w| ..)` method takes [`scup::W`](W) writer structure"]
impl crate::Writable for ScupSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCUP to value 0"]
impl crate::Resettable for ScupSpec {
    const RESET_VALUE: u32 = 0;
}
