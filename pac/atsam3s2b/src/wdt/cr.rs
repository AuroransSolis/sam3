#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `WDRSTT` writer - Watchdog Restart"]
pub type WDRSTT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Password."]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_AW {
    #[doc = "165: Writing any other value in this field aborts the write operation."]
    Passwd = 165,
}
impl From<KEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEY_AW {
    type Ux = u8;
}
#[doc = "Field `KEY` writer - Password."]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, KEY_AW>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_AW::Passwd)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Restart"]
    #[inline(always)]
    #[must_use]
    pub fn wdrstt(&mut self) -> WDRSTT_W<CR_SPEC> {
        WDRSTT_W::new(self, 0)
    }
    #[doc = "Bits 24:31 - Password."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<CR_SPEC> {
        KEY_W::new(self, 24)
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
#[doc = "Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
