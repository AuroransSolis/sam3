#[doc = "Register `DEVDMAADDRESS7` reader"]
pub type R = crate::R<Devdmaaddress7Spec>;
#[doc = "Register `DEVDMAADDRESS7` writer"]
pub type W = crate::W<Devdmaaddress7Spec>;
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
    pub fn buff_add(&mut self) -> BuffAddW<Devdmaaddress7Spec> {
        BuffAddW::new(self, 0)
    }
}
#[doc = "Device DMA Channel Address Register (n = 7)\n\nYou can [`read`](crate::Reg::read) this register and get [`devdmaaddress7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdmaaddress7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devdmaaddress7Spec;
impl crate::RegisterSpec for Devdmaaddress7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devdmaaddress7::R`](R) reader structure"]
impl crate::Readable for Devdmaaddress7Spec {}
#[doc = "`write(|w| ..)` method takes [`devdmaaddress7::W`](W) writer structure"]
impl crate::Writable for Devdmaaddress7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVDMAADDRESS7 to value 0"]
impl crate::Resettable for Devdmaaddress7Spec {
    const RESET_VALUE: u32 = 0;
}
