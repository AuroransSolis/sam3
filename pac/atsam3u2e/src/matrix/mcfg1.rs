#[doc = "Register `MCFG1` reader"]
pub type R = crate::R<Mcfg1Spec>;
#[doc = "Register `MCFG1` writer"]
pub type W = crate::W<Mcfg1Spec>;
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub type UlbtR = crate::FieldReader;
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub type UlbtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> UlbtR {
        UlbtR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    #[must_use]
    pub fn ulbt(&mut self) -> UlbtW<Mcfg1Spec> {
        UlbtW::new(self, 0)
    }
}
#[doc = "Master Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcfg1::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcfg1Spec;
impl crate::RegisterSpec for Mcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcfg1::R`](R) reader structure"]
impl crate::Readable for Mcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcfg1::W`](W) writer structure"]
impl crate::Writable for Mcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
