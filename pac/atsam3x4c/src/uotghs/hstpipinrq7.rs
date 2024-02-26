#[doc = "Register `HSTPIPINRQ7` reader"]
pub type R = crate::R<Hstpipinrq7Spec>;
#[doc = "Register `HSTPIPINRQ7` writer"]
pub type W = crate::W<Hstpipinrq7Spec>;
#[doc = "Field `INRQ` reader - IN Request Number before Freeze"]
pub type InrqR = crate::FieldReader;
#[doc = "Field `INRQ` writer - IN Request Number before Freeze"]
pub type InrqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INMODE` reader - IN Request Mode"]
pub type InmodeR = crate::BitReader;
#[doc = "Field `INMODE` writer - IN Request Mode"]
pub type InmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - IN Request Number before Freeze"]
    #[inline(always)]
    pub fn inrq(&self) -> InrqR {
        InrqR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    pub fn inmode(&self) -> InmodeR {
        InmodeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - IN Request Number before Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn inrq(&mut self) -> InrqW<Hstpipinrq7Spec> {
        InrqW::new(self, 0)
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    #[must_use]
    pub fn inmode(&mut self) -> InmodeW<Hstpipinrq7Spec> {
        InmodeW::new(self, 8)
    }
}
#[doc = "Host Pipe IN Request Register (n = 0) 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq7::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hstpipinrq7Spec;
impl crate::RegisterSpec for Hstpipinrq7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpipinrq7::R`](R) reader structure"]
impl crate::Readable for Hstpipinrq7Spec {}
#[doc = "`write(|w| ..)` method takes [`hstpipinrq7::W`](W) writer structure"]
impl crate::Writable for Hstpipinrq7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
