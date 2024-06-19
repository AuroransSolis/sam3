#[doc = "Register `HSTADDR2` reader"]
pub type R = crate::R<Hstaddr2Spec>;
#[doc = "Register `HSTADDR2` writer"]
pub type W = crate::W<Hstaddr2Spec>;
#[doc = "Field `HSTADDRP4` reader - USB Host Address"]
pub type Hstaddrp4R = crate::FieldReader;
#[doc = "Field `HSTADDRP4` writer - USB Host Address"]
pub type Hstaddrp4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HSTADDRP5` reader - USB Host Address"]
pub type Hstaddrp5R = crate::FieldReader;
#[doc = "Field `HSTADDRP5` writer - USB Host Address"]
pub type Hstaddrp5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HSTADDRP6` reader - USB Host Address"]
pub type Hstaddrp6R = crate::FieldReader;
#[doc = "Field `HSTADDRP6` writer - USB Host Address"]
pub type Hstaddrp6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HSTADDRP7` reader - USB Host Address"]
pub type Hstaddrp7R = crate::FieldReader;
#[doc = "Field `HSTADDRP7` writer - USB Host Address"]
pub type Hstaddrp7W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp4(&self) -> Hstaddrp4R {
        Hstaddrp4R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp5(&self) -> Hstaddrp5R {
        Hstaddrp5R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp6(&self) -> Hstaddrp6R {
        Hstaddrp6R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp7(&self) -> Hstaddrp7R {
        Hstaddrp7R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp4(&mut self) -> Hstaddrp4W<Hstaddr2Spec> {
        Hstaddrp4W::new(self, 0)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp5(&mut self) -> Hstaddrp5W<Hstaddr2Spec> {
        Hstaddrp5W::new(self, 8)
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp6(&mut self) -> Hstaddrp6W<Hstaddr2Spec> {
        Hstaddrp6W::new(self, 16)
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp7(&mut self) -> Hstaddrp7W<Hstaddr2Spec> {
        Hstaddrp7W::new(self, 24)
    }
}
#[doc = "Host Address 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hstaddr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstaddr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hstaddr2Spec;
impl crate::RegisterSpec for Hstaddr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstaddr2::R`](R) reader structure"]
impl crate::Readable for Hstaddr2Spec {}
#[doc = "`write(|w| ..)` method takes [`hstaddr2::W`](W) writer structure"]
impl crate::Writable for Hstaddr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSTADDR2 to value 0"]
impl crate::Resettable for Hstaddr2Spec {
    const RESET_VALUE: u32 = 0;
}
