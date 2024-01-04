#[doc = "Register `MDH7` reader"]
pub type R = crate::R<MDH7_SPEC>;
#[doc = "Register `MDH7` writer"]
pub type W = crate::W<MDH7_SPEC>;
#[doc = "Field `MDH` reader - Message Data High Value"]
pub type MDH_R = crate::FieldReader<u32>;
#[doc = "Field `MDH` writer - Message Data High Value"]
pub type MDH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Data High Value"]
    #[inline(always)]
    pub fn mdh(&self) -> MDH_R {
        MDH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Data High Value"]
    #[inline(always)]
    #[must_use]
    pub fn mdh(&mut self) -> MDH_W<MDH7_SPEC> {
        MDH_W::new(self, 0)
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
#[doc = "Mailbox Data High Register (MB = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdh7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdh7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDH7_SPEC;
impl crate::RegisterSpec for MDH7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdh7::R`](R) reader structure"]
impl crate::Readable for MDH7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdh7::W`](W) writer structure"]
impl crate::Writable for MDH7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDH7 to value 0"]
impl crate::Resettable for MDH7_SPEC {
    const RESET_VALUE: u32 = 0;
}
