#[doc = "Register `LINIR` reader"]
pub type R = crate::R<LinirSpec>;
#[doc = "Register `LINIR` writer"]
pub type W = crate::W<LinirSpec>;
#[doc = "Field `IDCHR` reader - Identifier Character"]
pub type IdchrR = crate::FieldReader;
#[doc = "Field `IDCHR` writer - Identifier Character"]
pub type IdchrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Identifier Character"]
    #[inline(always)]
    pub fn idchr(&self) -> IdchrR {
        IdchrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Identifier Character"]
    #[inline(always)]
    #[must_use]
    pub fn idchr(&mut self) -> IdchrW<LinirSpec> {
        IdchrW::new(self, 0)
    }
}
#[doc = "LIN Identifier Register\n\nYou can [`read`](crate::Reg::read) this register and get [`linir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinirSpec;
impl crate::RegisterSpec for LinirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linir::R`](R) reader structure"]
impl crate::Readable for LinirSpec {}
#[doc = "`write(|w| ..)` method takes [`linir::W`](W) writer structure"]
impl crate::Writable for LinirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINIR to value 0"]
impl crate::Resettable for LinirSpec {
    const RESET_VALUE: u32 = 0;
}
