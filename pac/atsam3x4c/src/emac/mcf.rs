#[doc = "Register `MCF` reader"]
pub type R = crate::R<McfSpec>;
#[doc = "Register `MCF` writer"]
pub type W = crate::W<McfSpec>;
#[doc = "Field `MCF` reader - Multicollision Frames"]
pub type McfR = crate::FieldReader<u16>;
#[doc = "Field `MCF` writer - Multicollision Frames"]
pub type McfW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Multicollision Frames"]
    #[inline(always)]
    pub fn mcf(&self) -> McfR {
        McfR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Multicollision Frames"]
    #[inline(always)]
    #[must_use]
    pub fn mcf(&mut self) -> McfW<McfSpec> {
        McfW::new(self, 0)
    }
}
#[doc = "Multiple Collision Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McfSpec;
impl crate::RegisterSpec for McfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcf::R`](R) reader structure"]
impl crate::Readable for McfSpec {}
#[doc = "`write(|w| ..)` method takes [`mcf::W`](W) writer structure"]
impl crate::Writable for McfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCF to value 0"]
impl crate::Resettable for McfSpec {
    const RESET_VALUE: u32 = 0;
}
