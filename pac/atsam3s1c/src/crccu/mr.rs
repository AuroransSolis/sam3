#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `ENABLE` reader - CRC Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - CRC Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPARE` reader - CRC Compare"]
pub type COMPARE_R = crate::BitReader;
#[doc = "Field `COMPARE` writer - CRC Compare"]
pub type COMPARE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTYPE` reader - Primitive Polynomial"]
pub type PTYPE_R = crate::FieldReader<PTYPE_A>;
#[doc = "Primitive Polynomial\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTYPE_A {
    #[doc = "0: Polynom 0x04C11DB7"]
    Ccitt8023 = 0,
    #[doc = "1: Polynom 0x1EDC6F41"]
    Castagnoli = 1,
    #[doc = "2: Polynom 0x1021"]
    Ccitt16 = 2,
}
impl From<PTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: PTYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PTYPE_A {
    type Ux = u8;
}
impl PTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PTYPE_A> {
        match self.bits {
            0 => Some(PTYPE_A::Ccitt8023),
            1 => Some(PTYPE_A::Castagnoli),
            2 => Some(PTYPE_A::Ccitt16),
            _ => None,
        }
    }
    #[doc = "Polynom 0x04C11DB7"]
    #[inline(always)]
    pub fn is_ccitt8023(&self) -> bool {
        *self == PTYPE_A::Ccitt8023
    }
    #[doc = "Polynom 0x1EDC6F41"]
    #[inline(always)]
    pub fn is_castagnoli(&self) -> bool {
        *self == PTYPE_A::Castagnoli
    }
    #[doc = "Polynom 0x1021"]
    #[inline(always)]
    pub fn is_ccitt16(&self) -> bool {
        *self == PTYPE_A::Ccitt16
    }
}
#[doc = "Field `PTYPE` writer - Primitive Polynomial"]
pub type PTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PTYPE_A>;
impl<'a, REG> PTYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Polynom 0x04C11DB7"]
    #[inline(always)]
    pub fn ccitt8023(self) -> &'a mut crate::W<REG> {
        self.variant(PTYPE_A::Ccitt8023)
    }
    #[doc = "Polynom 0x1EDC6F41"]
    #[inline(always)]
    pub fn castagnoli(self) -> &'a mut crate::W<REG> {
        self.variant(PTYPE_A::Castagnoli)
    }
    #[doc = "Polynom 0x1021"]
    #[inline(always)]
    pub fn ccitt16(self) -> &'a mut crate::W<REG> {
        self.variant(PTYPE_A::Ccitt16)
    }
}
#[doc = "Field `DIVIDER` reader - Request Divider"]
pub type DIVIDER_R = crate::FieldReader;
#[doc = "Field `DIVIDER` writer - Request Divider"]
pub type DIVIDER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC Compare"]
    #[inline(always)]
    pub fn compare(&self) -> COMPARE_R {
        COMPARE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Primitive Polynomial"]
    #[inline(always)]
    pub fn ptype(&self) -> PTYPE_R {
        PTYPE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Request Divider"]
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<MR_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - CRC Compare"]
    #[inline(always)]
    #[must_use]
    pub fn compare(&mut self) -> COMPARE_W<MR_SPEC> {
        COMPARE_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Primitive Polynomial"]
    #[inline(always)]
    #[must_use]
    pub fn ptype(&mut self) -> PTYPE_W<MR_SPEC> {
        PTYPE_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - Request Divider"]
    #[inline(always)]
    #[must_use]
    pub fn divider(&mut self) -> DIVIDER_W<MR_SPEC> {
        DIVIDER_W::new(self, 4)
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
#[doc = "CRCCU Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: u32 = 0;
}
