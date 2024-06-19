#[doc = "Register `DT5` reader"]
pub type R = crate::R<Dt5Spec>;
#[doc = "Register `DT5` writer"]
pub type W = crate::W<Dt5Spec>;
#[doc = "Field `DTH` reader - Dead-Time Value for PWMHx Output"]
pub type DthR = crate::FieldReader<u16>;
#[doc = "Field `DTH` writer - Dead-Time Value for PWMHx Output"]
pub type DthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DTL` reader - Dead-Time Value for PWMLx Output"]
pub type DtlR = crate::FieldReader<u16>;
#[doc = "Field `DTL` writer - Dead-Time Value for PWMLx Output"]
pub type DtlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Dead-Time Value for PWMHx Output"]
    #[inline(always)]
    pub fn dth(&self) -> DthR {
        DthR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Dead-Time Value for PWMLx Output"]
    #[inline(always)]
    pub fn dtl(&self) -> DtlR {
        DtlR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Dead-Time Value for PWMHx Output"]
    #[inline(always)]
    #[must_use]
    pub fn dth(&mut self) -> DthW<Dt5Spec> {
        DthW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Dead-Time Value for PWMLx Output"]
    #[inline(always)]
    #[must_use]
    pub fn dtl(&mut self) -> DtlW<Dt5Spec> {
        DtlW::new(self, 16)
    }
}
#[doc = "PWM Channel Dead Time Register (ch_num = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt5Spec;
impl crate::RegisterSpec for Dt5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt5::R`](R) reader structure"]
impl crate::Readable for Dt5Spec {}
#[doc = "`write(|w| ..)` method takes [`dt5::W`](W) writer structure"]
impl crate::Writable for Dt5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT5 to value 0"]
impl crate::Resettable for Dt5Spec {
    const RESET_VALUE: u32 = 0;
}
