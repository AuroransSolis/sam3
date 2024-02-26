#[doc = "Register `DEVDMAADDRESS5` reader"]
pub type R = crate::R<Devdmaaddress5Spec>;
#[doc = "Register `DEVDMAADDRESS5` writer"]
pub type W = crate::W<Devdmaaddress5Spec>;
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
    pub fn buff_add(&mut self) -> BuffAddW<Devdmaaddress5Spec> {
        BuffAddW::new(self, 0)
    }
}
#[doc = "Device DMA Channel Address Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devdmaaddress5Spec;
impl crate::RegisterSpec for Devdmaaddress5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devdmaaddress5::R`](R) reader structure"]
impl crate::Readable for Devdmaaddress5Spec {}
#[doc = "`write(|w| ..)` method takes [`devdmaaddress5::W`](W) writer structure"]
impl crate::Writable for Devdmaaddress5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVDMAADDRESS5 to value 0"]
impl crate::Resettable for Devdmaaddress5Spec {
    const RESET_VALUE: u32 = 0;
}
