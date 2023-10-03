#[doc = "Register `DEVDMAADDRESS5` reader"]
pub type R = crate::R<DEVDMAADDRESS5_SPEC>;
#[doc = "Register `DEVDMAADDRESS5` writer"]
pub type W = crate::W<DEVDMAADDRESS5_SPEC>;
#[doc = "Field `BUFF_ADD` reader - Buffer Address"]
pub type BUFF_ADD_R = crate::FieldReader<u32>;
#[doc = "Field `BUFF_ADD` writer - Buffer Address"]
pub type BUFF_ADD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    pub fn buff_add(&self) -> BUFF_ADD_R {
        BUFF_ADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    #[must_use]
    pub fn buff_add(&mut self) -> BUFF_ADD_W<DEVDMAADDRESS5_SPEC, 0> {
        BUFF_ADD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device DMA Channel Address Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVDMAADDRESS5_SPEC;
impl crate::RegisterSpec for DEVDMAADDRESS5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devdmaaddress5::R`](R) reader structure"]
impl crate::Readable for DEVDMAADDRESS5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devdmaaddress5::W`](W) writer structure"]
impl crate::Writable for DEVDMAADDRESS5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVDMAADDRESS5 to value 0"]
impl crate::Resettable for DEVDMAADDRESS5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
