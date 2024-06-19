#[doc = "Register `MDL2` reader"]
pub type R = crate::R<Mdl2Spec>;
#[doc = "Register `MDL2` writer"]
pub type W = crate::W<Mdl2Spec>;
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
    pub fn mdl(&mut self) -> MdlW<Mdl2Spec> {
        MdlW::new(self, 0)
    }
}
#[doc = "Mailbox Data Low Register (MB = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`mdl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mdl2Spec;
impl crate::RegisterSpec for Mdl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdl2::R`](R) reader structure"]
impl crate::Readable for Mdl2Spec {}
#[doc = "`write(|w| ..)` method takes [`mdl2::W`](W) writer structure"]
impl crate::Writable for Mdl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDL2 to value 0"]
impl crate::Resettable for Mdl2Spec {
    const RESET_VALUE: u32 = 0;
}
