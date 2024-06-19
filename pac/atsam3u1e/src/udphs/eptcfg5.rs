#[doc = "Register `EPTCFG5` reader"]
pub type R = crate::R<Eptcfg5Spec>;
#[doc = "Register `EPTCFG5` writer"]
pub type W = crate::W<Eptcfg5Spec>;
#[doc = "Endpoint Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EptSize {
    #[doc = "0: 8 bytes"]
    _8 = 0,
    #[doc = "1: 16 bytes"]
    _16 = 1,
    #[doc = "2: 32 bytes"]
    _32 = 2,
    #[doc = "3: 64 bytes"]
    _64 = 3,
    #[doc = "4: 128 bytes"]
    _128 = 4,
    #[doc = "5: 256 bytes"]
    _256 = 5,
    #[doc = "6: 512 bytes"]
    _512 = 6,
    #[doc = "7: 1024 bytes"]
    _1024 = 7,
}
impl From<EptSize> for u8 {
    #[inline(always)]
    fn from(variant: EptSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EptSize {
    type Ux = u8;
}
impl crate::IsEnum for EptSize {}
#[doc = "Field `EPT_SIZE` reader - Endpoint Size"]
pub type EptSizeR = crate::FieldReader<EptSize>;
impl EptSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EptSize {
        match self.bits {
            0 => EptSize::_8,
            1 => EptSize::_16,
            2 => EptSize::_32,
            3 => EptSize::_64,
            4 => EptSize::_128,
            5 => EptSize::_256,
            6 => EptSize::_512,
            7 => EptSize::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == EptSize::_8
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == EptSize::_16
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == EptSize::_32
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == EptSize::_64
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == EptSize::_128
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == EptSize::_256
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == EptSize::_512
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == EptSize::_1024
    }
}
#[doc = "Field `EPT_SIZE` writer - Endpoint Size"]
pub type EptSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, EptSize, crate::Safe>;
impl<'a, REG> EptSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(EptSize::_8)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(EptSize::_16)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(EptSize::_32)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(EptSize::_64)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(EptSize::_128)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(EptSize::_256)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(EptSize::_512)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(EptSize::_1024)
    }
}
#[doc = "Field `EPT_DIR` reader - Endpoint Direction"]
pub type EptDirR = crate::BitReader;
#[doc = "Field `EPT_DIR` writer - Endpoint Direction"]
pub type EptDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EptType {
    #[doc = "0: Control endpoint"]
    Ctrl8 = 0,
    #[doc = "1: Isochronous endpoint"]
    Iso = 1,
    #[doc = "2: Bulk endpoint"]
    Bulk = 2,
    #[doc = "3: Interrupt endpoint"]
    Int = 3,
}
impl From<EptType> for u8 {
    #[inline(always)]
    fn from(variant: EptType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EptType {
    type Ux = u8;
}
impl crate::IsEnum for EptType {}
#[doc = "Field `EPT_TYPE` reader - Endpoint Type"]
pub type EptTypeR = crate::FieldReader<EptType>;
impl EptTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EptType {
        match self.bits {
            0 => EptType::Ctrl8,
            1 => EptType::Iso,
            2 => EptType::Bulk,
            3 => EptType::Int,
            _ => unreachable!(),
        }
    }
    #[doc = "Control endpoint"]
    #[inline(always)]
    pub fn is_ctrl8(&self) -> bool {
        *self == EptType::Ctrl8
    }
    #[doc = "Isochronous endpoint"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == EptType::Iso
    }
    #[doc = "Bulk endpoint"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == EptType::Bulk
    }
    #[doc = "Interrupt endpoint"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == EptType::Int
    }
}
#[doc = "Field `EPT_TYPE` writer - Endpoint Type"]
pub type EptTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, EptType, crate::Safe>;
impl<'a, REG> EptTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control endpoint"]
    #[inline(always)]
    pub fn ctrl8(self) -> &'a mut crate::W<REG> {
        self.variant(EptType::Ctrl8)
    }
    #[doc = "Isochronous endpoint"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(EptType::Iso)
    }
    #[doc = "Bulk endpoint"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut crate::W<REG> {
        self.variant(EptType::Bulk)
    }
    #[doc = "Interrupt endpoint"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(EptType::Int)
    }
}
#[doc = "Number of Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BkNumber {
    #[doc = "0: Zero bank, the endpoint is not mapped in memory"]
    _0 = 0,
    #[doc = "1: One bank (bank 0)"]
    _1 = 1,
    #[doc = "2: Double bank (Ping-Pong: bank0/bank1)"]
    _2 = 2,
    #[doc = "3: Triple bank (bank0/bank1/bank2)"]
    _3 = 3,
}
impl From<BkNumber> for u8 {
    #[inline(always)]
    fn from(variant: BkNumber) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BkNumber {
    type Ux = u8;
}
impl crate::IsEnum for BkNumber {}
#[doc = "Field `BK_NUMBER` reader - Number of Banks"]
pub type BkNumberR = crate::FieldReader<BkNumber>;
impl BkNumberR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BkNumber {
        match self.bits {
            0 => BkNumber::_0,
            1 => BkNumber::_1,
            2 => BkNumber::_2,
            3 => BkNumber::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Zero bank, the endpoint is not mapped in memory"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BkNumber::_0
    }
    #[doc = "One bank (bank 0)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BkNumber::_1
    }
    #[doc = "Double bank (Ping-Pong: bank0/bank1)"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == BkNumber::_2
    }
    #[doc = "Triple bank (bank0/bank1/bank2)"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == BkNumber::_3
    }
}
#[doc = "Field `BK_NUMBER` writer - Number of Banks"]
pub type BkNumberW<'a, REG> = crate::FieldWriter<'a, REG, 2, BkNumber, crate::Safe>;
impl<'a, REG> BkNumberW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zero bank, the endpoint is not mapped in memory"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BkNumber::_0)
    }
    #[doc = "One bank (bank 0)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BkNumber::_1)
    }
    #[doc = "Double bank (Ping-Pong: bank0/bank1)"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(BkNumber::_2)
    }
    #[doc = "Triple bank (bank0/bank1/bank2)"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(BkNumber::_3)
    }
}
#[doc = "Field `NB_TRANS` reader - Number Of Transaction per Microframe"]
pub type NbTransR = crate::FieldReader;
#[doc = "Field `NB_TRANS` writer - Number Of Transaction per Microframe"]
pub type NbTransW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EPT_MAPD` reader - Endpoint Mapped"]
pub type EptMapdR = crate::BitReader;
#[doc = "Field `EPT_MAPD` writer - Endpoint Mapped"]
pub type EptMapdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Endpoint Size"]
    #[inline(always)]
    pub fn ept_size(&self) -> EptSizeR {
        EptSizeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Endpoint Direction"]
    #[inline(always)]
    pub fn ept_dir(&self) -> EptDirR {
        EptDirR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Endpoint Type"]
    #[inline(always)]
    pub fn ept_type(&self) -> EptTypeR {
        EptTypeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Number of Banks"]
    #[inline(always)]
    pub fn bk_number(&self) -> BkNumberR {
        BkNumberR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Number Of Transaction per Microframe"]
    #[inline(always)]
    pub fn nb_trans(&self) -> NbTransR {
        NbTransR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 31 - Endpoint Mapped"]
    #[inline(always)]
    pub fn ept_mapd(&self) -> EptMapdR {
        EptMapdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Endpoint Size"]
    #[inline(always)]
    #[must_use]
    pub fn ept_size(&mut self) -> EptSizeW<Eptcfg5Spec> {
        EptSizeW::new(self, 0)
    }
    #[doc = "Bit 3 - Endpoint Direction"]
    #[inline(always)]
    #[must_use]
    pub fn ept_dir(&mut self) -> EptDirW<Eptcfg5Spec> {
        EptDirW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Endpoint Type"]
    #[inline(always)]
    #[must_use]
    pub fn ept_type(&mut self) -> EptTypeW<Eptcfg5Spec> {
        EptTypeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Number of Banks"]
    #[inline(always)]
    #[must_use]
    pub fn bk_number(&mut self) -> BkNumberW<Eptcfg5Spec> {
        BkNumberW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Number Of Transaction per Microframe"]
    #[inline(always)]
    #[must_use]
    pub fn nb_trans(&mut self) -> NbTransW<Eptcfg5Spec> {
        NbTransW::new(self, 8)
    }
    #[doc = "Bit 31 - Endpoint Mapped"]
    #[inline(always)]
    #[must_use]
    pub fn ept_mapd(&mut self) -> EptMapdW<Eptcfg5Spec> {
        EptMapdW::new(self, 31)
    }
}
#[doc = "UDPHS Endpoint Configuration Register (endpoint = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`eptcfg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eptcfg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eptcfg5Spec;
impl crate::RegisterSpec for Eptcfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eptcfg5::R`](R) reader structure"]
impl crate::Readable for Eptcfg5Spec {}
#[doc = "`write(|w| ..)` method takes [`eptcfg5::W`](W) writer structure"]
impl crate::Writable for Eptcfg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPTCFG5 to value 0"]
impl crate::Resettable for Eptcfg5Spec {
    const RESET_VALUE: u32 = 0;
}
