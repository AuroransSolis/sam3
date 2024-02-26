#[doc = "Register `TBQP` reader"]
pub type R = crate::R<TbqpSpec>;
#[doc = "Register `TBQP` writer"]
pub type W = crate::W<TbqpSpec>;
#[doc = "Field `ADDR` reader - Transmit buffer queue pointer address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Transmit buffer queue pointer address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Transmit buffer queue pointer address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit buffer queue pointer address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<TbqpSpec> {
        AddrW::new(self, 2)
    }
}
#[doc = "Transmit Buffer Queue Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbqp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbqp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbqpSpec;
impl crate::RegisterSpec for TbqpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbqp::R`](R) reader structure"]
impl crate::Readable for TbqpSpec {}
#[doc = "`write(|w| ..)` method takes [`tbqp::W`](W) writer structure"]
impl crate::Writable for TbqpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBQP to value 0"]
impl crate::Resettable for TbqpSpec {
    const RESET_VALUE: u32 = 0;
}
