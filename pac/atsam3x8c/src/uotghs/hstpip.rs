#[doc = "Register `HSTPIP` reader"]
pub type R = crate::R<HSTPIP_SPEC>;
#[doc = "Register `HSTPIP` writer"]
pub type W = crate::W<HSTPIP_SPEC>;
#[doc = "Field `PEN0` reader - Pipe 0 Enable"]
pub type PEN0_R = crate::BitReader;
#[doc = "Field `PEN0` writer - Pipe 0 Enable"]
pub type PEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN1` reader - Pipe 1 Enable"]
pub type PEN1_R = crate::BitReader;
#[doc = "Field `PEN1` writer - Pipe 1 Enable"]
pub type PEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN2` reader - Pipe 2 Enable"]
pub type PEN2_R = crate::BitReader;
#[doc = "Field `PEN2` writer - Pipe 2 Enable"]
pub type PEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN3` reader - Pipe 3 Enable"]
pub type PEN3_R = crate::BitReader;
#[doc = "Field `PEN3` writer - Pipe 3 Enable"]
pub type PEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN4` reader - Pipe 4 Enable"]
pub type PEN4_R = crate::BitReader;
#[doc = "Field `PEN4` writer - Pipe 4 Enable"]
pub type PEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN5` reader - Pipe 5 Enable"]
pub type PEN5_R = crate::BitReader;
#[doc = "Field `PEN5` writer - Pipe 5 Enable"]
pub type PEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN6` reader - Pipe 6 Enable"]
pub type PEN6_R = crate::BitReader;
#[doc = "Field `PEN6` writer - Pipe 6 Enable"]
pub type PEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN7` reader - Pipe 7 Enable"]
pub type PEN7_R = crate::BitReader;
#[doc = "Field `PEN7` writer - Pipe 7 Enable"]
pub type PEN7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN8` reader - Pipe 8 Enable"]
pub type PEN8_R = crate::BitReader;
#[doc = "Field `PEN8` writer - Pipe 8 Enable"]
pub type PEN8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST0` reader - Pipe 0 Reset"]
pub type PRST0_R = crate::BitReader;
#[doc = "Field `PRST0` writer - Pipe 0 Reset"]
pub type PRST0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST1` reader - Pipe 1 Reset"]
pub type PRST1_R = crate::BitReader;
#[doc = "Field `PRST1` writer - Pipe 1 Reset"]
pub type PRST1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST2` reader - Pipe 2 Reset"]
pub type PRST2_R = crate::BitReader;
#[doc = "Field `PRST2` writer - Pipe 2 Reset"]
pub type PRST2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST3` reader - Pipe 3 Reset"]
pub type PRST3_R = crate::BitReader;
#[doc = "Field `PRST3` writer - Pipe 3 Reset"]
pub type PRST3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST4` reader - Pipe 4 Reset"]
pub type PRST4_R = crate::BitReader;
#[doc = "Field `PRST4` writer - Pipe 4 Reset"]
pub type PRST4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST5` reader - Pipe 5 Reset"]
pub type PRST5_R = crate::BitReader;
#[doc = "Field `PRST5` writer - Pipe 5 Reset"]
pub type PRST5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST6` reader - Pipe 6 Reset"]
pub type PRST6_R = crate::BitReader;
#[doc = "Field `PRST6` writer - Pipe 6 Reset"]
pub type PRST6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST7` reader - Pipe 7 Reset"]
pub type PRST7_R = crate::BitReader;
#[doc = "Field `PRST7` writer - Pipe 7 Reset"]
pub type PRST7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST8` reader - Pipe 8 Reset"]
pub type PRST8_R = crate::BitReader;
#[doc = "Field `PRST8` writer - Pipe 8 Reset"]
pub type PRST8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pipe 0 Enable"]
    #[inline(always)]
    pub fn pen0(&self) -> PEN0_R {
        PEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pipe 1 Enable"]
    #[inline(always)]
    pub fn pen1(&self) -> PEN1_R {
        PEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pipe 2 Enable"]
    #[inline(always)]
    pub fn pen2(&self) -> PEN2_R {
        PEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe 3 Enable"]
    #[inline(always)]
    pub fn pen3(&self) -> PEN3_R {
        PEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pipe 4 Enable"]
    #[inline(always)]
    pub fn pen4(&self) -> PEN4_R {
        PEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pipe 5 Enable"]
    #[inline(always)]
    pub fn pen5(&self) -> PEN5_R {
        PEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pipe 6 Enable"]
    #[inline(always)]
    pub fn pen6(&self) -> PEN6_R {
        PEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pipe 7 Enable"]
    #[inline(always)]
    pub fn pen7(&self) -> PEN7_R {
        PEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pipe 8 Enable"]
    #[inline(always)]
    pub fn pen8(&self) -> PEN8_R {
        PEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Pipe 0 Reset"]
    #[inline(always)]
    pub fn prst0(&self) -> PRST0_R {
        PRST0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pipe 1 Reset"]
    #[inline(always)]
    pub fn prst1(&self) -> PRST1_R {
        PRST1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pipe 2 Reset"]
    #[inline(always)]
    pub fn prst2(&self) -> PRST2_R {
        PRST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Pipe 3 Reset"]
    #[inline(always)]
    pub fn prst3(&self) -> PRST3_R {
        PRST3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pipe 4 Reset"]
    #[inline(always)]
    pub fn prst4(&self) -> PRST4_R {
        PRST4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pipe 5 Reset"]
    #[inline(always)]
    pub fn prst5(&self) -> PRST5_R {
        PRST5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pipe 6 Reset"]
    #[inline(always)]
    pub fn prst6(&self) -> PRST6_R {
        PRST6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Pipe 7 Reset"]
    #[inline(always)]
    pub fn prst7(&self) -> PRST7_R {
        PRST7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Pipe 8 Reset"]
    #[inline(always)]
    pub fn prst8(&self) -> PRST8_R {
        PRST8_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pipe 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen0(&mut self) -> PEN0_W<HSTPIP_SPEC> {
        PEN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pipe 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen1(&mut self) -> PEN1_W<HSTPIP_SPEC> {
        PEN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pipe 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen2(&mut self) -> PEN2_W<HSTPIP_SPEC> {
        PEN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pipe 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen3(&mut self) -> PEN3_W<HSTPIP_SPEC> {
        PEN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pipe 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen4(&mut self) -> PEN4_W<HSTPIP_SPEC> {
        PEN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pipe 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen5(&mut self) -> PEN5_W<HSTPIP_SPEC> {
        PEN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pipe 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen6(&mut self) -> PEN6_W<HSTPIP_SPEC> {
        PEN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pipe 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen7(&mut self) -> PEN7_W<HSTPIP_SPEC> {
        PEN7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Pipe 8 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen8(&mut self) -> PEN8_W<HSTPIP_SPEC> {
        PEN8_W::new(self, 8)
    }
    #[doc = "Bit 16 - Pipe 0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst0(&mut self) -> PRST0_W<HSTPIP_SPEC> {
        PRST0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Pipe 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst1(&mut self) -> PRST1_W<HSTPIP_SPEC> {
        PRST1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Pipe 2 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst2(&mut self) -> PRST2_W<HSTPIP_SPEC> {
        PRST2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Pipe 3 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst3(&mut self) -> PRST3_W<HSTPIP_SPEC> {
        PRST3_W::new(self, 19)
    }
    #[doc = "Bit 20 - Pipe 4 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst4(&mut self) -> PRST4_W<HSTPIP_SPEC> {
        PRST4_W::new(self, 20)
    }
    #[doc = "Bit 21 - Pipe 5 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst5(&mut self) -> PRST5_W<HSTPIP_SPEC> {
        PRST5_W::new(self, 21)
    }
    #[doc = "Bit 22 - Pipe 6 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst6(&mut self) -> PRST6_W<HSTPIP_SPEC> {
        PRST6_W::new(self, 22)
    }
    #[doc = "Bit 23 - Pipe 7 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst7(&mut self) -> PRST7_W<HSTPIP_SPEC> {
        PRST7_W::new(self, 23)
    }
    #[doc = "Bit 24 - Pipe 8 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prst8(&mut self) -> PRST8_W<HSTPIP_SPEC> {
        PRST8_W::new(self, 24)
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
#[doc = "Host Pipe Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpip::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpip::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTPIP_SPEC;
impl crate::RegisterSpec for HSTPIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpip::R`](R) reader structure"]
impl crate::Readable for HSTPIP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstpip::W`](W) writer structure"]
impl crate::Writable for HSTPIP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTPIP to value 0"]
impl crate::Resettable for HSTPIP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
