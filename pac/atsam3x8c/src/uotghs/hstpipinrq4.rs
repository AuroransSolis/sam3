#[doc = "Register `HSTPIPINRQ4` reader"]
pub type R = crate::R<Hstpipinrq4Spec>;
#[doc = "Register `HSTPIPINRQ4` writer"]
pub type W = crate::W<Hstpipinrq4Spec>;
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
    pub fn inrq(&mut self) -> InrqW<Hstpipinrq4Spec> {
        InrqW::new(self, 0)
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    #[must_use]
    pub fn inmode(&mut self) -> InmodeW<Hstpipinrq4Spec> {
        InmodeW::new(self, 8)
    }
}
#[doc = "Host Pipe IN Request Register (n = 0) 4\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpipinrq4::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpipinrq4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hstpipinrq4Spec;
impl crate::RegisterSpec for Hstpipinrq4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpipinrq4::R`](R) reader structure"]
impl crate::Readable for Hstpipinrq4Spec {}
#[doc = "`write(|w| ..)` method takes [`hstpipinrq4::W`](W) writer structure"]
impl crate::Writable for Hstpipinrq4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
