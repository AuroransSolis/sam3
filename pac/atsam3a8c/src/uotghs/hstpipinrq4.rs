#[doc = "Register `HSTPIPINRQ4` reader"]
pub type R = crate::R<HSTPIPINRQ4_SPEC>;
#[doc = "Register `HSTPIPINRQ4` writer"]
pub type W = crate::W<HSTPIPINRQ4_SPEC>;
#[doc = "Field `INRQ` reader - IN Request Number before Freeze"]
pub type INRQ_R = crate::FieldReader;
#[doc = "Field `INRQ` writer - IN Request Number before Freeze"]
pub type INRQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INMODE` reader - IN Request Mode"]
pub type INMODE_R = crate::BitReader;
#[doc = "Field `INMODE` writer - IN Request Mode"]
pub type INMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn inrq(&mut self) -> INRQ_W<HSTPIPINRQ4_SPEC, 0> {
        INRQ_W::new(self)
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    #[must_use]
    pub fn inmode(&mut self) -> INMODE_W<HSTPIPINRQ4_SPEC, 8> {
        INMODE_W::new(self)
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
#[doc = "Host Pipe IN Request Register (n = 0) 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq4::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTPIPINRQ4_SPEC;
impl crate::RegisterSpec for HSTPIPINRQ4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpipinrq4::R`](R) reader structure"]
impl crate::Readable for HSTPIPINRQ4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstpipinrq4::W`](W) writer structure"]
impl crate::Writable for HSTPIPINRQ4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
