#[doc = "Register `FMR` reader"]
pub type R = crate::R<FmrSpec>;
#[doc = "Register `FMR` writer"]
pub type W = crate::W<FmrSpec>;
#[doc = "Field `FRDY` reader - Ready Interrupt Enable"]
pub type FrdyR = crate::BitReader;
#[doc = "Field `FRDY` writer - Ready Interrupt Enable"]
pub type FrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWS` reader - Flash Wait State"]
pub type FwsR = crate::FieldReader;
#[doc = "Field `FWS` writer - Flash Wait State"]
pub type FwsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCOD` reader - Sequential Code Optimization Disable"]
pub type ScodR = crate::BitReader;
#[doc = "Field `SCOD` writer - Sequential Code Optimization Disable"]
pub type ScodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAM` reader - Flash Access Mode"]
pub type FamR = crate::BitReader;
#[doc = "Field `FAM` writer - Flash Access Mode"]
pub type FamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&self) -> FrdyR {
        FrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&self) -> FwsR {
        FwsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Sequential Code Optimization Disable"]
    #[inline(always)]
    pub fn scod(&self) -> ScodR {
        ScodR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Flash Access Mode"]
    #[inline(always)]
    pub fn fam(&self) -> FamR {
        FamR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frdy(&mut self) -> FrdyW<FmrSpec> {
        FrdyW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Flash Wait State"]
    #[inline(always)]
    #[must_use]
    pub fn fws(&mut self) -> FwsW<FmrSpec> {
        FwsW::new(self, 8)
    }
    #[doc = "Bit 16 - Sequential Code Optimization Disable"]
    #[inline(always)]
    #[must_use]
    pub fn scod(&mut self) -> ScodW<FmrSpec> {
        ScodW::new(self, 16)
    }
    #[doc = "Bit 24 - Flash Access Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fam(&mut self) -> FamW<FmrSpec> {
        FamW::new(self, 24)
    }
}
#[doc = "EEFC Flash Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmrSpec;
impl crate::RegisterSpec for FmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmr::R`](R) reader structure"]
impl crate::Readable for FmrSpec {}
#[doc = "`write(|w| ..)` method takes [`fmr::W`](W) writer structure"]
impl crate::Writable for FmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMR to value 0"]
impl crate::Resettable for FmrSpec {
    const RESET_VALUE: u32 = 0;
}
