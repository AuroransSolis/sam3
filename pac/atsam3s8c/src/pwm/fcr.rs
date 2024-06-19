#[doc = "Register `FCR` writer"]
pub type W = crate::W<FcrSpec>;
#[doc = "Field `FCLR` writer - Fault Clear (fault input bit varies from 0 to 5)"]
pub type FclrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Fault Clear (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fclr(&mut self) -> FclrW<FcrSpec> {
        FclrW::new(self, 0)
    }
}
#[doc = "PWM Fault Clear Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
