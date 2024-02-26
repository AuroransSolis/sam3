#[doc = "Register `MDH3` reader"]
pub type R = crate::R<Mdh3Spec>;
#[doc = "Register `MDH3` writer"]
pub type W = crate::W<Mdh3Spec>;
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
    pub fn mdh(&mut self) -> MdhW<Mdh3Spec> {
        MdhW::new(self, 0)
    }
}
#[doc = "Mailbox Data High Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mdh3Spec;
impl crate::RegisterSpec for Mdh3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdh3::R`](R) reader structure"]
impl crate::Readable for Mdh3Spec {}
#[doc = "`write(|w| ..)` method takes [`mdh3::W`](W) writer structure"]
impl crate::Writable for Mdh3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDH3 to value 0"]
impl crate::Resettable for Mdh3Spec {
    const RESET_VALUE: u32 = 0;
}
