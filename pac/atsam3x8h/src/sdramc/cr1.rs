#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Number of Column Bits\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nc {
    #[doc = "0: 8 column bits"]
    Col8 = 0,
    #[doc = "1: 9 column bits"]
    Col9 = 1,
    #[doc = "2: 10 column bits"]
    Col10 = 2,
    #[doc = "3: 11 column bits"]
    Col11 = 3,
}
impl From<Nc> for u8 {
    #[inline(always)]
    fn from(variant: Nc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nc {
    type Ux = u8;
}
impl crate::IsEnum for Nc {}
#[doc = "Field `NC` reader - Number of Column Bits"]
pub type NcR = crate::FieldReader<Nc>;
impl NcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nc {
        match self.bits {
            0 => Nc::Col8,
            1 => Nc::Col9,
            2 => Nc::Col10,
            3 => Nc::Col11,
            _ => unreachable!(),
        }
    }
    #[doc = "8 column bits"]
    #[inline(always)]
    pub fn is_col8(&self) -> bool {
        *self == Nc::Col8
    }
    #[doc = "9 column bits"]
    #[inline(always)]
    pub fn is_col9(&self) -> bool {
        *self == Nc::Col9
    }
    #[doc = "10 column bits"]
    #[inline(always)]
    pub fn is_col10(&self) -> bool {
        *self == Nc::Col10
    }
    #[doc = "11 column bits"]
    #[inline(always)]
    pub fn is_col11(&self) -> bool {
        *self == Nc::Col11
    }
}
#[doc = "Field `NC` writer - Number of Column Bits"]
pub type NcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nc, crate::Safe>;
impl<'a, REG> NcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 column bits"]
    #[inline(always)]
    pub fn col8(self) -> &'a mut crate::W<REG> {
        self.variant(Nc::Col8)
    }
    #[doc = "9 column bits"]
    #[inline(always)]
    pub fn col9(self) -> &'a mut crate::W<REG> {
        self.variant(Nc::Col9)
    }
    #[doc = "10 column bits"]
    #[inline(always)]
    pub fn col10(self) -> &'a mut crate::W<REG> {
        self.variant(Nc::Col10)
    }
    #[doc = "11 column bits"]
    #[inline(always)]
    pub fn col11(self) -> &'a mut crate::W<REG> {
        self.variant(Nc::Col11)
    }
}
#[doc = "Number of Row Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nr {
    #[doc = "0: 11 row bits"]
    Row11 = 0,
    #[doc = "1: 12 row bits"]
    Row12 = 1,
    #[doc = "2: 13 row bits"]
    Row13 = 2,
}
impl From<Nr> for u8 {
    #[inline(always)]
    fn from(variant: Nr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nr {
    type Ux = u8;
}
impl crate::IsEnum for Nr {}
#[doc = "Field `NR` reader - Number of Row Bits"]
pub type NrR = crate::FieldReader<Nr>;
impl NrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nr> {
        match self.bits {
            0 => Some(Nr::Row11),
            1 => Some(Nr::Row12),
            2 => Some(Nr::Row13),
            _ => None,
        }
    }
    #[doc = "11 row bits"]
    #[inline(always)]
    pub fn is_row11(&self) -> bool {
        *self == Nr::Row11
    }
    #[doc = "12 row bits"]
    #[inline(always)]
    pub fn is_row12(&self) -> bool {
        *self == Nr::Row12
    }
    #[doc = "13 row bits"]
    #[inline(always)]
    pub fn is_row13(&self) -> bool {
        *self == Nr::Row13
    }
}
#[doc = "Field `NR` writer - Number of Row Bits"]
pub type NrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nr>;
impl<'a, REG> NrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "11 row bits"]
    #[inline(always)]
    pub fn row11(self) -> &'a mut crate::W<REG> {
        self.variant(Nr::Row11)
    }
    #[doc = "12 row bits"]
    #[inline(always)]
    pub fn row12(self) -> &'a mut crate::W<REG> {
        self.variant(Nr::Row12)
    }
    #[doc = "13 row bits"]
    #[inline(always)]
    pub fn row13(self) -> &'a mut crate::W<REG> {
        self.variant(Nr::Row13)
    }
}
#[doc = "Number of Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nb {
    #[doc = "0: 2 banks"]
    Bank2 = 0,
    #[doc = "1: 4 banks"]
    Bank4 = 1,
}
impl From<Nb> for bool {
    #[inline(always)]
    fn from(variant: Nb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NB` reader - Number of Banks"]
pub type NbR = crate::BitReader<Nb>;
impl NbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nb {
        match self.bits {
            false => Nb::Bank2,
            true => Nb::Bank4,
        }
    }
    #[doc = "2 banks"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == Nb::Bank2
    }
    #[doc = "4 banks"]
    #[inline(always)]
    pub fn is_bank4(&self) -> bool {
        *self == Nb::Bank4
    }
}
#[doc = "Field `NB` writer - Number of Banks"]
pub type NbW<'a, REG> = crate::BitWriter<'a, REG, Nb>;
impl<'a, REG> NbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "2 banks"]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut crate::W<REG> {
        self.variant(Nb::Bank2)
    }
    #[doc = "4 banks"]
    #[inline(always)]
    pub fn bank4(self) -> &'a mut crate::W<REG> {
        self.variant(Nb::Bank4)
    }
}
#[doc = "CAS Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cas {
    #[doc = "1: 1 cycle CAS latency"]
    Latency1 = 1,
    #[doc = "2: 2 cycle CAS latency"]
    Latency2 = 2,
    #[doc = "3: 3 cycle CAS latency"]
    Latency3 = 3,
}
impl From<Cas> for u8 {
    #[inline(always)]
    fn from(variant: Cas) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cas {
    type Ux = u8;
}
impl crate::IsEnum for Cas {}
#[doc = "Field `CAS` reader - CAS Latency"]
pub type CasR = crate::FieldReader<Cas>;
impl CasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cas> {
        match self.bits {
            1 => Some(Cas::Latency1),
            2 => Some(Cas::Latency2),
            3 => Some(Cas::Latency3),
            _ => None,
        }
    }
    #[doc = "1 cycle CAS latency"]
    #[inline(always)]
    pub fn is_latency1(&self) -> bool {
        *self == Cas::Latency1
    }
    #[doc = "2 cycle CAS latency"]
    #[inline(always)]
    pub fn is_latency2(&self) -> bool {
        *self == Cas::Latency2
    }
    #[doc = "3 cycle CAS latency"]
    #[inline(always)]
    pub fn is_latency3(&self) -> bool {
        *self == Cas::Latency3
    }
}
#[doc = "Field `CAS` writer - CAS Latency"]
pub type CasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cas>;
impl<'a, REG> CasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 cycle CAS latency"]
    #[inline(always)]
    pub fn latency1(self) -> &'a mut crate::W<REG> {
        self.variant(Cas::Latency1)
    }
    #[doc = "2 cycle CAS latency"]
    #[inline(always)]
    pub fn latency2(self) -> &'a mut crate::W<REG> {
        self.variant(Cas::Latency2)
    }
    #[doc = "3 cycle CAS latency"]
    #[inline(always)]
    pub fn latency3(self) -> &'a mut crate::W<REG> {
        self.variant(Cas::Latency3)
    }
}
#[doc = "Field `DBW` reader - Data Bus Width"]
pub type DbwR = crate::BitReader;
#[doc = "Field `DBW` writer - Data Bus Width"]
pub type DbwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWR` reader - Write Recovery Delay"]
pub type TwrR = crate::FieldReader;
#[doc = "Field `TWR` writer - Write Recovery Delay"]
pub type TwrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRC_TRFC` reader - Row Cycle Delay and Row Refresh Cycle"]
pub type TrcTrfcR = crate::FieldReader;
#[doc = "Field `TRC_TRFC` writer - Row Cycle Delay and Row Refresh Cycle"]
pub type TrcTrfcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRP` reader - Row Precharge Delay"]
pub type TrpR = crate::FieldReader;
#[doc = "Field `TRP` writer - Row Precharge Delay"]
pub type TrpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRCD` reader - Row to Column Delay"]
pub type TrcdR = crate::FieldReader;
#[doc = "Field `TRCD` writer - Row to Column Delay"]
pub type TrcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRAS` reader - Active to Precharge Delay"]
pub type TrasR = crate::FieldReader;
#[doc = "Field `TRAS` writer - Active to Precharge Delay"]
pub type TrasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXSR` reader - Exit Self Refresh to Active Delay"]
pub type TxsrR = crate::FieldReader;
#[doc = "Field `TXSR` writer - Exit Self Refresh to Active Delay"]
pub type TxsrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Number of Column Bits"]
    #[inline(always)]
    pub fn nc(&self) -> NcR {
        NcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Number of Row Bits"]
    #[inline(always)]
    pub fn nr(&self) -> NrR {
        NrR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Number of Banks"]
    #[inline(always)]
    pub fn nb(&self) -> NbR {
        NbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - CAS Latency"]
    #[inline(always)]
    pub fn cas(&self) -> CasR {
        CasR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DbwR {
        DbwR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Write Recovery Delay"]
    #[inline(always)]
    pub fn twr(&self) -> TwrR {
        TwrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Row Cycle Delay and Row Refresh Cycle"]
    #[inline(always)]
    pub fn trc_trfc(&self) -> TrcTrfcR {
        TrcTrfcR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Row Precharge Delay"]
    #[inline(always)]
    pub fn trp(&self) -> TrpR {
        TrpR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Row to Column Delay"]
    #[inline(always)]
    pub fn trcd(&self) -> TrcdR {
        TrcdR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Active to Precharge Delay"]
    #[inline(always)]
    pub fn tras(&self) -> TrasR {
        TrasR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Exit Self Refresh to Active Delay"]
    #[inline(always)]
    pub fn txsr(&self) -> TxsrR {
        TxsrR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of Column Bits"]
    #[inline(always)]
    #[must_use]
    pub fn nc(&mut self) -> NcW<Cr1Spec> {
        NcW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Number of Row Bits"]
    #[inline(always)]
    #[must_use]
    pub fn nr(&mut self) -> NrW<Cr1Spec> {
        NrW::new(self, 2)
    }
    #[doc = "Bit 4 - Number of Banks"]
    #[inline(always)]
    #[must_use]
    pub fn nb(&mut self) -> NbW<Cr1Spec> {
        NbW::new(self, 4)
    }
    #[doc = "Bits 5:6 - CAS Latency"]
    #[inline(always)]
    #[must_use]
    pub fn cas(&mut self) -> CasW<Cr1Spec> {
        CasW::new(self, 5)
    }
    #[doc = "Bit 7 - Data Bus Width"]
    #[inline(always)]
    #[must_use]
    pub fn dbw(&mut self) -> DbwW<Cr1Spec> {
        DbwW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Write Recovery Delay"]
    #[inline(always)]
    #[must_use]
    pub fn twr(&mut self) -> TwrW<Cr1Spec> {
        TwrW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Row Cycle Delay and Row Refresh Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn trc_trfc(&mut self) -> TrcTrfcW<Cr1Spec> {
        TrcTrfcW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Row Precharge Delay"]
    #[inline(always)]
    #[must_use]
    pub fn trp(&mut self) -> TrpW<Cr1Spec> {
        TrpW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Row to Column Delay"]
    #[inline(always)]
    #[must_use]
    pub fn trcd(&mut self) -> TrcdW<Cr1Spec> {
        TrcdW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Active to Precharge Delay"]
    #[inline(always)]
    #[must_use]
    pub fn tras(&mut self) -> TrasW<Cr1Spec> {
        TrasW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Exit Self Refresh to Active Delay"]
    #[inline(always)]
    #[must_use]
    pub fn txsr(&mut self) -> TxsrW<Cr1Spec> {
        TxsrW::new(self, 28)
    }
}
#[doc = "SDRAMC Configuration Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0x02"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0x02;
}
