#[doc = "Register `DADDR4` reader"]
pub type R = crate::R<DADDR4_SPEC>;
#[doc = "Register `DADDR4` writer"]
pub type W = crate::W<DADDR4_SPEC>;
#[doc = "Field `DADDR` reader - Channel x Destination Address"]
pub type DADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DADDR` writer - Channel x Destination Address"]
pub type DADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    pub fn daddr(&self) -> DADDR_R {
        DADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    #[must_use]
    pub fn daddr(&mut self) -> DADDR_W<DADDR4_SPEC, 0> {
        DADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMAC Channel Destination Address Register (ch_num = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DADDR4_SPEC;
impl crate::RegisterSpec for DADDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daddr4::R`](R) reader structure"]
impl crate::Readable for DADDR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`daddr4::W`](W) writer structure"]
impl crate::Writable for DADDR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DADDR4 to value 0"]
impl crate::Resettable for DADDR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
