#[doc = "Register `TIMINGS1` reader"]
pub type R = crate::R<Timings1Spec>;
#[doc = "Register `TIMINGS1` writer"]
pub type W = crate::W<Timings1Spec>;
#[doc = "Field `TCLR` reader - CLE to REN Low Delay"]
pub type TclrR = crate::FieldReader;
#[doc = "Field `TCLR` writer - CLE to REN Low Delay"]
pub type TclrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TADL` reader - ALE to Data Start"]
pub type TadlR = crate::FieldReader;
#[doc = "Field `TADL` writer - ALE to Data Start"]
pub type TadlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TAR` reader - ALE to REN Low Delay"]
pub type TarR = crate::FieldReader;
#[doc = "Field `TAR` writer - ALE to REN Low Delay"]
pub type TarW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OCMS` reader - Off Chip Memory Scrambling Enable"]
pub type OcmsR = crate::BitReader;
#[doc = "Field `OCMS` writer - Off Chip Memory Scrambling Enable"]
pub type OcmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRR` reader - Ready to REN Low Delay"]
pub type TrrR = crate::FieldReader;
#[doc = "Field `TRR` writer - Ready to REN Low Delay"]
pub type TrrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TWB` reader - WEN High to REN to Busy"]
pub type TwbR = crate::FieldReader;
#[doc = "Field `TWB` writer - WEN High to REN to Busy"]
pub type TwbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RBNSEL` reader - Ready/Busy Line Selection"]
pub type RbnselR = crate::FieldReader;
#[doc = "Field `RBNSEL` writer - Ready/Busy Line Selection"]
pub type RbnselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NFSEL` reader - NAND Flash Selection"]
pub type NfselR = crate::BitReader;
#[doc = "Field `NFSEL` writer - NAND Flash Selection"]
pub type NfselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - CLE to REN Low Delay"]
    #[inline(always)]
    pub fn tclr(&self) -> TclrR {
        TclrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ALE to Data Start"]
    #[inline(always)]
    pub fn tadl(&self) -> TadlR {
        TadlR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ALE to REN Low Delay"]
    #[inline(always)]
    pub fn tar(&self) -> TarR {
        TarR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Off Chip Memory Scrambling Enable"]
    #[inline(always)]
    pub fn ocms(&self) -> OcmsR {
        OcmsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Ready to REN Low Delay"]
    #[inline(always)]
    pub fn trr(&self) -> TrrR {
        TrrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - WEN High to REN to Busy"]
    #[inline(always)]
    pub fn twb(&self) -> TwbR {
        TwbR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Ready/Busy Line Selection"]
    #[inline(always)]
    pub fn rbnsel(&self) -> RbnselR {
        RbnselR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - NAND Flash Selection"]
    #[inline(always)]
    pub fn nfsel(&self) -> NfselR {
        NfselR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - CLE to REN Low Delay"]
    #[inline(always)]
    #[must_use]
    pub fn tclr(&mut self) -> TclrW<Timings1Spec> {
        TclrW::new(self, 0)
    }
    #[doc = "Bits 4:7 - ALE to Data Start"]
    #[inline(always)]
    #[must_use]
    pub fn tadl(&mut self) -> TadlW<Timings1Spec> {
        TadlW::new(self, 4)
    }
    #[doc = "Bits 8:11 - ALE to REN Low Delay"]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TarW<Timings1Spec> {
        TarW::new(self, 8)
    }
    #[doc = "Bit 12 - Off Chip Memory Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocms(&mut self) -> OcmsW<Timings1Spec> {
        OcmsW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Ready to REN Low Delay"]
    #[inline(always)]
    #[must_use]
    pub fn trr(&mut self) -> TrrW<Timings1Spec> {
        TrrW::new(self, 16)
    }
    #[doc = "Bits 24:27 - WEN High to REN to Busy"]
    #[inline(always)]
    #[must_use]
    pub fn twb(&mut self) -> TwbW<Timings1Spec> {
        TwbW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Ready/Busy Line Selection"]
    #[inline(always)]
    #[must_use]
    pub fn rbnsel(&mut self) -> RbnselW<Timings1Spec> {
        RbnselW::new(self, 28)
    }
    #[doc = "Bit 31 - NAND Flash Selection"]
    #[inline(always)]
    #[must_use]
    pub fn nfsel(&mut self) -> NfselW<Timings1Spec> {
        NfselW::new(self, 31)
    }
}
#[doc = "SMC Timings Register (CS_number = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timings1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timings1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timings1Spec;
impl crate::RegisterSpec for Timings1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timings1::R`](R) reader structure"]
impl crate::Readable for Timings1Spec {}
#[doc = "`write(|w| ..)` method takes [`timings1::W`](W) writer structure"]
impl crate::Writable for Timings1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMINGS1 to value 0"]
impl crate::Resettable for Timings1Spec {
    const RESET_VALUE: u32 = 0;
}
