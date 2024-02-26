#[doc = "Register `SCF` reader"]
pub type R = crate::R<ScfSpec>;
#[doc = "Register `SCF` writer"]
pub type W = crate::W<ScfSpec>;
#[doc = "Field `SCF` reader - Single Collision Frames"]
pub type ScfR = crate::FieldReader<u16>;
#[doc = "Field `SCF` writer - Single Collision Frames"]
pub type ScfW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Single Collision Frames"]
    #[inline(always)]
    pub fn scf(&self) -> ScfR {
        ScfR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Single Collision Frames"]
    #[inline(always)]
    #[must_use]
    pub fn scf(&mut self) -> ScfW<ScfSpec> {
        ScfW::new(self, 0)
    }
}
#[doc = "Single Collision Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScfSpec;
impl crate::RegisterSpec for ScfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scf::R`](R) reader structure"]
impl crate::Readable for ScfSpec {}
#[doc = "`write(|w| ..)` method takes [`scf::W`](W) writer structure"]
impl crate::Writable for ScfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCF to value 0"]
impl crate::Resettable for ScfSpec {
    const RESET_VALUE: u32 = 0;
}
