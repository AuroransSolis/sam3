#[doc = "Register `TSR` reader"]
pub type R = crate::R<TSR_SPEC>;
#[doc = "Register `TSR` writer"]
pub type W = crate::W<TSR_SPEC>;
#[doc = "Field `UBR` reader - Used Bit Read"]
pub type UBR_R = crate::BitReader;
#[doc = "Field `UBR` writer - Used Bit Read"]
pub type UBR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COL` reader - Collision Occurred"]
pub type COL_R = crate::BitReader;
#[doc = "Field `COL` writer - Collision Occurred"]
pub type COL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLES` reader - Retry Limit exceeded"]
pub type RLES_R = crate::BitReader;
#[doc = "Field `RLES` writer - Retry Limit exceeded"]
pub type RLES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGO` reader - Transmit Go"]
pub type TGO_R = crate::BitReader;
#[doc = "Field `TGO` writer - Transmit Go"]
pub type TGO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEX` reader - Buffers exhausted mid frame"]
pub type BEX_R = crate::BitReader;
#[doc = "Field `BEX` writer - Buffers exhausted mid frame"]
pub type BEX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` reader - Transmit Complete"]
pub type COMP_R = crate::BitReader;
#[doc = "Field `COMP` writer - Transmit Complete"]
pub type COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UND` reader - Transmit Underrun"]
pub type UND_R = crate::BitReader;
#[doc = "Field `UND` writer - Transmit Underrun"]
pub type UND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Used Bit Read"]
    #[inline(always)]
    pub fn ubr(&self) -> UBR_R {
        UBR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Collision Occurred"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Retry Limit exceeded"]
    #[inline(always)]
    pub fn rles(&self) -> RLES_R {
        RLES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Go"]
    #[inline(always)]
    pub fn tgo(&self) -> TGO_R {
        TGO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffers exhausted mid frame"]
    #[inline(always)]
    pub fn bex(&self) -> BEX_R {
        BEX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Complete"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn und(&self) -> UND_R {
        UND_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used Bit Read"]
    #[inline(always)]
    #[must_use]
    pub fn ubr(&mut self) -> UBR_W<TSR_SPEC> {
        UBR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Collision Occurred"]
    #[inline(always)]
    #[must_use]
    pub fn col(&mut self) -> COL_W<TSR_SPEC> {
        COL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Retry Limit exceeded"]
    #[inline(always)]
    #[must_use]
    pub fn rles(&mut self) -> RLES_W<TSR_SPEC> {
        RLES_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Go"]
    #[inline(always)]
    #[must_use]
    pub fn tgo(&mut self) -> TGO_W<TSR_SPEC> {
        TGO_W::new(self, 3)
    }
    #[doc = "Bit 4 - Buffers exhausted mid frame"]
    #[inline(always)]
    #[must_use]
    pub fn bex(&mut self) -> BEX_W<TSR_SPEC> {
        BEX_W::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Complete"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<TSR_SPEC> {
        COMP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn und(&mut self) -> UND_W<TSR_SPEC> {
        UND_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSR_SPEC;
impl crate::RegisterSpec for TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsr::R`](R) reader structure"]
impl crate::Readable for TSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsr::W`](W) writer structure"]
impl crate::Writable for TSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
