#[doc = "Register `MATRIX_PRAS8` reader"]
pub type R = crate::R<MATRIX_PRAS8_SPEC>;
#[doc = "Register `MATRIX_PRAS8` writer"]
pub type W = crate::W<MATRIX_PRAS8_SPEC>;
#[doc = "Field `M0PR` reader - Master 0 Priority"]
pub type M0PR_R = crate::FieldReader;
#[doc = "Field `M0PR` writer - Master 0 Priority"]
pub type M0PR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `M1PR` reader - Master 1 Priority"]
pub type M1PR_R = crate::FieldReader;
#[doc = "Field `M1PR` writer - Master 1 Priority"]
pub type M1PR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `M2PR` reader - Master 2 Priority"]
pub type M2PR_R = crate::FieldReader;
#[doc = "Field `M2PR` writer - Master 2 Priority"]
pub type M2PR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `M3PR` reader - Master 3 Priority"]
pub type M3PR_R = crate::FieldReader;
#[doc = "Field `M3PR` writer - Master 3 Priority"]
pub type M3PR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `M4PR` reader - Master 4 Priority"]
pub type M4PR_R = crate::FieldReader;
#[doc = "Field `M4PR` writer - Master 4 Priority"]
pub type M4PR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `M5PR` reader - Master 5 Priority"]
pub type M5PR_R = crate::FieldReader;
#[doc = "Field `M5PR` writer - Master 5 Priority"]
pub type M5PR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline(always)]
    pub fn m0pr(&self) -> M0PR_R {
        M0PR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Master 1 Priority"]
    #[inline(always)]
    pub fn m1pr(&self) -> M1PR_R {
        M1PR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Master 2 Priority"]
    #[inline(always)]
    pub fn m2pr(&self) -> M2PR_R {
        M2PR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Master 3 Priority"]
    #[inline(always)]
    pub fn m3pr(&self) -> M3PR_R {
        M3PR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Master 4 Priority"]
    #[inline(always)]
    pub fn m4pr(&self) -> M4PR_R {
        M4PR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Master 5 Priority"]
    #[inline(always)]
    pub fn m5pr(&self) -> M5PR_R {
        M5PR_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m0pr(&mut self) -> M0PR_W<MATRIX_PRAS8_SPEC, 0> {
        M0PR_W::new(self)
    }
    #[doc = "Bits 4:5 - Master 1 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m1pr(&mut self) -> M1PR_W<MATRIX_PRAS8_SPEC, 4> {
        M1PR_W::new(self)
    }
    #[doc = "Bits 8:9 - Master 2 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m2pr(&mut self) -> M2PR_W<MATRIX_PRAS8_SPEC, 8> {
        M2PR_W::new(self)
    }
    #[doc = "Bits 12:13 - Master 3 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m3pr(&mut self) -> M3PR_W<MATRIX_PRAS8_SPEC, 12> {
        M3PR_W::new(self)
    }
    #[doc = "Bits 16:17 - Master 4 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m4pr(&mut self) -> M4PR_W<MATRIX_PRAS8_SPEC, 16> {
        M4PR_W::new(self)
    }
    #[doc = "Bits 20:21 - Master 5 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m5pr(&mut self) -> M5PR_W<MATRIX_PRAS8_SPEC, 20> {
        M5PR_W::new(self)
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
#[doc = "Priority Register A for Slave 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`matrix_pras8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`matrix_pras8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MATRIX_PRAS8_SPEC;
impl crate::RegisterSpec for MATRIX_PRAS8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`matrix_pras8::R`](R) reader structure"]
impl crate::Readable for MATRIX_PRAS8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`matrix_pras8::W`](W) writer structure"]
impl crate::Writable for MATRIX_PRAS8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MATRIX_PRAS8 to value 0"]
impl crate::Resettable for MATRIX_PRAS8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
