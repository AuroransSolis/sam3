#[doc = "Register `SMR` reader"]
pub type R = crate::R<SmrSpec>;
#[doc = "Register `SMR` writer"]
pub type W = crate::W<SmrSpec>;
#[doc = "Field `SADR` reader - Slave Address"]
pub type SadrR = crate::FieldReader;
#[doc = "Field `SADR` writer - Slave Address"]
pub type SadrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&self) -> SadrR {
        SadrR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn sadr(&mut self) -> SadrW<SmrSpec> {
        SadrW::new(self, 16)
    }
}
#[doc = "Slave Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmrSpec;
impl crate::RegisterSpec for SmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smr::R`](R) reader structure"]
impl crate::Readable for SmrSpec {}
#[doc = "`write(|w| ..)` method takes [`smr::W`](W) writer structure"]
impl crate::Writable for SmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMR to value 0"]
impl crate::Resettable for SmrSpec {
    const RESET_VALUE: u32 = 0;
}
