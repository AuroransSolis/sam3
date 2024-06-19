#[doc = "Register `RC0R` reader"]
pub type R = crate::R<Rc0rSpec>;
#[doc = "Register `RC0R` writer"]
pub type W = crate::W<Rc0rSpec>;
#[doc = "Field `CP0` reader - Receive Compare Data 0"]
pub type Cp0R = crate::FieldReader<u16>;
#[doc = "Field `CP0` writer - Receive Compare Data 0"]
pub type Cp0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Compare Data 0"]
    #[inline(always)]
    pub fn cp0(&self) -> Cp0R {
        Cp0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Compare Data 0"]
    #[inline(always)]
    #[must_use]
    pub fn cp0(&mut self) -> Cp0W<Rc0rSpec> {
        Cp0W::new(self, 0)
    }
}
#[doc = "Receive Compare 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rc0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rc0rSpec;
impl crate::RegisterSpec for Rc0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc0r::R`](R) reader structure"]
impl crate::Readable for Rc0rSpec {}
#[doc = "`write(|w| ..)` method takes [`rc0r::W`](W) writer structure"]
impl crate::Writable for Rc0rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RC0R to value 0"]
impl crate::Resettable for Rc0rSpec {
    const RESET_VALUE: u32 = 0;
}
