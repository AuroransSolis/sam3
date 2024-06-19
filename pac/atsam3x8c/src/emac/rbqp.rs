#[doc = "Register `RBQP` reader"]
pub type R = crate::R<RbqpSpec>;
#[doc = "Register `RBQP` writer"]
pub type W = crate::W<RbqpSpec>;
#[doc = "Field `ADDR` reader - Receive buffer queue pointer address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Receive buffer queue pointer address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Receive buffer queue pointer address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive buffer queue pointer address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<RbqpSpec> {
        AddrW::new(self, 2)
    }
}
#[doc = "Receive Buffer Queue Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rbqp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbqp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbqpSpec;
impl crate::RegisterSpec for RbqpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbqp::R`](R) reader structure"]
impl crate::Readable for RbqpSpec {}
#[doc = "`write(|w| ..)` method takes [`rbqp::W`](W) writer structure"]
impl crate::Writable for RbqpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RBQP to value 0"]
impl crate::Resettable for RbqpSpec {
    const RESET_VALUE: u32 = 0;
}
