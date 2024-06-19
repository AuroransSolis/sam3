#[doc = "Register `MMR` reader"]
pub type R = crate::R<MmrSpec>;
#[doc = "Register `MMR` writer"]
pub type W = crate::W<MmrSpec>;
#[doc = "Internal Device Address Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iadrsz {
    #[doc = "0: No internal device address"]
    None = 0,
    #[doc = "1: One-byte internal device address"]
    _1Byte = 1,
    #[doc = "2: Two-byte internal device address"]
    _2Byte = 2,
    #[doc = "3: Three-byte internal device address"]
    _3Byte = 3,
}
impl From<Iadrsz> for u8 {
    #[inline(always)]
    fn from(variant: Iadrsz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iadrsz {
    type Ux = u8;
}
impl crate::IsEnum for Iadrsz {}
#[doc = "Field `IADRSZ` reader - Internal Device Address Size"]
pub type IadrszR = crate::FieldReader<Iadrsz>;
impl IadrszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iadrsz {
        match self.bits {
            0 => Iadrsz::None,
            1 => Iadrsz::_1Byte,
            2 => Iadrsz::_2Byte,
            3 => Iadrsz::_3Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "No internal device address"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Iadrsz::None
    }
    #[doc = "One-byte internal device address"]
    #[inline(always)]
    pub fn is_1_byte(&self) -> bool {
        *self == Iadrsz::_1Byte
    }
    #[doc = "Two-byte internal device address"]
    #[inline(always)]
    pub fn is_2_byte(&self) -> bool {
        *self == Iadrsz::_2Byte
    }
    #[doc = "Three-byte internal device address"]
    #[inline(always)]
    pub fn is_3_byte(&self) -> bool {
        *self == Iadrsz::_3Byte
    }
}
#[doc = "Field `IADRSZ` writer - Internal Device Address Size"]
pub type IadrszW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iadrsz, crate::Safe>;
impl<'a, REG> IadrszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No internal device address"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Iadrsz::None)
    }
    #[doc = "One-byte internal device address"]
    #[inline(always)]
    pub fn _1_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Iadrsz::_1Byte)
    }
    #[doc = "Two-byte internal device address"]
    #[inline(always)]
    pub fn _2_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Iadrsz::_2Byte)
    }
    #[doc = "Three-byte internal device address"]
    #[inline(always)]
    pub fn _3_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Iadrsz::_3Byte)
    }
}
#[doc = "Field `MREAD` reader - Master Read Direction"]
pub type MreadR = crate::BitReader;
#[doc = "Field `MREAD` writer - Master Read Direction"]
pub type MreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DADR` reader - Device Address"]
pub type DadrR = crate::FieldReader;
#[doc = "Field `DADR` writer - Device Address"]
pub type DadrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline(always)]
    pub fn iadrsz(&self) -> IadrszR {
        IadrszR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline(always)]
    pub fn mread(&self) -> MreadR {
        MreadR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline(always)]
    pub fn dadr(&self) -> DadrR {
        DadrR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline(always)]
    #[must_use]
    pub fn iadrsz(&mut self) -> IadrszW<MmrSpec> {
        IadrszW::new(self, 8)
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline(always)]
    #[must_use]
    pub fn mread(&mut self) -> MreadW<MmrSpec> {
        MreadW::new(self, 12)
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn dadr(&mut self) -> DadrW<MmrSpec> {
        DadrW::new(self, 16)
    }
}
#[doc = "Master Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmrSpec;
impl crate::RegisterSpec for MmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr::R`](R) reader structure"]
impl crate::Readable for MmrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr::W`](W) writer structure"]
impl crate::Writable for MmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR to value 0"]
impl crate::Resettable for MmrSpec {
    const RESET_VALUE: u32 = 0;
}
