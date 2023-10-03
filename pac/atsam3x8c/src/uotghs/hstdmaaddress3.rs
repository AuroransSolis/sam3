#[doc = "Register `HSTDMAADDRESS3` reader"]
pub type R = crate::R<HSTDMAADDRESS3_SPEC>;
#[doc = "Register `HSTDMAADDRESS3` writer"]
pub type W = crate::W<HSTDMAADDRESS3_SPEC>;
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
    pub fn buff_add(&mut self) -> BUFF_ADD_W<HSTDMAADDRESS3_SPEC, 0> {
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
#[doc = "Host DMA Channel Address Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTDMAADDRESS3_SPEC;
impl crate::RegisterSpec for HSTDMAADDRESS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstdmaaddress3::R`](R) reader structure"]
impl crate::Readable for HSTDMAADDRESS3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstdmaaddress3::W`](W) writer structure"]
impl crate::Writable for HSTDMAADDRESS3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTDMAADDRESS3 to value 0"]
impl crate::Resettable for HSTDMAADDRESS3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
