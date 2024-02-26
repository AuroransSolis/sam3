#[doc = "Register `MDL6` reader"]
pub type R = crate::R<Mdl6Spec>;
#[doc = "Register `MDL6` writer"]
pub type W = crate::W<Mdl6Spec>;
#[doc = "Field `MDL` reader - Message Data Low Value"]
pub type MdlR = crate::FieldReader<u32>;
#[doc = "Field `MDL` writer - Message Data Low Value"]
pub type MdlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Data Low Value"]
    #[inline(always)]
    pub fn mdl(&self) -> MdlR {
        MdlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Data Low Value"]
    #[inline(always)]
    #[must_use]
    pub fn mdl(&mut self) -> MdlW<Mdl6Spec> {
        MdlW::new(self, 0)
    }
}
#[doc = "Mailbox Data Low Register (MB = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mdl6Spec;
impl crate::RegisterSpec for Mdl6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdl6::R`](R) reader structure"]
impl crate::Readable for Mdl6Spec {}
#[doc = "`write(|w| ..)` method takes [`mdl6::W`](W) writer structure"]
impl crate::Writable for Mdl6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDL6 to value 0"]
impl crate::Resettable for Mdl6Spec {
    const RESET_VALUE: u32 = 0;
}
