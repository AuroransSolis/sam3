#[doc = "Register `MCFG1` reader"]
pub type R = crate::R<MCFG1_SPEC>;
#[doc = "Register `MCFG1` writer"]
pub type W = crate::W<MCFG1_SPEC>;
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub type ULBT_R = crate::FieldReader;
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub type ULBT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> ULBT_R {
        ULBT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    #[must_use]
    pub fn ulbt(&mut self) -> ULBT_W<MCFG1_SPEC> {
        ULBT_W::new(self, 0)
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
#[doc = "Master Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcfg1::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCFG1_SPEC;
impl crate::RegisterSpec for MCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcfg1::R`](R) reader structure"]
impl crate::Readable for MCFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcfg1::W`](W) writer structure"]
impl crate::Writable for MCFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
