#[doc = "Register `FADDR` reader"]
pub type R = crate::R<FaddrSpec>;
#[doc = "Register `FADDR` writer"]
pub type W = crate::W<FaddrSpec>;
#[doc = "Field `FADD` reader - Function Address Value"]
pub type FaddR = crate::FieldReader;
#[doc = "Field `FADD` writer - Function Address Value"]
pub type FaddW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FEN` reader - Function Enable"]
pub type FenR = crate::BitReader;
#[doc = "Field `FEN` writer - Function Enable"]
pub type FenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Function Address Value"]
    #[inline(always)]
    pub fn fadd(&self) -> FaddR {
        FaddR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Function Enable"]
    #[inline(always)]
    pub fn fen(&self) -> FenR {
        FenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Function Address Value"]
    #[inline(always)]
    #[must_use]
    pub fn fadd(&mut self) -> FaddW<FaddrSpec> {
        FaddW::new(self, 0)
    }
    #[doc = "Bit 8 - Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FenW<FaddrSpec> {
        FenW::new(self, 8)
    }
}
#[doc = "Function Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`faddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaddrSpec;
impl crate::RegisterSpec for FaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`faddr::R`](R) reader structure"]
impl crate::Readable for FaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`faddr::W`](W) writer structure"]
impl crate::Writable for FaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FADDR to value 0x0100"]
impl crate::Resettable for FaddrSpec {
    const RESET_VALUE: u32 = 0x0100;
}
