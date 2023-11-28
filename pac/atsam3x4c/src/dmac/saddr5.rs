#[doc = "Register `SADDR5` reader"]
pub type R = crate::R<SADDR5_SPEC>;
#[doc = "Register `SADDR5` writer"]
pub type W = crate::W<SADDR5_SPEC>;
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
    pub fn saddr(&mut self) -> SADDR_W<SADDR5_SPEC> {
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
#[doc = "DMAC Channel Source Address Register (ch_num = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SADDR5_SPEC;
impl crate::RegisterSpec for SADDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddr5::R`](R) reader structure"]
impl crate::Readable for SADDR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`saddr5::W`](W) writer structure"]
impl crate::Writable for SADDR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SADDR5 to value 0"]
impl crate::Resettable for SADDR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
