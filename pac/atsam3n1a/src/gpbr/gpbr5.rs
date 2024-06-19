#[doc = "Register `GPBR5` reader"]
pub type R = crate::R<Gpbr5Spec>;
#[doc = "Register `GPBR5` writer"]
pub type W = crate::W<Gpbr5Spec>;
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
    pub fn gpbr_value(&mut self) -> GpbrValueW<Gpbr5Spec> {
        GpbrValueW::new(self, 0)
    }
}
#[doc = "General Purpose Backup Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpbr5::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpbr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpbr5Spec;
impl crate::RegisterSpec for Gpbr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpbr5::R`](R) reader structure"]
impl crate::Readable for Gpbr5Spec {}
#[doc = "`write(|w| ..)` method takes [`gpbr5::W`](W) writer structure"]
impl crate::Writable for Gpbr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
