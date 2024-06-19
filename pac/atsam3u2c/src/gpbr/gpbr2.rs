#[doc = "Register `GPBR2` reader"]
pub type R = crate::R<Gpbr2Spec>;
#[doc = "Register `GPBR2` writer"]
pub type W = crate::W<Gpbr2Spec>;
#[doc = "Field `GPBR_VALUE` reader - Value of GPBR x"]
pub type GpbrValueR = crate::FieldReader<u32>;
#[doc = "Field `GPBR_VALUE` writer - Value of GPBR x"]
pub type GpbrValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value of GPBR x"]
    #[inline(always)]
    pub fn gpbr_value(&self) -> GpbrValueR {
        GpbrValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of GPBR x"]
    #[inline(always)]
    #[must_use]
    pub fn gpbr_value(&mut self) -> GpbrValueW<Gpbr2Spec> {
        GpbrValueW::new(self, 0)
    }
}
#[doc = "General Purpose Backup Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpbr2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpbr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpbr2Spec;
impl crate::RegisterSpec for Gpbr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpbr2::R`](R) reader structure"]
impl crate::Readable for Gpbr2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpbr2::W`](W) writer structure"]
impl crate::Writable for Gpbr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
