#[doc = "Register `MAN` reader"]
pub type R = crate::R<MAN_SPEC>;
#[doc = "Register `MAN` writer"]
pub type W = crate::W<MAN_SPEC>;
#[doc = "Field `DATA` reader - "]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - "]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `CODE` reader - "]
pub type CODE_R = crate::FieldReader;
#[doc = "Field `CODE` writer - "]
pub type CODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `REGA` reader - Register Address"]
pub type REGA_R = crate::FieldReader;
#[doc = "Field `REGA` writer - Register Address"]
pub type REGA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PHYA` reader - PHY Address"]
pub type PHYA_R = crate::FieldReader;
#[doc = "Field `PHYA` writer - PHY Address"]
pub type PHYA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RW` reader - Read-write"]
pub type RW_R = crate::FieldReader;
#[doc = "Field `RW` writer - Read-write"]
pub type RW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SOF` reader - Start of frame"]
pub type SOF_R = crate::FieldReader;
#[doc = "Field `SOF` writer - Start of frame"]
pub type SOF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn rega(&self) -> REGA_R {
        REGA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn phya(&self) -> PHYA_R {
        PHYA_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Read-write"]
    #[inline(always)]
    pub fn rw(&self) -> RW_R {
        RW_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<MAN_SPEC, 0> {
        DATA_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CODE_W<MAN_SPEC, 16> {
        CODE_W::new(self)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    #[must_use]
    pub fn rega(&mut self) -> REGA_W<MAN_SPEC, 18> {
        REGA_W::new(self)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    #[must_use]
    pub fn phya(&mut self) -> PHYA_W<MAN_SPEC, 23> {
        PHYA_W::new(self)
    }
    #[doc = "Bits 28:29 - Read-write"]
    #[inline(always)]
    #[must_use]
    pub fn rw(&mut self) -> RW_W<MAN_SPEC, 28> {
        RW_W::new(self)
    }
    #[doc = "Bits 30:31 - Start of frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<MAN_SPEC, 30> {
        SOF_W::new(self)
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
#[doc = "Phy Maintenance Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`man::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`man::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAN_SPEC;
impl crate::RegisterSpec for MAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`man::R`](R) reader structure"]
impl crate::Readable for MAN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`man::W`](W) writer structure"]
impl crate::Writable for MAN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAN to value 0"]
impl crate::Resettable for MAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
