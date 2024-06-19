#[doc = "Register `OCMS` reader"]
pub type R = crate::R<OcmsSpec>;
#[doc = "Register `OCMS` writer"]
pub type W = crate::W<OcmsSpec>;
#[doc = "Field `SMSE` reader - Static Memory Controller Scrambling Enable"]
pub type SmseR = crate::BitReader;
#[doc = "Field `SMSE` writer - Static Memory Controller Scrambling Enable"]
pub type SmseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS0SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type Cs0seR = crate::BitReader;
#[doc = "Field `CS0SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type Cs0seW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS1SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type Cs1seR = crate::BitReader;
#[doc = "Field `CS1SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type Cs1seW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS2SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type Cs2seR = crate::BitReader;
#[doc = "Field `CS2SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type Cs2seW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS3SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type Cs3seR = crate::BitReader;
#[doc = "Field `CS3SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type Cs3seW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn smse(&self) -> SmseR {
        SmseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs0se(&self) -> Cs0seR {
        Cs0seR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs1se(&self) -> Cs1seR {
        Cs1seR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs2se(&self) -> Cs2seR {
        Cs2seR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs3se(&self) -> Cs3seR {
        Cs3seR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smse(&mut self) -> SmseW<OcmsSpec> {
        SmseW::new(self, 0)
    }
    #[doc = "Bit 16 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs0se(&mut self) -> Cs0seW<OcmsSpec> {
        Cs0seW::new(self, 16)
    }
    #[doc = "Bit 17 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs1se(&mut self) -> Cs1seW<OcmsSpec> {
        Cs1seW::new(self, 17)
    }
    #[doc = "Bit 18 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs2se(&mut self) -> Cs2seW<OcmsSpec> {
        Cs2seW::new(self, 18)
    }
    #[doc = "Bit 19 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs3se(&mut self) -> Cs3seW<OcmsSpec> {
        Cs3seW::new(self, 19)
    }
}
#[doc = "SMC OCMS MODE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ocms::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocms::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcmsSpec;
impl crate::RegisterSpec for OcmsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ocms::R`](R) reader structure"]
impl crate::Readable for OcmsSpec {}
#[doc = "`write(|w| ..)` method takes [`ocms::W`](W) writer structure"]
impl crate::Writable for OcmsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OCMS to value 0"]
impl crate::Resettable for OcmsSpec {
    const RESET_VALUE: u32 = 0;
}
