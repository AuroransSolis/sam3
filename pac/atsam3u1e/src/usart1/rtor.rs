#[doc = "Register `RTOR` reader"]
pub type R = crate::R<RtorSpec>;
#[doc = "Register `RTOR` writer"]
pub type W = crate::W<RtorSpec>;
#[doc = "Field `TO` reader - Time-out Value"]
pub type ToR = crate::FieldReader<u16>;
#[doc = "Field `TO` writer - Time-out Value"]
pub type ToW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Time-out Value"]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time-out Value"]
    #[inline(always)]
    #[must_use]
    pub fn to(&mut self) -> ToW<RtorSpec> {
        ToW::new(self, 0)
    }
}
#[doc = "Receiver Time-out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtorSpec;
impl crate::RegisterSpec for RtorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtor::R`](R) reader structure"]
impl crate::Readable for RtorSpec {}
#[doc = "`write(|w| ..)` method takes [`rtor::W`](W) writer structure"]
impl crate::Writable for RtorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTOR to value 0"]
impl crate::Resettable for RtorSpec {
    const RESET_VALUE: u32 = 0;
}
