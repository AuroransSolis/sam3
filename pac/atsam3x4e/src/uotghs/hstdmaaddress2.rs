#[doc = "Register `HSTDMAADDRESS2` reader"]
pub type R = crate::R<HSTDMAADDRESS2_SPEC>;
#[doc = "Register `HSTDMAADDRESS2` writer"]
pub type W = crate::W<HSTDMAADDRESS2_SPEC>;
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
    pub fn buff_add(&mut self) -> BUFF_ADD_W<HSTDMAADDRESS2_SPEC> {
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
#[doc = "Host DMA Channel Address Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTDMAADDRESS2_SPEC;
impl crate::RegisterSpec for HSTDMAADDRESS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstdmaaddress2::R`](R) reader structure"]
impl crate::Readable for HSTDMAADDRESS2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstdmaaddress2::W`](W) writer structure"]
impl crate::Writable for HSTDMAADDRESS2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSTDMAADDRESS2 to value 0"]
impl crate::Resettable for HSTDMAADDRESS2_SPEC {
    const RESET_VALUE: u32 = 0;
}
