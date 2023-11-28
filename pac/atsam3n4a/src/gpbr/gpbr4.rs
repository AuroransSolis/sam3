#[doc = "Register `GPBR4` reader"]
pub type R = crate::R<GPBR4_SPEC>;
#[doc = "Register `GPBR4` writer"]
pub type W = crate::W<GPBR4_SPEC>;
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
    pub fn gpbr_value(&mut self) -> GPBR_VALUE_W<GPBR4_SPEC> {
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
#[doc = "General Purpose Backup Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpbr4::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpbr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPBR4_SPEC;
impl crate::RegisterSpec for GPBR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpbr4::R`](R) reader structure"]
impl crate::Readable for GPBR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpbr4::W`](W) writer structure"]
impl crate::Writable for GPBR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
