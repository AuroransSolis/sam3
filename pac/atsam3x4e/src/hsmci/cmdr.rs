#[doc = "Register `CMDR` writer"]
pub type W = crate::W<CmdrSpec>;
#[doc = "Field `CMDNB` writer - Command Number"]
pub type CmdnbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Response Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rsptyp {
    #[doc = "0: No response"]
    Noresp = 0,
    #[doc = "1: 48-bit response"]
    _48Bit = 1,
    #[doc = "2: 136-bit response"]
    _136Bit = 2,
    #[doc = "3: R1b response type"]
    R1b = 3,
}
impl From<Rsptyp> for u8 {
    #[inline(always)]
    fn from(variant: Rsptyp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rsptyp {
    type Ux = u8;
}
impl crate::IsEnum for Rsptyp {}
#[doc = "Field `RSPTYP` writer - Response Type"]
pub type RsptypW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rsptyp, crate::Safe>;
impl<'a, REG> RsptypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No response"]
    #[inline(always)]
    pub fn noresp(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptyp::Noresp)
    }
    #[doc = "48-bit response"]
    #[inline(always)]
    pub fn _48_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptyp::_48Bit)
    }
    #[doc = "136-bit response"]
    #[inline(always)]
    pub fn _136_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptyp::_136Bit)
    }
    #[doc = "R1b response type"]
    #[inline(always)]
    pub fn r1b(self) -> &'a mut crate::W<REG> {
        self.variant(Rsptyp::R1b)
    }
}
#[doc = "Special Command"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spcmd {
    #[doc = "0: Not a special CMD."]
    Std = 0,
    #[doc = "1: Initialization CMD: 74 clock cycles for initialization sequence."]
    Init = 1,
    #[doc = "2: Synchronized CMD: Wait for the end of the current data block transfer before sending the pending command."]
    Sync = 2,
    #[doc = "3: CE-ATA Completion Signal disable Command. The host cancels the ability for the device to return a command completion signal on the command line."]
    CeAta = 3,
    #[doc = "4: Interrupt command: Corresponds to the Interrupt Mode (CMD40)."]
    ItCmd = 4,
    #[doc = "5: Interrupt response: Corresponds to the Interrupt Mode (CMD40)."]
    ItResp = 5,
    #[doc = "6: Boot Operation Request. Start a boot operation mode, the host processor can read boot data from the MMC device directly."]
    Bor = 6,
    #[doc = "7: End Boot Operation. This command allows the host processor to terminate the boot operation mode."]
    Ebo = 7,
}
impl From<Spcmd> for u8 {
    #[inline(always)]
    fn from(variant: Spcmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spcmd {
    type Ux = u8;
}
impl crate::IsEnum for Spcmd {}
#[doc = "Field `SPCMD` writer - Special Command"]
pub type SpcmdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Spcmd, crate::Safe>;
impl<'a, REG> SpcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not a special CMD."]
    #[inline(always)]
    pub fn std(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmd::Std)
    }
    #[doc = "Initialization CMD: 74 clock cycles for initialization sequence."]
    #[inline(always)]
    pub fn init(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmd::Init)
    }
    #[doc = "Synchronized CMD: Wait for the end of the current data block transfer before sending the pending command."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmd::Sync)
    }
    #[doc = "CE-ATA Completion Signal disable Command. The host cancels the ability for the device to return a command completion signal on the command line."]
    #[inline(always)]
    pub fn ce_ata(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmd::CeAta)
    }
    #[doc = "Interrupt command: Corresponds to the Interrupt Mode (CMD40)."]
    #[inline(always)]
    pub fn it_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmd::ItCmd)
    }
    #[doc = "Interrupt response: Corresponds to the Interrupt Mode (CMD40)."]
    #[inline(always)]
    pub fn it_resp(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmd::ItResp)
    }
    #[doc = "Boot Operation Request. Start a boot operation mode, the host processor can read boot data from the MMC device directly."]
    #[inline(always)]
    pub fn bor(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmd::Bor)
    }
    #[doc = "End Boot Operation. This command allows the host processor to terminate the boot operation mode."]
    #[inline(always)]
    pub fn ebo(self) -> &'a mut crate::W<REG> {
        self.variant(Spcmd::Ebo)
    }
}
#[doc = "Open Drain Command"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opdcmd {
    #[doc = "0: Push pull command."]
    Pushpull = 0,
    #[doc = "1: Open drain command."]
    Opendrain = 1,
}
impl From<Opdcmd> for bool {
    #[inline(always)]
    fn from(variant: Opdcmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPDCMD` writer - Open Drain Command"]
pub type OpdcmdW<'a, REG> = crate::BitWriter<'a, REG, Opdcmd>;
impl<'a, REG> OpdcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Push pull command."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Opdcmd::Pushpull)
    }
    #[doc = "Open drain command."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut crate::W<REG> {
        self.variant(Opdcmd::Opendrain)
    }
}
#[doc = "Max Latency for Command to Response"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Maxlat {
    #[doc = "0: 5-cycle max latency."]
    _5 = 0,
    #[doc = "1: 64-cycle max latency."]
    _64 = 1,
}
impl From<Maxlat> for bool {
    #[inline(always)]
    fn from(variant: Maxlat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXLAT` writer - Max Latency for Command to Response"]
pub type MaxlatW<'a, REG> = crate::BitWriter<'a, REG, Maxlat>;
impl<'a, REG> MaxlatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "5-cycle max latency."]
    #[inline(always)]
    pub fn _5(self) -> &'a mut crate::W<REG> {
        self.variant(Maxlat::_5)
    }
    #[doc = "64-cycle max latency."]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Maxlat::_64)
    }
}
#[doc = "Transfer Command"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trcmd {
    #[doc = "0: No data transfer"]
    NoData = 0,
    #[doc = "1: Start data transfer"]
    StartData = 1,
    #[doc = "2: Stop data transfer"]
    StopData = 2,
}
impl From<Trcmd> for u8 {
    #[inline(always)]
    fn from(variant: Trcmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trcmd {
    type Ux = u8;
}
impl crate::IsEnum for Trcmd {}
#[doc = "Field `TRCMD` writer - Transfer Command"]
pub type TrcmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trcmd>;
impl<'a, REG> TrcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No data transfer"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Trcmd::NoData)
    }
    #[doc = "Start data transfer"]
    #[inline(always)]
    pub fn start_data(self) -> &'a mut crate::W<REG> {
        self.variant(Trcmd::StartData)
    }
    #[doc = "Stop data transfer"]
    #[inline(always)]
    pub fn stop_data(self) -> &'a mut crate::W<REG> {
        self.variant(Trcmd::StopData)
    }
}
#[doc = "Transfer Direction"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trdir {
    #[doc = "0: Write."]
    Write = 0,
    #[doc = "1: Read."]
    Read = 1,
}
impl From<Trdir> for bool {
    #[inline(always)]
    fn from(variant: Trdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRDIR` writer - Transfer Direction"]
pub type TrdirW<'a, REG> = crate::BitWriter<'a, REG, Trdir>;
impl<'a, REG> TrdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write."]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Trdir::Write)
    }
    #[doc = "Read."]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Trdir::Read)
    }
}
#[doc = "Transfer Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trtyp {
    #[doc = "0: MMC/SD Card Single Block"]
    Single = 0,
    #[doc = "1: MMC/SD Card Multiple Block"]
    Multiple = 1,
    #[doc = "2: MMC Stream"]
    Stream = 2,
    #[doc = "4: SDIO Byte"]
    Byte = 4,
    #[doc = "5: SDIO Block"]
    Block = 5,
}
impl From<Trtyp> for u8 {
    #[inline(always)]
    fn from(variant: Trtyp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trtyp {
    type Ux = u8;
}
impl crate::IsEnum for Trtyp {}
#[doc = "Field `TRTYP` writer - Transfer Type"]
pub type TrtypW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trtyp>;
impl<'a, REG> TrtypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MMC/SD Card Single Block"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Trtyp::Single)
    }
    #[doc = "MMC/SD Card Multiple Block"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(Trtyp::Multiple)
    }
    #[doc = "MMC Stream"]
    #[inline(always)]
    pub fn stream(self) -> &'a mut crate::W<REG> {
        self.variant(Trtyp::Stream)
    }
    #[doc = "SDIO Byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Trtyp::Byte)
    }
    #[doc = "SDIO Block"]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(Trtyp::Block)
    }
}
#[doc = "SDIO Special Command"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iospcmd {
    #[doc = "0: Not an SDIO Special Command"]
    Std = 0,
    #[doc = "1: SDIO Suspend Command"]
    Suspend = 1,
    #[doc = "2: SDIO Resume Command"]
    Resume = 2,
}
impl From<Iospcmd> for u8 {
    #[inline(always)]
    fn from(variant: Iospcmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iospcmd {
    type Ux = u8;
}
impl crate::IsEnum for Iospcmd {}
#[doc = "Field `IOSPCMD` writer - SDIO Special Command"]
pub type IospcmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iospcmd>;
impl<'a, REG> IospcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not an SDIO Special Command"]
    #[inline(always)]
    pub fn std(self) -> &'a mut crate::W<REG> {
        self.variant(Iospcmd::Std)
    }
    #[doc = "SDIO Suspend Command"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(Iospcmd::Suspend)
    }
    #[doc = "SDIO Resume Command"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(Iospcmd::Resume)
    }
}
#[doc = "ATA with Command Completion Signal"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atacs {
    #[doc = "0: Normal operation mode."]
    Normal = 0,
    #[doc = "1: This bit indicates that a completion signal is expected within a programmed amount of time (HSMCI_CSTOR)."]
    Completion = 1,
}
impl From<Atacs> for bool {
    #[inline(always)]
    fn from(variant: Atacs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATACS` writer - ATA with Command Completion Signal"]
pub type AtacsW<'a, REG> = crate::BitWriter<'a, REG, Atacs>;
impl<'a, REG> AtacsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Atacs::Normal)
    }
    #[doc = "This bit indicates that a completion signal is expected within a programmed amount of time (HSMCI_CSTOR)."]
    #[inline(always)]
    pub fn completion(self) -> &'a mut crate::W<REG> {
        self.variant(Atacs::Completion)
    }
}
#[doc = "Field `BOOT_ACK` writer - Boot Operation Acknowledge"]
pub type BootAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:5 - Command Number"]
    #[inline(always)]
    #[must_use]
    pub fn cmdnb(&mut self) -> CmdnbW<CmdrSpec> {
        CmdnbW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Response Type"]
    #[inline(always)]
    #[must_use]
    pub fn rsptyp(&mut self) -> RsptypW<CmdrSpec> {
        RsptypW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Special Command"]
    #[inline(always)]
    #[must_use]
    pub fn spcmd(&mut self) -> SpcmdW<CmdrSpec> {
        SpcmdW::new(self, 8)
    }
    #[doc = "Bit 11 - Open Drain Command"]
    #[inline(always)]
    #[must_use]
    pub fn opdcmd(&mut self) -> OpdcmdW<CmdrSpec> {
        OpdcmdW::new(self, 11)
    }
    #[doc = "Bit 12 - Max Latency for Command to Response"]
    #[inline(always)]
    #[must_use]
    pub fn maxlat(&mut self) -> MaxlatW<CmdrSpec> {
        MaxlatW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Transfer Command"]
    #[inline(always)]
    #[must_use]
    pub fn trcmd(&mut self) -> TrcmdW<CmdrSpec> {
        TrcmdW::new(self, 16)
    }
    #[doc = "Bit 18 - Transfer Direction"]
    #[inline(always)]
    #[must_use]
    pub fn trdir(&mut self) -> TrdirW<CmdrSpec> {
        TrdirW::new(self, 18)
    }
    #[doc = "Bits 19:21 - Transfer Type"]
    #[inline(always)]
    #[must_use]
    pub fn trtyp(&mut self) -> TrtypW<CmdrSpec> {
        TrtypW::new(self, 19)
    }
    #[doc = "Bits 24:25 - SDIO Special Command"]
    #[inline(always)]
    #[must_use]
    pub fn iospcmd(&mut self) -> IospcmdW<CmdrSpec> {
        IospcmdW::new(self, 24)
    }
    #[doc = "Bit 26 - ATA with Command Completion Signal"]
    #[inline(always)]
    #[must_use]
    pub fn atacs(&mut self) -> AtacsW<CmdrSpec> {
        AtacsW::new(self, 26)
    }
    #[doc = "Bit 27 - Boot Operation Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack(&mut self) -> BootAckW<CmdrSpec> {
        BootAckW::new(self, 27)
    }
}
#[doc = "Command Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdrSpec;
impl crate::RegisterSpec for CmdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmdr::W`](W) writer structure"]
impl crate::Writable for CmdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
