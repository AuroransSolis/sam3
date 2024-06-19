#[doc = "Register `DSCR4` reader"]
pub type R = crate::R<Dscr4Spec>;
#[doc = "Register `DSCR4` writer"]
pub type W = crate::W<Dscr4Spec>;
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
    pub fn dscr(&mut self) -> DscrW<Dscr4Spec> {
        DscrW::new(self, 2)
    }
}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 4)\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dscr4Spec;
impl crate::RegisterSpec for Dscr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscr4::R`](R) reader structure"]
impl crate::Readable for Dscr4Spec {}
#[doc = "`write(|w| ..)` method takes [`dscr4::W`](W) writer structure"]
impl crate::Writable for Dscr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSCR4 to value 0"]
impl crate::Resettable for Dscr4Spec {
    const RESET_VALUE: u32 = 0;
}
