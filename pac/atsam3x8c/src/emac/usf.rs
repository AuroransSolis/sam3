#[doc = "Register `USF` reader"]
pub type R = crate::R<UsfSpec>;
#[doc = "Register `USF` writer"]
pub type W = crate::W<UsfSpec>;
#[doc = "Field `USF` reader - Undersize frames"]
pub type UsfR = crate::FieldReader;
#[doc = "Field `USF` writer - Undersize frames"]
pub type UsfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Undersize frames"]
    #[inline(always)]
    pub fn usf(&self) -> UsfR {
        UsfR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Undersize frames"]
    #[inline(always)]
    #[must_use]
    pub fn usf(&mut self) -> UsfW<UsfSpec> {
        UsfW::new(self, 0)
    }
}
#[doc = "Undersize Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsfSpec;
impl crate::RegisterSpec for UsfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usf::R`](R) reader structure"]
impl crate::Readable for UsfSpec {}
#[doc = "`write(|w| ..)` method takes [`usf::W`](W) writer structure"]
impl crate::Writable for UsfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USF to value 0"]
impl crate::Resettable for UsfSpec {
    const RESET_VALUE: u32 = 0;
}
