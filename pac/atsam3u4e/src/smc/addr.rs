#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<AddrSpec>;
#[doc = "Field `ADDR_CYCLE0` reader - NAND Flash Array Address cycle 0"]
pub type AddrCycle0R = crate::FieldReader;
#[doc = "Field `ADDR_CYCLE0` writer - NAND Flash Array Address cycle 0"]
pub type AddrCycle0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NAND Flash Array Address cycle 0"]
    #[inline(always)]
    pub fn addr_cycle0(&self) -> AddrCycle0R {
        AddrCycle0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NAND Flash Array Address cycle 0"]
    #[inline(always)]
    #[must_use]
    pub fn addr_cycle0(&mut self) -> AddrCycle0W<AddrSpec> {
        AddrCycle0W::new(self, 0)
    }
}
#[doc = "SMC NFC Address Cycle Zero Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for AddrSpec {
    const RESET_VALUE: u32 = 0;
}
