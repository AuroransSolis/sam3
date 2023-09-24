#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `PROCRST` writer - Processor Reset"]
pub type PROCRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERRST` writer - Peripheral Reset"]
pub type PERRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTRST` writer - External Reset"]
pub type EXTRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `KEY` writer - System Reset Key"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl W {
    #[doc = "Bit 0 - Processor Reset"]
    #[inline(always)]
    #[must_use]
    pub fn procrst(&mut self) -> PROCRST_W<CR_SPEC, 0> {
        PROCRST_W::new(self)
    }
    #[doc = "Bit 2 - Peripheral Reset"]
    #[inline(always)]
    #[must_use]
    pub fn perrst(&mut self) -> PERRST_W<CR_SPEC, 2> {
        PERRST_W::new(self)
    }
    #[doc = "Bit 3 - External Reset"]
    #[inline(always)]
    #[must_use]
    pub fn extrst(&mut self) -> EXTRST_W<CR_SPEC, 3> {
        EXTRST_W::new(self)
    }
    #[doc = "Bits 24:31 - System Reset Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<CR_SPEC, 24> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
