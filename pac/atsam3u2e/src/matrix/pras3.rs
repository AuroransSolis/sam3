#[doc = "Register `PRAS3` reader"]
pub type R = crate::R<Pras3Spec>;
#[doc = "Register `PRAS3` writer"]
pub type W = crate::W<Pras3Spec>;
#[doc = "Field `M0PR` reader - Master 0 Priority"]
pub type M0prR = crate::FieldReader;
#[doc = "Field `M0PR` writer - Master 0 Priority"]
pub type M0prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M1PR` reader - Master 1 Priority"]
pub type M1prR = crate::FieldReader;
#[doc = "Field `M1PR` writer - Master 1 Priority"]
pub type M1prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M2PR` reader - Master 2 Priority"]
pub type M2prR = crate::FieldReader;
#[doc = "Field `M2PR` writer - Master 2 Priority"]
pub type M2prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M3PR` reader - Master 3 Priority"]
pub type M3prR = crate::FieldReader;
#[doc = "Field `M3PR` writer - Master 3 Priority"]
pub type M3prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M4PR` reader - Master 4 Priority"]
pub type M4prR = crate::FieldReader;
#[doc = "Field `M4PR` writer - Master 4 Priority"]
pub type M4prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline(always)]
    pub fn m0pr(&self) -> M0prR {
        M0prR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Master 1 Priority"]
    #[inline(always)]
    pub fn m1pr(&self) -> M1prR {
        M1prR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Master 2 Priority"]
    #[inline(always)]
    pub fn m2pr(&self) -> M2prR {
        M2prR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Master 3 Priority"]
    #[inline(always)]
    pub fn m3pr(&self) -> M3prR {
        M3prR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Master 4 Priority"]
    #[inline(always)]
    pub fn m4pr(&self) -> M4prR {
        M4prR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m0pr(&mut self) -> M0prW<Pras3Spec> {
        M0prW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Master 1 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m1pr(&mut self) -> M1prW<Pras3Spec> {
        M1prW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Master 2 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m2pr(&mut self) -> M2prW<Pras3Spec> {
        M2prW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Master 3 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m3pr(&mut self) -> M3prW<Pras3Spec> {
        M3prW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Master 4 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn m4pr(&mut self) -> M4prW<Pras3Spec> {
        M4prW::new(self, 16)
    }
}
#[doc = "Priority Register A for Slave 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pras3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pras3Spec;
impl crate::RegisterSpec for Pras3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pras3::R`](R) reader structure"]
impl crate::Readable for Pras3Spec {}
#[doc = "`write(|w| ..)` method takes [`pras3::W`](W) writer structure"]
impl crate::Writable for Pras3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRAS3 to value 0"]
impl crate::Resettable for Pras3Spec {
    const RESET_VALUE: u32 = 0;
}
