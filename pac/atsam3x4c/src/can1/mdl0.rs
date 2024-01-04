#[doc = "Register `MDL0` reader"]
pub type R = crate::R<MDL0_SPEC>;
#[doc = "Register `MDL0` writer"]
pub type W = crate::W<MDL0_SPEC>;
#[doc = "Field `MDL` reader - Message Data Low Value"]
pub type MDL_R = crate::FieldReader<u32>;
#[doc = "Field `MDL` writer - Message Data Low Value"]
pub type MDL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Data Low Value"]
    #[inline(always)]
    pub fn mdl(&self) -> MDL_R {
        MDL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Data Low Value"]
    #[inline(always)]
    #[must_use]
    pub fn mdl(&mut self) -> MDL_W<MDL0_SPEC> {
        MDL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Mailbox Data Low Register (MB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDL0_SPEC;
impl crate::RegisterSpec for MDL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdl0::R`](R) reader structure"]
impl crate::Readable for MDL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdl0::W`](W) writer structure"]
impl crate::Writable for MDL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDL0 to value 0"]
impl crate::Resettable for MDL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
