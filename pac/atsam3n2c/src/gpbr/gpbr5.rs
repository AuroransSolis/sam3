#[doc = "Register `GPBR5` reader"]
pub type R = crate::R<GPBR5_SPEC>;
#[doc = "Register `GPBR5` writer"]
pub type W = crate::W<GPBR5_SPEC>;
#[doc = "Field `GPBR_VALUE` reader - Value of GPBR x"]
pub type GPBR_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `GPBR_VALUE` writer - Value of GPBR x"]
pub type GPBR_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value of GPBR x"]
    #[inline(always)]
    pub fn gpbr_value(&self) -> GPBR_VALUE_R {
        GPBR_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of GPBR x"]
    #[inline(always)]
    #[must_use]
    pub fn gpbr_value(&mut self) -> GPBR_VALUE_W<GPBR5_SPEC> {
        GPBR_VALUE_W::new(self, 0)
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
#[doc = "General Purpose Backup Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpbr5::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpbr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPBR5_SPEC;
impl crate::RegisterSpec for GPBR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpbr5::R`](R) reader structure"]
impl crate::Readable for GPBR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpbr5::W`](W) writer structure"]
impl crate::Writable for GPBR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
