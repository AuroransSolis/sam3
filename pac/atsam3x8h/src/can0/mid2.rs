#[doc = "Register `MID2` reader"]
pub type R = crate::R<Mid2Spec>;
#[doc = "Register `MID2` writer"]
pub type W = crate::W<Mid2Spec>;
#[doc = "Field `MIDvB` reader - Complementary bits for identifier in extended frame mode"]
pub type MidvBR = crate::FieldReader<u32>;
#[doc = "Field `MIDvB` writer - Complementary bits for identifier in extended frame mode"]
pub type MidvBW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `MIDvA` reader - Identifier for standard frame mode"]
pub type MidvAR = crate::FieldReader<u16>;
#[doc = "Field `MIDvA` writer - Identifier for standard frame mode"]
pub type MidvAW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `MIDE` reader - Identifier Version"]
pub type MideR = crate::BitReader;
#[doc = "Field `MIDE` writer - Identifier Version"]
pub type MideW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:17 - Complementary bits for identifier in extended frame mode"]
    #[inline(always)]
    pub fn midv_b(&self) -> MidvBR {
        MidvBR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:28 - Identifier for standard frame mode"]
    #[inline(always)]
    pub fn midv_a(&self) -> MidvAR {
        MidvAR::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - Identifier Version"]
    #[inline(always)]
    pub fn mide(&self) -> MideR {
        MideR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - Complementary bits for identifier in extended frame mode"]
    #[inline(always)]
    #[must_use]
    pub fn midv_b(&mut self) -> MidvBW<Mid2Spec> {
        MidvBW::new(self, 0)
    }
    #[doc = "Bits 18:28 - Identifier for standard frame mode"]
    #[inline(always)]
    #[must_use]
    pub fn midv_a(&mut self) -> MidvAW<Mid2Spec> {
        MidvAW::new(self, 18)
    }
    #[doc = "Bit 29 - Identifier Version"]
    #[inline(always)]
    #[must_use]
    pub fn mide(&mut self) -> MideW<Mid2Spec> {
        MideW::new(self, 29)
    }
}
#[doc = "Mailbox ID Register (MB = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`mid2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mid2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mid2Spec;
impl crate::RegisterSpec for Mid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mid2::R`](R) reader structure"]
impl crate::Readable for Mid2Spec {}
#[doc = "`write(|w| ..)` method takes [`mid2::W`](W) writer structure"]
impl crate::Writable for Mid2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MID2 to value 0"]
impl crate::Resettable for Mid2Spec {
    const RESET_VALUE: u32 = 0;
}
