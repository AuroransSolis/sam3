#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "USART Mode of Operation"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UsartMode {
    #[doc = "0: Normal mode"]
    Normal = 0,
    #[doc = "1: RS485"]
    Rs485 = 1,
    #[doc = "2: Hardware Handshaking"]
    HwHandshaking = 2,
    #[doc = "3: Modem"]
    Modem = 3,
    #[doc = "4: IS07816 Protocol: T = 0"]
    Is07816T0 = 4,
    #[doc = "6: IS07816 Protocol: T = 1"]
    Is07816T1 = 6,
    #[doc = "8: IrDA"]
    Irda = 8,
    #[doc = "14: SPI Master"]
    SpiMaster = 14,
    #[doc = "15: SPI Slave"]
    SpiSlave = 15,
}
impl From<UsartMode> for u8 {
    #[inline(always)]
    fn from(variant: UsartMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UsartMode {
    type Ux = u8;
}
#[doc = "Field `USART_MODE` reader - USART Mode of Operation"]
pub type UsartModeR = crate::FieldReader<UsartMode>;
impl UsartModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UsartMode> {
        match self.bits {
            0 => Some(UsartMode::Normal),
            1 => Some(UsartMode::Rs485),
            2 => Some(UsartMode::HwHandshaking),
            3 => Some(UsartMode::Modem),
            4 => Some(UsartMode::Is07816T0),
            6 => Some(UsartMode::Is07816T1),
            8 => Some(UsartMode::Irda),
            14 => Some(UsartMode::SpiMaster),
            15 => Some(UsartMode::SpiSlave),
            _ => None,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == UsartMode::Normal
    }
    #[doc = "RS485"]
    #[inline(always)]
    pub fn is_rs485(&self) -> bool {
        *self == UsartMode::Rs485
    }
    #[doc = "Hardware Handshaking"]
    #[inline(always)]
    pub fn is_hw_handshaking(&self) -> bool {
        *self == UsartMode::HwHandshaking
    }
    #[doc = "Modem"]
    #[inline(always)]
    pub fn is_modem(&self) -> bool {
        *self == UsartMode::Modem
    }
    #[doc = "IS07816 Protocol: T = 0"]
    #[inline(always)]
    pub fn is_is07816_t_0(&self) -> bool {
        *self == UsartMode::Is07816T0
    }
    #[doc = "IS07816 Protocol: T = 1"]
    #[inline(always)]
    pub fn is_is07816_t_1(&self) -> bool {
        *self == UsartMode::Is07816T1
    }
    #[doc = "IrDA"]
    #[inline(always)]
    pub fn is_irda(&self) -> bool {
        *self == UsartMode::Irda
    }
    #[doc = "SPI Master"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == UsartMode::SpiMaster
    }
    #[doc = "SPI Slave"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == UsartMode::SpiSlave
    }
}
#[doc = "Field `USART_MODE` writer - USART Mode of Operation"]
pub type UsartModeW<'a, REG> = crate::FieldWriter<'a, REG, 4, UsartMode>;
impl<'a, REG> UsartModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(UsartMode::Normal)
    }
    #[doc = "RS485"]
    #[inline(always)]
    pub fn rs485(self) -> &'a mut crate::W<REG> {
        self.variant(UsartMode::Rs485)
    }
    #[doc = "Hardware Handshaking"]
    #[inline(always)]
    pub fn hw_handshaking(self) -> &'a mut crate::W<REG> {
        self.variant(UsartMode::HwHandshaking)
    }
    #[doc = "Modem"]
    #[inline(always)]
    pub fn modem(self) -> &'a mut crate::W<REG> {
        self.variant(UsartMode::Modem)
    }
    #[doc = "IS07816 Protocol: T = 0"]
    #[inline(always)]
    pub fn is07816_t_0(self) -> &'a mut crate::W<REG> {
        self.variant(UsartMode::Is07816T0)
    }
    #[doc = "IS07816 Protocol: T = 1"]
    #[inline(always)]
    pub fn is07816_t_1(self) -> &'a mut crate::W<REG> {
        self.variant(UsartMode::Is07816T1)
    }
    #[doc = "IrDA"]
    #[inline(always)]
    pub fn irda(self) -> &'a mut crate::W<REG> {
        self.variant(UsartMode::Irda)
    }
    #[doc = "SPI Master"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(UsartMode::SpiMaster)
    }
    #[doc = "SPI Slave"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut crate::W<REG> {
        self.variant(UsartMode::SpiSlave)
    }
}
#[doc = "Clock Selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usclks {
    #[doc = "0: Master Clock MCK is selected"]
    Mck = 0,
    #[doc = "1: Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    Div = 1,
    #[doc = "3: Serial Clock SLK is selected"]
    Sck = 3,
}
impl From<Usclks> for u8 {
    #[inline(always)]
    fn from(variant: Usclks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usclks {
    type Ux = u8;
}
#[doc = "Field `USCLKS` reader - Clock Selection"]
pub type UsclksR = crate::FieldReader<Usclks>;
impl UsclksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Usclks> {
        match self.bits {
            0 => Some(Usclks::Mck),
            1 => Some(Usclks::Div),
            3 => Some(Usclks::Sck),
            _ => None,
        }
    }
    #[doc = "Master Clock MCK is selected"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Usclks::Mck
    }
    #[doc = "Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == Usclks::Div
    }
    #[doc = "Serial Clock SLK is selected"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == Usclks::Sck
    }
}
#[doc = "Field `USCLKS` writer - Clock Selection"]
pub type UsclksW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usclks>;
impl<'a, REG> UsclksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Master Clock MCK is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(Usclks::Mck)
    }
    #[doc = "Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    #[inline(always)]
    pub fn div(self) -> &'a mut crate::W<REG> {
        self.variant(Usclks::Div)
    }
    #[doc = "Serial Clock SLK is selected"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut crate::W<REG> {
        self.variant(Usclks::Sck)
    }
}
#[doc = "Character Length."]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chrl {
    #[doc = "0: Character length is 5 bits"]
    _5Bit = 0,
    #[doc = "1: Character length is 6 bits"]
    _6Bit = 1,
    #[doc = "2: Character length is 7 bits"]
    _7Bit = 2,
    #[doc = "3: Character length is 8 bits"]
    _8Bit = 3,
}
impl From<Chrl> for u8 {
    #[inline(always)]
    fn from(variant: Chrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chrl {
    type Ux = u8;
}
#[doc = "Field `CHRL` reader - Character Length."]
pub type ChrlR = crate::FieldReader<Chrl>;
impl ChrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chrl {
        match self.bits {
            0 => Chrl::_5Bit,
            1 => Chrl::_6Bit,
            2 => Chrl::_7Bit,
            3 => Chrl::_8Bit,
            _ => unreachable!(),
        }
    }
    #[doc = "Character length is 5 bits"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == Chrl::_5Bit
    }
    #[doc = "Character length is 6 bits"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == Chrl::_6Bit
    }
    #[doc = "Character length is 7 bits"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == Chrl::_7Bit
    }
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Chrl::_8Bit
    }
}
#[doc = "Field `CHRL` writer - Character Length."]
pub type ChrlW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Chrl>;
impl<'a, REG> ChrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Character length is 5 bits"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chrl::_5Bit)
    }
    #[doc = "Character length is 6 bits"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chrl::_6Bit)
    }
    #[doc = "Character length is 7 bits"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chrl::_7Bit)
    }
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chrl::_8Bit)
    }
}
#[doc = "Field `SYNC` reader - Synchronous Mode Select"]
pub type SyncR = crate::BitReader;
#[doc = "Field `SYNC` writer - Synchronous Mode Select"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Parity Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Par {
    #[doc = "0: Even parity"]
    Even = 0,
    #[doc = "1: Odd parity"]
    Odd = 1,
    #[doc = "2: Parity forced to 0 (Space)"]
    Space = 2,
    #[doc = "3: Parity forced to 1 (Mark)"]
    Mark = 3,
    #[doc = "4: No parity"]
    No = 4,
    #[doc = "6: Multidrop mode"]
    Multidrop = 6,
}
impl From<Par> for u8 {
    #[inline(always)]
    fn from(variant: Par) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Par {
    type Ux = u8;
}
#[doc = "Field `PAR` reader - Parity Type"]
pub type ParR = crate::FieldReader<Par>;
impl ParR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Par> {
        match self.bits {
            0 => Some(Par::Even),
            1 => Some(Par::Odd),
            2 => Some(Par::Space),
            3 => Some(Par::Mark),
            4 => Some(Par::No),
            6 => Some(Par::Multidrop),
            _ => None,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Par::Even
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Par::Odd
    }
    #[doc = "Parity forced to 0 (Space)"]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        *self == Par::Space
    }
    #[doc = "Parity forced to 1 (Mark)"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        *self == Par::Mark
    }
    #[doc = "No parity"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Par::No
    }
    #[doc = "Multidrop mode"]
    #[inline(always)]
    pub fn is_multidrop(&self) -> bool {
        *self == Par::Multidrop
    }
}
#[doc = "Field `PAR` writer - Parity Type"]
pub type ParW<'a, REG> = crate::FieldWriter<'a, REG, 3, Par>;
impl<'a, REG> ParW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Par::Even)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Par::Odd)
    }
    #[doc = "Parity forced to 0 (Space)"]
    #[inline(always)]
    pub fn space(self) -> &'a mut crate::W<REG> {
        self.variant(Par::Space)
    }
    #[doc = "Parity forced to 1 (Mark)"]
    #[inline(always)]
    pub fn mark(self) -> &'a mut crate::W<REG> {
        self.variant(Par::Mark)
    }
    #[doc = "No parity"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Par::No)
    }
    #[doc = "Multidrop mode"]
    #[inline(always)]
    pub fn multidrop(self) -> &'a mut crate::W<REG> {
        self.variant(Par::Multidrop)
    }
}
#[doc = "Number of Stop Bits"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nbstop {
    #[doc = "0: 1 stop bit"]
    _1Bit = 0,
    #[doc = "1: 1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    _1_5Bit = 1,
    #[doc = "2: 2 stop bits"]
    _2Bit = 2,
}
impl From<Nbstop> for u8 {
    #[inline(always)]
    fn from(variant: Nbstop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nbstop {
    type Ux = u8;
}
#[doc = "Field `NBSTOP` reader - Number of Stop Bits"]
pub type NbstopR = crate::FieldReader<Nbstop>;
impl NbstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nbstop> {
        match self.bits {
            0 => Some(Nbstop::_1Bit),
            1 => Some(Nbstop::_1_5Bit),
            2 => Some(Nbstop::_2Bit),
            _ => None,
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_1_bit(&self) -> bool {
        *self == Nbstop::_1Bit
    }
    #[doc = "1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    #[inline(always)]
    pub fn is_1_5_bit(&self) -> bool {
        *self == Nbstop::_1_5Bit
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn is_2_bit(&self) -> bool {
        *self == Nbstop::_2Bit
    }
}
#[doc = "Field `NBSTOP` writer - Number of Stop Bits"]
pub type NbstopW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nbstop>;
impl<'a, REG> NbstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn _1_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Nbstop::_1Bit)
    }
    #[doc = "1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    #[inline(always)]
    pub fn _1_5_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Nbstop::_1_5Bit)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn _2_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Nbstop::_2Bit)
    }
}
#[doc = "Channel Mode"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chmode {
    #[doc = "0: Normal Mode"]
    Normal = 0,
    #[doc = "1: Automatic Echo. Receiver input is connected to the TXD pin."]
    Automatic = 1,
    #[doc = "2: Local Loopback. Transmitter output is connected to the Receiver Input."]
    LocalLoopback = 2,
    #[doc = "3: Remote Loopback. RXD pin is internally connected to the TXD pin."]
    RemoteLoopback = 3,
}
impl From<Chmode> for u8 {
    #[inline(always)]
    fn from(variant: Chmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chmode {
    type Ux = u8;
}
#[doc = "Field `CHMODE` reader - Channel Mode"]
pub type ChmodeR = crate::FieldReader<Chmode>;
impl ChmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chmode {
        match self.bits {
            0 => Chmode::Normal,
            1 => Chmode::Automatic,
            2 => Chmode::LocalLoopback,
            3 => Chmode::RemoteLoopback,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Chmode::Normal
    }
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin."]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == Chmode::Automatic
    }
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input."]
    #[inline(always)]
    pub fn is_local_loopback(&self) -> bool {
        *self == Chmode::LocalLoopback
    }
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin."]
    #[inline(always)]
    pub fn is_remote_loopback(&self) -> bool {
        *self == Chmode::RemoteLoopback
    }
}
#[doc = "Field `CHMODE` writer - Channel Mode"]
pub type ChmodeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Chmode>;
impl<'a, REG> ChmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Chmode::Normal)
    }
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin."]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(Chmode::Automatic)
    }
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input."]
    #[inline(always)]
    pub fn local_loopback(self) -> &'a mut crate::W<REG> {
        self.variant(Chmode::LocalLoopback)
    }
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin."]
    #[inline(always)]
    pub fn remote_loopback(self) -> &'a mut crate::W<REG> {
        self.variant(Chmode::RemoteLoopback)
    }
}
#[doc = "Field `MSBF` reader - Bit Order"]
pub type MsbfR = crate::BitReader;
#[doc = "Field `MSBF` writer - Bit Order"]
pub type MsbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE9` reader - 9-bit Character Length"]
pub type Mode9R = crate::BitReader;
#[doc = "Field `MODE9` writer - 9-bit Character Length"]
pub type Mode9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKO` reader - Clock Output Select"]
pub type ClkoR = crate::BitReader;
#[doc = "Field `CLKO` writer - Clock Output Select"]
pub type ClkoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVER` reader - Oversampling Mode"]
pub type OverR = crate::BitReader;
#[doc = "Field `OVER` writer - Oversampling Mode"]
pub type OverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INACK` reader - Inhibit Non Acknowledge"]
pub type InackR = crate::BitReader;
#[doc = "Field `INACK` writer - Inhibit Non Acknowledge"]
pub type InackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSNACK` reader - Disable Successive NACK"]
pub type DsnackR = crate::BitReader;
#[doc = "Field `DSNACK` writer - Disable Successive NACK"]
pub type DsnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VAR_SYNC` reader - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
pub type VarSyncR = crate::BitReader;
#[doc = "Field `VAR_SYNC` writer - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
pub type VarSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVDATA` reader - Inverted Data"]
pub type InvdataR = crate::BitReader;
#[doc = "Field `INVDATA` writer - Inverted Data"]
pub type InvdataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAX_ITERATION` reader - Maximum Number of Automatic Iteration"]
pub type MaxIterationR = crate::FieldReader;
#[doc = "Field `MAX_ITERATION` writer - Maximum Number of Automatic Iteration"]
pub type MaxIterationW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FILTER` reader - Infrared Receive Line Filter"]
pub type FilterR = crate::BitReader;
#[doc = "Field `FILTER` writer - Infrared Receive Line Filter"]
pub type FilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAN` reader - Manchester Encoder/Decoder Enable"]
pub type ManR = crate::BitReader;
#[doc = "Field `MAN` writer - Manchester Encoder/Decoder Enable"]
pub type ManW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODSYNC` reader - Manchester Synchronization Mode"]
pub type ModsyncR = crate::BitReader;
#[doc = "Field `MODSYNC` writer - Manchester Synchronization Mode"]
pub type ModsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONEBIT` reader - Start Frame Delimiter Selector"]
pub type OnebitR = crate::BitReader;
#[doc = "Field `ONEBIT` writer - Start Frame Delimiter Selector"]
pub type OnebitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    pub fn usart_mode(&self) -> UsartModeR {
        UsartModeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&self) -> UsclksR {
        UsclksR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Character Length."]
    #[inline(always)]
    pub fn chrl(&self) -> ChrlR {
        ChrlR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Synchronous Mode Select"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&self) -> ParR {
        ParR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline(always)]
    pub fn nbstop(&self) -> NbstopR {
        NbstopR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&self) -> ChmodeR {
        ChmodeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Bit Order"]
    #[inline(always)]
    pub fn msbf(&self) -> MsbfR {
        MsbfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline(always)]
    pub fn mode9(&self) -> Mode9R {
        Mode9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&self) -> ClkoR {
        ClkoR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline(always)]
    pub fn over(&self) -> OverR {
        OverR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline(always)]
    pub fn inack(&self) -> InackR {
        InackR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&self) -> DsnackR {
        DsnackR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
    #[inline(always)]
    pub fn var_sync(&self) -> VarSyncR {
        VarSyncR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Inverted Data"]
    #[inline(always)]
    pub fn invdata(&self) -> InvdataR {
        InvdataR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Maximum Number of Automatic Iteration"]
    #[inline(always)]
    pub fn max_iteration(&self) -> MaxIterationR {
        MaxIterationR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Infrared Receive Line Filter"]
    #[inline(always)]
    pub fn filter(&self) -> FilterR {
        FilterR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Manchester Encoder/Decoder Enable"]
    #[inline(always)]
    pub fn man(&self) -> ManR {
        ManR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Manchester Synchronization Mode"]
    #[inline(always)]
    pub fn modsync(&self) -> ModsyncR {
        ModsyncR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Start Frame Delimiter Selector"]
    #[inline(always)]
    pub fn onebit(&self) -> OnebitR {
        OnebitR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    #[must_use]
    pub fn usart_mode(&mut self) -> UsartModeW<MrSpec> {
        UsartModeW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn usclks(&mut self) -> UsclksW<MrSpec> {
        UsclksW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Character Length."]
    #[inline(always)]
    #[must_use]
    pub fn chrl(&mut self) -> ChrlW<MrSpec> {
        ChrlW::new(self, 6)
    }
    #[doc = "Bit 8 - Synchronous Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SyncW<MrSpec> {
        SyncW::new(self, 8)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    #[must_use]
    pub fn par(&mut self) -> ParW<MrSpec> {
        ParW::new(self, 9)
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline(always)]
    #[must_use]
    pub fn nbstop(&mut self) -> NbstopW<MrSpec> {
        NbstopW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    #[must_use]
    pub fn chmode(&mut self) -> ChmodeW<MrSpec> {
        ChmodeW::new(self, 14)
    }
    #[doc = "Bit 16 - Bit Order"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MsbfW<MrSpec> {
        MsbfW::new(self, 16)
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> Mode9W<MrSpec> {
        Mode9W::new(self, 17)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn clko(&mut self) -> ClkoW<MrSpec> {
        ClkoW::new(self, 18)
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline(always)]
    #[must_use]
    pub fn over(&mut self) -> OverW<MrSpec> {
        OverW::new(self, 19)
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn inack(&mut self) -> InackW<MrSpec> {
        InackW::new(self, 20)
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline(always)]
    #[must_use]
    pub fn dsnack(&mut self) -> DsnackW<MrSpec> {
        DsnackW::new(self, 21)
    }
    #[doc = "Bit 22 - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
    #[inline(always)]
    #[must_use]
    pub fn var_sync(&mut self) -> VarSyncW<MrSpec> {
        VarSyncW::new(self, 22)
    }
    #[doc = "Bit 23 - Inverted Data"]
    #[inline(always)]
    #[must_use]
    pub fn invdata(&mut self) -> InvdataW<MrSpec> {
        InvdataW::new(self, 23)
    }
    #[doc = "Bits 24:26 - Maximum Number of Automatic Iteration"]
    #[inline(always)]
    #[must_use]
    pub fn max_iteration(&mut self) -> MaxIterationW<MrSpec> {
        MaxIterationW::new(self, 24)
    }
    #[doc = "Bit 28 - Infrared Receive Line Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FilterW<MrSpec> {
        FilterW::new(self, 28)
    }
    #[doc = "Bit 29 - Manchester Encoder/Decoder Enable"]
    #[inline(always)]
    #[must_use]
    pub fn man(&mut self) -> ManW<MrSpec> {
        ManW::new(self, 29)
    }
    #[doc = "Bit 30 - Manchester Synchronization Mode"]
    #[inline(always)]
    #[must_use]
    pub fn modsync(&mut self) -> ModsyncW<MrSpec> {
        ModsyncW::new(self, 30)
    }
    #[doc = "Bit 31 - Start Frame Delimiter Selector"]
    #[inline(always)]
    #[must_use]
    pub fn onebit(&mut self) -> OnebitW<MrSpec> {
        OnebitW::new(self, 31)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
