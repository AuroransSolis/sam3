#[doc = "Register `DSCR3` reader"]
pub type R = crate::R<Dscr3Spec>;
#[doc = "Register `DSCR3` writer"]
pub type W = crate::W<Dscr3Spec>;
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
    pub fn dscr(&mut self) -> DscrW<Dscr3Spec> {
        DscrW::new(self, 2)
    }
}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dscr3Spec;
impl crate::RegisterSpec for Dscr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscr3::R`](R) reader structure"]
impl crate::Readable for Dscr3Spec {}
#[doc = "`write(|w| ..)` method takes [`dscr3::W`](W) writer structure"]
impl crate::Writable for Dscr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSCR3 to value 0"]
impl crate::Resettable for Dscr3Spec {
    const RESET_VALUE: u32 = 0;
}
