#[doc = "Register `ARGR` reader"]
pub type R = crate::R<ArgrSpec>;
#[doc = "Register `ARGR` writer"]
pub type W = crate::W<ArgrSpec>;
#[doc = "Field `ARG` reader - Command Argument"]
pub type ArgR = crate::FieldReader<u32>;
#[doc = "Field `ARG` writer - Command Argument"]
pub type ArgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn arg(&self) -> ArgR {
        ArgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    #[must_use]
    pub fn arg(&mut self) -> ArgW<ArgrSpec> {
        ArgW::new(self, 0)
    }
}
#[doc = "Argument Register\n\nYou can [`read`](crate::Reg::read) this register and get [`argr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArgrSpec;
impl crate::RegisterSpec for ArgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`argr::R`](R) reader structure"]
impl crate::Readable for ArgrSpec {}
#[doc = "`write(|w| ..)` method takes [`argr::W`](W) writer structure"]
impl crate::Writable for ArgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARGR to value 0"]
impl crate::Resettable for ArgrSpec {
    const RESET_VALUE: u32 = 0;
}
