#[doc = "Register `HSTDMAADDRESS1` reader"]
pub type R = crate::R<Hstdmaaddress1Spec>;
#[doc = "Register `HSTDMAADDRESS1` writer"]
pub type W = crate::W<Hstdmaaddress1Spec>;
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
    pub fn buff_add(&mut self) -> BuffAddW<Hstdmaaddress1Spec> {
        BuffAddW::new(self, 0)
    }
}
#[doc = "Host DMA Channel Address Register (n = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmaaddress1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmaaddress1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hstdmaaddress1Spec;
impl crate::RegisterSpec for Hstdmaaddress1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstdmaaddress1::R`](R) reader structure"]
impl crate::Readable for Hstdmaaddress1Spec {}
#[doc = "`write(|w| ..)` method takes [`hstdmaaddress1::W`](W) writer structure"]
impl crate::Writable for Hstdmaaddress1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSTDMAADDRESS1 to value 0"]
impl crate::Resettable for Hstdmaaddress1Spec {
    const RESET_VALUE: u32 = 0;
}
