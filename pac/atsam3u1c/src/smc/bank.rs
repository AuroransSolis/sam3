#[doc = "Register `BANK` reader"]
pub type R = crate::R<BankSpec>;
#[doc = "Register `BANK` writer"]
pub type W = crate::W<BankSpec>;
#[doc = "Field `BANK` reader - Bank Identifier"]
pub type BankR = crate::FieldReader;
#[doc = "Field `BANK` writer - Bank Identifier"]
pub type BankW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Bank Identifier"]
    #[inline(always)]
    pub fn bank(&self) -> BankR {
        BankR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bank Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn bank(&mut self) -> BankW<BankSpec> {
        BankW::new(self, 0)
    }
}
#[doc = "SMC Bank Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bank::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bank::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BankSpec;
impl crate::RegisterSpec for BankSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bank::R`](R) reader structure"]
impl crate::Readable for BankSpec {}
#[doc = "`write(|w| ..)` method takes [`bank::W`](W) writer structure"]
impl crate::Writable for BankSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BANK to value 0"]
impl crate::Resettable for BankSpec {
    const RESET_VALUE: u32 = 0;
}
