#[doc = "Register `HSTPIPINRQ2` reader"]
pub type R = crate::R<HSTPIPINRQ2_SPEC>;
#[doc = "Register `HSTPIPINRQ2` writer"]
pub type W = crate::W<HSTPIPINRQ2_SPEC>;
#[doc = "Field `INRQ` reader - IN Request Number before Freeze"]
pub type INRQ_R = crate::FieldReader;
#[doc = "Field `INRQ` writer - IN Request Number before Freeze"]
pub type INRQ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INMODE` reader - IN Request Mode"]
pub type INMODE_R = crate::BitReader;
#[doc = "Field `INMODE` writer - IN Request Mode"]
pub type INMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - IN Request Number before Freeze"]
    #[inline(always)]
    pub fn inrq(&self) -> INRQ_R {
        INRQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    pub fn inmode(&self) -> INMODE_R {
        INMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - IN Request Number before Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn inrq(&mut self) -> INRQ_W<HSTPIPINRQ2_SPEC> {
        INRQ_W::new(self, 0)
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    #[must_use]
    pub fn inmode(&mut self) -> INMODE_W<HSTPIPINRQ2_SPEC> {
        INMODE_W::new(self, 8)
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
#[doc = "Host Pipe IN Request Register (n = 0) 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq2::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTPIPINRQ2_SPEC;
impl crate::RegisterSpec for HSTPIPINRQ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpipinrq2::R`](R) reader structure"]
impl crate::Readable for HSTPIPINRQ2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstpipinrq2::W`](W) writer structure"]
impl crate::Writable for HSTPIPINRQ2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
