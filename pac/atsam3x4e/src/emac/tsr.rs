#[doc = "Register `TSR` reader"]
pub type R = crate::R<TsrSpec>;
#[doc = "Register `TSR` writer"]
pub type W = crate::W<TsrSpec>;
#[doc = "Field `UBR` reader - Used Bit Read"]
pub type UbrR = crate::BitReader;
#[doc = "Field `UBR` writer - Used Bit Read"]
pub type UbrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COL` reader - Collision Occurred"]
pub type ColR = crate::BitReader;
#[doc = "Field `COL` writer - Collision Occurred"]
pub type ColW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLES` reader - Retry Limit exceeded"]
pub type RlesR = crate::BitReader;
#[doc = "Field `RLES` writer - Retry Limit exceeded"]
pub type RlesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGO` reader - Transmit Go"]
pub type TgoR = crate::BitReader;
#[doc = "Field `TGO` writer - Transmit Go"]
pub type TgoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEX` reader - Buffers exhausted mid frame"]
pub type BexR = crate::BitReader;
#[doc = "Field `BEX` writer - Buffers exhausted mid frame"]
pub type BexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` reader - Transmit Complete"]
pub type CompR = crate::BitReader;
#[doc = "Field `COMP` writer - Transmit Complete"]
pub type CompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UND` reader - Transmit Underrun"]
pub type UndR = crate::BitReader;
#[doc = "Field `UND` writer - Transmit Underrun"]
pub type UndW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Used Bit Read"]
    #[inline(always)]
    pub fn ubr(&self) -> UbrR {
        UbrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Collision Occurred"]
    #[inline(always)]
    pub fn col(&self) -> ColR {
        ColR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Retry Limit exceeded"]
    #[inline(always)]
    pub fn rles(&self) -> RlesR {
        RlesR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Go"]
    #[inline(always)]
    pub fn tgo(&self) -> TgoR {
        TgoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffers exhausted mid frame"]
    #[inline(always)]
    pub fn bex(&self) -> BexR {
        BexR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Complete"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn und(&self) -> UndR {
        UndR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn ubr(&mut self) -> UbrW<TsrSpec> {
        UbrW::new(self, 0)
    }
    #[doc = "Bit 1 - Collision Occurred"]
    #[inline(always)]
    #[must_use]
    pub fn col(&mut self) -> ColW<TsrSpec> {
        ColW::new(self, 1)
    }
    #[doc = "Bit 2 - Retry Limit exceeded"]
    #[inline(always)]
    #[must_use]
    pub fn rles(&mut self) -> RlesW<TsrSpec> {
        RlesW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Go"]
    #[inline(always)]
    #[must_use]
    pub fn tgo(&mut self) -> TgoW<TsrSpec> {
        TgoW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffers exhausted mid frame"]
    #[inline(always)]
    #[must_use]
    pub fn bex(&mut self) -> BexW<TsrSpec> {
        BexW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> CompW<TsrSpec> {
        CompW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn und(&mut self) -> UndW<TsrSpec> {
        UndW::new(self, 6)
    }
}
#[doc = "Transmit Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsrSpec;
impl crate::RegisterSpec for TsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsr::R`](R) reader structure"]
impl crate::Readable for TsrSpec {}
#[doc = "`write(|w| ..)` method takes [`tsr::W`](W) writer structure"]
impl crate::Writable for TsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TsrSpec {
    const RESET_VALUE: u32 = 0;
}
