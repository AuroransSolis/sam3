#[doc = "Register `HSTADDR1` reader"]
pub type R = crate::R<Hstaddr1Spec>;
#[doc = "Register `HSTADDR1` writer"]
pub type W = crate::W<Hstaddr1Spec>;
#[doc = "Field `HSTADDRP0` reader - USB Host Address"]
pub type Hstaddrp0R = crate::FieldReader;
#[doc = "Field `HSTADDRP0` writer - USB Host Address"]
pub type Hstaddrp0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HSTADDRP1` reader - USB Host Address"]
pub type Hstaddrp1R = crate::FieldReader;
#[doc = "Field `HSTADDRP1` writer - USB Host Address"]
pub type Hstaddrp1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HSTADDRP2` reader - USB Host Address"]
pub type Hstaddrp2R = crate::FieldReader;
#[doc = "Field `HSTADDRP2` writer - USB Host Address"]
pub type Hstaddrp2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HSTADDRP3` reader - USB Host Address"]
pub type Hstaddrp3R = crate::FieldReader;
#[doc = "Field `HSTADDRP3` writer - USB Host Address"]
pub type Hstaddrp3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp0(&self) -> Hstaddrp0R {
        Hstaddrp0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp1(&self) -> Hstaddrp1R {
        Hstaddrp1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp2(&self) -> Hstaddrp2R {
        Hstaddrp2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp3(&self) -> Hstaddrp3R {
        Hstaddrp3R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp0(&mut self) -> Hstaddrp0W<Hstaddr1Spec> {
        Hstaddrp0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp1(&mut self) -> Hstaddrp1W<Hstaddr1Spec> {
        Hstaddrp1W::new(self, 8)
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp2(&mut self) -> Hstaddrp2W<Hstaddr1Spec> {
        Hstaddrp2W::new(self, 16)
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp3(&mut self) -> Hstaddrp3W<Hstaddr1Spec> {
        Hstaddrp3W::new(self, 24)
    }
}
#[doc = "Host Address 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstaddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstaddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hstaddr1Spec;
impl crate::RegisterSpec for Hstaddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstaddr1::R`](R) reader structure"]
impl crate::Readable for Hstaddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`hstaddr1::W`](W) writer structure"]
impl crate::Writable for Hstaddr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSTADDR1 to value 0"]
impl crate::Resettable for Hstaddr1Spec {
    const RESET_VALUE: u32 = 0;
}
