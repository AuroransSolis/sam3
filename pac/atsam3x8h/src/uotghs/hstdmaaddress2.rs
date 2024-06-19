#[doc = "Register `HSTDMAADDRESS2` reader"]
pub type R = crate::R<Hstdmaaddress2Spec>;
#[doc = "Register `HSTDMAADDRESS2` writer"]
pub type W = crate::W<Hstdmaaddress2Spec>;
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
    pub fn buff_add(&mut self) -> BuffAddW<Hstdmaaddress2Spec> {
        BuffAddW::new(self, 0)
    }
}
#[doc = "Host DMA Channel Address Register (n = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`hstdmaaddress2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstdmaaddress2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hstdmaaddress2Spec;
impl crate::RegisterSpec for Hstdmaaddress2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstdmaaddress2::R`](R) reader structure"]
impl crate::Readable for Hstdmaaddress2Spec {}
#[doc = "`write(|w| ..)` method takes [`hstdmaaddress2::W`](W) writer structure"]
impl crate::Writable for Hstdmaaddress2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSTDMAADDRESS2 to value 0"]
impl crate::Resettable for Hstdmaaddress2Spec {
    const RESET_VALUE: u32 = 0;
}
