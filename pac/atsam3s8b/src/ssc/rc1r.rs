#[doc = "Register `RC1R` reader"]
pub type R = crate::R<Rc1rSpec>;
#[doc = "Register `RC1R` writer"]
pub type W = crate::W<Rc1rSpec>;
#[doc = "Field `CP1` reader - Receive Compare Data 1"]
pub type Cp1R = crate::FieldReader<u16>;
#[doc = "Field `CP1` writer - Receive Compare Data 1"]
pub type Cp1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Compare Data 1"]
    #[inline(always)]
    pub fn cp1(&self) -> Cp1R {
        Cp1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Compare Data 1"]
    #[inline(always)]
    #[must_use]
    pub fn cp1(&mut self) -> Cp1W<Rc1rSpec> {
        Cp1W::new(self, 0)
    }
}
#[doc = "Receive Compare 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rc1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rc1rSpec;
impl crate::RegisterSpec for Rc1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc1r::R`](R) reader structure"]
impl crate::Readable for Rc1rSpec {}
#[doc = "`write(|w| ..)` method takes [`rc1r::W`](W) writer structure"]
impl crate::Writable for Rc1rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RC1R to value 0"]
impl crate::Resettable for Rc1rSpec {
    const RESET_VALUE: u32 = 0;
}
