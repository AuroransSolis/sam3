#[doc = "Register `FADDR` reader"]
pub type R = crate::R<FADDR_SPEC>;
#[doc = "Register `FADDR` writer"]
pub type W = crate::W<FADDR_SPEC>;
#[doc = "Field `FADD` reader - Function Address Value"]
pub type FADD_R = crate::FieldReader;
#[doc = "Field `FADD` writer - Function Address Value"]
pub type FADD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FEN` reader - Function Enable"]
pub type FEN_R = crate::BitReader;
#[doc = "Field `FEN` writer - Function Enable"]
pub type FEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Function Address Value"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Function Enable"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Function Address Value"]
    #[inline(always)]
    #[must_use]
    pub fn fadd(&mut self) -> FADD_W<FADDR_SPEC> {
        FADD_W::new(self, 0)
    }
    #[doc = "Bit 8 - Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FEN_W<FADDR_SPEC> {
        FEN_W::new(self, 8)
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
#[doc = "Function Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FADDR_SPEC;
impl crate::RegisterSpec for FADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`faddr::R`](R) reader structure"]
impl crate::Readable for FADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`faddr::W`](W) writer structure"]
impl crate::Writable for FADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FADDR to value 0x0100"]
impl crate::Resettable for FADDR_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
