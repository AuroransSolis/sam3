#[doc = "Register `HSTADDR3` reader"]
pub type R = crate::R<Hstaddr3Spec>;
#[doc = "Register `HSTADDR3` writer"]
pub type W = crate::W<Hstaddr3Spec>;
#[doc = "Field `HSTADDRP8` reader - USB Host Address"]
pub type Hstaddrp8R = crate::FieldReader;
#[doc = "Field `HSTADDRP8` writer - USB Host Address"]
pub type Hstaddrp8W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HSTADDRP9` reader - USB Host Address"]
pub type Hstaddrp9R = crate::FieldReader;
#[doc = "Field `HSTADDRP9` writer - USB Host Address"]
pub type Hstaddrp9W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp8(&self) -> Hstaddrp8R {
        Hstaddrp8R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp9(&self) -> Hstaddrp9R {
        Hstaddrp9R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp8(&mut self) -> Hstaddrp8W<Hstaddr3Spec> {
        Hstaddrp8W::new(self, 0)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp9(&mut self) -> Hstaddrp9W<Hstaddr3Spec> {
        Hstaddrp9W::new(self, 8)
    }
}
#[doc = "Host Address 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstaddr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstaddr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hstaddr3Spec;
impl crate::RegisterSpec for Hstaddr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstaddr3::R`](R) reader structure"]
impl crate::Readable for Hstaddr3Spec {}
#[doc = "`write(|w| ..)` method takes [`hstaddr3::W`](W) writer structure"]
impl crate::Writable for Hstaddr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSTADDR3 to value 0"]
impl crate::Resettable for Hstaddr3Spec {
    const RESET_VALUE: u32 = 0;
}
