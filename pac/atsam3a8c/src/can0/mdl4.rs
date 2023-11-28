#[doc = "Register `MDL4` reader"]
pub type R = crate::R<MDL4_SPEC>;
#[doc = "Register `MDL4` writer"]
pub type W = crate::W<MDL4_SPEC>;
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
    pub fn mdl(&mut self) -> MDL_W<MDL4_SPEC> {
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
#[doc = "Mailbox Data Low Register (MB = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDL4_SPEC;
impl crate::RegisterSpec for MDL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdl4::R`](R) reader structure"]
impl crate::Readable for MDL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdl4::W`](W) writer structure"]
impl crate::Writable for MDL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDL4 to value 0"]
impl crate::Resettable for MDL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
