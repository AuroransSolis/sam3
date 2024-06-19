#[doc = "Register `ECOL` reader"]
pub type R = crate::R<EcolSpec>;
#[doc = "Register `ECOL` writer"]
pub type W = crate::W<EcolSpec>;
#[doc = "Field `EXCOL` reader - Excessive Collisions"]
pub type ExcolR = crate::FieldReader;
#[doc = "Field `EXCOL` writer - Excessive Collisions"]
pub type ExcolW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Excessive Collisions"]
    #[inline(always)]
    pub fn excol(&self) -> ExcolR {
        ExcolR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Excessive Collisions"]
    #[inline(always)]
    #[must_use]
    pub fn excol(&mut self) -> ExcolW<EcolSpec> {
        ExcolW::new(self, 0)
    }
}
#[doc = "Excessive Collisions Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcolSpec;
impl crate::RegisterSpec for EcolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecol::R`](R) reader structure"]
impl crate::Readable for EcolSpec {}
#[doc = "`write(|w| ..)` method takes [`ecol::W`](W) writer structure"]
impl crate::Writable for EcolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECOL to value 0"]
impl crate::Resettable for EcolSpec {
    const RESET_VALUE: u32 = 0;
}
