#[doc = "Register `SADDR0` reader"]
pub type R = crate::R<SADDR0_SPEC>;
#[doc = "Register `SADDR0` writer"]
pub type W = crate::W<SADDR0_SPEC>;
#[doc = "Field `SADDR` reader - Channel x Source Address"]
pub type SADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SADDR` writer - Channel x Source Address"]
pub type SADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
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
    pub fn saddr(&mut self) -> SADDR_W<SADDR0_SPEC, 0> {
        SADDR_W::new(self)
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
#[doc = "DMAC Channel Source Address Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SADDR0_SPEC;
impl crate::RegisterSpec for SADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddr0::R`](R) reader structure"]
impl crate::Readable for SADDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`saddr0::W`](W) writer structure"]
impl crate::Writable for SADDR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SADDR0 to value 0"]
impl crate::Resettable for SADDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
