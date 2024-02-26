#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `CLKDIV` reader - Clock Divider"]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock Divider"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWSDIV` reader - Power Saving Divider"]
pub type PwsdivR = crate::FieldReader;
#[doc = "Field `PWSDIV` writer - Power Saving Divider"]
pub type PwsdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RDPROOF` reader - "]
pub type RdproofR = crate::BitReader;
#[doc = "Field `RDPROOF` writer - "]
pub type RdproofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPROOF` reader - "]
pub type WrproofR = crate::BitReader;
#[doc = "Field `WRPROOF` writer - "]
pub type WrproofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBYTE` reader - Force Byte Transfer"]
pub type FbyteR = crate::BitReader;
#[doc = "Field `FBYTE` writer - Force Byte Transfer"]
pub type FbyteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADV` reader - Padding Value"]
pub type PadvR = crate::BitReader;
#[doc = "Field `PADV` writer - Padding Value"]
pub type PadvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDCMODE` reader - PDC-oriented Mode"]
pub type PdcmodeR = crate::BitReader;
#[doc = "Field `PDCMODE` writer - PDC-oriented Mode"]
pub type PdcmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Clock Divider"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Power Saving Divider"]
    #[inline(always)]
    pub fn pwsdiv(&self) -> PwsdivR {
        PwsdivR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rdproof(&self) -> RdproofR {
        RdproofR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wrproof(&self) -> WrproofR {
        WrproofR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force Byte Transfer"]
    #[inline(always)]
    pub fn fbyte(&self) -> FbyteR {
        FbyteR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Padding Value"]
    #[inline(always)]
    pub fn padv(&self) -> PadvR {
        PadvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDC-oriented Mode"]
    #[inline(always)]
    pub fn pdcmode(&self) -> PdcmodeR {
        PdcmodeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> ClkdivW<MrSpec> {
        ClkdivW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Power Saving Divider"]
    #[inline(always)]
    #[must_use]
    pub fn pwsdiv(&mut self) -> PwsdivW<MrSpec> {
        PwsdivW::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn rdproof(&mut self) -> RdproofW<MrSpec> {
        RdproofW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn wrproof(&mut self) -> WrproofW<MrSpec> {
        WrproofW::new(self, 12)
    }
    #[doc = "Bit 13 - Force Byte Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn fbyte(&mut self) -> FbyteW<MrSpec> {
        FbyteW::new(self, 13)
    }
    #[doc = "Bit 14 - Padding Value"]
    #[inline(always)]
    #[must_use]
    pub fn padv(&mut self) -> PadvW<MrSpec> {
        PadvW::new(self, 14)
    }
    #[doc = "Bit 15 - PDC-oriented Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pdcmode(&mut self) -> PdcmodeW<MrSpec> {
        PdcmodeW::new(self, 15)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0;
}
