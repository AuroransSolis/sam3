#[doc = "Register `DMAADDRESS2` reader"]
pub type R = crate::R<Dmaaddress2Spec>;
#[doc = "Register `DMAADDRESS2` writer"]
pub type W = crate::W<Dmaaddress2Spec>;
#[doc = "Field `BUFF_ADD` reader - Buffer Address"]
pub type BuffAddR = crate::FieldReader<u32>;
#[doc = "Field `BUFF_ADD` writer - Buffer Address"]
pub type BuffAddW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    pub fn buff_add(&self) -> BuffAddR {
        BuffAddR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    #[must_use]
    pub fn buff_add(&mut self) -> BuffAddW<Dmaaddress2Spec> {
        BuffAddW::new(self, 0)
    }
}
#[doc = "UDPHS DMA Channel Address Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaaddress2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaaddress2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmaaddress2Spec;
impl crate::RegisterSpec for Dmaaddress2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaaddress2::R`](R) reader structure"]
impl crate::Readable for Dmaaddress2Spec {}
#[doc = "`write(|w| ..)` method takes [`dmaaddress2::W`](W) writer structure"]
impl crate::Writable for Dmaaddress2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAADDRESS2 to value 0"]
impl crate::Resettable for Dmaaddress2Spec {
    const RESET_VALUE: u32 = 0;
}
