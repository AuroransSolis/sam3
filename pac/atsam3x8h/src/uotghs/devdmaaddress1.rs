#[doc = "Register `DEVDMAADDRESS1` reader"]
pub type R = crate::R<DEVDMAADDRESS1_SPEC>;
#[doc = "Register `DEVDMAADDRESS1` writer"]
pub type W = crate::W<DEVDMAADDRESS1_SPEC>;
#[doc = "Field `BUFF_ADD` reader - Buffer Address"]
pub type BUFF_ADD_R = crate::FieldReader<u32>;
#[doc = "Field `BUFF_ADD` writer - Buffer Address"]
pub type BUFF_ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
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
    pub fn buff_add(&mut self) -> BUFF_ADD_W<DEVDMAADDRESS1_SPEC> {
        BUFF_ADD_W::new(self, 0)
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
#[doc = "Device DMA Channel Address Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVDMAADDRESS1_SPEC;
impl crate::RegisterSpec for DEVDMAADDRESS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devdmaaddress1::R`](R) reader structure"]
impl crate::Readable for DEVDMAADDRESS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devdmaaddress1::W`](W) writer structure"]
impl crate::Writable for DEVDMAADDRESS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVDMAADDRESS1 to value 0"]
impl crate::Resettable for DEVDMAADDRESS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
