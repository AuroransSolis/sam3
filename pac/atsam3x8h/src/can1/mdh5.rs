#[doc = "Register `MDH5` reader"]
pub type R = crate::R<Mdh5Spec>;
#[doc = "Register `MDH5` writer"]
pub type W = crate::W<Mdh5Spec>;
#[doc = "Field `MDH` reader - Message Data High Value"]
pub type MdhR = crate::FieldReader<u32>;
#[doc = "Field `MDH` writer - Message Data High Value"]
pub type MdhW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Data High Value"]
    #[inline(always)]
    pub fn mdh(&self) -> MdhR {
        MdhR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Data High Value"]
    #[inline(always)]
    #[must_use]
    pub fn mdh(&mut self) -> MdhW<Mdh5Spec> {
        MdhW::new(self, 0)
    }
}
#[doc = "Mailbox Data High Register (MB = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdh5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdh5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mdh5Spec;
impl crate::RegisterSpec for Mdh5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdh5::R`](R) reader structure"]
impl crate::Readable for Mdh5Spec {}
#[doc = "`write(|w| ..)` method takes [`mdh5::W`](W) writer structure"]
impl crate::Writable for Mdh5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDH5 to value 0"]
impl crate::Resettable for Mdh5Spec {
    const RESET_VALUE: u32 = 0;
}
