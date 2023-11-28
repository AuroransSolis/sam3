#[doc = "Register `MID3` reader"]
pub type R = crate::R<MID3_SPEC>;
#[doc = "Register `MID3` writer"]
pub type W = crate::W<MID3_SPEC>;
#[doc = "Field `MIDvB` reader - Complementary bits for identifier in extended frame mode"]
pub type MIDV_B_R = crate::FieldReader<u32>;
#[doc = "Field `MIDvB` writer - Complementary bits for identifier in extended frame mode"]
pub type MIDV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `MIDvA` reader - Identifier for standard frame mode"]
pub type MIDV_A_R = crate::FieldReader<u16>;
#[doc = "Field `MIDvA` writer - Identifier for standard frame mode"]
pub type MIDV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `MIDE` reader - Identifier Version"]
pub type MIDE_R = crate::BitReader;
#[doc = "Field `MIDE` writer - Identifier Version"]
pub type MIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:17 - Complementary bits for identifier in extended frame mode"]
    #[inline(always)]
    pub fn midv_b(&self) -> MIDV_B_R {
        MIDV_B_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:28 - Identifier for standard frame mode"]
    #[inline(always)]
    pub fn midv_a(&self) -> MIDV_A_R {
        MIDV_A_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - Identifier Version"]
    #[inline(always)]
    pub fn mide(&self) -> MIDE_R {
        MIDE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - Complementary bits for identifier in extended frame mode"]
    #[inline(always)]
    #[must_use]
    pub fn midv_b(&mut self) -> MIDV_B_W<MID3_SPEC> {
        MIDV_B_W::new(self, 0)
    }
    #[doc = "Bits 18:28 - Identifier for standard frame mode"]
    #[inline(always)]
    #[must_use]
    pub fn midv_a(&mut self) -> MIDV_A_W<MID3_SPEC> {
        MIDV_A_W::new(self, 18)
    }
    #[doc = "Bit 29 - Identifier Version"]
    #[inline(always)]
    #[must_use]
    pub fn mide(&mut self) -> MIDE_W<MID3_SPEC> {
        MIDE_W::new(self, 29)
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
#[doc = "Mailbox ID Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mid3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MID3_SPEC;
impl crate::RegisterSpec for MID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mid3::R`](R) reader structure"]
impl crate::Readable for MID3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mid3::W`](W) writer structure"]
impl crate::Writable for MID3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MID3 to value 0"]
impl crate::Resettable for MID3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
