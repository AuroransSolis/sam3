#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `ENABLE` reader - CRC Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - CRC Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPARE` reader - CRC Compare"]
pub type CompareR = crate::BitReader;
#[doc = "Field `COMPARE` writer - CRC Compare"]
pub type CompareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Primitive Polynomial\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ptype {
    #[doc = "0: Polynom 0x04C11DB7"]
    Ccitt8023 = 0,
    #[doc = "1: Polynom 0x1EDC6F41"]
    Castagnoli = 1,
    #[doc = "2: Polynom 0x1021"]
    Ccitt16 = 2,
}
impl From<Ptype> for u8 {
    #[inline(always)]
    fn from(variant: Ptype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ptype {
    type Ux = u8;
}
impl crate::IsEnum for Ptype {}
#[doc = "Field `PTYPE` reader - Primitive Polynomial"]
pub type PtypeR = crate::FieldReader<Ptype>;
impl PtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ptype> {
        match self.bits {
            0 => Some(Ptype::Ccitt8023),
            1 => Some(Ptype::Castagnoli),
            2 => Some(Ptype::Ccitt16),
            _ => None,
        }
    }
    #[doc = "Polynom 0x04C11DB7"]
    #[inline(always)]
    pub fn is_ccitt8023(&self) -> bool {
        *self == Ptype::Ccitt8023
    }
    #[doc = "Polynom 0x1EDC6F41"]
    #[inline(always)]
    pub fn is_castagnoli(&self) -> bool {
        *self == Ptype::Castagnoli
    }
    #[doc = "Polynom 0x1021"]
    #[inline(always)]
    pub fn is_ccitt16(&self) -> bool {
        *self == Ptype::Ccitt16
    }
}
#[doc = "Field `PTYPE` writer - Primitive Polynomial"]
pub type PtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ptype>;
impl<'a, REG> PtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Polynom 0x04C11DB7"]
    #[inline(always)]
    pub fn ccitt8023(self) -> &'a mut crate::W<REG> {
        self.variant(Ptype::Ccitt8023)
    }
    #[doc = "Polynom 0x1EDC6F41"]
    #[inline(always)]
    pub fn castagnoli(self) -> &'a mut crate::W<REG> {
        self.variant(Ptype::Castagnoli)
    }
    #[doc = "Polynom 0x1021"]
    #[inline(always)]
    pub fn ccitt16(self) -> &'a mut crate::W<REG> {
        self.variant(Ptype::Ccitt16)
    }
}
#[doc = "Field `DIVIDER` reader - Request Divider"]
pub type DividerR = crate::FieldReader;
#[doc = "Field `DIVIDER` writer - Request Divider"]
pub type DividerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC Compare"]
    #[inline(always)]
    pub fn compare(&self) -> CompareR {
        CompareR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Primitive Polynomial"]
    #[inline(always)]
    pub fn ptype(&self) -> PtypeR {
        PtypeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Request Divider"]
    #[inline(always)]
    pub fn divider(&self) -> DividerR {
        DividerR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<MrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - CRC Compare"]
    #[inline(always)]
    #[must_use]
    pub fn compare(&mut self) -> CompareW<MrSpec> {
        CompareW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Primitive Polynomial"]
    #[inline(always)]
    #[must_use]
    pub fn ptype(&mut self) -> PtypeW<MrSpec> {
        PtypeW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Request Divider"]
    #[inline(always)]
    #[must_use]
    pub fn divider(&mut self) -> DividerW<MrSpec> {
        DividerW::new(self, 4)
    }
}
#[doc = "CRCCU Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0;
}
