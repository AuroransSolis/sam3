#[doc = "Register `CUPD0` writer"]
pub type W = crate::W<Cupd0Spec>;
#[doc = "Field `CUPD` writer - "]
pub type CupdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cupd(&mut self) -> CupdW<Cupd0Spec> {
        CupdW::new(self, 0)
    }
}
#[doc = "PWM Channel Update Register (ch_num = 0)\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cupd0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cupd0Spec;
impl crate::RegisterSpec for Cupd0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cupd0::W`](W) writer structure"]
impl crate::Writable for Cupd0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
