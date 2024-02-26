#[doc = "Register `MDL0` reader"]
pub type R = crate::R<Mdl0Spec>;
#[doc = "Register `MDL0` writer"]
pub type W = crate::W<Mdl0Spec>;
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
    pub fn mdl(&mut self) -> MdlW<Mdl0Spec> {
        MdlW::new(self, 0)
    }
}
#[doc = "Mailbox Data Low Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mdl0Spec;
impl crate::RegisterSpec for Mdl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdl0::R`](R) reader structure"]
impl crate::Readable for Mdl0Spec {}
#[doc = "`write(|w| ..)` method takes [`mdl0::W`](W) writer structure"]
impl crate::Writable for Mdl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDL0 to value 0"]
impl crate::Resettable for Mdl0Spec {
    const RESET_VALUE: u32 = 0;
}
