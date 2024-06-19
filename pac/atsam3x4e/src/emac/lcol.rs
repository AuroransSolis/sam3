#[doc = "Register `LCOL` reader"]
pub type R = crate::R<LcolSpec>;
#[doc = "Register `LCOL` writer"]
pub type W = crate::W<LcolSpec>;
#[doc = "Field `LCOL` reader - Late Collisions"]
pub type LcolR = crate::FieldReader;
#[doc = "Field `LCOL` writer - Late Collisions"]
pub type LcolW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Late Collisions"]
    #[inline(always)]
    pub fn lcol(&self) -> LcolR {
        LcolR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Late Collisions"]
    #[inline(always)]
    #[must_use]
    pub fn lcol(&mut self) -> LcolW<LcolSpec> {
        LcolW::new(self, 0)
    }
}
#[doc = "Late Collisions Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcolSpec;
impl crate::RegisterSpec for LcolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcol::R`](R) reader structure"]
impl crate::Readable for LcolSpec {}
#[doc = "`write(|w| ..)` method takes [`lcol::W`](W) writer structure"]
impl crate::Writable for LcolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCOL to value 0"]
impl crate::Resettable for LcolSpec {
    const RESET_VALUE: u32 = 0;
}
