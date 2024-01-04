#[doc = "Register `SADDR4` reader"]
pub type R = crate::R<SADDR4_SPEC>;
#[doc = "Register `SADDR4` writer"]
pub type W = crate::W<SADDR4_SPEC>;
#[doc = "Field `SADDR` reader - Channel x Source Address"]
pub type SADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SADDR` writer - Channel x Source Address"]
pub type SADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    #[must_use]
    pub fn saddr(&mut self) -> SADDR_W<SADDR4_SPEC> {
        SADDR_W::new(self, 0)
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
#[doc = "DMAC Channel Source Address Register (ch_num = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SADDR4_SPEC;
impl crate::RegisterSpec for SADDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddr4::R`](R) reader structure"]
impl crate::Readable for SADDR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`saddr4::W`](W) writer structure"]
impl crate::Writable for SADDR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SADDR4 to value 0"]
impl crate::Resettable for SADDR4_SPEC {
    const RESET_VALUE: u32 = 0;
}
