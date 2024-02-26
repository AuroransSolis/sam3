#[doc = "Register `DSCR1` reader"]
pub type R = crate::R<Dscr1Spec>;
#[doc = "Register `DSCR1` writer"]
pub type W = crate::W<Dscr1Spec>;
#[doc = "Field `DSCR` reader - Buffer Transfer Descriptor Address"]
pub type DscrR = crate::FieldReader<u32>;
#[doc = "Field `DSCR` writer - Buffer Transfer Descriptor Address"]
pub type DscrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Buffer Transfer Descriptor Address"]
    #[inline(always)]
    pub fn dscr(&self) -> DscrR {
        DscrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Buffer Transfer Descriptor Address"]
    #[inline(always)]
    #[must_use]
    pub fn dscr(&mut self) -> DscrW<Dscr1Spec> {
        DscrW::new(self, 2)
    }
}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dscr1Spec;
impl crate::RegisterSpec for Dscr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscr1::R`](R) reader structure"]
impl crate::Readable for Dscr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dscr1::W`](W) writer structure"]
impl crate::Writable for Dscr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSCR1 to value 0"]
impl crate::Resettable for Dscr1Spec {
    const RESET_VALUE: u32 = 0;
}
